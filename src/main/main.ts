import { app, BrowserWindow, ipcMain, dialog, Menu } from "electron";
import { join } from "path";
import fs from "fs";
import proteusMenu from "./static/proteusMenu";
import {save, load, Project} from "./static/fileOptions";
import mime from "mime";
import isDev from "electron-is-dev";

function createWindow(data?:Project) {
  // We cannot require the screen module until the app is ready.
  const { screen } = require("electron");

  // Create a window that fills the screen's available work area.
  const primaryDisplay = screen.getPrimaryDisplay();
  const { width, height } = primaryDisplay.workAreaSize;

  const mainWindow = new BrowserWindow({
    width: Math.min(width - 200, 1240),
    height: Math.min(height - 100, 775),
    webPreferences: {
      preload: join(__dirname, "preload.js"),
      nodeIntegration: false,
      // nodeIntegrationInWorker: true,
      contextIsolation: true,
      webSecurity: false,
    },
  });

  if (process.env.NODE_ENV === "development") {
    // console.log("yup yup");
    const rendererPort = process.argv[2];
    mainWindow.loadURL(`http://localhost:${rendererPort}`);
  } else {
    mainWindow.loadFile(join(app.getAppPath(), "renderer", "index.html"));
  }
}

app.whenReady().then(() => {
  const menu = Menu.buildFromTemplate(proteusMenu);
  Menu.setApplicationMenu(menu);
  // console.log(Menu.getApplicationMenu());

  createWindow();

  app.on("activate", function () {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on("window-all-closed", function () {
  if (process.platform !== "darwin") app.quit();
});

ipcMain.on("message", async (event, message) => {
  console.log(message);
});

// ipcMain.on("copyFileToTemp", async (event, message) => {
//   console.log(join(__dirname));
// });
interface SimpleFile {
  name: string;
  path: string;
}
ipcMain.handle("copyFileToTemp", async (event, filePaths: SimpleFile[]) => {
  const files: string[] = [];
  filePaths.forEach((file) => {
    // ToDo: add unique id so that files can have the same name
    files.push(`/temp/${file.name}`);
    // fs.copyFileSync(file.path, file.path.replace(file.name, (name) => name.replace(/^\w+/, (match) => `${match}2`)));
    // fs.copyFileSync(file.path, join(__dirname, file.name));
    // fs.copyFileSync(file.path, join(tempLocation, file.name));
  });

  return { dev: isDev, files };
});

ipcMain.handle("openFile", async (event, ...args) => {
  const file = await dialog.showOpenDialog({ properties: ["openFile"] });
  console.log(args);
  if (file.canceled) return "canceled";
  const filePath = file.filePaths[0];
  const fileName = (filePath.match(/[\w]*\..*$/) || [""])[0];
  // ipcMain.emit("openFileReturn", { key, filename: "filename" });
  const base64 = fs.readFileSync(file.filePaths[0]).toString("base64");
  const type = mime.getType(filePath);
  const src = `data:${type};base64,${base64}`;
  return { fileName, filePath, src, type };
});

ipcMain.handle("chooseDir", async (event, key) => {
  const file = await dialog.showOpenDialog({ properties: ["openDirectory", "createDirectory"] });
  if (file.canceled) return "canceled";
  console.log(file);
  const filePath = file.filePaths[0] + "/";
  // ipcMain.emit("openFileReturn", { key, filename: "filename" });
  // console.log(fs.readFileSync(file.filePaths[0]))
  return filePath;
});

ipcMain.on("newWindow", () => {
  createWindow();
});

ipcMain.on("save", (event) => {
  console.log(
    BrowserWindow.getFocusedWindow()?.webContents.executeJavaScript(
      "document.getElementById('saveButton').click()"
    )
  );
});

ipcMain.handle("save", async (event, project: Project) => {
  project.location = project.location || "";
  let fileLocation = project.location;

  if (fileLocation === "") {
    const chosenLocation = await dialog.showSaveDialog({
      // title: "",
      defaultPath: "prot.protproject",
      filters: [{ name: "Prot Project", extensions: [".protproject"] }],
      properties: [],
    });

    fileLocation = chosenLocation.filePath || fileLocation;
    if(chosenLocation.canceled) return {tracks: false, location: fileLocation};
  }

  if (/\.protproject/.test(fileLocation)) {
    await save(project.tracks, fileLocation);
    return {tracks: await load(fileLocation), location: fileLocation};
  }
  return {tracks: false, location: fileLocation};
});

ipcMain.on("load", (event) => {
  console.log(
    BrowserWindow.getFocusedWindow()?.webContents.executeJavaScript(
      "document.getElementById('loadButton').click()"
    )
  );
});

ipcMain.handle("load", async () => {
    const chosenLocation = await dialog.showOpenDialog({
      filters: [{ name: "Prot Project", extensions: [".protproject"] }],
      properties: ["openFile"],
    });

    const fileLocation = chosenLocation.filePaths[0];
    if(chosenLocation.canceled) return {tracks: false, location: fileLocation};;

  if (/\.protproject/.test(fileLocation)) {
    return {tracks: await load(fileLocation), location: fileLocation};
  }
});
