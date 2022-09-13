type Effect = 'Compressor' | 'Reverb'

class ReverbSettings {
  decay: number
  preDelay: number
  mix: number
  ready: boolean

  constructor() {
    this.decay = 20
    this.preDelay = 0
    this.mix = 0.2
    this.ready = false
  }

  get wet() {
    return this.mix
  }
}

class CompressorSettings {
  attack: number
  knee: number
  ratio: number
  release: number
  threshold: number

  constructor() {
    this.threshold = -15
    this.attack = 0.2
    this.knee = 0
    this.ratio = 2
    this.release = 0.1
  }
}

type EffectSettingsType = ReverbSettings | CompressorSettings

interface EffectSettings {
  id: number
  type: Effect
  effect: EffectSettingsType | undefined
}

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
  effects: EffectSettings[]
}

export { Project, TrackSkeleton }
