<template>
  <div class="effects-dialog" v-if="effect">
    <header class="dialog-header">
      <h2>{{ label }}</h2>
      <AnalogIndicator :state="enabledState" label="Active" color="green" />
    </header>

    <section class="analog-panel dialog-panel">
      <template v-if="type === 'BasicReverb'">
        <div class="control-grid">
          <AnalogToggle v-model="basicEnabled" label="Enabled" />
          <AnalogKnob v-model="basicMix" label="Mix" :min="0" :max="1" :step="0.01" />
          <AnalogKnob
            v-model="basicDuration"
            label="Duration"
            :min="0"
            :max="2000"
            :step="1"
            units="ms"
          />
          <AnalogKnob v-model="basicAmplitude" label="Amplitude" :min="0" :max="0.8" :step="0.01" />
        </div>
      </template>

      <template v-else-if="type === 'ConvolutionReverb'">
        <div class="control-grid">
          <AnalogToggle v-model="convolutionEnabled" label="Enabled" />
          <AnalogKnob v-model="convolutionMix" label="Dry/Wet" :min="0" :max="1" :step="0.01" />
          <AnalogKnob
            v-model="convolutionTailDb"
            label="Tail"
            :min="-120"
            :max="0"
            :step="1"
            units="dB"
          />
          <div class="ir-picker">
            <span class="analog-label">Impulse Response</span>
            <input
              class="ir-input"
              v-model="convolutionImpulse"
              placeholder="attachment:ir.wav or /path/to/ir.wav"
            />
            <div class="ir-actions">
              <el-button size="small" @click="pickImpulseResponse">Choose File</el-button>
              <el-button size="small" @click="clearImpulseResponse">Clear</el-button>
            </div>
          </div>
        </div>
      </template>

      <template v-else-if="type === 'Compressor'">
        <div class="control-grid">
          <AnalogToggle v-model="compressorEnabled" label="Enabled" />
          <AnalogKnob
            v-model="compressorThreshold"
            label="Threshold"
            :min="-60"
            :max="0"
            :step="0.5"
            units="dB"
          />
          <AnalogKnob v-model="compressorRatio" label="Ratio" :min="1" :max="20" :step="0.1" />
          <AnalogKnob
            v-model="compressorAttack"
            label="Attack"
            :min="1"
            :max="200"
            :step="1"
            units="ms"
          />
          <AnalogKnob
            v-model="compressorRelease"
            label="Release"
            :min="1"
            :max="500"
            :step="1"
            units="ms"
          />
          <AnalogKnob
            v-model="compressorMakeup"
            label="Makeup"
            :min="-12"
            :max="12"
            :step="0.5"
            units="dB"
          />
        </div>
      </template>

      <template v-else-if="type === 'Limiter'">
        <div class="control-grid">
          <AnalogToggle v-model="limiterEnabled" label="Enabled" />
          <AnalogKnob
            v-model="limiterThreshold"
            label="Threshold"
            :min="-20"
            :max="0"
            :step="0.5"
            units="dB"
          />
          <AnalogKnob
            v-model="limiterKnee"
            label="Knee"
            :min="0"
            :max="12"
            :step="0.5"
            units="dB"
          />
          <AnalogKnob
            v-model="limiterAttack"
            label="Attack"
            :min="1"
            :max="100"
            :step="1"
            units="ms"
          />
          <AnalogKnob
            v-model="limiterRelease"
            label="Release"
            :min="1"
            :max="500"
            :step="1"
            units="ms"
          />
        </div>
      </template>

      <template v-else-if="type === 'LowPassFilter'">
        <div class="control-grid">
          <AnalogToggle v-model="lowPassEnabled" label="Enabled" />
          <AnalogKnob
            v-model="lowPassFreq"
            label="Freq"
            :min="20"
            :max="20000"
            :step="10"
            units="Hz"
          />
          <AnalogKnob v-model="lowPassQ" label="Res" :min="0.1" :max="2" :step="0.01" />
        </div>
      </template>

      <template v-else-if="type === 'HighPassFilter'">
        <div class="control-grid">
          <AnalogToggle v-model="highPassEnabled" label="Enabled" />
          <AnalogKnob
            v-model="highPassFreq"
            label="Freq"
            :min="20"
            :max="20000"
            :step="10"
            units="Hz"
          />
          <AnalogKnob v-model="highPassQ" label="Res" :min="0.1" :max="2" :step="0.01" />
        </div>
      </template>

      <template v-else-if="type === 'Distortion'">
        <div class="control-grid">
          <AnalogToggle v-model="distortionEnabled" label="Enabled" />
          <AnalogKnob v-model="distortionGain" label="Gain" :min="0" :max="10" :step="0.1" />
          <AnalogKnob
            v-model="distortionThreshold"
            label="Threshold"
            :min="0.1"
            :max="2"
            :step="0.01"
          />
        </div>
      </template>

      <template v-else-if="type === 'Gain'">
        <div class="control-grid">
          <AnalogToggle v-model="gainEnabled" label="Enabled" />
          <AnalogKnob v-model="gainAmount" label="Gain" :min="0" :max="4" :step="0.01" />
        </div>
      </template>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAudioStore } from '../../stores/audio'
import { getEffectLabel, getEffectType } from '../../assets/effects'
import type { AudioEffectPayload } from '../../typings/effects'
import AnalogIndicator from '../analog/AnalogIndicator.vue'
import AnalogKnob from '../analog/AnalogKnob.vue'
import AnalogToggle from '../analog/AnalogToggle.vue'

interface Props {
  effectIndex: number
}

const audio = useAudioStore()
const props = defineProps<Props>()

const effect = computed((): AudioEffectPayload | undefined => {
  return audio.effects[props.effectIndex]?.effect
})

const type = computed(() => (effect.value ? getEffectType(effect.value) : undefined))
const label = computed(() => (effect.value ? getEffectLabel(effect.value) : 'Effect'))

const basicSettings = computed(() =>
  effect.value && 'BasicReverbSettings' in effect.value
    ? effect.value.BasicReverbSettings
    : undefined,
)

const convolutionSettings = computed(() =>
  effect.value && 'ConvolutionReverbSettings' in effect.value
    ? effect.value.ConvolutionReverbSettings
    : undefined,
)

const compressorSettings = computed(() =>
  effect.value && 'CompressorSettings' in effect.value
    ? effect.value.CompressorSettings
    : undefined,
)

const limiterSettings = computed(() =>
  effect.value && 'LimiterSettings' in effect.value ? effect.value.LimiterSettings : undefined,
)

const lowPassSettings = computed(() =>
  effect.value && 'LowPassFilterSettings' in effect.value
    ? effect.value.LowPassFilterSettings
    : undefined,
)

const highPassSettings = computed(() =>
  effect.value && 'HighPassFilterSettings' in effect.value
    ? effect.value.HighPassFilterSettings
    : undefined,
)

const distortionSettings = computed(() =>
  effect.value && 'DistortionSettings' in effect.value
    ? effect.value.DistortionSettings
    : undefined,
)

const gainSettings = computed(() =>
  effect.value && 'GainSettings' in effect.value ? effect.value.GainSettings : undefined,
)

const basicEnabled = computed({
  get: () => basicSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (basicSettings.value) basicSettings.value.enabled = value
  },
})

const basicMix = computed({
  get: () => basicSettings.value?.mix ?? 0,
  set: (value: number) => {
    if (basicSettings.value) basicSettings.value.mix = value
  },
})

const basicDuration = computed({
  get: () => basicSettings.value?.duration_ms ?? 0,
  set: (value: number) => {
    if (basicSettings.value) basicSettings.value.duration_ms = value
  },
})

const basicAmplitude = computed({
  get: () => basicSettings.value?.amplitude ?? 0,
  set: (value: number) => {
    if (basicSettings.value) basicSettings.value.amplitude = value
  },
})

const convolutionEnabled = computed({
  get: () => convolutionSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (convolutionSettings.value) convolutionSettings.value.enabled = value
  },
})

const convolutionMix = computed({
  get: () => convolutionSettings.value?.dry_wet ?? 0,
  set: (value: number) => {
    if (convolutionSettings.value) convolutionSettings.value.dry_wet = value
  },
})

const convolutionImpulse = computed({
  get: () =>
    convolutionSettings.value?.impulse_response_path ||
    convolutionSettings.value?.impulse_response ||
    '',
  set: (value: string) => {
    if (convolutionSettings.value) {
      const next = value || null
      convolutionSettings.value.impulse_response = next
      convolutionSettings.value.impulse_response_path = null
      convolutionSettings.value.impulse_response_attachment = null
    }
  },
})

const pickImpulseResponse = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const result = await open({
    multiple: false,
    filters: [
      {
        name: 'Impulse Response',
        extensions: ['wav', 'aif', 'aiff', 'flac', 'ogg'],
      },
    ],
  })

  if (!convolutionSettings.value) return
  if (!result) return
  const path = Array.isArray(result) ? result[0] : result
  if (!path) return

  convolutionSettings.value.impulse_response = null
  convolutionSettings.value.impulse_response_attachment = null
  convolutionSettings.value.impulse_response_path = path
}

const clearImpulseResponse = () => {
  if (!convolutionSettings.value) return
  convolutionSettings.value.impulse_response = null
  convolutionSettings.value.impulse_response_attachment = null
  convolutionSettings.value.impulse_response_path = null
}

const convolutionTailDb = computed({
  get: () => convolutionSettings.value?.impulse_response_tail_db ?? -60,
  set: (value: number) => {
    if (convolutionSettings.value) convolutionSettings.value.impulse_response_tail_db = value
  },
})

const compressorEnabled = computed({
  get: () => compressorSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (compressorSettings.value) compressorSettings.value.enabled = value
  },
})

const compressorThreshold = computed({
  get: () => compressorSettings.value?.threshold_db ?? -18,
  set: (value: number) => {
    if (compressorSettings.value) compressorSettings.value.threshold_db = value
  },
})

const compressorRatio = computed({
  get: () => compressorSettings.value?.ratio ?? 4,
  set: (value: number) => {
    if (compressorSettings.value) compressorSettings.value.ratio = value
  },
})

const compressorAttack = computed({
  get: () => compressorSettings.value?.attack_ms ?? 10,
  set: (value: number) => {
    if (compressorSettings.value) compressorSettings.value.attack_ms = value
  },
})

const compressorRelease = computed({
  get: () => compressorSettings.value?.release_ms ?? 100,
  set: (value: number) => {
    if (compressorSettings.value) compressorSettings.value.release_ms = value
  },
})

const compressorMakeup = computed({
  get: () => compressorSettings.value?.makeup_gain_db ?? 0,
  set: (value: number) => {
    if (compressorSettings.value) compressorSettings.value.makeup_gain_db = value
  },
})

const limiterEnabled = computed({
  get: () => limiterSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (limiterSettings.value) limiterSettings.value.enabled = value
  },
})

const limiterThreshold = computed({
  get: () => limiterSettings.value?.threshold_db ?? -1,
  set: (value: number) => {
    if (limiterSettings.value) limiterSettings.value.threshold_db = value
  },
})

const limiterKnee = computed({
  get: () => limiterSettings.value?.knee_width_db ?? 4,
  set: (value: number) => {
    if (limiterSettings.value) limiterSettings.value.knee_width_db = value
  },
})

const limiterAttack = computed({
  get: () => limiterSettings.value?.attack_ms ?? 5,
  set: (value: number) => {
    if (limiterSettings.value) limiterSettings.value.attack_ms = value
  },
})

const limiterRelease = computed({
  get: () => limiterSettings.value?.release_ms ?? 100,
  set: (value: number) => {
    if (limiterSettings.value) limiterSettings.value.release_ms = value
  },
})

const lowPassEnabled = computed({
  get: () => lowPassSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (lowPassSettings.value) lowPassSettings.value.enabled = value
  },
})

const lowPassFreq = computed({
  get: () => lowPassSettings.value?.freq_hz ?? 1000,
  set: (value: number) => {
    if (lowPassSettings.value) lowPassSettings.value.freq_hz = Math.round(value)
  },
})

const lowPassQ = computed({
  get: () => lowPassSettings.value?.q ?? 0.5,
  set: (value: number) => {
    if (lowPassSettings.value) lowPassSettings.value.q = value
  },
})

const highPassEnabled = computed({
  get: () => highPassSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (highPassSettings.value) highPassSettings.value.enabled = value
  },
})

const highPassFreq = computed({
  get: () => highPassSettings.value?.freq_hz ?? 1000,
  set: (value: number) => {
    if (highPassSettings.value) highPassSettings.value.freq_hz = Math.round(value)
  },
})

const highPassQ = computed({
  get: () => highPassSettings.value?.q ?? 0.5,
  set: (value: number) => {
    if (highPassSettings.value) highPassSettings.value.q = value
  },
})

const distortionEnabled = computed({
  get: () => distortionSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (distortionSettings.value) distortionSettings.value.enabled = value
  },
})

const distortionGain = computed({
  get: () => distortionSettings.value?.gain ?? 1,
  set: (value: number) => {
    if (distortionSettings.value) distortionSettings.value.gain = value
  },
})

const distortionThreshold = computed({
  get: () => distortionSettings.value?.threshold ?? 1,
  set: (value: number) => {
    if (distortionSettings.value) distortionSettings.value.threshold = value
  },
})

const gainEnabled = computed({
  get: () => gainSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (gainSettings.value) gainSettings.value.enabled = value
  },
})

const gainAmount = computed({
  get: () => gainSettings.value?.gain ?? 1,
  set: (value: number) => {
    if (gainSettings.value) gainSettings.value.gain = value
  },
})

const enabledState = computed(() => {
  if (!effect.value) return false
  if ('BasicReverbSettings' in effect.value) return effect.value.BasicReverbSettings.enabled
  if ('ConvolutionReverbSettings' in effect.value)
    return effect.value.ConvolutionReverbSettings.enabled
  if ('CompressorSettings' in effect.value) return effect.value.CompressorSettings.enabled
  if ('LimiterSettings' in effect.value) return effect.value.LimiterSettings.enabled
  if ('LowPassFilterSettings' in effect.value) return effect.value.LowPassFilterSettings.enabled
  if ('HighPassFilterSettings' in effect.value) return effect.value.HighPassFilterSettings.enabled
  if ('DistortionSettings' in effect.value) return effect.value.DistortionSettings.enabled
  if ('GainSettings' in effect.value) return effect.value.GainSettings.enabled
  return false
})
</script>

<style lang="scss" scoped>
.effects-dialog {
  display: grid;
  gap: 1.25rem;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.dialog-header h2 {
  margin: 0;
  font-size: 1.2rem;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.dialog-panel {
  padding: 1.5rem;
  border-radius: 18px;
}

.control-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 1.5rem;
  align-items: start;
}

.ir-picker {
  display: grid;
  gap: 0.6rem;
  align-content: start;
}

.ir-input {
  width: 100%;
  padding: 0.4rem 0.6rem;
  border-radius: 6px;
  border: 1px solid #2a241d;
  background: #1f1b18;
  color: var(--analog-text);
  font-family: var(--analog-font);
  font-size: 0.85rem;
}

.ir-actions {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(110px, 1fr));
  gap: 0.5rem;
}
</style>
