import type {
  AudioEffectKey,
  AudioEffectPayload,
  AudioEffectType,
  BasicReverbSettings,
  CompressorSettings,
  ConvolutionReverbSettings,
  DistortionSettings,
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
  ConvolutionReverb: 'ConvolutionReverbSettings',
  LowPassFilter: 'LowPassFilterSettings',
  HighPassFilter: 'HighPassFilterSettings',
  Distortion: 'DistortionSettings',
  Compressor: 'CompressorSettings',
  Limiter: 'LimiterSettings',
}

const effectKeyToType: Record<AudioEffectKey, AudioEffectType> = {
  BasicReverbSettings: 'BasicReverb',
  ConvolutionReverbSettings: 'ConvolutionReverb',
  LowPassFilterSettings: 'LowPassFilter',
  HighPassFilterSettings: 'HighPassFilter',
  DistortionSettings: 'Distortion',
  CompressorSettings: 'Compressor',
  LimiterSettings: 'Limiter',
}

export const effectTypes: AudioEffectType[] = [
  'BasicReverb',
  'ConvolutionReverb',
  'Compressor',
  'Limiter',
  'LowPassFilter',
  'HighPassFilter',
  'Distortion',
]

export const effectTypeLabels: Record<AudioEffectType, string> = {
  BasicReverb: 'Basic Reverb',
  ConvolutionReverb: 'Convolution Reverb',
  Compressor: 'Compressor',
  Limiter: 'Limiter',
  LowPassFilter: 'Low-Pass Filter',
  HighPassFilter: 'High-Pass Filter',
  Distortion: 'Distortion',
}

const defaultBasicReverb = (): BasicReverbSettings => ({
  enabled: true,
  mix: 0.0,
  duration_ms: 100,
  amplitude: 0.7,
})

const defaultConvolutionReverb = (): ConvolutionReverbSettings => ({
  enabled: true,
  dry_wet: 0.000001,
  impulse_response: null,
  impulse_response_attachment: null,
  impulse_response_path: null,
  impulse_response_tail_db: null,
  impulse_response_tail: null,
})

const defaultLowPassFilter = (): LowPassFilterSettings => ({
  enabled: false,
  freq_hz: 1000,
  q: 0.5,
})

const defaultHighPassFilter = (): HighPassFilterSettings => ({
  enabled: false,
  freq_hz: 1000,
  q: 0.5,
})

const defaultDistortion = (): DistortionSettings => ({
  enabled: false,
  gain: 1.0,
  threshold: 1.0,
})

const defaultCompressor = (): CompressorSettings => ({
  enabled: false,
  threshold_db: -18.0,
  ratio: 4.0,
  attack_ms: 10.0,
  release_ms: 100.0,
  makeup_gain_db: 0.0,
})

const defaultLimiter = (): LimiterSettings => ({
  enabled: false,
  threshold_db: -1.0,
  knee_width_db: 4.0,
  attack_ms: 5.0,
  release_ms: 100.0,
})

export const createEffect = (type: AudioEffectType): AudioEffectPayload => {
  switch (type) {
    case 'BasicReverb':
      return { BasicReverbSettings: defaultBasicReverb() }
    case 'ConvolutionReverb':
      return { ConvolutionReverbSettings: defaultConvolutionReverb() }
    case 'LowPassFilter':
      return { LowPassFilterSettings: defaultLowPassFilter() }
    case 'HighPassFilter':
      return { HighPassFilterSettings: defaultHighPassFilter() }
    case 'Distortion':
      return { DistortionSettings: defaultDistortion() }
    case 'Limiter':
      return { LimiterSettings: defaultLimiter() }
    case 'Compressor':
    default:
      return { CompressorSettings: defaultCompressor() }
  }
}

export const getEffectKey = (effect: AudioEffectPayload): AudioEffectKey => {
  return Object.keys(effect)[0] as AudioEffectKey
}

export const getEffectType = (effect: AudioEffectPayload): AudioEffectType => {
  const key = getEffectKey(effect)
  return effectKeyToType[key]
}

export const getEffectLabel = (effect: AudioEffectPayload): string => {
  return effectTypeLabels[getEffectType(effect)]
}

export const normalizeEffect = (effect: AudioEffectPayload): AudioEffectPayload => {
  const key = getEffectKey(effect)
  switch (key) {
    case 'BasicReverbSettings':
      return { BasicReverbSettings: { ...defaultBasicReverb(), ...effect[key] } }
    case 'ConvolutionReverbSettings':
      return { ConvolutionReverbSettings: { ...defaultConvolutionReverb(), ...effect[key] } }
    case 'LowPassFilterSettings':
      return { LowPassFilterSettings: { ...defaultLowPassFilter(), ...effect[key] } }
    case 'HighPassFilterSettings':
      return { HighPassFilterSettings: { ...defaultHighPassFilter(), ...effect[key] } }
    case 'DistortionSettings':
      return { DistortionSettings: { ...defaultDistortion(), ...effect[key] } }
    case 'LimiterSettings':
      return { LimiterSettings: { ...defaultLimiter(), ...effect[key] } }
    case 'CompressorSettings':
    default:
      return { CompressorSettings: { ...defaultCompressor(), ...effect[key] } }
  }
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
