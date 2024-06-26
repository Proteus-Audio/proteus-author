use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::api::dialog;
use tauri::api::process::Command;
use tauri::api::process::CommandEvent;
use tauri::AppHandle;
use tauri::Manager;
use tauri::State;
use tauri::Window;
use regex::Regex;

use crate::project::*;
use crate::peaks::*;

fn split_arguments(string: &str) -> Vec<&str> {
    let re = Regex::new(r#"[^"\s]*("[^"]*)"|([^"\s]+)"#).unwrap();
    re.find_iter(string)
        .map(|m| {
            let string = m.as_str();
            let first = string.chars().next().unwrap();
            let last = string.chars().last().unwrap();
            let quote = "\"".chars().next().unwrap();
            if first == quote && last == quote {
                &string[1..string.len() - 1]
            } else {
                string
            }
        })
        .collect()
}



#[tauri::command]
pub fn push_file_id(track_id: u32, file_id: String, project_state: State<Arc<Mutex<ProjectSkeleton>>>) {
    let mut project = project_state.lock().unwrap();

    let track = project.tracks.iter_mut().find(|t| t.id == track_id);

    match track {
        Some(track) => {
            track.file_ids.push(file_id);
        }
        None => {
            // Create new track
            let track = TrackSkeleton {
                id: track_id,
                name: "".to_string(),
                selection: Some(file_id.clone()),
                file_ids: vec![file_id],
            };

            project.tracks.push(track);
        }
    }
}

#[tauri::command]
pub async fn register_file(file_path: &str, track_id: u32, window: Window) -> Result<FileInfoSkeleton, String> {
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();

    let project = project_state.lock().unwrap();

    let project_clone = project.clone();
    drop(project);
    
    // See if file is already registered
    let mut found_file = project_clone.files.iter().find_map(|file| {
        if file.path == file_path {
            Some(file)
        } else {
            None
        }
    }).clone();


    // If file is not registered, register it
    if found_file.is_none() {
        // let peaks = proteus_audio::peaks::get_peaks(file_path, true);

        let path = std::path::Path::new(file_path);

        let extention = match path.extension().unwrap().to_str() {
            Some(ext) => Some(ext.to_string()),
            None => None,
        };

        let file = FileInfo {
            id: uuid::Uuid::new_v4().to_string(),
            path: file_path.to_string(),
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            extension: extention,
            // peaks: Some(peaks),
        };

        let mut project = project_state.lock().unwrap();
        project.files.push(file.clone());
        drop(project);

        push_file_id(track_id, file.id.clone(), project_state.clone());

        let peaks_start = std::time::Instant::now();
        let _peaks = get_json_peaks(&window, &file.id, None);
        println!("Peaks took {}ms", peaks_start.elapsed().as_millis());
    }

    let project = project_state.lock().unwrap();
    found_file = project.files.last();

    let file_unwraped= found_file.unwrap().clone();

    Ok(FileInfoSkeleton {
        id: file_unwraped.id.clone(),
        path: file_unwraped.path.clone(),
        name: file_unwraped.name.clone(),
        extension: file_unwraped.extension.clone(),
    })
}

#[tauri::command]
pub async fn get_simplified_peaks(file_id: String, zoom: usize, window: Window) -> Vec<SimplifiedPeaks> {
    let timer = std::time::Instant::now();
    let peaks = get_json_peaks(&window, &file_id, None);
    let simplified_peaks = simplify_peaks(peaks, zoom);
    println!("Simplifying peaks took {}ms", timer.elapsed().as_millis());

    simplified_peaks
}

#[tauri::command]
pub fn get_peaks(file_path: &str) -> Vec<Vec<(f32, f32)>> {
    // let project = PROJECT.lock().unwrap();

    return proteus_audio::peaks::get_peaks(file_path, true);
}

#[tauri::command]
pub fn project_changes(new_project: ProjectSkeleton, window: Window, project_state: State<Arc<Mutex<ProjectSkeleton>>>) -> String {
    println!("new_project: {:?}", new_project);
    let project = project_state.lock().unwrap();
    let project_json = serde_json::to_string(&*project).unwrap();
    let new_project_json = serde_json::to_string(&new_project).unwrap();

    let file_name = project.name.clone().unwrap_or("Untitled".to_string());

    if project_json != new_project_json {
        UNSAVED_CHANGES.store(true, std::sync::atomic::Ordering::Relaxed);
        window
            .set_title(&format!("{}*", file_name).as_str())
            .unwrap();
        "Unsaved Changes".to_string()
    } else {
        UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
        window.set_title(&format!("{}", file_name)).unwrap();
        "Saved".to_string()
    }
}

#[tauri::command]
pub fn auto_save(new_project: ProjectSkeleton, project_state: State<Arc<Mutex<ProjectSkeleton>>>) {
    println!("Auto Saving");
    let mut project = project_state.lock().unwrap();
    println!("Project: {:?}", project);
    project.tracks = new_project.tracks.clone();
    println!("Project: {:?}", project);
    project.effects = new_project.effects.clone();
    println!("Project: {:?}", project);
    drop(project);
}

#[tauri::command]
pub async fn save_file(window: Window) -> Option<ProjectSkeleton> {
    // auto_save(new_project.clone());
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();

    let project_state_clone = project_state.clone();
    if UNSAVED_CHANGES.load(std::sync::atomic::Ordering::Relaxed) == false {
        let project = project_state_clone.lock().unwrap();
        println!("No changes to save");
        if !project.location.is_none() {
            return None;
        }
        drop(project);
    }

    let project_already_saved = project_state.lock().unwrap().location.is_some();
    // let project_already_saved = new_project.location.is_some();
    // drop(project);

    if !project_already_saved {
        return save_file_as(window).await;
    }

    let project = project_state.lock().unwrap();
    let project_json = serde_json::to_string(&*project).unwrap();

    let mut file = File::create(project.location.clone().unwrap()).unwrap();
    file.write_all(project_json.as_bytes()).unwrap();

    let file_name = project.name.clone().unwrap_or("Untitled".to_string());
    UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
    window.set_title(&format!("{}", file_name)).unwrap();

    Some(project.clone())
}

#[tauri::command]
pub async fn save_file_as(window: Window) -> Option<ProjectSkeleton> {
    let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window.state();
    let project = project_state.lock().unwrap();
    let file_name = project.name.clone().unwrap_or("untitled".to_string());
    drop(project);

    // let file_name = new_project.name.clone().unwrap_or("untitled".to_string()) + ".protproject";
    // auto_save(new_project);
    let save_dialog = dialog::blocking::FileDialogBuilder::new()
        .add_filter("Proteus Author Project", &["protproject"])
        .set_title("Save Project")
        .set_file_name(&file_name.as_str());

    let handle = window.app_handle();
    let window_label = String::from(window.label());

    let file_path = save_dialog.save_file();

    if file_path.is_none() {
        println!("No file selected");
        return None;
    }

    let path_buff = file_path.unwrap();
    let file_name =
        String::from(path_buff.file_name().unwrap().to_str().unwrap()).replace(".protproject", "");

    let mut project = project_state.lock().unwrap();

    project.name = Some(file_name.clone());
    project.location = Some(path_buff.to_str().unwrap().to_string());

    println!("Project: {:?}", serde_json::to_string(&project.clone()));

    let project_json = serde_json::to_string(&*project).unwrap();

    let mut file = File::create(path_buff).unwrap();
    file.write_all(project_json.as_bytes()).unwrap();

    UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
    let window = handle.get_window(&window_label).unwrap();
    window.set_title(&file_name).unwrap();

    Some(project.clone())
}

#[tauri::command]
pub fn load_file(handle: &AppHandle, label: &String) {
    let load_dialog =
        dialog::FileDialogBuilder::new().add_filter("Proteus Project", &["protproject"]);

    // let project_state: State<Arc<Mutex<ProjectSkeleton>>> = handle.state();
    // let project_state_clone = project_state.clone();
    
    let window = handle.get_window(label);
    
    load_dialog.pick_file(|file_path| {
        let window_clone = window.clone().unwrap();
        let project_state: State<Arc<Mutex<ProjectSkeleton>>> = window_clone.state();
        // let window = Window::get_window(&self, "main-window-1");

        if file_path.is_none() {
            println!("No file selected");
            ()
        }

        // If file extension is not .protproject, return error
        let path_buff = file_path.unwrap();

        if path_buff.extension().unwrap() != "protproject" {
            println!("File extension is not .protproject");
            ()
        }

        let file_name = path_buff.file_name().unwrap().to_str().unwrap();
        let project_location = path_buff.to_str().unwrap().to_string();

        let file_contents = std::fs::read_to_string(path_buff.clone()).unwrap();
        let project_result: Result<ProjectSkeleton, serde_json::Error> =
            serde_json::from_str(&file_contents);

        let mut project = project_state.lock().unwrap();

        match project_result {
            Ok(new_project) => {
                project.name = Some(file_name.to_string());
                project.location = Some(project_location.to_string());
                project.tracks = new_project.tracks.clone();
                project.effects = new_project.effects.clone();
                project.files = new_project.files.clone();
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        match window {
            Some(window) => {
                let file_name = project.name.clone().unwrap();
                window.set_title(&file_name.as_str()).unwrap();
                window
                    .emit("FILE_LOADED", project.clone())
                    .expect("failed to emit event");
            }
            None => {
                println!("Window not found");
            }
        }

        drop(project);
        ()
    });
}

pub fn load_empty_project(handle: &AppHandle) {
    let empty_project = Arc::new(Mutex::new(empty_project()));
    let project = empty_project.lock().unwrap();

    let window = handle.get_window(&"main-window-1".to_string()).unwrap();
    window
        .set_title(
            &project
                .name
                .clone()
                .unwrap_or("untitled".to_string())
                .as_str(),
        )
        .unwrap();
    window
        .emit("FILE_LOADED", project.clone())
        .expect("failed to emit event");
    drop(project);
}

#[tauri::command]
pub fn export_prot(project_state: State<Arc<Mutex<ProjectSkeleton>>>, window: Window) {
    let project = project_state.lock().unwrap().clone();
    let file_name = project.name.clone().unwrap_or("export".to_string()) + ".prot";
    let save_dialog = dialog::FileDialogBuilder::new()
        .add_filter("Proteus Audio", &["prot"])
        .set_title("Save Project")
        .set_file_name(file_name.as_str());

    let handle = window.app_handle().clone();

    save_dialog.save_file(move |file_path| {
        if file_path.is_none() {
            println!("No file selected");
            handle.emit_all("EXPORTING", "Cancelled").unwrap();
            ()
        }

        let file_path = file_path.clone().unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        handle
            .emit_all("EXPORTING", format!("Exporting {}", file_name))
            .unwrap();
        // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
        let mut reduced_file_list = Vec::new();

        let mut play_settings = PlaySettings {
            effects: project.effects.clone(),
            tracks: Vec::new(),
        };

        for track in project.tracks.iter() {
            let mut settings_track = SettingsTrack {
                level: 1.0,
                pan: 0.0,
                ids: Vec::new(),
                name: track.name.clone(),
                safe_name: track.name.clone(),
            };

            for file_id in &track.file_ids {
                let file = project
                    .files
                    .iter()
                    .find(|f| f.id == *file_id)
                    .unwrap()
                    .clone();

                // If filepath is already in reduced_file_list, skip
                if !reduced_file_list.contains(&file.path) {
                    reduced_file_list.push(file.path.clone());
                }

                // Get index of filepath in reduced_file_list
                let index = reduced_file_list
                    .iter()
                    .position(|r| r == &file.path)
                    .unwrap();
                settings_track.ids.push((index + 1) as u32);
            }

            play_settings.tracks.push(settings_track);
        }

        let mut input_list = String::new();
        let mut map_list = String::new();
        let mut metadata_list = String::new();

        for (index, file) in reduced_file_list.iter().enumerate() {
            input_list.push_str(&format!("-i \"{}\" ", file));
            map_list.push_str(&format!("-map {} ", index));
            metadata_list.push_str(&format!("-metadata:s:a:{} title=\"{}\" ", index, file));
        }

        let settings_encoder = SettingsEncoder {
            play_settings: play_settings.clone(),
            encoder_version: 1.0,
        };

        let json_settings = serde_json::to_string(&settings_encoder).unwrap();

        let output_dir = file_path
            .clone()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let settings_file_path = format!("{}/play_settings.json", output_dir);
        let mut settings_file = File::create(settings_file_path.clone()).unwrap();
        settings_file.write_all(json_settings.as_bytes()).unwrap();

        // Replace extension .prot with .mka
        // TODO: Replace with regex
        let output_file = file_path
            .clone()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".prot", ".mka");

        let out_command = format!(
            "-y {}{}{}{}\"{}\"",
            input_list,
            map_list,
            format!(
                "-attach \"{}\" -metadata:s:t:0 mimetype=application/json ",
                settings_file_path
            ),
            metadata_list,
            output_file
        );

        println!("{}", out_command);
        println!("{:?}", split_arguments(out_command.as_str()));

        let (mut rx, ..) = Command::new_sidecar("ffmpeg")
            .expect("failed to create `my-sidecar` binary command")
            .args(split_arguments(out_command.as_str()))
            .spawn()
            .expect("Failed to spawn sidecar");

        tauri::async_runtime::spawn(async move {
            // read events such as stdout
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        println!("Line: {}", line);
                    }
                    CommandEvent::Stderr(line) => {
                        println!("Error: {}", line);
                    }
                    CommandEvent::Terminated(exit_status) => {
                        println!("Exit: {:#?}", exit_status);
                    }
                    _ => {}
                }
            }

            // Remove settings file
            std::fs::remove_file(settings_file_path).unwrap();
            // Rename output file if it exists from .mka to .prot
            if std::path::Path::new(&output_file).exists() {
                std::fs::rename(output_file.clone(), output_file.replace(".mka", ".prot")).unwrap();
            }

            handle.emit_all("EXPORTING", "Export Finished").unwrap();
        });
    });
}
