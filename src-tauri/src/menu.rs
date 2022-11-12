use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

pub fn get_menu() -> Menu {
    let new_window =
        CustomMenuItem::new("new_window".to_string(), "New Window").accelerator("CmdOrCtrl+N");
    let save = CustomMenuItem::new("save".to_string(), "Save").accelerator("CmdOrCtrl+S");
    let save_as =
        CustomMenuItem::new("save_as".to_string(), "Save As").accelerator("CmdOrCtrl+Shift+S");
    let load = CustomMenuItem::new("load".to_string(), "Open").accelerator("CmdOrCtrl+O");

    // This is a placeholder, as this whole menu
    // should be added to the 'File' menu.
    // Either this needs for the default menu to be deconstructed (started below)
    // and the items need to be added to the proper places in the menu
    // or the default menus must be re-written from scratch with OS checks
    let extra_menu = Submenu::new(
        "Extra Options",
        Menu::new()
            .add_item(new_window)
            .add_native_item(MenuItem::Separator)
            .add_item(save)
            .add_item(save_as)
            .add_item(load),
    );
    let default_menu = Menu::os_default("Proteus Author").add_submenu(extra_menu);
    let menu_items = default_menu.items.into_iter().map(|menu_item: MenuEntry| {
        // Somehow add to specific submenus
        return menu_item;
    });
    let main_menu = Menu::with_items(menu_items);

    return main_menu;
}
