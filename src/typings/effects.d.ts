export type AudioEffectKey =
  | 'BasicReverbSettings'
  | 'DiffusionReverbSettings'
  | 'ConvolutionReverbSettings'
  | 'LowPassFilterSettings'
  | 'HighPassFilterSettings'
  | 'DistortionSettings'
  | 'GainSettings'
  | 'CompressorSettings'
  | 'LimiterSettings'
  | 'MultibandEqSettings'

export type AudioEffectType =
  | 'BasicReverb'
  | 'DiffusionReverb'
  | 'ConvolutionReverb'
  | 'LowPassFilter'
  | 'HighPassFilter'
  | 'Distortion'
  | 'Gain'
  | 'Compressor'
  | 'Limiter'
  | 'MultibandEq'

export interface BasicReverbSettings {
  enabled: boolean
  mix: number
  duration_ms: number
  amplitude: number
}

export interface DiffusionReverbSettings {
  enabled: boolean
  mix: number
  pre_delay_ms: number
  room_size_ms: number
  decay: number
  damping: number
  diffusion: number
}

export interface ConvolutionReverbSettings {
  enabled: boolean
  dry_wet: number
  impulse_response?: string | null
  impulse_response_attachment?: string | null
  impulse_response_path?: string | null
  impulse_response_tail_db?: number | null
  impulse_response_tail?: number | null
}

export interface LowPassFilterSettings {
  enabled: boolean
  freq_hz: number
  q: number
}

export interface HighPassFilterSettings {
  enabled: boolean
  freq_hz: number
  q: number
}

export interface DistortionSettings {
  enabled: boolean
  gain: number
  threshold: number
}

export interface GainSettings {
  enabled: boolean
  gain: number
}

export interface CompressorSettings {
  enabled: boolean
  threshold_db: number
  ratio: number
  attack_ms: number
  release_ms: number
  makeup_gain_db: number
}

export interface LimiterSettings {
  enabled: boolean
  threshold_db: number
  knee_width_db: number
  attack_ms: number
  release_ms: number
}

export interface EqPointSettings {
  freq_hz: number
  q: number
  gain_db: number
}

export type LowEdgeFilterSettings =
  | {
      type: 'high_pass'
      freq_hz: number
      q: number
    }
  | {
      type: 'low_shelf'
      freq_hz: number
      q: number
      gain_db: number
    }

export type HighEdgeFilterSettings =
  | {
      type: 'low_pass'
      freq_hz: number
      q: number
    }
  | {
      type: 'high_shelf'
      freq_hz: number
      q: number
      gain_db: number
    }

export interface MultibandEqSettings {
  enabled: boolean
  points: EqPointSettings[]
  low_edge?: LowEdgeFilterSettings | null
  high_edge?: HighEdgeFilterSettings | null
}

export type EffectSettings =
  | BasicReverbSettings
  | DiffusionReverbSettings
  | ConvolutionReverbSettings
  | LowPassFilterSettings
  | HighPassFilterSettings
  | DistortionSettings
  | GainSettings
  | CompressorSettings
  | LimiterSettings
  | MultibandEqSettings

export type AudioEffectPayload =
  | { BasicReverbSettings: BasicReverbSettings }
  | { DiffusionReverbSettings: DiffusionReverbSettings }
  | { ConvolutionReverbSettings: ConvolutionReverbSettings }
  | { LowPassFilterSettings: LowPassFilterSettings }
  | { HighPassFilterSettings: HighPassFilterSettings }
  | { DistortionSettings: DistortionSettings }
  | { GainSettings: GainSettings }
  | { CompressorSettings: CompressorSettings }
  | { LimiterSettings: LimiterSettings }
  | { MultibandEqSettings: MultibandEqSettings }

export interface LevelSnapshot {
  peak: number[]
  rms: number[]
}

export interface EffectLevelSnapshot {
  input: LevelSnapshot
  output: LevelSnapshot
}

export interface FrequencyResponsePoint {
  freq_hz: number
  gain_db: number
}

export interface FilterResponseCurve {
  composite: FrequencyResponsePoint[]
  per_band: FrequencyResponsePoint[][]
}

export interface BandLevels {
  bands_db: number[]
  band_centers_hz: number[]
}

export interface EffectBandSnapshot {
  input: BandLevels
  output: BandLevels
}
