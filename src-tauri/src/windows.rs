use tauri::{App, LogicalPosition, LogicalSize, Position, Size, Window, WindowBuilder, WindowUrl, Manager};
use crate::{file::*, project::{self, PROJECT}};

pub fn create_main_window(app: &App) -> Window {
    create_window(app, 1)
}

pub fn create_window(app: &App, count:i32) -> Window {
    let handle = app.handle();
    let window = WindowBuilder::new(
        &handle,
        format!("{}-{}", "main-window", count),
        WindowUrl::App("index.html".into()),
    )
    .title("Proteus Author")
    .min_inner_size(600.0, 600.0)
    .build()
    .unwrap();

    let current_monitor = Window::current_monitor(&window)
        .unwrap()
        .expect("error getting current monitor");
    let scale_size = current_monitor.scale_factor().clone();
    let monitor_size = current_monitor.size().clone().to_logical(scale_size) as LogicalSize<f64>;
    let width = vec![monitor_size.width - 150.0, 1240.0]
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .expect("Couldn't get width")
        .clone();
    let height = vec![monitor_size.height - 150.0, 775.0]
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .expect("Couldn't get height")
        .clone();
    let new_window_size = Size::from(LogicalSize::new(width, height));

    window.set_size(new_window_size).unwrap();
    let window_position = Position::new(LogicalPosition::new(
        (monitor_size.width - width) / 2.0,
        (monitor_size.height - height) / 2.0,
    ));

    window.set_position(window_position).unwrap();

    if count < 1 {create_window(app, count + 1);}

    println!(
        "M Height: {} M Width: {}",
        monitor_size.height, monitor_size.width
    );
    println!("Height: {} Width: {}", height, width);

    let label = String::from(window.label());

    window.on_menu_event(move |event| match event.menu_item_id() {
        "save" => {
            let window = handle.get_window(&label).unwrap();
            window
                .emit("SAVE_FILE", "")
                .expect("failed to emit event");
            // let new_handle = app.handle();
            // save_file();
        }
        "save_as" => {
            let window = handle.get_window(&label).unwrap();
            window
                .emit("SAVE_FILE_AS", "")
                .expect("failed to emit event");
            // save_file_as();
        }
        "load" => {
            load_file(&handle, &label);
        }
        "export_prot" => {
            println!("EXPORT!");
            export_prot(&handle);
        }
        _ => {}
    });

    window
}

// pub fn create_docs_window(app: &AppHandle) {
//     let window = WindowBuilder::new(
//         app,
//         "label",
//         WindowUrl::External("https://tauri.app/".parse().unwrap()),
//     )
//     .build()
//     .unwrap();
// }
