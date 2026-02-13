import type { AudioEffectPayload } from './effects'
import type { DropFileSkeleton, Track } from './tracks'

export type AlertType = 'success' | 'warning' | 'info' | 'error'

export interface Alert {
  contents: string
  type: AlertType
  autoClose: boolean
}

export type AlertClass = 'fresh' | 'stale'

export interface AlertView extends Alert {
  class: AlertClass
  added: Date
}

export interface ProjectHead {
  name: string
  path?: string
}

// interface TrackSkeleton {
//   id: number
//   name: string
//   files: {
//     id: number
//     path: string
//     name: string
//     extension: string
//     peaks: [number, number][][]
//   }[]
// }

export interface TrackSkeleton {
  id: number
  name: string
  selection?: string
  file_ids: string[]
}

export interface ProjectSkeleton {
  location?: string
  name?: string
  tracks: Track[]
  effects: AudioEffectPayload[]
  files: DropFileSkeleton[]
}
