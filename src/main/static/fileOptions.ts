import { dialog } from 'electron'
import { existsSync, mkdirSync, writeFile } from 'fs'
import { readJson } from 'fs-extra'
import { copyFile } from 'node:fs/promises'
import { sep } from 'path'
import { ProjectSkeleton as Project, TrackSkeleton } from '../../renderer/typings/proteus'

const copyFilesMakeDirs = async (src: string, dest: string): Promise<void> => {
  mkdirIfNone(dest)

  try {
    await copyFile(src, dest)
  } catch (error) {
    console.log('ERROR COPYING FILE:', error)
  }
}

const mkdirIfNone = (dir: string): void => {
  const destArr = dir.split(sep)
  let dirString = ''
  for (let i = 0; i < destArr.length; i++) {
    const dir = destArr[i]
    dirString += sep + dir
    const nonExistantDirectory = !existsSync(dirString) && !/\.[^/\\]+$/.test(dir)
    if (nonExistantDirectory) {
      mkdirSync(dirString)
    }
  }
}

const save = async (project: Project, fileLocation: string, fileName: string): Promise<Project> => {
  const fileData: Project = {
    name: fileName,
    location: fileLocation,
    tracks: [],
    effects: project.effects,
  }
  const promises: Promise<void>[] = []
  const trackDir = fileLocation.replace(/[\\/]$/, '')
  const tracks = project.tracks

  if (tracks.length > 0) {
    console.log(trackDir)
    tracks.forEach((t) => {
      const track: TrackSkeleton = { id: t.id, name: t.name, files: [] }
      t.files.forEach((file) => {
        const filePath = `/track${track.id}/${file.name}`
        console.log(`Saving ${trackDir}${filePath}`)
        promises.push(copyFilesMakeDirs(file.path, `${trackDir}${filePath}`))
        track.files.push({ id: file.id, name: file.name, path: filePath })
      })
      fileData.tracks.push(track)
    })
  }

  await Promise.all(promises)

  if (!/\.protproject/i.test(fileName)) fileName += '.protproject'
  writeFile(fileLocation + sep + fileName, JSON.stringify(fileData), () => {
    console.log('created')
  })

  return fileData
}

const loadData = async (fileLocation: string): Promise<Project | undefined> => {
  if (!/\.protproject/i.test(fileLocation)) fileLocation += '.protproject'

  const fileName = (fileLocation.match(/[^\\/]+$/) || [''])[0]
  const fileDir = fileLocation.replace(fileName, '')

  try {
    const fileData: Project = await readJson(fileDir + sep + fileName)

    fileData.tracks.forEach((skeleton) => {
      skeleton.files.forEach((f) => {
        f.path = fileDir + f.path
      })
    })
    return fileData
  } catch (err) {
    console.error(err)
    return
  }
}

const load = async (filePath?: string): Promise<Project | undefined> => {
  let fileLocation = filePath

  if (!fileLocation) {
    const chosenLocation = await dialog.showOpenDialog({
      filters: [{ name: 'Prot Project', extensions: ['.protproject'] }],
      properties: ['openFile'],
    })
    fileLocation = chosenLocation.filePaths[0]
    if (chosenLocation.canceled) return { tracks: [], effects: [], location: fileLocation }
  }

  return await loadData(fileLocation)
}

export { save, loadData, load, Project }
