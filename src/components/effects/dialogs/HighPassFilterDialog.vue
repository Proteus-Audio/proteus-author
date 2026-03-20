<template>
  <EffectDialogShell
    title="High-Pass Filter"
    description="Analytical high-pass response with live filter monitoring."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    :spectral-enabled="spectralEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <EffectResponseChart
        :response="responseCurve"
        :spectral="spectralSnapshot"
        :enabled="settings.enabled"
        :is-playing="isPlaying"
        :spectral-enabled="spectralEnabled"
      />
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.freq_hz"
          label="Cutoff"
          :min="20"
          :max="20000"
          :step="1"
          units="Hz"
          scale="log"
        />
        <EffectDialControl
          v-model="settings.q"
          label="Resonance"
          :min="0.1"
          :max="2"
          :step="0.01"
          :precision="2"
        />
      </div>
    </template>

    <template #meters>
      <EffectLiveMeters
        :snapshot="liveLevels"
        :enabled="meteringEnabled"
        :is-playing="isPlaying"
        :bypassed="!settings.enabled"
      />
    </template>
  </EffectDialogShell>
</template>

<script setup lang="ts">
import type {
  EffectBandSnapshot,
  EffectLevelSnapshot,
  FilterResponseCurve,
  HighPassFilterSettings,
} from '../../../typings/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'
import EffectResponseChart from './EffectResponseChart.vue'

interface Props {
  settings: HighPassFilterSettings
  responseCurve: FilterResponseCurve | null
  spectralSnapshot: EffectBandSnapshot | null
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
  spectralEnabled: boolean
}

defineProps<Props>()
</script>
