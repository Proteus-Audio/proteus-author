import { dialog } from 'electron'
import { existsSync, mkdirSync, writeFile } from 'fs'
import { readJson } from 'fs-extra'
import { copyFile } from 'node:fs/promises'
import { sep } from 'path'

interface TrackSkeleton {
  id: number
  name: string
  files: {
    id: number
    path: string
    name: string
  }[]
}

interface Project {
  location?: string
  name?: string
  tracks: TrackSkeleton[]
}

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
    if (!existsSync(dirString)) {
      mkdirSync(dirString)
    }
  }
}

const save = async (
  tracks: TrackSkeleton[],
  fileLocation: string,
  fileName: string,
): Promise<Project> => {
  const exitTracks: TrackSkeleton[] = []
  const promises: Promise<void>[] = []
  const trackDir = fileLocation.replace(/[\\/]$/, '')

  if (tracks && tracks.length > 0) {
    console.log(trackDir)
    tracks.forEach((t) => {
      const track: TrackSkeleton = { id: t.id, name: t.name, files: [] }
      t.files.forEach((file) => {
        const filePath = `/track${track.id}/${file.name}`
        console.log(`Saving ${trackDir}${filePath}`)
        promises.push(copyFilesMakeDirs(file.path, `${trackDir}${filePath}`))
        track.files.push({ id: file.id, name: file.name, path: filePath })
      })
      exitTracks.push(track)
    })
  }

  await Promise.all(promises)

  if (!/\.protproject/i.test(fileName)) fileName += '.protproject'
  writeFile(fileLocation + sep + fileName, JSON.stringify(exitTracks), () => {
    console.log('created')
  })

  return { location: fileLocation, tracks: exitTracks }
}

const loadData = async (fileLocation: string, fileName: string): Promise<TrackSkeleton[]> => {
  if (!/\.protproject/i.test(fileName)) fileName += '.protproject'
  try {
    const trackDir = fileLocation.replace(/[\\/]$/, '')
    const details: TrackSkeleton[] = await readJson(fileLocation + sep + fileName)

    details.forEach((skeleton) => {
      skeleton.files.forEach((f) => {
        f.path = trackDir + f.path
      })
    })
    return details
  } catch (err) {
    console.error(err)
    return []
  }
}

const load = async (filePath?: string): Promise<Project> => {
  let fileLocation = filePath

  if (!fileLocation) {
    const chosenLocation = await dialog.showOpenDialog({
      filters: [{ name: 'Prot Project', extensions: ['.protproject'] }],
      properties: ['openFile'],
    })
    fileLocation = chosenLocation.filePaths[0]
    if (chosenLocation.canceled) return { tracks: [], location: fileLocation }
  }

  const fileName = (fileLocation.match(/[^\\/]+$/) || [''])[0]
  const fileDir = fileLocation.replace(fileName, '')

  return { tracks: await loadData(fileDir, fileName), location: fileLocation, name: fileName }
}

export { save, loadData, load, Project }
