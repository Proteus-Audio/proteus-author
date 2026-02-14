use tauri::{
    AppHandle, LogicalPosition, TitleBarStyle, WebviewWindow, WebviewWindowBuilder,
};

pub fn create_main_window(app_handle: &AppHandle) -> WebviewWindow {
    create_window(app_handle, 1)
}

pub fn create_window(app_handle: &AppHandle, count: i32) -> WebviewWindow {
    let (width, height, position) = compute_main_window_geometry(app_handle);

    let win_builder = WebviewWindowBuilder::new(
        app_handle,
        format!("{}-{}", "main-window", count),
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Proteus Author")
    .min_inner_size(600.0, 600.0)
    .inner_size(width, height)
    .position(position.x, position.y);

    // set transparent title bar only when building for macOS
    #[cfg(target_os = "macos")]
    let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

    let window = win_builder.build().unwrap();

    if count < 1 {
        create_window(app_handle, count + 1);
    }

    window
}

fn compute_main_window_geometry(app_handle: &AppHandle) -> (f64, f64, LogicalPosition<f64>) {
    let fallback_width = 1240.0;
    let fallback_height = 775.0;

    let monitor = app_handle
        .primary_monitor()
        .ok()
        .flatten()
        .or_else(|| app_handle.available_monitors().ok().and_then(|mut m| m.pop()));

    if let Some(monitor) = monitor {
        let monitor_size = monitor.size().to_logical::<f64>(monitor.scale_factor());
        let width = (monitor_size.width - 150.0).min(fallback_width).max(600.0);
        let height = (monitor_size.height - 150.0).min(fallback_height).max(600.0);
        let position =
            LogicalPosition::new((monitor_size.width - width) / 2.0, (monitor_size.height - height) / 2.0);
        return (width, height, position);
    }

    (
        fallback_width,
        fallback_height,
        LogicalPosition::new(100.0, 100.0),
    )
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
