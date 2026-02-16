<template>
  <div v-if="effect" class="grid gap-5">
    <header class="flex items-center justify-between gap-4">
      <h2 class="m-0 text-[1.2rem] uppercase tracking-[0.12em]">{{ label }}</h2>
      <AnalogIndicator :state="enabledState" label="Active" color="green" />
    </header>

    <section class="analog-panel rounded-[18px] p-6">
      <template v-if="type === 'BasicReverb'">
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
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

      <template v-else-if="type === 'DiffusionReverb'">
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
          <AnalogToggle v-model="diffusionEnabled" label="Enabled" />
          <AnalogKnob v-model="diffusionMix" label="Mix" :min="0" :max="1" :step="0.01" />
          <AnalogKnob
            v-model="diffusionPreDelay"
            label="Pre-delay"
            :min="0"
            :max="250"
            :step="1"
            units="ms"
          />
          <AnalogKnob
            v-model="diffusionRoomSize"
            label="Room"
            :min="1"
            :max="500"
            :step="1"
            units="ms"
          />
          <AnalogKnob v-model="diffusionDecay" label="Decay" :min="0" :max="1" :step="0.01" />
          <AnalogKnob v-model="diffusionDamping" label="Damping" :min="0" :max="1" :step="0.01" />
          <AnalogKnob
            v-model="diffusionDiffusion"
            label="Diffusion"
            :min="0"
            :max="1"
            :step="0.01"
          />
        </div>
      </template>

      <template v-else-if="type === 'ConvolutionReverb'">
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
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
          <div class="grid content-start gap-2.5">
            <span class="analog-label">Impulse Response</span>
            <UInput
              v-model="convolutionImpulse"
              color="neutral"
              variant="outline"
              :ui="{
                base: 'font-[var(--font-analog)] text-[var(--color-analog-text)] bg-[#1f1b18] border-[#2a241d]',
              }"
              placeholder="attachment:ir.wav or /path/to/ir.wav"
            />
            <div class="grid grid-cols-[repeat(auto-fit,minmax(110px,1fr))] gap-2">
              <UButton size="xs" variant="outline" color="neutral" @click="pickImpulseResponse"
                >Choose File</UButton
              >
              <UButton size="xs" variant="outline" color="neutral" @click="clearImpulseResponse"
                >Clear</UButton
              >
            </div>
          </div>
        </div>
      </template>

      <template v-else-if="type === 'Compressor'">
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
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
            :min="-30"
            :max="30"
            :step="0.5"
            units="dB"
          />
        </div>
      </template>

      <template v-else-if="type === 'Limiter'">
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
          <AnalogToggle v-model="limiterEnabled" label="Enabled" />
          <AnalogKnob
            v-model="limiterThreshold"
            label="Threshold"
            :min="-30"
            :max="0"
            :step="0.5"
            units="dB"
          />
          <AnalogKnob
            v-model="limiterKnee"
            label="Knee"
            :min="0.1"
            :max="30"
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
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
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
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
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
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
          <AnalogToggle v-model="distortionEnabled" label="Enabled" />
          <AnalogKnob
            v-model="distortionGain"
            label="Gain"
            :min="-30"
            :max="30"
            :step="0.5"
            units="dB"
          />
          <AnalogKnob
            v-model="distortionThreshold"
            label="Threshold"
            :min="-30"
            :max="0"
            :step="0.5"
            units="dB"
          />
        </div>
      </template>

      <template v-else-if="type === 'Gain'">
        <div
          class="grid auto-rows-auto grid-cols-[repeat(auto-fit,minmax(140px,1fr))] items-start gap-6"
        >
          <AnalogToggle v-model="gainEnabled" label="Enabled" />
          <AnalogKnob
            v-model="gainAmount"
            label="Gain"
            :min="-30"
            :max="30"
            :step="0.5"
            units="dB"
          />
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

const diffusionSettings = computed(() =>
  effect.value && 'DiffusionReverbSettings' in effect.value
    ? effect.value.DiffusionReverbSettings
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

const diffusionEnabled = computed({
  get: () => diffusionSettings.value?.enabled ?? false,
  set: (value: boolean) => {
    if (diffusionSettings.value) diffusionSettings.value.enabled = value
  },
})

const diffusionMix = computed({
  get: () => diffusionSettings.value?.mix ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.mix = value
  },
})

const diffusionPreDelay = computed({
  get: () => diffusionSettings.value?.pre_delay_ms ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.pre_delay_ms = value
  },
})

const diffusionRoomSize = computed({
  get: () => diffusionSettings.value?.room_size_ms ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.room_size_ms = value
  },
})

const diffusionDecay = computed({
  get: () => diffusionSettings.value?.decay ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.decay = value
  },
})

const diffusionDamping = computed({
  get: () => diffusionSettings.value?.damping ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.damping = value
  },
})

const diffusionDiffusion = computed({
  get: () => diffusionSettings.value?.diffusion ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.diffusion = value
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
  get: () => distortionSettings.value?.gain ?? 0,
  set: (value: number) => {
    if (distortionSettings.value) distortionSettings.value.gain = value
  },
})

const distortionThreshold = computed({
  get: () => distortionSettings.value?.threshold ?? 0,
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
  get: () => gainSettings.value?.gain ?? 0,
  set: (value: number) => {
    if (gainSettings.value) gainSettings.value.gain = value
  },
})

const enabledState = computed(() => {
  if (!effect.value) return false
  if ('BasicReverbSettings' in effect.value) return effect.value.BasicReverbSettings.enabled
  if ('DiffusionReverbSettings' in effect.value) return effect.value.DiffusionReverbSettings.enabled
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
