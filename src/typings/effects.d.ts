export type AudioEffectKey =
  | 'BasicReverbSettings'
  | 'DiffusionReverbSettings'
  | 'ConvolutionReverbSettings'
  | 'LowPassFilterSettings'
  | 'HighPassFilterSettings'
  | 'DistortionSettings'
  | 'CompressorSettings'
  | 'LimiterSettings'

export type AudioEffectType =
  | 'BasicReverb'
  | 'DiffusionReverb'
  | 'ConvolutionReverb'
  | 'LowPassFilter'
  | 'HighPassFilter'
  | 'Distortion'
  | 'Compressor'
  | 'Limiter'

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

export type EffectSettings =
  | BasicReverbSettings
  | ConvolutionReverbSettings
  | LowPassFilterSettings
  | HighPassFilterSettings
  | DistortionSettings
  | CompressorSettings
  | LimiterSettings

export type AudioEffectPayload =
  | { BasicReverbSettings: BasicReverbSettings }
  | { DiffusionReverbSettings: DiffusionReverbSettings }
  | { ConvolutionReverbSettings: ConvolutionReverbSettings }
  | { LowPassFilterSettings: LowPassFilterSettings }
  | { HighPassFilterSettings: HighPassFilterSettings }
  | { DistortionSettings: DistortionSettings }
  | { CompressorSettings: CompressorSettings }
  | { LimiterSettings: LimiterSettings }
