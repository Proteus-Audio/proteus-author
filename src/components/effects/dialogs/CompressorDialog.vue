<template>
  <EffectDialogShell
    title="Compressor"
    description="Dynamic range control with threshold, ratio, timing, and makeup gain."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <EffectTransferCurve
          :points="curvePoints"
          :x-min="-60"
          :x-max="12"
          :y-min="-60"
          :y-max="18"
          :enabled="settings.enabled"
          description="Threshold bends the line while ratio and makeup gain define how aggressively loud material is folded back."
          :markers="markers"
        />

        <div class="grid gap-3 md:grid-cols-4">
          <div
            v-for="stat in stats"
            :key="stat.label"
            class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4"
          >
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
              {{ stat.label }}
            </div>
            <div class="mt-2 text-xl font-semibold text-zinc-950">{{ stat.value }}</div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.threshold_db"
          label="Threshold"
          :min="-60"
          :max="0"
          :step="0.5"
          units="dB"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.ratio"
          label="Ratio"
          :min="1"
          :max="20"
          :step="0.1"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.attack_ms"
          label="Attack"
          :min="1"
          :max="200"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.release_ms"
          label="Release"
          :min="1"
          :max="500"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.makeup_gain_db"
          label="Makeup Gain"
          :min="-30"
          :max="30"
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
import type { CompressorSettings, EffectLevelSnapshot } from '../../../typings/effects'
import { buildCompressorTransferPoints } from '../../../utils/effect-visuals'
import { formatDbValue } from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'
import EffectTransferCurve from './EffectTransferCurve.vue'

interface Props {
  settings: CompressorSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const curvePoints = computed(() =>
  buildCompressorTransferPoints(
    props.settings.threshold_db,
    props.settings.ratio,
    props.settings.makeup_gain_db,
  ),
)

const markers = computed(() => [
  { axis: 'x' as const, value: props.settings.threshold_db, label: 'Threshold' },
  {
    axis: 'y' as const,
    value: props.settings.threshold_db + props.settings.makeup_gain_db,
    label: 'Output target',
  },
])

const stats = computed(() => [
  { label: 'Threshold', value: formatDbValue(props.settings.threshold_db, 1) },
  { label: 'Ratio', value: `${props.settings.ratio.toFixed(1)}:1` },
  { label: 'Attack', value: `${Math.round(props.settings.attack_ms)} ms` },
  { label: 'Release', value: `${Math.round(props.settings.release_ms)} ms` },
])
</script>
