<template>
  <EffectDialogShell
    title="Gain"
    description="Output trim with immediate delta and live level feedback."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <div class="grid gap-3 md:grid-cols-[minmax(0,1fr)_220px]">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-950 p-4 text-zinc-100">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">Gain Delta</div>
            <div class="mt-3 text-4xl font-semibold">{{ gainReadout }}</div>
            <div class="mt-4 h-2 rounded-full bg-white/10">
              <div
                class="h-full rounded-full transition-[width] duration-150"
                :class="settings.gain >= 0 ? 'bg-emerald-400' : 'bg-amber-400'"
                :style="{ width: `${deltaWidth}%` }"
              ></div>
            </div>
            <div class="mt-3 text-sm leading-6 text-zinc-300">
              Positive gain lifts the output meter above the input trace. Negative gain trims
              headroom before the next processor.
            </div>
          </div>

          <div class="grid gap-3 rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div>
              <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Direction</div>
              <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ directionLabel }}</div>
            </div>
            <div class="rounded-[16px] border border-zinc-200 bg-white p-3 text-sm text-zinc-600">
              {{ directionText }}
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.gain"
          label="Gain"
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
import type { EffectLevelSnapshot, GainSettings } from '../../../typings/effects'
import { formatDbValue } from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'

interface Props {
  settings: GainSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const gainReadout = computed(() => formatDbValue(props.settings.gain, 1))
const deltaWidth = computed(() => Math.min(100, (Math.abs(props.settings.gain) / 30) * 100))
const directionLabel = computed(() => {
  if (props.settings.gain > 0.05) return 'Boost'
  if (props.settings.gain < -0.05) return 'Trim'
  return 'Unity'
})
const directionText = computed(() => {
  if (props.settings.gain > 0.05) return 'Use this when the chain needs more drive or makeup level.'
  if (props.settings.gain < -0.05) return 'Use this when downstream stages need more headroom.'
  return 'Unity gain keeps the effect transparent until the next stage.'
})
</script>
