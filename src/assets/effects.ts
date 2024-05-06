import { CompressorSettingsInterface, Effect, ReverbSettingsInterface } from '../typings/effects'

class ReverbSettings implements ReverbSettingsInterface {
  decay: number
  preDelay: number
  mix: number
  active: boolean

  constructor(reverb?: ReverbSettingsInterface) {
    let defaults: ReverbSettingsInterface = { decay: 20, preDelay: 0, mix: 0.2, active: false }
    if (reverb) defaults = reverb

    // ToDo: Object.assign is better if I can get it to
    // play nicely with ts
    // Object.assign(this, defaults)

    this.decay = defaults.decay
    this.preDelay = defaults.preDelay
    this.mix = defaults.mix
    this.active = defaults.active
  }

  get wet() {
    return this.mix
  }
}

class CompressorSettings implements CompressorSettingsInterface {
  attack: number
  knee: number
  ratio: number
  release: number
  threshold: number
  active: boolean

  constructor(compressor?: CompressorSettingsInterface) {
    let defaults: CompressorSettingsInterface = {
      threshold: -15,
      attack: 0.2,
      knee: 0,
      ratio: 2,
      release: 0.1,
      active: false,
    }
    if (compressor) defaults = compressor

    // ToDo: Object.assign is better if I can get it to
    // play nicely with ts
    // Object.assign(this, defaults)
    this.attack = defaults.attack
    this.knee = defaults.knee
    this.ratio = defaults.ratio
    this.release = defaults.release
    this.threshold = defaults.threshold
    this.active = defaults.active
  }
}

type EffectSettingsType = ReverbSettings | CompressorSettings

class EffectSettings {
  id: number
  type: Effect
  effect: EffectSettingsType | undefined

  constructor(type: Effect, id: number, effect?: EffectSettingsType) {
    this.id = id
    this.type = type
    this.effect = getEffect(type, effect)
  }
}

const isReverbSetting = (effect: EffectSettingsType) => {
  const r = new ReverbSettings()
  let pass = true
  for (const key in r) {
    if (!Object.hasOwn(effect, key)) {
      pass = false
      break
    }
  }
  return pass
}

const isCompressorSetting = (effect: EffectSettingsType) => {
  const c = new CompressorSettings()
  let pass = true
  for (const key in c) {
    if (!Object.hasOwn(effect, key)) {
      pass = false
      break
    }
  }
  return pass
}

const getEffect = (
  effectType: Effect,
  effect?: EffectSettingsType,
): EffectSettingsType | undefined => {
  if ((effect && isReverbSetting(effect)) || (!effect && effectType === 'Reverb'))
    return new ReverbSettings(effect as ReverbSettingsInterface)
  if ((effect && isCompressorSetting(effect)) || (!effect && effectType === 'Compressor'))
    return new CompressorSettings(effect as CompressorSettingsInterface)
}

export { ReverbSettings, CompressorSettings, EffectSettings }
