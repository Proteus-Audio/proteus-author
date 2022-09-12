import { Effect } from '../typings/effects'

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

class EffectSettings {
  id: number
  type: Effect
  effect: EffectSettingsType | undefined

  constructor(type: Effect, id: number) {
    this.id = id
    this.type = type
    this.effect = getEffect(type)
  }
}

const getEffect = (effectType: Effect): EffectSettingsType | undefined => {
  if (effectType === 'Reverb') return new ReverbSettings()
  if (effectType === 'Compressor') return new CompressorSettings()
}

export { ReverbSettings, CompressorSettings, EffectSettings }
