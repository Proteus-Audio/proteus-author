<template>
  <EffectDialogShell
    title="Distortion"
    description="Gain-staged clipping curve with threshold-driven saturation."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <EffectTransferCurve
          :points="curvePoints"
          :x-min="-1"
          :x-max="1"
          :y-min="-1"
          :y-max="1"
          :enabled="settings.enabled"
          description="Drive pushes the signal into the threshold ceiling, creating steeper clipping and stronger harmonic density."
          :markers="[{ axis: 'y', value: 0, label: 'Center' }]"
        />

        <div class="grid gap-3 md:grid-cols-2">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Drive</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ driveReadout }}</div>
          </div>
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Clip Threshold</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ thresholdReadout }}</div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.gain"
          label="Drive"
          :min="-30"
          :max="30"
          :step="0.5"
          units="dB"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.threshold"
          label="Threshold"
          :min="-30"
          :max="0"
          :step="0.5"
          units="dB"
          :precision="1"
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
import { computed } from 'vue'
import type { DistortionSettings, EffectLevelSnapshot } from '../../../typings/effects'
import { buildDistortionTransferPoints } from '../../../utils/effect-visuals'
import { formatDbValue } from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'
import EffectTransferCurve from './EffectTransferCurve.vue'

interface Props {
  settings: DistortionSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const curvePoints = computed(() =>
  buildDistortionTransferPoints(props.settings.gain, props.settings.threshold),
)

const driveReadout = computed(() => formatDbValue(props.settings.gain, 1))
const thresholdReadout = computed(() => formatDbValue(props.settings.threshold, 1))
</script>
