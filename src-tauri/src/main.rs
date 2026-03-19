#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod alerts;
mod effects;
mod file;
mod menu;
mod peaks;
mod player;
mod project;
mod startup;
mod windows;

use std::sync::{Arc, Mutex};
use std::{env, path::PathBuf};

use alerts::*;
use dotenv::dotenv;
use file::*;
use menu::{
    set_follow_mode_menu, set_shuffle_point_tool_mode_menu, FollowModeState,
    ShufflePointToolModeState,
};
use player::*;
use project::*;
use startup::*;
use tauri::{Manager, RunEvent, WindowEvent};

fn main() {
    dotenv().ok();
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        // .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_http::init())
        // .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_process::init())
        // .plugin(tauri_plugin_fs::init())
        .manage(project::create_project_state())
        .manage(player::create_player_actor_state())
        .manage(project::create_unsaved_state())
        .manage(project::create_saved_snapshot_state())
        .manage(startup::create_startup_trace_state())
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
            get_missing_project_files,
            locate_project_file,
            apply_found_files,
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
            set_track_mix,
            set_effects_chain,
            startup_trace,
            set_follow_mode_menu,
            set_shuffle_point_tool_mode_menu,
            alert_current_window,
            alert_all_windows
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let startup_trace_state: tauri::State<StartupTraceState> = app.state();
    log_rust(&startup_trace_state, "app", "tauri builder initialized");

    let main_window = windows::create_main_window(&app.handle());
    log_rust(&startup_trace_state, "app", "main window requested");
    if let Some(project_path) = env::args().skip(1).find_map(|arg| {
        let path = PathBuf::from(arg);
        if is_project_file_path(&path) {
            Some(path)
        } else {
            None
        }
    }) {
        open_project_file_at_path(&main_window, project_path);
    }
    // windows::create_docs_window(&handle);

    app.run(|_app_handle, event| match event {
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::Destroyed,
            ..
        } => {
            let project_state: tauri::State<WindowProjectState> = _app_handle.state();
            let unsaved_state: tauri::State<WindowUnsavedState> = _app_handle.state();
            let saved_snapshot_state: tauri::State<WindowSavedSnapshotState> = _app_handle.state();
            clear_window_state_by_label(
                &label,
                &project_state,
                &unsaved_state,
                &saved_snapshot_state,
            );
            let player_state: tauri::State<PlayerActorState> = _app_handle.state();
            clear_window_player_by_label(&label, &player_state);
        }
        #[cfg(target_os = "macos")]
        RunEvent::ExitRequested { api, .. } => {
            println!("exit requested");
            api.prevent_exit();
        }
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        RunEvent::Reopen {
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
        RunEvent::Opened { urls, .. } => {
            println!("opened: {:?}", urls);
            let window = windows::get_or_create_main_window(&_app_handle);
            let project_path = urls.into_iter().find_map(|url| {
                url.to_file_path().ok().and_then(|path| {
                    if is_project_file_path(&path) {
                        Some(path)
                    } else {
                        None
                    }
                })
            });

            if let Some(path) = project_path {
                open_project_file_at_path_and_emit(&window, path);
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    });

    println!("Hello, world!");
}
