import type {
  AudioEffectKey,
  AudioEffectPayload,
  AudioEffectType,
  BasicReverbSettings,
  CompressorSettings,
  ConvolutionReverbSettings,
  DiffusionReverbSettings,
  DistortionSettings,
  GainSettings,
  HighPassFilterSettings,
  LimiterSettings,
  LowPassFilterSettings,
} from '../typings/effects'

export interface EffectChainItem {
  id: number
  effect: AudioEffectPayload
}

const effectTypeToKey: Record<AudioEffectType, AudioEffectKey> = {
  BasicReverb: 'BasicReverbSettings',
  DiffusionReverb: 'DiffusionReverbSettings',
  ConvolutionReverb: 'ConvolutionReverbSettings',
  LowPassFilter: 'LowPassFilterSettings',
  HighPassFilter: 'HighPassFilterSettings',
  Distortion: 'DistortionSettings',
  Gain: 'GainSettings',
  Compressor: 'CompressorSettings',
  Limiter: 'LimiterSettings',
}

const effectKeyToType: Record<AudioEffectKey, AudioEffectType> = {
  BasicReverbSettings: 'BasicReverb',
  DiffusionReverbSettings: 'DiffusionReverb',
  ConvolutionReverbSettings: 'ConvolutionReverb',
  LowPassFilterSettings: 'LowPassFilter',
  HighPassFilterSettings: 'HighPassFilter',
  DistortionSettings: 'Distortion',
  GainSettings: 'Gain',
  CompressorSettings: 'Compressor',
  LimiterSettings: 'Limiter',
}

export const effectTypes: AudioEffectType[] = [
  'BasicReverb',
  'DiffusionReverb',
  'ConvolutionReverb',
  'Compressor',
  'Limiter',
  'LowPassFilter',
  'HighPassFilter',
  'Distortion',
  'Gain',
]

export const effectTypeLabels: Record<AudioEffectType, string> = {
  BasicReverb: 'Basic Reverb',
  DiffusionReverb: 'Diffusion Reverb',
  ConvolutionReverb: 'Convolution Reverb',
  Compressor: 'Compressor',
  Limiter: 'Limiter',
  LowPassFilter: 'Low-Pass Filter',
  HighPassFilter: 'High-Pass Filter',
  Distortion: 'Distortion',
  Gain: 'Gain',
}

const defaultBasicReverb = (): BasicReverbSettings => ({
  enabled: true,
  mix: 0.25,
  duration_ms: 120,
  amplitude: 0.7,
})

const defaultDiffusionReverb = (): DiffusionReverbSettings => ({
  enabled: true,
  mix: 0.35,
  pre_delay_ms: 12,
  room_size_ms: 48,
  decay: 0.72,
  damping: 0.35,
  diffusion: 0.72,
})

const defaultConvolutionReverb = (): ConvolutionReverbSettings => ({
  enabled: true,
  dry_wet: 0.25,
  impulse_response: null,
  impulse_response_attachment: null,
  impulse_response_path: null,
  impulse_response_tail_db: null,
  impulse_response_tail: null,
})

const defaultLowPassFilter = (): LowPassFilterSettings => ({
  enabled: true,
  freq_hz: 1200,
  q: 0.7,
})

const defaultHighPassFilter = (): HighPassFilterSettings => ({
  enabled: true,
  freq_hz: 120,
  q: 0.7,
})

const defaultDistortion = (): DistortionSettings => ({
  enabled: true,
  gain: 1.0,
  threshold: 1.0,
})

const defaultGain = (): GainSettings => ({
  enabled: true,
  gain: 1.0,
})

const defaultCompressor = (): CompressorSettings => ({
  enabled: true,
  threshold_db: -18.0,
  ratio: 4.0,
  attack_ms: 10.0,
  release_ms: 100.0,
  makeup_gain_db: 0.0,
})

const defaultLimiter = (): LimiterSettings => ({
  enabled: true,
  threshold_db: -1.0,
  knee_width_db: 4.0,
  attack_ms: 5.0,
  release_ms: 100.0,
})

export const createEffect = (type: AudioEffectType): AudioEffectPayload => {
  switch (type) {
    case 'BasicReverb':
      return { BasicReverbSettings: defaultBasicReverb() }
    case 'DiffusionReverb':
      return { DiffusionReverbSettings: defaultDiffusionReverb() }
    case 'ConvolutionReverb':
      return { ConvolutionReverbSettings: defaultConvolutionReverb() }
    case 'LowPassFilter':
      return { LowPassFilterSettings: defaultLowPassFilter() }
    case 'HighPassFilter':
      return { HighPassFilterSettings: defaultHighPassFilter() }
    case 'Distortion':
      return { DistortionSettings: defaultDistortion() }
    case 'Gain':
      return { GainSettings: defaultGain() }
    case 'Limiter':
      return { LimiterSettings: defaultLimiter() }
    case 'Compressor':
      return { CompressorSettings: defaultCompressor() }
    default:
      return { CompressorSettings: defaultCompressor() }
  }
}

export const getEffectKey = (effect: AudioEffectPayload): AudioEffectKey => {
  if ('BasicReverbSettings' in effect) return 'BasicReverbSettings'
  if ('DiffusionReverbSettings' in effect) return 'DiffusionReverbSettings'
  if ('ConvolutionReverbSettings' in effect) return 'ConvolutionReverbSettings'
  if ('LowPassFilterSettings' in effect) return 'LowPassFilterSettings'
  if ('HighPassFilterSettings' in effect) return 'HighPassFilterSettings'
  if ('DistortionSettings' in effect) return 'DistortionSettings'
  if ('GainSettings' in effect) return 'GainSettings'
  if ('LimiterSettings' in effect) return 'LimiterSettings'
  return 'CompressorSettings'
}

export const getEffectType = (effect: AudioEffectPayload): AudioEffectType => {
  const key = getEffectKey(effect)
  return effectKeyToType[key]
}

export const getEffectLabel = (effect: AudioEffectPayload): string => {
  return effectTypeLabels[getEffectType(effect)]
}

export const normalizeEffect = (effect: AudioEffectPayload): AudioEffectPayload => {
  if ('BasicReverbSettings' in effect) {
    return { BasicReverbSettings: { ...defaultBasicReverb(), ...effect.BasicReverbSettings } }
  }
  if ('DiffusionReverbSettings' in effect) {
    return {
      DiffusionReverbSettings: {
        ...defaultDiffusionReverb(),
        ...effect.DiffusionReverbSettings,
      },
    }
  }
  if ('ConvolutionReverbSettings' in effect) {
    return {
      ConvolutionReverbSettings: {
        ...defaultConvolutionReverb(),
        ...effect.ConvolutionReverbSettings,
      },
    }
  }
  if ('LowPassFilterSettings' in effect) {
    return { LowPassFilterSettings: { ...defaultLowPassFilter(), ...effect.LowPassFilterSettings } }
  }
  if ('HighPassFilterSettings' in effect) {
    return {
      HighPassFilterSettings: { ...defaultHighPassFilter(), ...effect.HighPassFilterSettings },
    }
  }
  if ('DistortionSettings' in effect) {
    return { DistortionSettings: { ...defaultDistortion(), ...effect.DistortionSettings } }
  }
  if ('GainSettings' in effect) {
    return { GainSettings: { ...defaultGain(), ...effect.GainSettings } }
  }
  if ('LimiterSettings' in effect) {
    return { LimiterSettings: { ...defaultLimiter(), ...effect.LimiterSettings } }
  }

  return { CompressorSettings: { ...defaultCompressor(), ...effect.CompressorSettings } }
}

export const effectKeyFromType = (type: AudioEffectType): AudioEffectKey => effectTypeToKey[type]

export const isEffectType = (effect: AudioEffectPayload, type: AudioEffectType): boolean => {
  return getEffectType(effect) === type
}

export const effectChainFromPayload = (effects: AudioEffectPayload[]): EffectChainItem[] => {
  return effects.map((effect, index) => ({
    id: index + 1,
    effect: normalizeEffect(effect),
  }))
}
