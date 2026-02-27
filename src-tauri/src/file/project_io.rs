use crate::file::utils::{
    canonical_project_json, ensure_saved_snapshot_baseline, get_saved_snapshot_by_label,
    set_saved_snapshot_by_label, update_window_title,
};
use crate::project::{
    empty_project, get_unsaved, read_project, set_project, set_project_by_label, set_unsaved,
    set_unsaved_by_label, ProjectSkeleton, WindowProjectState, WindowSavedSnapshotState,
    WindowUnsavedState,
};
use std::fs::File;
use std::io::prelude::*;
use tauri::Emitter;
use tauri::Manager;
use tauri::State;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn project_changes(
    new_project: ProjectSkeleton,
    window: Window,
    project_state: State<WindowProjectState>,
    unsaved_state: State<WindowUnsavedState>,
    saved_snapshot_state: State<WindowSavedSnapshotState>,
) -> String {
    let label = window.label().to_string();
    ensure_saved_snapshot_baseline(&label, &project_state, &saved_snapshot_state);

    crate::project::with_project_mut(&window, &project_state, |project| {
        project.name = new_project.name.clone();
        project.tracks = new_project.tracks.clone();
        project.effects = new_project.effects.clone();
    });

    let project = read_project(&window, &project_state);
    let current_snapshot = canonical_project_json(&project);
    let saved_snapshot = get_saved_snapshot_by_label(&label, &saved_snapshot_state)
        .unwrap_or_else(|| current_snapshot.clone());
    let is_unsaved = current_snapshot != saved_snapshot;

    set_unsaved(&window, &unsaved_state, is_unsaved);
    update_window_title(&window, &project, is_unsaved);

    if is_unsaved {
        "Unsaved Changes".to_string()
    } else {
        "Saved".to_string()
    }
}

#[tauri::command]
pub fn auto_save(
    new_project: ProjectSkeleton,
    window: Window,
    project_state: State<WindowProjectState>,
) {
    println!("Auto Saving");
    crate::project::with_project_mut(&window, &project_state, |project| {
        project.tracks = new_project.tracks.clone();
        project.effects = new_project.effects.clone();
    });
}

#[tauri::command]
pub async fn save_file(window: Window) -> Option<ProjectSkeleton> {
    let project_state: State<WindowProjectState> = window.state();
    let unsaved_state: State<WindowUnsavedState> = window.state();
    let saved_snapshot_state: State<WindowSavedSnapshotState> = window.state();

    if !get_unsaved(&window, &unsaved_state) {
        let project = read_project(&window, &project_state);
        println!("No changes to save");
        if project.location.is_some() {
            return Some(project);
        }
    }

    let project_already_saved = read_project(&window, &project_state).location.is_some();

    if !project_already_saved {
        return save_file_as(window).await;
    }

    let project = read_project(&window, &project_state);
    let project_json = match serde_json::to_string(&project) {
        Ok(json) => json,
        Err(err) => {
            println!("Failed to serialize project: {:?}", err);
            return None;
        }
    };

    let Some(location) = project.location.clone() else {
        println!("Project location missing during save");
        return None;
    };

    let mut file = match File::create(location) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to create project file: {:?}", err);
            return None;
        }
    };
    if let Err(err) = file.write_all(project_json.as_bytes()) {
        println!("Failed to write project file: {:?}", err);
        return None;
    }

    let label = window.label().to_string();
    set_saved_snapshot_by_label(
        &label,
        &saved_snapshot_state,
        canonical_project_json(&project),
    );
    set_unsaved(&window, &unsaved_state, false);
    update_window_title(&window, &project, false);

    Some(project.clone())
}

#[tauri::command]
pub async fn save_file_as(window: Window) -> Option<ProjectSkeleton> {
    let project_state: State<WindowProjectState> = window.state();
    let unsaved_state: State<WindowUnsavedState> = window.state();
    let saved_snapshot_state: State<WindowSavedSnapshotState> = window.state();
    let project = read_project(&window, &project_state);
    let file_name = project.name.clone().unwrap_or("untitled".to_string());

    let file_path = window
        .dialog()
        .file()
        .add_filter("Proteus Author Project", &["protproject"])
        .set_title("Save Project")
        .set_file_name(file_name.as_str())
        .blocking_save_file();

    if file_path.is_none() {
        println!("No file selected");
        return None;
    }

    let path_buff = match file_path.unwrap().into_path() {
        Ok(path) => path,
        Err(err) => {
            println!("Invalid file path: {:?}", err);
            return None;
        }
    };
    let file_name =
        String::from(path_buff.file_name().unwrap().to_str().unwrap()).replace(".protproject", "");

    let mut project = read_project(&window, &project_state);
    project.name = Some(file_name.clone());
    project.location = Some(path_buff.to_str().unwrap().to_string());
    let project_json = match serde_json::to_string(&project) {
        Ok(json) => json,
        Err(err) => {
            println!("Failed to serialize project: {:?}", err);
            return None;
        }
    };
    set_project(&window, &project_state, project.clone());

    let mut file = match File::create(&path_buff) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to create project file: {:?}", err);
            return None;
        }
    };
    if let Err(err) = file.write_all(project_json.as_bytes()) {
        println!("Failed to write project file: {:?}", err);
        return None;
    }

    let label = window.label().to_string();
    set_saved_snapshot_by_label(
        &label,
        &saved_snapshot_state,
        canonical_project_json(&project),
    );
    set_unsaved(&window, &unsaved_state, false);

    update_window_title(&window, &project, false);

    Some(project.clone())
}

#[tauri::command]
pub async fn open_file(window: Window) {
    println!("Window: {:?}", window);
    let app = window.app_handle().clone();
    let label = window.label().to_string();
    window
        .dialog()
        .file()
        .add_filter("Proteus Project", &["protproject"])
        .pick_file(move |file| {
            let Some(window) = app.get_webview_window(&label) else {
                return;
            };
            if file.is_none() {
                println!("No file selected");
                return;
            }

            let file_path = file.unwrap();
            let path_buff = match file_path.into_path() {
                Ok(path) => path,
                Err(err) => {
                    println!("Invalid file path: {:?}", err);
                    return;
                }
            };
            if path_buff.extension().unwrap() != "protproject" {
                println!("File extension is not .protproject");
                ()
            }

            let file_name = path_buff.file_name().unwrap().to_str().unwrap();
            let project_location = path_buff.to_str().unwrap().to_string();

            let file_contents = std::fs::read_to_string(path_buff.clone()).unwrap();
            let project_result: Result<ProjectSkeleton, serde_json::Error> =
                serde_json::from_str(&file_contents);
            let project_state: State<WindowProjectState> = app.state();
            let unsaved_state: State<WindowUnsavedState> = app.state();
            let saved_snapshot_state: State<WindowSavedSnapshotState> = app.state();

            match project_result {
                Ok(mut new_project) => {
                    let project_name = file_name.to_string().replace(".protproject", "");
                    new_project.name = Some(project_name);
                    new_project.location = Some(project_location.to_string());
                    set_project_by_label(&label, &project_state, new_project.clone());
                    set_unsaved_by_label(&label, &unsaved_state, false);
                    set_saved_snapshot_by_label(
                        &label,
                        &saved_snapshot_state,
                        canonical_project_json(&new_project),
                    );
                    let title = new_project.name.clone().unwrap_or("Untitled".to_string());
                    if let Err(err) = window.set_title(&title) {
                        println!("Failed to set title: {:?}", err);
                    }
                    app.emit_to(label.as_str(), "FILE_LOADED", new_project)
                        .expect("failed to emit event");
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });
}

#[tauri::command]
pub async fn load_empty_project(window: Window) {
    let project_state: State<WindowProjectState> = window.state();
    let unsaved_state: State<WindowUnsavedState> = window.state();
    let saved_snapshot_state: State<WindowSavedSnapshotState> = window.state();
    let project = empty_project();
    set_project(&window, &project_state, project.clone());
    let label = window.label().to_string();
    set_saved_snapshot_by_label(
        &label,
        &saved_snapshot_state,
        canonical_project_json(&project),
    );
    set_unsaved(&window, &unsaved_state, false);
    update_window_title(&window, &project, false);
    window
        .app_handle()
        .emit_to(window.label(), "FILE_LOADED", project.clone())
        .expect("failed to emit event");
}
