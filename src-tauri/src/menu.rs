use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, AboutMetadata};

pub fn make_menu(#[allow(unused)] app_name: &str) -> Menu {
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
      menu = menu.add_submenu(Submenu::new(
        app_name,
        Menu::new()
          .add_native_item(MenuItem::About(
            app_name.to_string(),
            AboutMetadata::default(),
          ))
          .add_native_item(MenuItem::Separator)
          .add_native_item(MenuItem::Services)
          .add_native_item(MenuItem::Separator)
          .add_native_item(MenuItem::Hide)
          .add_native_item(MenuItem::HideOthers)
          .add_native_item(MenuItem::ShowAll)
          .add_native_item(MenuItem::Separator)
          .add_native_item(MenuItem::Quit),
      ));
    }

    let mut file_menu = Menu::new();
    let new_window =
        CustomMenuItem::new("new_window".to_string(), "New Window").accelerator("CmdOrCtrl+N");
    let save = CustomMenuItem::new("save".to_string(), "Save").accelerator("CmdOrCtrl+S");
    let save_as =
        CustomMenuItem::new("save_as".to_string(), "Save As").accelerator("CmdOrCtrl+Shift+S");
    let load = CustomMenuItem::new("load".to_string(), "Open").accelerator("CmdOrCtrl+O");
    file_menu = file_menu.add_item(new_window);
    file_menu = file_menu.add_native_item(MenuItem::Separator);
    file_menu = file_menu.add_item(save);
    file_menu = file_menu.add_item(save_as);
    file_menu = file_menu.add_item(load);
    file_menu = file_menu.add_native_item(MenuItem::Separator);

    let export_sub_menu = Menu::new()
        .add_item(CustomMenuItem::new(
            "export_prot",
            "Export Prot File",
        ).accelerator("CmdOrCtrl+Shift+E"));

    file_menu = file_menu.add_submenu(Submenu::new("Export", export_sub_menu));
    file_menu = file_menu.add_native_item(MenuItem::Separator);

    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);


    
    #[cfg(not(target_os = "macos"))]
    {
      file_menu = file_menu.add_native_item(MenuItem::Quit);
    }
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    #[cfg(not(target_os = "linux"))]
    let mut edit_menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
      edit_menu = edit_menu.add_native_item(MenuItem::Undo);
      edit_menu = edit_menu.add_native_item(MenuItem::Redo);
      edit_menu = edit_menu.add_native_item(MenuItem::Separator);
    }
    #[cfg(not(target_os = "linux"))]
    {
      edit_menu = edit_menu.add_native_item(MenuItem::Cut);
      edit_menu = edit_menu.add_native_item(MenuItem::Copy);
      edit_menu = edit_menu.add_native_item(MenuItem::Paste);
    }
    #[cfg(target_os = "macos")]
    {
      edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
    }
    #[cfg(not(target_os = "linux"))]
    {
      menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
    }
    #[cfg(target_os = "macos")]
    {
      menu = menu.add_submenu(Submenu::new(
        "View",
        Menu::new().add_native_item(MenuItem::EnterFullScreen),
      ));
    }

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
      window_menu = window_menu.add_native_item(MenuItem::Zoom);
      window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    menu
  }

pub fn get_menu() -> Menu {
    let default_menu = make_menu("Proteus Author");
    let main_menu = Menu::with_items(default_menu.items);

    return main_menu;
}
