use std::collections::HashMap;
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

use crate::project::PlaySettings;
use crate::project::ProjectSkeleton;
use crate::project::SettingsEncoder;
use crate::project::SettingsTrack;
use crate::project::PROJECT;
use crate::project::UNSAVED_CHANGES;
use crate::windows;

#[tauri::command]
pub fn project_changes(new_project: ProjectSkeleton, window: Window) -> String {
    let project = PROJECT.lock().unwrap();
    let project_json = serde_json::to_string(&*project).unwrap();
    let new_project_json = serde_json::to_string(&new_project).unwrap();

    let file_name = project.name.clone().unwrap_or("Untitled".to_string());

    if project_json != new_project_json {
        UNSAVED_CHANGES.store(true, std::sync::atomic::Ordering::Relaxed);
        window.set_title(&format!("{}*", file_name).as_str()).unwrap();
        "Unsaved Changes".to_string()
    } else {
        UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
        window.set_title(&format!("{}", file_name)).unwrap();
        "Saved".to_string()
    }
}

#[tauri::command]
pub fn auto_save(new_project: ProjectSkeleton) {
    println!("Auto Saving");
    let mut project = PROJECT.lock().unwrap();
    println!("Project: {:?}", project);
    project.tracks = new_project.tracks.clone();
    println!("Project: {:?}", project);
    project.effects = new_project.effects.clone();
    println!("Project: {:?}", project);
    drop(project);
}

#[tauri::command]
pub fn save_file(new_project: ProjectSkeleton, window: Window) {
    let project = PROJECT.lock().unwrap();

    if UNSAVED_CHANGES.load(std::sync::atomic::Ordering::Relaxed) == false {
        auto_save(new_project.clone());
        println!("No changes to save");
        if !project.location.is_none() {
            return;
        }
    }
    println!("Window: {:?}", window);
    println!("Saving File");

    if project.location.is_none() {
        println!("No file location, saving as");
        drop(project);
        save_file_as(new_project, window);
        return;
    }

    println!("File location found, saving from json");
    let project_json = serde_json::to_string(&*project).unwrap();

    let mut file = File::create(project.location.clone().unwrap()).unwrap();
    file.write_all(project_json.as_bytes()).unwrap();

    let file_name = project.name.clone().unwrap_or("Untitled".to_string());
    UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
    window.set_title(&format!("{}", file_name)).unwrap();

    drop(project);

    println!("File Saved");
}

#[tauri::command]
pub fn save_file_as(new_project: ProjectSkeleton, window: Window) {
    println!("Saving File To New Object");
    let file_name = new_project.name.clone().unwrap_or("untitled".to_string()) + ".protproject";
    auto_save(new_project);
    println!("Saving File As");
    let save_dialog = dialog::FileDialogBuilder::new()
        .add_filter("Proteus Author Project", &["protproject"])
        .set_title("Save Project")
        .set_file_name(&file_name.as_str());

    println!("Getting Window Handle");
    let handle = window.app_handle();
    println!("Getting Window Label");
    let window_label = String::from(window.label());
    
    println!("Opening Save Dialog");
    save_dialog.save_file(move |file_path| {
        if file_path.is_none() {
            println!("No file selected");
            ()
        }

        println!("File Path: {:?}", file_path);

        let path_buff = file_path.unwrap();
        let file_name = String::from(path_buff.file_name().unwrap().to_str().unwrap()).replace(".protproject", "");

        println!("File Name: {:?}", file_name);

        let mut project = PROJECT.lock().unwrap();

        project.name = Some(file_name.clone());
        project.location = Some(path_buff.to_str().unwrap().to_string());

        let project_json = serde_json::to_string(&*project).unwrap();

        let mut file = File::create(path_buff).unwrap();
        file.write_all(project_json.as_bytes()).unwrap();

        UNSAVED_CHANGES.store(false, std::sync::atomic::Ordering::Relaxed);
        let window = handle.get_window(&window_label).unwrap();
        window.set_title(&file_name).unwrap();

        drop(project);
    });
}

// #[tauri::command]
// pub async fn load_file(handle: &AppHandle) -> Result<String, String> {
//     let open_dialog = dialog::blocking::FileDialogBuilder::new();

//     println!("Opening File Dialog");

//     let file_path = open_dialog.pick_file();

//     if file_path.is_none() {
//         println!("No file selected");
//         return Err("No file selected".to_string());
//     }

//     // If file extension is not .protproject, return error
//     let path_buff = file_path.unwrap();

//     if path_buff.extension().unwrap() != "protproject" {
//         println!("File extension is not .protproject");
//         return Err("File extension is not .protproject".to_string());
//     }

//     let file_name = path_buff.file_name().unwrap().to_str().unwrap();
//     let project_location = path_buff.to_str().unwrap().to_string();

//     let file_contents = std::fs::read_to_string(path_buff.clone()).unwrap();
//     let project_result: Result<ProjectSkeleton, serde_json::Error> =
//         serde_json::from_str(&file_contents);

//     let mut project = PROJECT.lock().unwrap();

//     match project_result {
//         Ok(new_project) => {
//             project.name = Some(file_name.to_string());
//             project.location = Some(project_location.to_string());
//             project.tracks = new_project.tracks.clone();
//             project.effects = new_project.effects.clone();
//         }
//         Err(e) => {
//             println!("Error: {:?}", e);
//         }
//     }

//     handle.emit_all("FILE_LOADED", project.clone());

//     drop(project);

//     Ok("File Loaded".to_string())
// }

#[tauri::command]
pub fn load_file(handle: &AppHandle, label: &String) {
    let load_dialog = dialog::FileDialogBuilder::new().add_filter("Proteus Project", &["protproject"]);

    let window = handle.get_window(label);

    load_dialog.pick_file(|file_path| {
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

        let mut project = PROJECT.lock().unwrap();

        match project_result {
            Ok(new_project) => {
                project.name = Some(file_name.to_string());
                project.location = Some(project_location.to_string());
                project.tracks = new_project.tracks.clone();
                project.effects = new_project.effects.clone();
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        match window {
            Some(window) => {
                let file_name = project.name.clone().unwrap();
                window.set_title(&file_name.as_str()).unwrap();
                window.emit("FILE_LOADED", project.clone())
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

pub fn export_prot(handle: &AppHandle) {
    let project = PROJECT.lock().unwrap();
    let file_name = project.name.clone().unwrap_or("export".to_string()) + ".prot";
    drop(project);
    let save_dialog = dialog::FileDialogBuilder::new()
            .add_filter("Proteus Audio", &["prot"])
            .set_title("Save Project")
            .set_file_name(file_name.as_str());

    let handle = handle.clone();
    handle.emit_all("EXPORTING", true).unwrap();

    save_dialog.save_file(move |file_path| {
        if file_path.is_none() {
            println!("No file selected");
            handle.emit_all("EXPORTING", false).unwrap();
            ()
        }
        // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
        let mut reduced_file_list = Vec::new();

        let project = PROJECT.lock().unwrap();

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

            for file in &track.files {
                // If filepath is already in reduced_file_list, skip
                if !reduced_file_list.contains(&file.path) {
                    reduced_file_list.push(file.path.clone());
                }

                // Get index of filepath in reduced_file_list
                let index = reduced_file_list.iter().position(|r| r == &file.path).unwrap();
                settings_track.ids.push((index + 1) as u32);
            }

            play_settings.tracks.push(settings_track);
        };

        let mut input_list = String::new();
        let mut map_list = String::new();
        let mut metadata_list = String::new();

        for (index, file) in reduced_file_list.iter().enumerate() {
            input_list.push_str(&format!("-i {} ", file));
            map_list.push_str(&format!("-map {} ", index));
            metadata_list.push_str(&format!("-metadata:s:a:{} title=\"{}\" ", index, file));
        }

        let settings_encoder = SettingsEncoder {
            play_settings: play_settings.clone(),
            encoder_version: 1.0,
        };

        let json_settings = serde_json::to_string(&settings_encoder).unwrap();

        let output_dir = file_path.clone().unwrap().parent().unwrap().to_str().unwrap().to_string();
        let settings_file_path = format!("{}/play_settings.json", output_dir);
        let mut settings_file = File::create(settings_file_path.clone()).unwrap();
        settings_file.write_all(json_settings.as_bytes()).unwrap();

        // Replace extension .prot with .mka
        // TODO: Replace with regex
        let output_file = file_path.clone().unwrap().to_str().unwrap().to_string().replace(".prot", ".mka");

        let out_command = format!(
            "-y {}{}{}{}{}",
            input_list,
            map_list,
            format!("-attach {} -metadata:s:t:0 mimetype=application/json ", settings_file_path),
            metadata_list, 
            output_file
        );

        println!("{}", out_command);

        let (mut rx, mut child) = Command::new_sidecar("ffmpeg")
        .expect("failed to create `my-sidecar` binary command")
        .args(out_command.split(" "))
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

            handle.emit_all("EXPORTING", false).unwrap();
        });
    });
}
