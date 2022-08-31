import { app, ipcMain } from "electron";

const isMac = process.platform === "darwin";

const proteusMenu: Electron.MenuItemConstructorOptions[] = [
  {
    label: "Title",
    submenu: [
      {
        label: "About Proteus",
        click() {
          require("electron").shell.openExternal("https://prot.dev/");
        },
      },
      { type: "separator" },
      { role: "quit" },
    ],
  },
  {
    label: "File",
    submenu: [
      {
        label: "New Window",
        click() {
          ipcMain.emit("newWindow")
        },
        accelerator: 'CmdOrCtrl+N',
      },
      { type: "separator" },
      {
        label: "Load Project",
        click() {
          ipcMain.emit("load")
        },
        accelerator: 'CmdOrCtrl+O',
      },
      {
        label: "Save Project",
        click() {
          ipcMain.emit("save")
        },
        accelerator: 'CmdOrCtrl+S',
      },
      { type: "separator" },
      { role: "close" },
    ],
  },
  {
    label: "Edit",
    submenu: [
      { role: "undo" },
      { role: "redo" },
      { type: "separator" },
      { role: "cut" },
      { role: "copy" },
      { role: "paste" },
      { role: "pasteAndMatchStyle" },
      { role: "delete" },
      { role: "selectAll" },
    ],
  },
  {
    label: "View",
    submenu: [
      { role: "reload" },
      { role: "forceReload" },
      { role: "toggleDevTools" },
      { type: "separator" },
      { role: "resetZoom" },
      { role: "zoomIn" },
      { role: "zoomOut" },
      { type: "separator" },
      { role: "togglefullscreen" },
    ],
  },
  { role: "window", submenu: [{ role: "minimize" }, { role: "close" }] },
  {
    role: "help",
    submenu: [
      {
        label: "Learn More",
        click() {
          require("electron").shell.openExternal("https://electron.atom.io");
        },
      },
    ],
  },
];

export default proteusMenu;
