use tauri::{App, AppHandle, LogicalPosition, LogicalSize, Position, Size, Window, WindowBuilder, WindowUrl};

pub fn create_main_window(app: &App) {
    create_window(app, 1)
}

pub fn create_window(app: &App, count:i32) {
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
    println!("COME ON");

    window.on_menu_event(move |event| match event.menu_item_id() {
        "save" => {
            // &window.emit("window_commands", "SAVE");
            println!("SAVED! {}", count);
        }
        "save_as" => {
            println!("SAVED AS! {}", count);
        }
        "load" => {
            println!("LOAD! {}", count);
            // create_window(&app);
        }
        _ => {}
    });
}

pub fn create_docs_window(app: &AppHandle) {
    let window = WindowBuilder::new(
        app,
        "label",
        WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}
