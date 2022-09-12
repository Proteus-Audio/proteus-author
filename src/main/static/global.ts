import { Project } from './fileOptions'

class EntryData {
  loaded: boolean
  index: number
  projects: { [key: string]: Project | undefined }

  constructor() {
    this.index = 0
    this.loaded = false
    this.projects = {}
  }
}

const entryData = new EntryData()

export { entryData }
