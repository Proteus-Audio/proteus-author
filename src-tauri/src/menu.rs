use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

use crate::windows;

const ID_ABOUT: &str = "about";
const ID_NEW_WINDOW: &str = "new_window";
const ID_SAVE: &str = "save";
const ID_SAVE_AS: &str = "save_as";
const ID_OPEN: &str = "open";
const ID_EXPORT_PROT: &str = "export_prot";
const ID_ZOOM_IN: &str = "zoom";
const ID_ZOOM_OUT: &str = "zoom_out";
const ID_ZOOM_IN_VERTICAL: &str = "zoom_vertical_in";
const ID_ZOOM_OUT_VERTICAL: &str = "zoom_vertical_out";
const ID_SCROLL_LEFT: &str = "scroll_left";
const ID_SCROLL_RIGHT: &str = "scroll_right";
const ID_FOLLOW_MODE: &str = "follow_mode";
const ID_SHUFFLE_POINT_TOOL_MODE: &str = "shuffle_point_tool_mode";

pub struct FollowModeState(pub Arc<Mutex<bool>>);
pub struct ShufflePointToolModeState(pub Arc<Mutex<bool>>);

#[derive(Debug, Clone, Serialize)]
struct AlertPayload {
    message: String,
    r#type: String,
}

#[derive(Debug, Clone, Serialize)]
struct FollowModePayload {
    enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ShufflePointToolModePayload {
    enabled: bool,
}

fn emit_to_main<R: Runtime, S: serde::Serialize>(app: &AppHandle<R>, event: &str, payload: S) {
    let had_windows = !app.webview_windows().is_empty();
    if !had_windows {
        let window = windows::get_or_create_main_window(app);
        let _ = window.show();
        let _ = window.set_focus();
    }

    for window in app.webview_windows().values() {
        let _ = window.emit(event, &payload);
    }

    if !had_windows {
        let app_handle = app.clone();
        let event_name = event.to_string();
        if let Ok(json_payload) = serde_json::to_value(&payload) {
            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_millis(220));
                for window in app_handle.webview_windows().values() {
                    let _ = window.emit(&event_name, &json_payload);
                }
            });
        }
    }
}

fn create_new_window<R: Runtime>(app: &AppHandle<R>) {
    let mut count = 1;
    loop {
        let label = format!("main-window-{}", count);
        if app.get_webview_window(&label).is_none() {
            break;
        }
        count += 1;
    }

    let window = windows::create_window(app, count);
    let _ = window.show();
    let _ = window.set_focus();
}

fn find_check_menu_item<R: Runtime>(menu: &Menu<R>, target_id: &str) -> Option<CheckMenuItem<R>> {
    fn find_in_items<R: Runtime>(
        items: Vec<tauri::menu::MenuItemKind<R>>,
        target_id: &str,
    ) -> Option<CheckMenuItem<R>> {
        for item in items {
            if item.id().as_ref() == target_id {
                if let Some(check_item) = item.as_check_menuitem() {
                    return Some(check_item.clone());
                }
            }

            if let Some(submenu) = item.as_submenu() {
                if let Ok(sub_items) = submenu.items() {
                    if let Some(found) = find_in_items(sub_items, target_id) {
                        return Some(found);
                    }
                }
            }
        }
        None
    }

    menu.items().ok().and_then(|items| find_in_items(items, target_id))
}

pub fn build_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let about = MenuItem::with_id(app, ID_ABOUT, "About Proteus Author", true, None::<&str>)?;

    let separator = PredefinedMenuItem::separator(app)?;

    #[cfg(target_os = "macos")]
    let prot_menu = Some(Submenu::with_id_and_items(
        app,
        "prot",
        "Proteus Author",
        true,
        &[
            &about,
            &separator,
            &PredefinedMenuItem::services(app, None::<&str>)?,
            &separator,
            &PredefinedMenuItem::hide(app, None::<&str>)?,
            &PredefinedMenuItem::hide_others(app, None::<&str>)?,
            &PredefinedMenuItem::show_all(app, None::<&str>)?,
            &separator,
            &PredefinedMenuItem::quit(app, None::<&str>)?,
        ],
    )?);

    #[cfg(not(target_os = "macos"))]
    let prot_menu: Option<Submenu<R>> = None;

    let new_window = MenuItem::with_id(
        app,
        ID_NEW_WINDOW,
        "New Window",
        true,
        Some("CmdOrCtrl+N"),
    )?;

    let save = MenuItem::with_id(app, ID_SAVE, "Save", true, Some("CmdOrCtrl+S"))?;

    let save_as = MenuItem::with_id(
        app,
        ID_SAVE_AS,
        "Save As",
        true,
        Some("CmdOrCtrl+Shift+S"),
    )?;

    let open = MenuItem::with_id(app, ID_OPEN, "Open", true, Some("CmdOrCtrl+O"))?;

    let export_prot = MenuItem::with_id(
        app,
        ID_EXPORT_PROT,
        "Export Prot File",
        true,
        Some("CmdOrCtrl+Shift+E"),
    )?;

    let file_menu = Submenu::with_id_and_items(
        app,
        "file",
        "File",
        true,
        &[&new_window, &separator, &save, &save_as, &open, &separator, &export_prot],
    )?;

    let edit_menu = Submenu::with_id_and_items(
        app,
        "edit",
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app, None::<&str>)?,
            &PredefinedMenuItem::redo(app, None::<&str>)?,
            &separator,
            &PredefinedMenuItem::cut(app, None::<&str>)?,
            &PredefinedMenuItem::copy(app, None::<&str>)?,
            &PredefinedMenuItem::paste(app, None::<&str>)?,
        ],
    )?;

    let zoom_in = MenuItem::with_id(app, ID_ZOOM_IN, "Zoom In", true, Some("CmdOrCtrl+="))?;
    let zoom_out = MenuItem::with_id(app, ID_ZOOM_OUT, "Zoom Out", true, Some("CmdOrCtrl+-"))?;
    let zoom_vertical_in = MenuItem::with_id(
        app,
        ID_ZOOM_IN_VERTICAL,
        "Zoom In Vertical",
        true,
        Some("CmdOrCtrl+Shift+="),
    )?;
    let zoom_vertical_out = MenuItem::with_id(
        app,
        ID_ZOOM_OUT_VERTICAL,
        "Zoom Out Vertical",
        true,
        Some("CmdOrCtrl+Shift+-"),
    )?;
    let pan_left =
        MenuItem::with_id(app, ID_SCROLL_LEFT, "Scroll Left", true, Some("Alt+Left"))?;
    let pan_right =
        MenuItem::with_id(app, ID_SCROLL_RIGHT, "Scroll Right", true, Some("Alt+Right"))?;
    let follow_mode = CheckMenuItem::with_id(
        app,
        ID_FOLLOW_MODE,
        "Follow Mode",
        true,
        false,
        Some("Alt+F"),
    )?;
    let shuffle_point_tool_mode = CheckMenuItem::with_id(
        app,
        ID_SHUFFLE_POINT_TOOL_MODE,
        "Shuffle Point Tool",
        true,
        false,
        Some("Alt+P"),
    )?;

    let view_menu = Submenu::with_id_and_items(
        app,
        "view",
        "View",
        true,
        &[
            &zoom_in,
            &zoom_out,
            &zoom_vertical_in,
            &zoom_vertical_out,
            &separator,
            &pan_left,
            &pan_right,
            &separator,
            &follow_mode,
        ],
    )?;

    let tools_menu = Submenu::with_id_and_items(
        app,
        "tools",
        "Tools",
        true,
        &[&shuffle_point_tool_mode],
    )?;

    let window_menu = Submenu::with_id_and_items(
        app,
        "window",
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None::<&str>)?,
            &separator,
            &PredefinedMenuItem::close_window(app, None::<&str>)?,
        ],
    )?;

    let mut items: Vec<&dyn tauri::menu::IsMenuItem<R>> = Vec::new();
    if let Some(prot_menu) = &prot_menu {
        items.push(prot_menu);
    }
    items.push(&file_menu);
    items.push(&edit_menu);
    items.push(&view_menu);
    items.push(&tools_menu);
    items.push(&window_menu);

    Menu::with_id_and_items(app, "main", &items)
}

pub fn handle_menu_event<R: Runtime>(
    app: &AppHandle<R>,
    event: MenuEvent,
    follow_mode_state: &FollowModeState,
    shuffle_point_tool_mode_state: &ShufflePointToolModeState,
) {
    let id = event.id();

    if id == ID_ABOUT {
            let app_name = app.package_info().name.clone();
            let version = app.package_info().version.to_string();
            emit_to_main(
                app,
                "ALERT",
                AlertPayload {
                    message: format!("{} v{}\\n©Adam Thomas Howard 2024", app_name, version),
                    r#type: "info".to_string(),
                },
            );
    } else if id == ID_NEW_WINDOW {
        create_new_window(app)
    } else if id == ID_SAVE {
        emit_to_main(app, "SAVE_FILE", ())
    } else if id == ID_SAVE_AS {
        emit_to_main(app, "SAVE_FILE_AS", ())
    } else if id == ID_OPEN {
        emit_to_main(app, "OPEN_FILE", ())
    } else if id == ID_EXPORT_PROT {
        emit_to_main(app, "START_EXPORT", ())
    } else if id == ID_ZOOM_IN {
        emit_to_main(app, "MENU_ZOOM_IN", ())
    } else if id == ID_ZOOM_OUT {
        emit_to_main(app, "MENU_ZOOM_OUT", ())
    } else if id == ID_ZOOM_IN_VERTICAL {
        emit_to_main(app, "MENU_ZOOM_IN_VERTICAL", ())
    } else if id == ID_ZOOM_OUT_VERTICAL {
        emit_to_main(app, "MENU_ZOOM_OUT_VERTICAL", ())
    } else if id == ID_SCROLL_LEFT {
        emit_to_main(app, "MENU_PAN_LEFT", ())
    } else if id == ID_SCROLL_RIGHT {
        emit_to_main(app, "MENU_PAN_RIGHT", ())
    } else if id == ID_FOLLOW_MODE {
        let mut follow_mode = follow_mode_state.0.lock().unwrap();
        *follow_mode = !*follow_mode;
        emit_to_main(
            app,
            "MENU_FOLLOW_MODE",
            FollowModePayload {
                enabled: *follow_mode,
            },
        );
    } else if id == ID_SHUFFLE_POINT_TOOL_MODE {
        let mut shuffle_point_tool_mode = shuffle_point_tool_mode_state.0.lock().unwrap();
        *shuffle_point_tool_mode = !*shuffle_point_tool_mode;
        emit_to_main(
            app,
            "MENU_SHUFFLE_POINT_TOOL_MODE",
            ShufflePointToolModePayload {
                enabled: *shuffle_point_tool_mode,
            },
        );
    }
}

#[tauri::command]
pub fn set_follow_mode_menu(
    enabled: bool,
    app: AppHandle,
    follow_mode_state: State<FollowModeState>,
) {
    {
        let mut follow_mode = follow_mode_state.0.lock().unwrap();
        *follow_mode = enabled;
    }

    if let Some(menu) = app.menu() {
        if let Some(check_item) = find_check_menu_item(&menu, ID_FOLLOW_MODE) {
            let _ = check_item.set_checked(enabled);
        }
    }
}

#[tauri::command]
pub fn set_shuffle_point_tool_mode_menu(
    enabled: bool,
    app: AppHandle,
    shuffle_point_tool_mode_state: State<ShufflePointToolModeState>,
) {
    {
        let mut shuffle_point_tool_mode = shuffle_point_tool_mode_state.0.lock().unwrap();
        *shuffle_point_tool_mode = enabled;
    }

    if let Some(menu) = app.menu() {
        if let Some(check_item) = find_check_menu_item(&menu, ID_SHUFFLE_POINT_TOOL_MODE) {
            let _ = check_item.set_checked(enabled);
        }
    }
}
