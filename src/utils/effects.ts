import type {
  AudioEffectKey,
  AudioEffectPayload,
  AudioEffectType,
  BasicReverbSettings,
  CompressorSettings,
  ConvolutionReverbSettings,
  DiffusionReverbSettings,
  DistortionSettings,
  EqPointSettings,
  GainSettings,
  HighEdgeFilterSettings,
  HighPassFilterSettings,
  LimiterSettings,
  LowPassFilterSettings,
  LowEdgeFilterSettings,
  MultibandEqSettings,
} from '../typings/effects'

export interface EffectChainItem {
  id: number
  effect: AudioEffectPayload
}

const DB_MIN_FLOOR = -120
const DB_SUFFIX = 'db'

const linearToDb = (linear: number): number => 20 * Math.log10(Math.max(linear, 1e-8))
const formatDb = (db: number): string => {
  const rounded = Number(db.toFixed(2))
  return `${rounded}${DB_SUFFIX}`
}

const effectTypeToKey: Record<AudioEffectType, AudioEffectKey> = {
  BasicReverb: 'BasicReverbSettings',
  DiffusionReverb: 'DiffusionReverbSettings',
  ConvolutionReverb: 'ConvolutionReverbSettings',
  LowPassFilter: 'LowPassFilterSettings',
  HighPassFilter: 'HighPassFilterSettings',
  MultibandEq: 'MultibandEqSettings',
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
  MultibandEqSettings: 'MultibandEq',
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
  'MultibandEq',
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
  MultibandEq: 'Multiband EQ',
  Distortion: 'Distortion',
  Gain: 'Gain',
}

export const effectDescriptions: Record<AudioEffectType, string> = {
  BasicReverb: 'Short algorithmic wash with direct control over mix, decay, and amplitude.',
  DiffusionReverb: 'Diffuse ambience with pre-delay, room shaping, and damping controls.',
  ConvolutionReverb: 'Impulse-response ambience with file-backed loading and dry/wet blending.',
  Compressor: 'Dynamic range control with threshold, ratio, timing, and makeup gain.',
  Limiter: 'Fast peak control with ceiling, knee width, and release behaviour.',
  LowPassFilter: 'Analytical low-pass response with live filter monitoring.',
  HighPassFilter: 'Analytical high-pass response with live filter monitoring.',
  MultibandEq: 'Parametric EQ with optional edge filters and analytical response guidance.',
  Distortion: 'Gain-staged clipping curve with threshold-driven saturation.',
  Gain: 'Output trim with immediate delta and live level feedback.',
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
  gain: 0.0,
  threshold: 0.0,
})

const defaultGain = (): GainSettings => ({
  enabled: true,
  gain: 0.0,
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

export const createEqPoint = (freq_hz: number, gain_db = 0, q = 0.8): EqPointSettings => ({
  freq_hz,
  q,
  gain_db,
})

export const createLowEdgeFilter = (type: LowEdgeFilterSettings['type']): LowEdgeFilterSettings => {
  if (type === 'low_shelf') {
    return {
      type: 'low_shelf',
      freq_hz: 80,
      q: 0.8,
      gain_db: 0,
    }
  }

  return {
    type: 'high_pass',
    freq_hz: 50,
    q: 0.7,
  }
}

export const createHighEdgeFilter = (
  type: HighEdgeFilterSettings['type'],
): HighEdgeFilterSettings => {
  if (type === 'high_shelf') {
    return {
      type: 'high_shelf',
      freq_hz: 10_000,
      q: 0.8,
      gain_db: 0,
    }
  }

  return {
    type: 'low_pass',
    freq_hz: 14_000,
    q: 0.7,
  }
}

const defaultMultibandEq = (): MultibandEqSettings => ({
  enabled: true,
  points: [createEqPoint(120, 0, 0.8), createEqPoint(1_000, 0, 0.8), createEqPoint(8_000, 0, 0.8)],
  low_edge: null,
  high_edge: null,
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
    case 'MultibandEq':
      return { MultibandEqSettings: defaultMultibandEq() }
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
  if ('MultibandEqSettings' in effect) return 'MultibandEqSettings'
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
  if ('MultibandEqSettings' in effect) {
    const normalized = { ...defaultMultibandEq(), ...effect.MultibandEqSettings }
    return {
      MultibandEqSettings: {
        ...normalized,
        points:
          normalized.points?.map((point) => ({
            ...createEqPoint(1_000),
            ...point,
          })) ?? defaultMultibandEq().points,
        low_edge: normalized.low_edge ?? null,
        high_edge: normalized.high_edge ?? null,
      },
    }
  }
  if ('DistortionSettings' in effect) {
    const normalized = { ...defaultDistortion(), ...effect.DistortionSettings }
    return {
      DistortionSettings: {
        ...normalized,
        gain: linearToDb(normalized.gain),
        threshold: linearToDb(normalized.threshold),
      },
    }
  }
  if ('GainSettings' in effect) {
    const normalized = { ...defaultGain(), ...effect.GainSettings }
    return {
      GainSettings: {
        ...normalized,
        gain: linearToDb(normalized.gain),
      },
    }
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

export const effectSupportsFrequencyResponse = (type: AudioEffectType): boolean => {
  return type === 'LowPassFilter' || type === 'HighPassFilter' || type === 'MultibandEq'
}

export const effectSupportsSpectralOverlay = (type: AudioEffectType): boolean => {
  return effectSupportsFrequencyResponse(type)
}

export const getEffectEnabled = (effect: AudioEffectPayload): boolean => {
  if ('BasicReverbSettings' in effect) return effect.BasicReverbSettings.enabled
  if ('DiffusionReverbSettings' in effect) return effect.DiffusionReverbSettings.enabled
  if ('ConvolutionReverbSettings' in effect) return effect.ConvolutionReverbSettings.enabled
  if ('LowPassFilterSettings' in effect) return effect.LowPassFilterSettings.enabled
  if ('HighPassFilterSettings' in effect) return effect.HighPassFilterSettings.enabled
  if ('MultibandEqSettings' in effect) return effect.MultibandEqSettings.enabled
  if ('DistortionSettings' in effect) return effect.DistortionSettings.enabled
  if ('GainSettings' in effect) return effect.GainSettings.enabled
  if ('LimiterSettings' in effect) return effect.LimiterSettings.enabled
  return effect.CompressorSettings.enabled
}

export const setEffectEnabled = (effect: AudioEffectPayload, enabled: boolean) => {
  if ('BasicReverbSettings' in effect) effect.BasicReverbSettings.enabled = enabled
  else if ('DiffusionReverbSettings' in effect) effect.DiffusionReverbSettings.enabled = enabled
  else if ('ConvolutionReverbSettings' in effect) {
    effect.ConvolutionReverbSettings.enabled = enabled
  } else if ('LowPassFilterSettings' in effect) effect.LowPassFilterSettings.enabled = enabled
  else if ('HighPassFilterSettings' in effect) effect.HighPassFilterSettings.enabled = enabled
  else if ('MultibandEqSettings' in effect) effect.MultibandEqSettings.enabled = enabled
  else if ('DistortionSettings' in effect) effect.DistortionSettings.enabled = enabled
  else if ('GainSettings' in effect) effect.GainSettings.enabled = enabled
  else if ('LimiterSettings' in effect) effect.LimiterSettings.enabled = enabled
  else effect.CompressorSettings.enabled = enabled
}

const formatSignedNumber = (value: number, digits = 1) => {
  const rounded = Number(value.toFixed(digits))
  return `${rounded > 0 ? '+' : ''}${rounded}`
}

export const formatDbValue = (value: number, digits = 1): string =>
  `${formatSignedNumber(value, digits)} dB`

export const formatFrequencyValue = (value: number): string => {
  if (value >= 1000) {
    const rounded = Number((value / 1000).toFixed(value >= 10_000 ? 1 : 2))
    return `${rounded} kHz`
  }
  return `${Math.round(value)} Hz`
}

const describeEdge = (
  edge: LowEdgeFilterSettings | HighEdgeFilterSettings | null | undefined,
): string | null => {
  if (!edge) return null
  if (edge.type === 'high_pass') return `HP ${formatFrequencyValue(edge.freq_hz)}`
  if (edge.type === 'low_pass') return `LP ${formatFrequencyValue(edge.freq_hz)}`
  if (edge.type === 'low_shelf') return `Low Shelf ${formatDbValue(edge.gain_db)}`
  return `High Shelf ${formatDbValue(edge.gain_db)}`
}

export const getEffectSummary = (effect: AudioEffectPayload): string => {
  if ('BasicReverbSettings' in effect) {
    return `${Math.round(effect.BasicReverbSettings.mix * 100)}% mix · ${Math.round(effect.BasicReverbSettings.duration_ms)} ms`
  }

  if ('DiffusionReverbSettings' in effect) {
    return `${Math.round(effect.DiffusionReverbSettings.mix * 100)}% mix · ${Math.round(effect.DiffusionReverbSettings.room_size_ms)} ms room`
  }

  if ('ConvolutionReverbSettings' in effect) {
    const ir =
      effect.ConvolutionReverbSettings.impulse_response_path ||
      effect.ConvolutionReverbSettings.impulse_response_attachment ||
      effect.ConvolutionReverbSettings.impulse_response
    return `${Math.round(effect.ConvolutionReverbSettings.dry_wet * 100)}% wet · ${ir ? 'IR loaded' : 'No IR'}`
  }

  if ('CompressorSettings' in effect) {
    return `${formatDbValue(effect.CompressorSettings.threshold_db)} · ${effect.CompressorSettings.ratio.toFixed(1)}:1`
  }

  if ('LimiterSettings' in effect) {
    return `${formatDbValue(effect.LimiterSettings.threshold_db)} ceiling`
  }

  if ('LowPassFilterSettings' in effect) {
    return `${formatFrequencyValue(effect.LowPassFilterSettings.freq_hz)} · Q ${effect.LowPassFilterSettings.q.toFixed(2)}`
  }

  if ('HighPassFilterSettings' in effect) {
    return `${formatFrequencyValue(effect.HighPassFilterSettings.freq_hz)} · Q ${effect.HighPassFilterSettings.q.toFixed(2)}`
  }

  if ('MultibandEqSettings' in effect) {
    const points = effect.MultibandEqSettings.points.length
    const low = describeEdge(effect.MultibandEqSettings.low_edge)
    const high = describeEdge(effect.MultibandEqSettings.high_edge)
    return [points === 1 ? '1 band' : `${points} bands`, low, high].filter(Boolean).join(' · ')
  }

  if ('DistortionSettings' in effect) {
    return `${formatDbValue(effect.DistortionSettings.gain)} drive · ${formatDbValue(effect.DistortionSettings.threshold)} clip`
  }

  if ('GainSettings' in effect) {
    return formatDbValue(effect.GainSettings.gain)
  }

  return effectTypeLabels[getEffectType(effect)]
}

export const effectChainFromPayload = (effects: AudioEffectPayload[]): EffectChainItem[] => {
  return effects.map((effect, index) => ({
    id: index + 1,
    effect: normalizeEffect(effect),
  }))
}

const normalizeDbForBackend = (value: number): number => {
  if (!Number.isFinite(value)) return 0
  return Math.max(DB_MIN_FLOOR, value)
}

export const serializeEffectForBackend = (effect: AudioEffectPayload): Record<string, unknown> => {
  if ('DistortionSettings' in effect) {
    const value = effect.DistortionSettings
    return {
      DistortionSettings: {
        ...value,
        gain: formatDb(normalizeDbForBackend(value.gain)),
        threshold: formatDb(normalizeDbForBackend(value.threshold)),
      },
    }
  }

  if ('GainSettings' in effect) {
    const value = effect.GainSettings
    return {
      GainSettings: {
        ...value,
        gain: formatDb(normalizeDbForBackend(value.gain)),
      },
    }
  }

  if ('CompressorSettings' in effect) {
    const value = effect.CompressorSettings
    return {
      CompressorSettings: {
        ...value,
        threshold_db: formatDb(normalizeDbForBackend(value.threshold_db)),
        makeup_gain_db: formatDb(normalizeDbForBackend(value.makeup_gain_db)),
      },
    }
  }

  if ('LimiterSettings' in effect) {
    const value = effect.LimiterSettings
    return {
      LimiterSettings: {
        ...value,
        threshold_db: formatDb(normalizeDbForBackend(value.threshold_db)),
        knee_width_db: formatDb(normalizeDbForBackend(value.knee_width_db)),
      },
    }
  }

  if ('ConvolutionReverbSettings' in effect) {
    const value = effect.ConvolutionReverbSettings
    return {
      ConvolutionReverbSettings: {
        ...value,
        impulse_response_tail_db:
          value.impulse_response_tail_db == null
            ? value.impulse_response_tail_db
            : formatDb(normalizeDbForBackend(value.impulse_response_tail_db)),
      },
    }
  }

  return effect as unknown as Record<string, unknown>
}

export const serializeEffectChainForBackend = (
  effects: AudioEffectPayload[],
): Record<string, unknown>[] => effects.map((effect) => serializeEffectForBackend(effect))
