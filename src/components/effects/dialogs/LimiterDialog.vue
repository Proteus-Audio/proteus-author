<template>
  <EffectDialogShell
    title="Limiter"
    description="Fast peak control with ceiling, knee width, and release behaviour."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <EffectTransferCurve
          :points="curvePoints"
          :x-min="-36"
          :x-max="12"
          :y-min="-36"
          :y-max="12"
          :enabled="settings.enabled"
          description="The limiter keeps peaks below the ceiling while the knee softens how abruptly that ceiling engages."
          :markers="markers"
        />

        <div class="grid gap-3 md:grid-cols-3">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Ceiling</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ thresholdReadout }}</div>
          </div>
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Knee</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ kneeReadout }}</div>
          </div>
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Recovery</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ releaseReadout }}</div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.threshold_db"
          label="Threshold"
          :min="-30"
          :max="0"
          :step="0.5"
          units="dB"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.knee_width_db"
          label="Knee Width"
          :min="0.1"
          :max="30"
          :step="0.5"
          units="dB"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.attack_ms"
          label="Attack"
          :min="1"
          :max="100"
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
import type { EffectLevelSnapshot, LimiterSettings } from '../../../typings/effects'
import { buildLimiterTransferPoints } from '../../../utils/effect-visuals'
import { formatDbValue } from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'
import EffectTransferCurve from './EffectTransferCurve.vue'

interface Props {
  settings: LimiterSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const curvePoints = computed(() =>
  buildLimiterTransferPoints(props.settings.threshold_db, props.settings.knee_width_db),
)

const markers = computed(() => [
  { axis: 'x' as const, value: props.settings.threshold_db, label: 'Threshold' },
  { axis: 'y' as const, value: props.settings.threshold_db, label: 'Ceiling' },
])

const thresholdReadout = computed(() => formatDbValue(props.settings.threshold_db, 1))
const kneeReadout = computed(() => formatDbValue(props.settings.knee_width_db, 1))
const releaseReadout = computed(() => `${Math.round(props.settings.release_ms)} ms`)
</script>
