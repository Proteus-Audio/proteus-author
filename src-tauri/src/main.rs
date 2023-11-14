#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod menu;
mod windows;
mod file;
mod project;

use file::*;
use project::*;

fn main() {
    let app = tauri::Builder::default()
        .manage(project::create_project())
        .invoke_handler(tauri::generate_handler![project_changes, auto_save, save_file, save_file_as, check_status])
        .on_menu_event(move |event| match event.menu_item_id() {
            "new_window" => {
                // let application = tauri::Builder::default().build(t)
                println!("NEW WINDOW!");
            }
            _ => {}
        })
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