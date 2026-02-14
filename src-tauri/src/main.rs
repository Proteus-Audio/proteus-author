#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod file;
mod menu;
mod helpers;
mod peaks;
mod player;
mod project;
mod windows;

use std::sync::{Arc, Mutex};

use file::*;
use menu::{
    set_follow_mode_menu, set_shuffle_point_tool_mode_menu, FollowModeState,
    ShufflePointToolModeState,
};
use player::*;
use project::*;
use proteus_lib::playback::player::Player;
use dotenv::dotenv;
use tauri::Manager;

fn main() {
    dotenv().ok();
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new().level(log::LevelFilter::Info).build())
        // .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_http::init())
        // .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_process::init())
        // .plugin(tauri_plugin_fs::init())
        .manage(project::create_project())
        .manage(Arc::new(Mutex::new(None::<Player>)))
        .manage(FollowModeState(Arc::new(Mutex::new(false))))
        .manage(ShufflePointToolModeState(Arc::new(Mutex::new(false))))
        .menu(|app_handle| menu::build_menu(app_handle))
        .on_menu_event(|app_handle, event| {
            let follow_mode_state: tauri::State<FollowModeState> = app_handle.state();
            let shuffle_point_tool_mode_state: tauri::State<ShufflePointToolModeState> =
                app_handle.state();
            menu::handle_menu_event(
                app_handle,
                event,
                follow_mode_state.inner(),
                shuffle_point_tool_mode_state.inner(),
            );
        })
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
            get_waveform_peaks,
            get_track_waveform_peaks,
            init_player,
            get_project_state,
            play,
            pause,
            stop,
            seek,
            shuffle,
            get_duration,
            get_position,
            get_possible_combinations,
            get_play_state,
            get_levels,
            get_levels_db,
            get_volume,
            set_selections,
            add_shuffle_point,
            remove_shuffle_point,
            set_volume,
            set_effects_chain,
            set_follow_mode_menu,
            set_shuffle_point_tool_mode_menu
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let _main_window = windows::create_main_window(&app.handle());
    // windows::create_docs_window(&handle);

    app.run(|_app_handle, event| match event {
        #[cfg(target_os = "macos")]
        tauri::RunEvent::ExitRequested { api, .. } => {
            println!("exit requested");
            api.prevent_exit();
        }
         #[cfg(any(target_os = "macos", target_os = "ios"))]
        tauri::RunEvent::Reopen {
            has_visible_windows,
            ..
        } => {
            if !has_visible_windows {
                if let Some(window) = _app_handle.get_webview_window("main-window-1") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let window = windows::create_main_window(&_app_handle);
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }

         #[cfg(any(target_os = "macos", target_os = "ios"))]
        tauri::RunEvent::Opened { urls, .. } => {
            println!("opened: {:?}", urls);
        }
        _ => {}
    });

    println!("Hello, world!");
}
