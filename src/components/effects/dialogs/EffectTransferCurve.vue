<template>
  <div class="grid gap-4">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Transfer Curve</div>
        <div class="mt-1 text-sm text-zinc-700">{{ description }}</div>
      </div>

      <div class="text-xs uppercase tracking-[0.16em] text-zinc-500">
        {{ enabled ? 'Processing active' : 'Bypassed preview' }}
      </div>
    </div>

    <div class="overflow-hidden rounded-[18px] border border-zinc-300 bg-zinc-950 p-3">
      <svg viewBox="0 0 640 260" class="h-[260px] w-full">
        <g>
          <line
            v-for="tick in ticks"
            :key="`horizontal-${tick}`"
            x1="44"
            x2="624"
            :y1="mapY(tick)"
            :y2="mapY(tick)"
            stroke="rgba(255,255,255,0.08)"
            stroke-width="1"
          />
          <line
            v-for="tick in ticks"
            :key="`vertical-${tick}`"
            :x1="mapX(tick)"
            :x2="mapX(tick)"
            y1="16"
            y2="228"
            stroke="rgba(255,255,255,0.08)"
            stroke-width="1"
          />

          <path :d="identityPath" fill="none" stroke="rgba(148,163,184,0.55)" stroke-width="1.5" />
          <path
            v-for="marker in verticalMarkers"
            :key="`vx-${marker.label}`"
            :d="markerPath(marker.value, 'x')"
            fill="none"
            stroke="rgba(56,189,248,0.75)"
            stroke-width="1.25"
            stroke-dasharray="5 4"
          />
          <path
            v-for="marker in horizontalMarkers"
            :key="`hy-${marker.label}`"
            :d="markerPath(marker.value, 'y')"
            fill="none"
            stroke="rgba(34,197,94,0.75)"
            stroke-width="1.25"
            stroke-dasharray="5 4"
          />
          <path
            v-if="curvePath"
            :d="curvePath"
            fill="none"
            :stroke="enabled ? '#f8fafc' : '#94a3b8'"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </g>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { buildSvgPath, type ChartPoint } from '../../../utils/effect-visuals'

interface Marker {
  axis: 'x' | 'y'
  value: number
  label: string
}

interface Props {
  points: ChartPoint[]
  xMin: number
  xMax: number
  yMin: number
  yMax: number
  enabled: boolean
  description: string
  markers?: Marker[]
}

const props = withDefaults(defineProps<Props>(), {
  markers: () => [],
})

const GRAPH_LEFT = 44
const GRAPH_RIGHT = 624
const GRAPH_TOP = 16
const GRAPH_BOTTOM = 228

const mapX = (value: number) => {
  const ratio = (value - props.xMin) / Math.max(props.xMax - props.xMin, 1e-6)
  return GRAPH_LEFT + ratio * (GRAPH_RIGHT - GRAPH_LEFT)
}

const mapY = (value: number) => {
  const ratio = (props.yMax - value) / Math.max(props.yMax - props.yMin, 1e-6)
  return GRAPH_TOP + ratio * (GRAPH_BOTTOM - GRAPH_TOP)
}

const ticks = computed(() => {
  return Array.from({ length: 5 }, (_, index) => {
    const ratio = index / 4
    return props.xMin + ratio * (props.xMax - props.xMin)
  })
})

const curvePath = computed(() => buildSvgPath(props.points, mapX, mapY))
const identityPath = computed(() => {
  const identityPoints: ChartPoint[] = [
    { x: props.xMin, y: props.yMin },
    { x: props.xMax, y: props.yMax },
  ]
  return buildSvgPath(identityPoints, mapX, mapY)
})

const verticalMarkers = computed(() => props.markers.filter((marker) => marker.axis === 'x'))
const horizontalMarkers = computed(() => props.markers.filter((marker) => marker.axis === 'y'))

const markerPath = (value: number, axis: 'x' | 'y') => {
  if (axis === 'x') {
    const x = mapX(value)
    return `M ${x} ${GRAPH_TOP} L ${x} ${GRAPH_BOTTOM}`
  }

  const y = mapY(value)
  return `M ${GRAPH_LEFT} ${y} L ${GRAPH_RIGHT} ${y}`
}
</script>
