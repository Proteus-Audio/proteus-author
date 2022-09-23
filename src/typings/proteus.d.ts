import { EffectSettings } from '../public/effects'

type AlertType = 'success' | 'warning' | 'info' | 'error'

interface Alert {
  contents: string
  type: AlertType
  autoClose: boolean
}

type AlertClass = 'fresh' | 'stale'

interface AlertView extends Alert {
  class: AlertClass
  added: Date
}

interface ProjectHead {
  name: string
  path: string
}

interface TrackSkeleton {
  id: number
  name: string
  files: {
    id: number
    path: string
    name: string
    extension: string
  }[]
}

interface ProjectSkeleton {
  location?: string
  name?: string
  tracks: TrackSkeleton[]
  effects: EffectSettings[]
}

export { Alert, AlertType, AlertClass, AlertView, ProjectSkeleton, TrackSkeleton, ProjectHead }
