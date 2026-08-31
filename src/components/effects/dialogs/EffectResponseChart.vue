<template>
  <div class="grid gap-4">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Analytical Response</div>
        <div class="mt-1 text-sm text-zinc-700">{{ responseStatus }}</div>
      </div>

      <div class="text-xs uppercase tracking-[0.16em] text-zinc-500">
        {{ enabled ? 'Target curve active' : 'Bypassed target curve' }}
      </div>
    </div>

    <div class="overflow-hidden rounded-[18px] border border-zinc-300 bg-zinc-950 p-3">
      <svg viewBox="0 0 640 260" class="h-[260px] w-full">
        <g>
          <line
            v-for="tick in gainTicks"
            :key="`gain-${tick}`"
            x1="48"
            x2="624"
            :y1="mapGain(tick)"
            :y2="mapGain(tick)"
            stroke="rgba(255,255,255,0.1)"
            stroke-width="1"
          />

          <line
            v-for="tick in frequencyTicks"
            :key="`freq-${tick}`"
            :x1="mapFrequency(tick)"
            :x2="mapFrequency(tick)"
            y1="16"
            y2="228"
            stroke="rgba(255,255,255,0.08)"
            stroke-width="1"
          />

          <line
            x1="48"
            x2="624"
            :y1="mapGain(0)"
            :y2="mapGain(0)"
            stroke="#38bdf8"
            stroke-width="1.5"
          />

          <path
            v-for="(path, index) in perBandPaths"
            :key="`band-${index}`"
            :d="path"
            fill="none"
            stroke="rgba(125,211,252,0.34)"
            stroke-width="1.25"
            stroke-dasharray="4 4"
          />

          <path
            v-if="compositePath"
            :d="compositePath"
            fill="none"
            :stroke="enabled ? '#f8fafc' : '#94a3b8'"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </g>

        <g>
          <text
            v-for="tick in gainTicks"
            :key="`gain-label-${tick}`"
            x="42"
            :y="mapGain(tick) + 4"
            text-anchor="end"
            font-size="11"
            fill="rgba(255,255,255,0.65)"
          >
            {{ tick }} dB
          </text>

          <text
            v-for="tick in frequencyTicks"
            :key="`freq-label-${tick}`"
            :x="mapFrequency(tick)"
            y="248"
            text-anchor="middle"
            font-size="11"
            fill="rgba(255,255,255,0.65)"
          >
            {{ formatFrequencyValue(tick) }}
          </text>
        </g>
      </svg>
    </div>

    <div
      v-if="spectralEnabled || spectralBands.length"
      class="grid gap-3 rounded-[18px] border border-zinc-300 bg-zinc-50 p-3"
    >
      <div class="flex flex-wrap items-center justify-between gap-2">
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Live Spectrum</div>
        <div class="text-xs text-zinc-500">{{ spectralStatus }}</div>
      </div>

      <div v-if="spectralBands.length" class="grid gap-3">
        <div class="grid gap-1">
          <div class="text-[10px] uppercase tracking-[0.16em] text-zinc-500">Input</div>
          <div class="grid gap-1" :style="bandGridStyle">
            <div
              v-for="(band, index) in spectralBands"
              :key="`input-${index}`"
              class="rounded-t-sm bg-cyan-500"
              :style="{ height: `${band.inputHeight}%` }"
            ></div>
          </div>
        </div>

        <div class="grid gap-1">
          <div class="text-[10px] uppercase tracking-[0.16em] text-zinc-500">Output</div>
          <div class="grid gap-1" :style="bandGridStyle">
            <div
              v-for="(band, index) in spectralBands"
              :key="`output-${index}`"
              class="rounded-t-sm bg-emerald-500"
              :style="{ height: `${band.outputHeight}%` }"
            ></div>
          </div>
        </div>
      </div>

      <div
        v-else
        class="rounded-[16px] border border-dashed border-zinc-300 bg-white/80 px-3 py-5 text-sm text-zinc-500"
      >
        {{ spectralStatus }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { EffectBandSnapshot, FilterResponseCurve } from '../../../typings/effects'
import { normalizeDb } from '../../../utils/effect-visuals'
import { formatFrequencyValue } from '../../../utils/effects'

interface Props {
  response: FilterResponseCurve | null
  spectral?: EffectBandSnapshot | null
  enabled: boolean
  isPlaying?: boolean
  spectralEnabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  spectral: null,
  isPlaying: false,
  spectralEnabled: false,
})

const frequencyTicks = [20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000]
const gainTicks = [-24, -12, 0, 12, 24]
const MIN_GAIN_DB = -36
const MAX_GAIN_DB = 24
const GRAPH_LEFT = 48
const GRAPH_RIGHT = 624
const GRAPH_TOP = 16
const GRAPH_BOTTOM = 228
const MIN_SPECTRAL_DB = -72
const MAX_SPECTRAL_DB = 12

const mapFrequency = (frequency: number) => {
  const minLog = Math.log10(20)
  const maxLog = Math.log10(20_000)
  const ratio = (Math.log10(Math.max(frequency, 20)) - minLog) / (maxLog - minLog)
  return GRAPH_LEFT + ratio * (GRAPH_RIGHT - GRAPH_LEFT)
}

const mapGain = (gainDb: number) => {
  const ratio = (MAX_GAIN_DB - gainDb) / (MAX_GAIN_DB - MIN_GAIN_DB)
  return GRAPH_TOP + ratio * (GRAPH_BOTTOM - GRAPH_TOP)
}

const buildPath = (points: { freq_hz: number; gain_db: number }[]) => {
  if (!points.length) return ''

  return points
    .map((point, index) => {
      const prefix = index === 0 ? 'M' : 'L'
      return `${prefix} ${mapFrequency(point.freq_hz).toFixed(2)} ${mapGain(point.gain_db).toFixed(2)}`
    })
    .join(' ')
}

const compositePath = computed(() => buildPath(props.response?.composite ?? []))
const perBandPaths = computed(
  () => props.response?.per_band.map((points) => buildPath(points)) ?? [],
)

const responseStatus = computed(() => {
  if (!props.response) return 'Analytical curve not available yet.'
  if (!props.enabled) return 'Configured target stays visible while the effect is bypassed.'
  return 'Target response updates immediately with every filter change.'
})

const spectralStatus = computed(() => {
  if (!props.spectralEnabled) return 'Spectral overlay idle.'
  if (!props.isPlaying) return 'Start playback for live spectral motion.'
  if (!props.spectral) return 'Spectral analysis is warming up.'
  return 'Input and output buckets are tracking audible-time playback.'
})

const spectralBands = computed(() => {
  if (!props.spectral) return []
  const input = props.spectral.input.bands_db
  const output = props.spectral.output.bands_db
  const centers = props.spectral.input.band_centers_hz

  return centers.map((center, index) => ({
    center,
    inputHeight:
      normalizeDb(input[index] ?? MIN_SPECTRAL_DB, MIN_SPECTRAL_DB, MAX_SPECTRAL_DB) * 100,
    outputHeight:
      normalizeDb(output[index] ?? MIN_SPECTRAL_DB, MIN_SPECTRAL_DB, MAX_SPECTRAL_DB) * 100,
  }))
})

const bandGridStyle = computed(() => ({
  gridTemplateColumns: `repeat(${Math.max(spectralBands.value.length, 1)}, minmax(0, 1fr))`,
  alignItems: 'end',
  height: '72px',
}))
</script>
