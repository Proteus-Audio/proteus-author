import { app } from "electron";

const isMac = process.platform === "darwin";

const proteusMenu: Electron.MenuItemConstructorOptions[] = [
  {
    label: "File",
    submenu: [
      {
        label: "About Proteus",
        click() {
          require("electron").shell.openExternal("https://prot.dev/");
        },
      },
      { type: "separator" },
      { role: "close" },
      { role: "quit" },
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
