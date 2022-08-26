import { app, BrowserWindow, ipcMain, dialog } from "electron";
import { join } from "path";
import fs from "fs";

function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: join(__dirname, "preload.js"),
      nodeIntegration: false,
      contextIsolation: true,
    },
  });

  if (process.env.NODE_ENV === "development") {
    const rendererPort = process.argv[2];
    mainWindow.loadURL(`http://localhost:${rendererPort}`);
  } else {
    mainWindow.loadFile(join(app.getAppPath(), "renderer", "index.html"));
  }
}

app.whenReady().then(() => {
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

ipcMain.handle("openFile", async (event, key) => {
  const file = await dialog.showOpenDialog({ properties: ["openFile"] });
  if(file.canceled) return 'canceled';
  const fileName = (file.filePaths[0].match(/[\w]*\..*$/) || [''])[0];
  // ipcMain.emit("openFileReturn", { key, filename: "filename" });
  console.log(fs.readFileSync(file.filePaths[0]))
  return fileName;
});
