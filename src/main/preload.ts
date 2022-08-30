import {contextBridge, ipcRenderer} from 'electron';
import fs from 'fs';
import path from 'path';
// import tools from './static/tools';

contextBridge.exposeInMainWorld('electron', {
  ipcRenderer: ipcRenderer,
  // tools: tools,
  fs: fs,
  path: path
})
