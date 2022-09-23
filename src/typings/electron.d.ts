/**
 * Should match main/preload.ts for typescript support in renderer
 */

export default interface ElectronApi {
  ipcRenderer: Electron.IpcRenderer
  fs: typeof import('fs')
  path: typeof import('path')
  // tools: { [key: string]: any };
}

declare global {
  interface Window {
    electron: ElectronApi
    fs: ElectronApi
    path: ElectronApi
    // tools: ElectronApi;
  }
}
