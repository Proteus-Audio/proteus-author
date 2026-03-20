<template>
  <EffectDialogShell
    title="Basic Reverb"
    description="Short algorithmic wash with direct control over mix, decay, and amplitude."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <div class="grid gap-3 md:grid-cols-3">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Dry / Wet</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ mixPercent }}%</div>
            <div class="mt-3 h-2 rounded-full bg-zinc-200">
              <div
                class="h-full rounded-full bg-cyan-600"
                :style="{ width: `${mixPercent}%` }"
              ></div>
            </div>
          </div>

          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Decay Span</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">
              {{ settings.duration_ms }} ms
            </div>
            <div class="mt-3 h-2 rounded-full bg-zinc-200">
              <div
                class="h-full rounded-full bg-violet-600"
                :style="{ width: `${durationPercent}%` }"
              ></div>
            </div>
          </div>

          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Tail Energy</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ amplitudePercent }}%</div>
            <div class="mt-3 h-2 rounded-full bg-zinc-200">
              <div
                class="h-full rounded-full bg-emerald-600"
                :style="{ width: `${amplitudePercent}%` }"
              ></div>
            </div>
          </div>
        </div>

        <div class="rounded-[18px] border border-zinc-200 bg-zinc-950 p-4 text-zinc-100">
          <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">Decay Profile</div>
          <div class="mt-4 flex h-32 items-end gap-2">
            <div
              v-for="(height, index) in decayBars"
              :key="index"
              class="flex-1 rounded-t-md bg-gradient-to-t from-cyan-500 to-cyan-200"
              :style="{ height: `${height}%`, opacity: `${0.35 + index * 0.08}` }"
            ></div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.mix"
          label="Mix"
          :min="0"
          :max="1"
          :step="0.01"
          :precision="2"
        />
        <EffectDialControl
          v-model="settings.duration_ms"
          label="Duration"
          :min="0"
          :max="2000"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.amplitude"
          label="Amplitude"
          :min="0"
          :max="0.8"
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
import { computed } from 'vue'
import type { BasicReverbSettings, EffectLevelSnapshot } from '../../../typings/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'

interface Props {
  settings: BasicReverbSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const mixPercent = computed(() => Math.round(props.settings.mix * 100))
const durationPercent = computed(() => Math.round((props.settings.duration_ms / 2000) * 100))
const amplitudePercent = computed(() => Math.round((props.settings.amplitude / 0.8) * 100))
const decayBars = computed(() => {
  const amplitude = Math.max(props.settings.amplitude, 0.05)
  const durationShape = Math.max(props.settings.duration_ms / 2000, 0.05)
  return Array.from({ length: 8 }, (_, index) => {
    const progression = 1 - index / 7
    return Math.max(12, (progression * amplitude * 70 + durationShape * 30) * (1 - index * 0.04))
  })
})
</script>
