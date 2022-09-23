interface Track {
  id: number
  name: string
  selection?: number
  files: TrackFileSkeleton[]
}

interface TrackFileSkeleton {
  id: number
  parentId: number
  name: string
  path: string
  extension: string
}

interface DropFileSkeleton {
  name: string
  path: string
  extension: string
  data: Uint8Array
}

interface DropFile {
  lastModified: number
  lastModifiedDate: Date
  name: string
  path: string
  size: number
  type: string
  webkitRelativePath: string
}

interface TrackFile extends DropFileSkeleton {
  id: number
  parentId: number
}

export { Track, TrackFile, TrackFileSkeleton, DropFile, DropFileSkeleton }
