<template>
  <div v-if="effect" class="grid gap-4">
    <BasicReverbDialog
      v-if="'BasicReverbSettings' in effect"
      :settings="effect.BasicReverbSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />

    <DiffusionReverbDialog
      v-else-if="'DiffusionReverbSettings' in effect"
      :settings="effect.DiffusionReverbSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />

    <ConvolutionReverbDialog
      v-else-if="'ConvolutionReverbSettings' in effect"
      :settings="effect.ConvolutionReverbSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />

    <CompressorDialog
      v-else-if="'CompressorSettings' in effect"
      :settings="effect.CompressorSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />

    <LimiterDialog
      v-else-if="'LimiterSettings' in effect"
      :settings="effect.LimiterSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />

    <LowPassFilterDialog
      v-else-if="'LowPassFilterSettings' in effect"
      :settings="effect.LowPassFilterSettings"
      :response-curve="responseCurve"
      :spectral-snapshot="spectralSnapshot"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
      :spectral-enabled="audio.effectSpectralAnalysisEnabled"
    />

    <HighPassFilterDialog
      v-else-if="'HighPassFilterSettings' in effect"
      :settings="effect.HighPassFilterSettings"
      :response-curve="responseCurve"
      :spectral-snapshot="spectralSnapshot"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
      :spectral-enabled="audio.effectSpectralAnalysisEnabled"
    />

    <MultibandEqDialog
      v-else-if="'MultibandEqSettings' in effect"
      :settings="effect.MultibandEqSettings"
      :response-curve="responseCurve"
      :spectral-snapshot="spectralSnapshot"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
      :spectral-enabled="audio.effectSpectralAnalysisEnabled"
    />

    <DistortionDialog
      v-else-if="'DistortionSettings' in effect"
      :settings="effect.DistortionSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />

    <GainDialog
      v-else-if="'GainSettings' in effect"
      :settings="effect.GainSettings"
      :live-levels="liveLevels"
      :is-playing="audio.isPlaying"
      :metering-enabled="audio.effectLevelMeteringEnabled"
    />
  </div>

  <div
    v-else
    class="rounded-[22px] border border-zinc-300 bg-zinc-50 p-6 text-sm leading-6 text-zinc-600"
  >
    This effect is no longer available in the chain.
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, watch } from 'vue'
import { useAudioStore } from '../../stores/audio'
import type {
  AudioEffectPayload,
  EffectBandSnapshot,
  EffectLevelSnapshot,
  FilterResponseCurve,
} from '../../typings/effects'
import BasicReverbDialog from './dialogs/BasicReverbDialog.vue'
import CompressorDialog from './dialogs/CompressorDialog.vue'
import ConvolutionReverbDialog from './dialogs/ConvolutionReverbDialog.vue'
import DiffusionReverbDialog from './dialogs/DiffusionReverbDialog.vue'
import DistortionDialog from './dialogs/DistortionDialog.vue'
import GainDialog from './dialogs/GainDialog.vue'
import HighPassFilterDialog from './dialogs/HighPassFilterDialog.vue'
import LimiterDialog from './dialogs/LimiterDialog.vue'
import LowPassFilterDialog from './dialogs/LowPassFilterDialog.vue'
import MultibandEqDialog from './dialogs/MultibandEqDialog.vue'

interface Props {
  effectId: number
}

const props = defineProps<Props>()
const audio = useAudioStore()

const effect = computed((): AudioEffectPayload | undefined => {
  return audio.effects.find((item) => item.id === props.effectId)?.effect
})

const liveLevels = computed((): EffectLevelSnapshot | null =>
  audio.effectLevelSnapshotForId(props.effectId),
)

const spectralSnapshot = computed((): EffectBandSnapshot | null =>
  audio.effectBandSnapshotForId(props.effectId),
)

const responseCurve = computed((): FilterResponseCurve | null =>
  audio.effectResponseCurveForId(props.effectId),
)

onMounted(() => {
  audio.openEffectDialog(props.effectId)
})

watch(
  () => props.effectId,
  (next, previous) => {
    if (previous != null) {
      audio.closeEffectDialog(previous)
    }
    audio.openEffectDialog(next)
  },
)

onUnmounted(() => {
  audio.closeEffectDialog(props.effectId)
})
</script>
