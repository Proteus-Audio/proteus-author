/*
   Add all your exposed Electron API's here.
   The purpose of this is to get static analysis in Vue files without additional plug-ins.
 */

const ipcRenderer = window.electron.ipcRenderer
const fs = window.electron.fs
const path = window.electron.path
// const tools = window.electron.tools as { [key: string]: any };

export { ipcRenderer, fs, path }
