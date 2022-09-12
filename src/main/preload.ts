import { contextBridge, ipcRenderer } from 'electron'
import fs from 'fs'
import path from 'path'

contextBridge.exposeInMainWorld('electron', {
  ipcRenderer,
  fs,
  path,
})
