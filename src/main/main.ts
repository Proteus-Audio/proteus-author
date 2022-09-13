import { app, BrowserWindow, ipcMain, dialog, Menu } from 'electron'
import { join, sep } from 'path'
import { readFileSync } from 'fs'
import proteusMenu from './static/proteusMenu'
import { save, load, loadData } from './static/fileOptions'
import { Project } from './static/typings'
import mime from 'mime'
import { entryData } from './static/global'
import { randomUUID } from 'crypto'

function createWindow(data?: Project) {
  // We cannot require the screen module until the app is ready.
  const { screen } = require('electron')

  // Create a window that fills the screen's available work area.
  const primaryDisplay = screen.getPrimaryDisplay()
  const { width, height } = primaryDisplay.workAreaSize

  const id = randomUUID()
  entryData.projects[id] = data

  const mainWindow = new BrowserWindow({
    width: Math.min(width - 200, 1240),
    height: Math.min(height - 100, 775),
    webPreferences: {
      preload: join(__dirname, 'preload.js'),
      nodeIntegration: false,
      // nodeIntegrationInWorker: true,
      sandbox: false,
      contextIsolation: true,
      webSecurity: false,
    },
  })

  if (process.env.NODE_ENV === 'development') {
    const rendererPort = process.argv[2]
    mainWindow.loadURL(`http://localhost:${rendererPort}?id=${id}`)
  } else {
    mainWindow.loadFile(join(app.getAppPath(), 'renderer', 'index.html'), { query: { id } })
  }
}

app.whenReady().then(() => {
  const menu = Menu.buildFromTemplate(proteusMenu)
  Menu.setApplicationMenu(menu)
  // console.log(Menu.getApplicationMenu());

  createWindow()

  app.on('activate', function () {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow()
    }
  })
})

app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') app.quit()
})

ipcMain.on('message', async (_event, message) => {
  console.log(message)
})

ipcMain.handle('init', async (_event, id) => {
  return entryData.projects[id]
})

ipcMain.handle('openFile', async (_event, ...args) => {
  const file = await dialog.showOpenDialog({ properties: ['openFile'] })
  // console.log(args)
  if (file.canceled) return 'canceled'
  const filePath = file.filePaths[0]
  const fileName = (filePath.match(/[\w]*\..*$/) != null || [''])[0]
  const base64 = readFileSync(file.filePaths[0]).toString('base64')
  const type = mime.getType(filePath)
  const src = `data:${type};base64,${base64}`
  return { fileName, filePath, src, type }
})

ipcMain.handle('chooseDir', async () => {
  const file = await dialog.showOpenDialog({ properties: ['openDirectory', 'createDirectory'] })
  if (file.canceled) return 'canceled'
  const filePath = file.filePaths[0] + '/'
  return filePath
})

ipcMain.on('newWindow', () => {
  createWindow()
})

ipcMain.on('save', () => {
  BrowserWindow.getFocusedWindow()?.webContents.executeJavaScript(
    "document.getElementById('saveButton').click()",
  )
})

ipcMain.on('saveAs', () => {
  BrowserWindow.getFocusedWindow()?.webContents.executeJavaScript(
    "document.getElementById('saveAsButton').click()",
  )
})

ipcMain.handle('save', async (_event, project: Project): Promise<Project | undefined> => {
  project.location = project.location || ''
  let fileLocation = project.location
  let fileName = project.name || ''

  console.log('project')
  console.log(project)
  console.log(fileLocation, fileName)

  if (fileLocation === '') {
    const chosenLocation = await dialog.showSaveDialog({
      // title: "",
      defaultPath: 'prot',
      properties: [],
    })

    fileLocation = chosenLocation.filePath || fileLocation
    fileName = ((chosenLocation.filePath || '').match(/[^\\/]+$/) || [''])[0] || fileName
    if (chosenLocation.canceled) return { tracks: [], effects: [], location: fileLocation }
    fileLocation = fileLocation.replace('.protproject', '')
  }

  console.log(fileLocation, fileName)

  if (fileLocation !== '') {
    await save(project, fileLocation, fileName)
    return loadData(fileLocation + sep + fileName)
  }
  return { tracks: [], effects: [], location: fileLocation }
})

ipcMain.on('load', async () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    const data = await load()
    createWindow(data)
  } else {
    BrowserWindow.getFocusedWindow()?.webContents.executeJavaScript(
      "document.getElementById('loadButton').click()",
    )
  }
})

ipcMain.handle('load', async () => {
  return await load()
})

app.on('open-file', async (_event, path) => {
  const data = await load(path)
  createWindow(data)
})
