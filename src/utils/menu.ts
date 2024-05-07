import { invoke } from '@tauri-apps/api/core'
import { Menu, MenuItem, PredefinedMenuItem, Submenu } from '@tauri-apps/api/menu'
import { message } from '@tauri-apps/plugin-dialog'

export const defaultMenu = async () => {
  const about = await MenuItem.new({
    text: 'About Proteus Author',
    id: 'about',
  })

  const separator = await PredefinedMenuItem.new({ item: 'Separator' })
  const services = await PredefinedMenuItem.new({ item: 'Services' })
  const hide = await PredefinedMenuItem.new({ item: 'Hide' })
  const hideOthers = await PredefinedMenuItem.new({ item: 'HideOthers' })
  const showAll = await PredefinedMenuItem.new({ item: 'ShowAll' })
  const quit = await PredefinedMenuItem.new({ item: 'Quit' })

  const protMenu = await Submenu.new({
    text: 'Proteus Author',
    id: 'prot',
    items: [about, separator, services, separator, hide, hideOthers, showAll, separator, quit],
  })

  const newWindow = await MenuItem.new({
    text: 'New Window',
    id: 'new_window',
    accelerator: 'CmdOrCtrl+N',
    action: async () => {
      //   await dialog.open({
      //     directory: false,
      //     multiple: false,
      //     filter: 'All Files',
      //   })
      await message('New Window command not setup', {
        title: 'New Window',
        kind: 'info',
      })
      console.log('New Window')
    },
  })

  const save = await MenuItem.new({
    text: 'Save',
    id: 'save',
    accelerator: 'CmdOrCtrl+S',
    action: async () => {
      await invoke('save_file')
    },
  })

  const saveAs = await MenuItem.new({
    text: 'Save As',
    id: 'save_as',
    accelerator: 'CmdOrCtrl+Shift+S',
    action: async () => {
      await invoke('save_file_as')
    },
  })

  const open = await MenuItem.new({
    text: 'Open',
    id: 'open',
    accelerator: 'CmdOrCtrl+O',
    action: async () => {
      await invoke('open_file')
    },
  })

  const exportProt = await MenuItem.new({
    text: 'Export Prot File',
    id: 'export_prot',
    accelerator: 'CmdOrCtrl+Shift+E',
    action: async () => {
      await invoke('export_prot')
    },
  })

  const fileMenu = await Submenu.new({
    text: 'File',
    id: 'file',
    items: [newWindow, separator, save, saveAs, open, separator, exportProt],
  })

  const undo = await PredefinedMenuItem.new({ item: 'Undo' })
  const redo = await PredefinedMenuItem.new({ item: 'Redo' })
  const cut = await PredefinedMenuItem.new({ item: 'Cut' })
  const copy = await PredefinedMenuItem.new({ item: 'Copy' })
  const paste = await PredefinedMenuItem.new({ item: 'Paste' })

  const editMenu = await Submenu.new({
    text: 'Edit',
    id: 'edit',
    items: [undo, redo, separator, cut, copy, paste],
  })

  const zoomIn = await MenuItem.new({
    text: 'Zoom In',
    id: 'zoom',
    accelerator: 'CmdOrCtrl+=',
  })

  const zoomOut = await MenuItem.new({
    text: 'Zoom Out',
    id: 'zoom_out',
    accelerator: 'CmdOrCtrl+-',
  })

  const viewMenu = await Submenu.new({
    text: 'View',
    id: 'view',
    items: [zoomIn, zoomOut],
  })

  const minimize = await PredefinedMenuItem.new({ item: 'Minimize' })

  const closeWindow = await PredefinedMenuItem.new({ item: 'CloseWindow' })

  const windowMenu = await Submenu.new({
    text: 'Window',
    id: 'window',
    items: [minimize, separator, closeWindow],
  })

  const menu = await Menu.new({
    id: 'main',
    items: [protMenu, fileMenu, editMenu, viewMenu, windowMenu],
  })

  return menu
}

// use tauri::menu::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu};

// pub fn make_menu(#[allow(unused)] app_name: &str) -> Menu {
//     let mut menu = Menu::new();
//     #[cfg(target_os = "macos")]
//     {
//         menu = menu.add_submenu(Submenu::new(
//             app_name,
//             Menu::new()
//                 .add_native_item(MenuItem::About(
//                     app_name.to_string(),
//                     AboutMetadata::default(),
//                 ))
//                 .add_native_item(MenuItem::Separator)
//                 .add_native_item(MenuItem::Services)
//                 .add_native_item(MenuItem::Separator)
//                 .add_native_item(MenuItem::Hide)
//                 .add_native_item(MenuItem::HideOthers)
//                 .add_native_item(MenuItem::ShowAll)
//                 .add_native_item(MenuItem::Separator)
//                 .add_native_item(MenuItem::Quit),
//         ));
//     }

//     let mut file_menu = Menu::new();
//     let new_window =
//         CustomMenuItem::new("new_window".to_string(), "New Window").accelerator("CmdOrCtrl+N");
//     let save = CustomMenuItem::new("save".to_string(), "Save").accelerator("CmdOrCtrl+S");
//     let save_as =
//         CustomMenuItem::new("save_as".to_string(), "Save As").accelerator("CmdOrCtrl+Shift+S");
//     let load = CustomMenuItem::new("load".to_string(), "Open").accelerator("CmdOrCtrl+O");
//     file_menu = file_menu.add_item(new_window);
//     file_menu = file_menu.add_native_item(MenuItem::Separator);
//     file_menu = file_menu.add_item(save);
//     file_menu = file_menu.add_item(save_as);
//     file_menu = file_menu.add_item(load);
//     file_menu = file_menu.add_native_item(MenuItem::Separator);

//     let export_sub_menu = Menu::new().add_item(
//         CustomMenuItem::new("export_prot", "Export Prot File").accelerator("CmdOrCtrl+Shift+E"),
//     );

//     file_menu = file_menu.add_submenu(Submenu::new("Export", export_sub_menu));
//     file_menu = file_menu.add_native_item(MenuItem::Separator);

//     file_menu = file_menu.add_native_item(MenuItem::CloseWindow);

//     #[cfg(not(target_os = "macos"))]
//     {
//         file_menu = file_menu.add_native_item(MenuItem::Quit);
//     }
//     menu = menu.add_submenu(Submenu::new("File", file_menu));

//     #[cfg(not(target_os = "linux"))]
//     let mut edit_menu = Menu::new();
//     #[cfg(target_os = "macos")]
//     {
//         edit_menu = edit_menu.add_native_item(MenuItem::Undo);
//         edit_menu = edit_menu.add_native_item(MenuItem::Redo);
//         edit_menu = edit_menu.add_native_item(MenuItem::Separator);
//     }
//     #[cfg(not(target_os = "linux"))]
//     {
//         edit_menu = edit_menu.add_native_item(MenuItem::Cut);
//         edit_menu = edit_menu.add_native_item(MenuItem::Copy);
//         edit_menu = edit_menu.add_native_item(MenuItem::Paste);
//     }
//     #[cfg(target_os = "macos")]
//     {
//         edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
//     }
//     #[cfg(not(target_os = "linux"))]
//     {
//         menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
//     }
//     #[cfg(target_os = "macos")]
//     {
//         menu = menu.add_submenu(Submenu::new(
//             "View",
//             Menu::new().add_native_item(MenuItem::EnterFullScreen),
//         ));
//     }

//     let mut window_menu = Menu::new();
//     window_menu = window_menu.add_native_item(MenuItem::Minimize);
//     #[cfg(target_os = "macos")]
//     {
//         window_menu = window_menu.add_native_item(MenuItem::Zoom);
//         window_menu = window_menu.add_native_item(MenuItem::Separator);
//     }
//     window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
//     menu = menu.add_submenu(Submenu::new("Window", window_menu));

//     menu
// }

// pub fn get_menu() -> Menu {
//     let default_menu = make_menu("Proteus Author");
//     let main_menu = Menu::with_items(default_menu.items);

//     return main_menu;
// }
