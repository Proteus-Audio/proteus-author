#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod menu;
mod windows;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_file(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, open_file])
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


    let handle = app.handle();
    windows::create_main_window(&app);
    // windows::create_docs_window(&handle);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn main_playground() {
    // let app = tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet, open_file])
    //     .on_menu_event(move |event| match event.menu_item_id() {
    //         "new_window" => {
    //             // let application = tauri::Builder::default().build(t)
    //             println!("NEW WINDOW!");
    //         }
    //         _ => {}
    //     })
    //     .menu(menu::get_menu())
    //     .build(tauri::generate_context!())
    //     .expect("error while running tauri application");

    // let handle = app.handle();
    // windows::create_main_window(&app);
    // // windows::create_docs_window(&handle);
    // let open_window = tauri::Manager::get_window(&app, "main-window-0").unwrap();
    // println!("Open Window Size: {:?}", open_window.inner_size());

    // app.run(|_app_handle, event| match event {
    //     tauri::RunEvent::ExitRequested { api, .. } => {
    //         api.prevent_exit();
    //     }
    //     _ => {}
    // });
}
