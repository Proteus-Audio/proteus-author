import { app, BrowserWindow, ipcMain, dialog, Menu } from "electron";
import { join } from "path";
import { readFileSync } from "fs";
import proteusMenu from "./static/proteusMenu";
import { save, load, Project } from "./static/fileOptions";
import mime from "mime";

function createWindow(data?: Project) {
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

ipcMain.handle("openFile", async (event, ...args) => {
  const file = await dialog.showOpenDialog({ properties: ["openFile"] });
  console.log(args);
  if (file.canceled) return "canceled";
  const filePath = file.filePaths[0];
  const fileName = (filePath.match(/[\w]*\..*$/) || [""])[0];
  const base64 = readFileSync(file.filePaths[0]).toString("base64");
  const type = mime.getType(filePath);
  const src = `data:${type};base64,${base64}`;
  return { fileName, filePath, src, type };
});

ipcMain.handle("chooseDir", async (event, key) => {
  const file = await dialog.showOpenDialog({ properties: ["openDirectory", "createDirectory"] });
  if (file.canceled) return "canceled";
  console.log(file);
  const filePath = file.filePaths[0] + "/";
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

ipcMain.on("saveAs", (event) => {
  console.log("No no, there's something to say");
  console.log(
    BrowserWindow.getFocusedWindow()?.webContents.executeJavaScript(
      "document.getElementById('saveAsButton').click()"
    )
  );
});

ipcMain.handle("save", async (event, project: Project) => {
  project.location = project.location || "";
  let fileLocation = project.location;
  let fileName = project.name || "";

  if (fileLocation === "") {
    const chosenLocation = await dialog.showSaveDialog({
      // title: "",
      defaultPath: "prot",
      properties: [],
    });

    fileLocation = chosenLocation.filePath || fileLocation;
    fileName = ((chosenLocation.filePath || "").match(/[^\\\/]+$/) || [""])[0] || fileName;
    if (chosenLocation.canceled) return { tracks: false, location: fileLocation };
  }

  console.log(fileLocation, fileName);

  if (fileLocation !== "") {
    if (/\.protproject/.test(fileLocation)) fileLocation = fileLocation.replace(fileName, "");
    await save(project.tracks, fileLocation, fileName);
    return { tracks: await load(fileLocation, fileName), location: fileLocation, name: fileName };
  }
  return { tracks: false, location: fileLocation };
});

ipcMain.on("load", (event) => {
  if(BrowserWindow.getAllWindows().length === 0) createWindow();
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

  let fileLocation = chosenLocation.filePaths[0];
  if (chosenLocation.canceled) return { tracks: false, location: fileLocation };
  const fileName = (fileLocation.match(/[^\\\/]+$/) || [""])[0];
  fileLocation = fileLocation.replace(fileName, "");

  if (/\.protproject/.test(fileName)) {
    return { tracks: await load(fileLocation, fileName), location: fileLocation, name: fileName };
  }
});
