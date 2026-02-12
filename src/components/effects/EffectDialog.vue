<template>
  <div class="effects-controls" v-if="effect">
    <h2>{{ label }}</h2>
    <div class="control-bin">
      <template v-if="type === 'BasicReverb'">
        <div>enabled</div>
        <el-switch v-model="basicEnabled" />
        <div></div>

        <div>mix</div>
        <el-slider v-model="basicMix" :min="0" :max="1" :step="0.01" size="small" />
        <el-input type="text" v-model="basicMix" disabled="true" />

        <div>duration (ms)</div>
        <el-slider v-model="basicDuration" :min="0" :max="2000" :step="1" size="small" />
        <el-input type="text" v-model="basicDuration" disabled="true" />

        <div>amplitude</div>
        <el-slider v-model="basicAmplitude" :min="0" :max="0.8" :step="0.01" size="small" />
        <el-input type="text" v-model="basicAmplitude" disabled="true" />
      </template>

      <template v-else-if="type === 'DiffusionReverb'">
        <div>enabled</div>
        <el-switch v-model="diffusionEnabled" />
        <div></div>

        <div>mix</div>
        <el-slider v-model="diffusionMix" :min="0" :max="1" :step="0.01" size="small" />
        <el-input type="text" v-model="diffusionMix" disabled="true" />

        <div>pre-delay (ms)</div>
        <el-slider v-model="diffusionPreDelay" :min="0" :max="200" :step="1" size="small" />
        <el-input type="text" v-model="diffusionPreDelay" disabled="true" />

        <div>room size (ms)</div>
        <el-slider v-model="diffusionRoomSize" :min="0" :max="200" :step="1" size="small" />
        <el-input type="text" v-model="diffusionRoomSize" disabled="true" />

        <div>decay</div>
        <el-slider v-model="diffusionDecay" :min="0" :max="0.98" :step="0.01" size="small" />
        <el-input type="text" v-model="diffusionDecay" disabled="true" />

        <div>damping</div>
        <el-slider v-model="diffusionDamping" :min="0" :max="0.99" :step="0.01" size="small" />
        <el-input type="text" v-model="diffusionDamping" disabled="true" />

        <div>diffusion</div>
        <el-slider v-model="diffusionAmount" :min="0" :max="0.9" :step="0.01" size="small" />
        <el-input type="text" v-model="diffusionAmount" disabled="true" />
      </template>

      <template v-else-if="type === 'ConvolutionReverb'">
        <div>enabled</div>
        <el-switch v-model="convolutionEnabled" />
        <div></div>

        <div>dry/wet</div>
        <el-slider v-model="convolutionMix" :min="0" :max="1" :step="0.01" size="small" />
        <el-input type="text" v-model="convolutionMix" disabled="true" />

        <div>impulse response</div>
        <div class="ir-picker">
          <el-input
            v-model="convolutionImpulse"
            placeholder="attachment:ir.wav or /path/to/ir.wav"
          />
          <el-button size="small" @click="pickImpulseResponse">Choose File</el-button>
          <el-button size="small" @click="clearImpulseResponse">Clear</el-button>
        </div>
        <div></div>

        <div>tail (dB)</div>
        <el-slider v-model="convolutionTailDb" :min="-120" :max="0" :step="1" size="small" />
        <el-input type="text" v-model="convolutionTailDb" disabled="true" />
      </template>

      <template v-else-if="type === 'Compressor'">
        <div>enabled</div>
        <el-switch v-model="compressorEnabled" />
        <div></div>

        <div>threshold (dB)</div>
        <el-slider v-model="compressorThreshold" :min="-60" :max="0" :step="0.5" size="small" />
        <el-input type="text" v-model="compressorThreshold" disabled="true" />

        <div>ratio</div>
        <el-slider v-model="compressorRatio" :min="1" :max="20" :step="0.1" size="small" />
        <el-input type="text" v-model="compressorRatio" disabled="true" />

        <div>attack (ms)</div>
        <el-slider v-model="compressorAttack" :min="1" :max="200" :step="1" size="small" />
        <el-input type="text" v-model="compressorAttack" disabled="true" />

        <div>release (ms)</div>
        <el-slider v-model="compressorRelease" :min="1" :max="500" :step="1" size="small" />
        <el-input type="text" v-model="compressorRelease" disabled="true" />

        <div>makeup (dB)</div>
        <el-slider v-model="compressorMakeup" :min="-12" :max="12" :step="0.5" size="small" />
        <el-input type="text" v-model="compressorMakeup" disabled="true" />
      </template>

      <template v-else-if="type === 'Limiter'">
        <div>enabled</div>
        <el-switch v-model="limiterEnabled" />
        <div></div>

        <div>threshold (dB)</div>
        <el-slider v-model="limiterThreshold" :min="-20" :max="0" :step="0.5" size="small" />
        <el-input type="text" v-model="limiterThreshold" disabled="true" />

        <div>knee width (dB)</div>
        <el-slider v-model="limiterKnee" :min="0" :max="12" :step="0.5" size="small" />
        <el-input type="text" v-model="limiterKnee" disabled="true" />

        <div>attack (ms)</div>
        <el-slider v-model="limiterAttack" :min="1" :max="100" :step="1" size="small" />
        <el-input type="text" v-model="limiterAttack" disabled="true" />

        <div>release (ms)</div>
        <el-slider v-model="limiterRelease" :min="1" :max="500" :step="1" size="small" />
        <el-input type="text" v-model="limiterRelease" disabled="true" />
      </template>

      <template v-else-if="type === 'LowPassFilter'">
        <div>enabled</div>
        <el-switch v-model="lowPassEnabled" />
        <div></div>

        <div>freq (Hz)</div>
        <el-slider v-model="lowPassFreq" :min="20" :max="20000" :step="10" size="small" />
        <el-input type="text" v-model="lowPassFreq" disabled="true" />

        <div>q</div>
        <el-slider v-model="lowPassQ" :min="0.1" :max="2" :step="0.01" size="small" />
        <el-input type="text" v-model="lowPassQ" disabled="true" />
      </template>

      <template v-else-if="type === 'HighPassFilter'">
        <div>enabled</div>
        <el-switch v-model="highPassEnabled" />
        <div></div>

        <div>freq (Hz)</div>
        <el-slider v-model="highPassFreq" :min="20" :max="20000" :step="10" size="small" />
        <el-input type="text" v-model="highPassFreq" disabled="true" />

        <div>q</div>
        <el-slider v-model="highPassQ" :min="0.1" :max="2" :step="0.01" size="small" />
        <el-input type="text" v-model="highPassQ" disabled="true" />
      </template>

      <template v-else-if="type === 'Distortion'">
        <div>enabled</div>
        <el-switch v-model="distortionEnabled" />
        <div></div>

        <div>gain</div>
        <el-slider v-model="distortionGain" :min="0" :max="10" :step="0.1" size="small" />
        <el-input type="text" v-model="distortionGain" disabled="true" />

        <div>threshold</div>
        <el-slider v-model="distortionThreshold" :min="0.1" :max="2" :step="0.01" size="small" />
        <el-input type="text" v-model="distortionThreshold" disabled="true" />
      </template>

      <template v-else-if="type === 'Gain'">
        <div>enabled</div>
        <el-switch v-model="gainEnabled" />
        <div></div>

        <div>gain</div>
        <el-slider v-model="gainAmount" :min="0" :max="4" :step="0.01" size="small" />
        <el-input type="text" v-model="gainAmount" disabled="true" />
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAudioStore } from '../../stores/audio'
import { getEffectLabel, getEffectType } from '../../assets/effects'
import type { AudioEffectPayload } from '../../typings/effects'

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

const diffusionSettings = computed(() =>
  effect.value && 'DiffusionReverbSettings' in effect.value
    ? effect.value.DiffusionReverbSettings
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
    if (diffusionSettings.value) diffusionSettings.value.pre_delay_ms = Math.round(value)
  },
})

const diffusionRoomSize = computed({
  get: () => diffusionSettings.value?.room_size_ms ?? 0,
  set: (value: number) => {
    if (diffusionSettings.value) diffusionSettings.value.room_size_ms = Math.round(value)
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

const diffusionAmount = computed({
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
</script>

<style lang="scss" scoped>
.effects-controls {
  padding-bottom: 3em;

  h2 {
    margin-top: 0;
  }
  .control-bin {
    display: grid;
    grid-template-columns: 140px 1fr 140px;
    column-gap: 1em;
    row-gap: 1em;
    text-align: right;
    align-items: center;
  }

  .ir-picker {
    display: grid;
    grid-template-columns: 1fr auto auto;
    gap: 0.5em;
    align-items: center;
  }
}
</style>
