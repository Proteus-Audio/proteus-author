import { path } from '@tauri-apps/api'
import { upperCamelCase } from 'case-anything'

const baseDirEnum: { [key: string]: number } = {
  App: 18,
  AppCache: 24,
  AppConfig: 21,
  AppData: 22,
  AppLocalData: 23,
  AppLog: 25,
  Audio: 1,
  Cache: 2,
  Config: 3,
  Data: 4,
  Desktop: 6,
  Document: 7,
  Download: 8,
  Executable: 9,
  Font: 10,
  Home: 11,
  LocalData: 5,
  Log: 19,
  Picture: 12,
  Public: 13,
  Resource: 17,
  Runtime: 14,
  Temp: 20,
  Template: 15,
  Video: 16,
}

export class System {
  directories: { [key: string]: string }

  constructor() {
    this.directories = {}
    this.populateDirectories()
  }

  dirPath(dir: string): string {
    return this.directories[dir]
  }

  dirId(dir: string): number {
    return baseDirEnum[dir] || 0
  }

  async populateDirectories() {
    await Promise.all(
      Object.entries(path).map(async ([key, value]) => {
        if (key.match(/base/i)) {
          return ''
        }

        if (key.match(/Dir$/) && typeof value === 'function') {
          key = upperCamelCase(key.replace(/Dir$/, ''))
          this.directories[key] = await (value as () => Promise<string>)()
        }
      }),
    )
  }

  async getParentDir(filePath: string): Promise<string | undefined> {
    if (Object.keys(this.directories).length === 0) {
      await this.populateDirectories()
    }

    const directoriesByPathLength = Object.entries(this.directories).sort(
      (a, b) => b[1].length - a[1].length,
    )

    for (const [key, dir] of directoriesByPathLength) {
      if (filePath.match(dir)) {
        return key
      }
    }
  }
}
