interface Track {
  id: number
  name: string
  selection?: string
  file_ids: string[]
  // files: TrackFile[]
}

interface TrackFileSkeleton {
  id: string
  parentId: number
  name: string
  path: string
  extension: string
}

interface DropFileSkeleton {
  id: string
  name: string
  path: string
  extension: string
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

interface LegacyTrackFile extends DropFileSkeleton {
  parentId: number
}

export { Track, TrackFile, TrackFileSkeleton, DropFile, DropFileSkeleton }
