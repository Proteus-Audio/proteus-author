<template>
  <EffectDialogShell
    title="Diffusion Reverb"
    description="Diffuse ambience with pre-delay, room shaping, and damping controls."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <div class="grid gap-3 md:grid-cols-2">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-950 p-4 text-zinc-100">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">Spatial Build</div>
            <div class="mt-4 grid gap-3">
              <div v-for="metric in metrics" :key="metric.label" class="grid gap-1.5">
                <div class="flex items-center justify-between gap-3 text-xs text-zinc-300">
                  <span class="uppercase tracking-[0.14em]">{{ metric.label }}</span>
                  <span>{{ metric.readout }}</span>
                </div>
                <div class="h-2 rounded-full bg-white/10">
                  <div
                    class="h-full rounded-full bg-cyan-500"
                    :style="{ width: `${metric.percent}%` }"
                  ></div>
                </div>
              </div>
            </div>
          </div>

          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
              Reflection Lattice
            </div>
            <div class="mt-4 grid grid-cols-6 gap-2">
              <div
                v-for="(column, index) in diffusionColumns"
                :key="index"
                class="rounded-t-md bg-gradient-to-t from-indigo-700 to-cyan-400"
                :style="{ height: `${column}%`, minHeight: '48px' }"
              ></div>
            </div>
            <div class="mt-4 text-sm leading-6 text-zinc-600">
              Pre-delay opens space before the cloud, while damping and diffusion shape how dense
              and dark the tail becomes.
            </div>
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
          v-model="settings.pre_delay_ms"
          label="Pre-delay"
          :min="0"
          :max="250"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.room_size_ms"
          label="Room Size"
          :min="1"
          :max="500"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.decay"
          label="Decay"
          :min="0"
          :max="1"
          :step="0.01"
          :precision="2"
        />
        <EffectDialControl
          v-model="settings.damping"
          label="Damping"
          :min="0"
          :max="1"
          :step="0.01"
          :precision="2"
        />
        <EffectDialControl
          v-model="settings.diffusion"
          label="Diffusion"
          :min="0"
          :max="1"
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
import type { DiffusionReverbSettings, EffectLevelSnapshot } from '../../../typings/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'

interface Props {
  settings: DiffusionReverbSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const metrics = computed(() => [
  {
    label: 'Mix',
    readout: `${Math.round(props.settings.mix * 100)}%`,
    percent: Math.round(props.settings.mix * 100),
  },
  {
    label: 'Pre-delay',
    readout: `${Math.round(props.settings.pre_delay_ms)} ms`,
    percent: Math.round((props.settings.pre_delay_ms / 250) * 100),
  },
  {
    label: 'Room',
    readout: `${Math.round(props.settings.room_size_ms)} ms`,
    percent: Math.round((props.settings.room_size_ms / 500) * 100),
  },
  {
    label: 'Decay',
    readout: props.settings.decay.toFixed(2),
    percent: Math.round(props.settings.decay * 100),
  },
  {
    label: 'Damping',
    readout: props.settings.damping.toFixed(2),
    percent: Math.round(props.settings.damping * 100),
  },
  {
    label: 'Diffusion',
    readout: props.settings.diffusion.toFixed(2),
    percent: Math.round(props.settings.diffusion * 100),
  },
])

const diffusionColumns = computed(() => {
  const base = Math.max(props.settings.diffusion, 0.15) * 50
  const decay = Math.max(props.settings.decay, 0.15) * 45
  const damping = (1 - props.settings.damping) * 18
  const spread = props.settings.pre_delay_ms / 12
  return Array.from({ length: 6 }, (_, index) => {
    return Math.max(
      24,
      base + decay - index * 5 + damping + spread + index * props.settings.mix * 6,
    )
  })
})
</script>
