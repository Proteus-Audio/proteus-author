/*
   Add all your exposed Electron API's here.
   The purpose of this is to get static analysis in Vue files without additional plug-ins.
 */
import { IpcRenderer } from "electron";

const ipcRenderer = window.electron.ipcRenderer as IpcRenderer;
const fs = window.electron.fs as typeof import("fs");
const path = window.electron.path as typeof import("path");
// const tools = window.electron.tools as { [key: string]: any };

export { ipcRenderer, fs, path };
