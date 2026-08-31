<template>
  <div class="grid gap-4">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">Live Metering</div>
        <div class="mt-1 text-sm text-zinc-100">{{ statusText }}</div>
      </div>

      <div class="text-right text-xs uppercase tracking-[0.16em] text-zinc-400">
        Delta {{ deltaReadout }}
      </div>
    </div>

    <div class="grid gap-3 md:grid-cols-2">
      <div class="grid gap-3 rounded-[18px] border border-white/10 bg-white/5 p-3">
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">Input</div>
        <MeterBar label="Peak" :db="inputPeakDb" />
        <MeterBar label="RMS" :db="inputRmsDb" color="bg-cyan-500" />
      </div>

      <div class="grid gap-3 rounded-[18px] border border-white/10 bg-white/5 p-3">
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">Output</div>
        <MeterBar label="Peak" :db="outputPeakDb" color="bg-emerald-400" />
        <MeterBar label="RMS" :db="outputRmsDb" color="bg-emerald-600" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, defineComponent } from 'vue'
import type { EffectLevelSnapshot } from '../../../typings/effects'
import { maxChannelDb, normalizeDb } from '../../../utils/effect-visuals'

const MIN_DB = -72
const MAX_DB = 12

const MeterBar = defineComponent({
  name: 'MeterBar',
  props: {
    label: { type: String, required: true },
    db: { type: Number, required: true },
    color: { type: String, default: 'bg-violet-400' },
  },
  setup(props) {
    const fill = computed(() => `${normalizeDb(props.db, MIN_DB, MAX_DB) * 100}%`)
    const readout = computed(() => `${props.db > 0 ? '+' : ''}${props.db.toFixed(1)} dB`)
    return {
      fill,
      readout,
    }
  },
  template: `
    <div class="grid gap-1.5">
      <div class="flex items-center justify-between gap-3 text-xs text-zinc-300">
        <span class="uppercase tracking-[0.14em]">{{ label }}</span>
        <span>{{ readout }}</span>
      </div>
      <div class="h-2.5 overflow-hidden rounded-full bg-white/10">
        <div class="h-full rounded-full transition-[width] duration-150" :class="color" :style="{ width: fill }"></div>
      </div>
    </div>
  `,
})

interface Props {
  snapshot: EffectLevelSnapshot | null
  enabled: boolean
  isPlaying: boolean
  bypassed: boolean
}

const props = defineProps<Props>()

const inputPeakDb = computed(() => maxChannelDb(props.snapshot?.input.peak ?? [], MIN_DB, MAX_DB))
const inputRmsDb = computed(() => maxChannelDb(props.snapshot?.input.rms ?? [], MIN_DB, MAX_DB))
const outputPeakDb = computed(() => maxChannelDb(props.snapshot?.output.peak ?? [], MIN_DB, MAX_DB))
const outputRmsDb = computed(() => maxChannelDb(props.snapshot?.output.rms ?? [], MIN_DB, MAX_DB))

const deltaReadout = computed(() => {
  const delta = outputPeakDb.value - inputPeakDb.value
  return `${delta > 0 ? '+' : ''}${delta.toFixed(1)} dB`
})

const statusText = computed(() => {
  if (!props.enabled) return 'Effect metering is disabled.'
  if (props.bypassed) return 'Bypassed state shown with live input and output level context.'
  if (!props.isPlaying) return 'Start playback to read audible-time input and output levels.'
  if (!props.snapshot) return 'Meter ring is warming up.'
  return 'Audible-time peak and RMS levels are updating.'
})
</script>
