#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod file;
mod menu;
mod project;
mod windows;
mod peaks;
mod player;

use std::sync::{Arc, Mutex};

use file::*;
use project::*;
use player::*;
use proteus_audio::player::Player;

fn main() {
    let app = tauri::Builder::default()
        .manage(project::create_project())
        .manage(Arc::new(Mutex::new(None::<Player>)))
        .invoke_handler(tauri::generate_handler![
            project_changes,
            auto_save,
            save_file,
            save_file_as,
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
        .menu(menu::get_menu())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let main_window = windows::create_main_window(&app);
    // windows::create_docs_window(&handle);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });

    println!("Hello, world!");
}
