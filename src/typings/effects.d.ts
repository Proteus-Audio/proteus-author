type Effect = 'Compressor' | 'Reverb'

interface ReverbSettingsInterface {
  decay: number
  preDelay: number
  mix: number
  active: boolean
}

interface CompressorSettingsInterface {
  attack: number
  knee: number
  ratio: number
  release: number
  threshold: number
  active: boolean
}

type EffectSettings = ReverbSettingsInterface | CompressorSettingsInterface

interface EffectSkeleton {
  id: number
  type: Effect
  effect: ReverbSettingsInterface | CompressorSettingsInterface | undefined
}

export { Effect, ReverbSettingsInterface, CompressorSettingsInterface, EffectSettings, EffectSkeleton }
