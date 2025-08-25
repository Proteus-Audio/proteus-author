#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod file;
// mod menu;
mod peaks;
mod player;
mod project;
mod windows;

use std::sync::{Arc, Mutex};

use file::*;
use player::*;
use project::*;
use proteus_audio::player::Player;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .manage(project::create_project())
        .manage(Arc::new(Mutex::new(None::<Player>)))
        .invoke_handler(tauri::generate_handler![
            project_changes,
            auto_save,
            save_file,
            save_file_as,
            open_file,
            load_empty_project,
            check_status,
            export_prot,
            get_peaks,
            register_file,
            get_simplified_peaks,
            init_player,
            get_project_state,
            play,
            pause,
            stop,
            seek,
            shuffle,
            get_duration,
            get_position,
            get_play_state,
            set_selections,
            set_volume
        ])
        // .menu(menu::get_menu())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let main_window = windows::create_main_window(&app.handle());
    // windows::create_docs_window(&handle);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            println!("exit requested");
            api.prevent_exit();
        }
        tauri::RunEvent::Reopen { has_visible_windows, .. } => {
            if !has_visible_windows {
                windows::create_main_window(&_app_handle);
            }
        }
        tauri::RunEvent::Opened { urls, .. } => {
            println!("opened: {:?}", urls);
        }
        _ => {}
    });

    println!("Hello, world!");
}
