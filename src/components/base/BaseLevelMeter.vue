<template>
  <div
    ref="meterRef"
    class="grid items-stretch gap-1.5 border border-zinc-300 bg-zinc-100 p-1"
    :class="vertical ? 'h-full border-y-0 border-r-0 rounded-none px-1.5 py-2' : 'h-10 rounded-md'"
    :style="{ gridTemplateColumns: `repeat(${levelsDb.length}, 1fr)` }"
  >
    <div v-for="(level, index) in levelsDb" :key="index" class="flex h-full flex-col gap-1.5">
      <button
        type="button"
        class="cursor-pointer bg-transparent px-1 py-0"
        :aria-pressed="clipPeaks[index] ? 'true' : 'false'"
        @click="clearAllClipPeaks"
      >
        <DigitalIndicator :state="clipPeaks[index] ?? false" color="dark-red" size="small" />
      </button>

      <div class="grid flex-1 auto-rows-fr gap-[3px] rounded p-1">
        <DigitalIndicator
          v-for="indicator in channelIndicators[index]"
          :key="indicator.id"
          :state="indicator.on"
          :color="indicator.color"
          :frozen="!audio.isPlaying"
          size="medium"
        />
      </div>

      <div
        class="text-center text-[11px] leading-none tabular-nums"
        :class="audio.isPlaying ? 'text-zinc-600' : 'text-zinc-400'"
      >
        {{ `${level.toFixed(1)} dB` }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useElementSize } from '@vueuse/core'
import { computed, ref, watch } from 'vue'
import { useAudioStore } from '../../stores/audio'
import { DigitalIndicator } from '../digital'

const props = defineProps<{
  vertical?: boolean
}>()

type IndicatorColor = 'red' | 'green' | 'lime' | 'amber' | 'yellow' | 'orange' | 'dark-red'

const audio = useAudioStore()
const minDb = -60
const indicatorGapPx = 10
const indicatorHeightPx = 20
const meterRef = ref<HTMLElement | null>(null)
const { height: meterHeight } = useElementSize(meterRef)

const vertical = computed(() => props.vertical ?? false)

const rawLevelsDb = computed(() => {
  const levels = audio.getLevelsDb.length ? audio.getLevelsDb : [minDb, minDb]
  return levels.map((value) => (Number.isFinite(value) ? value : minDb))
})

const levelsDb = computed(() => {
  return rawLevelsDb.value.map((value) => Math.max(minDb, Math.min(0, value)))
})

const indicatorCount = computed(() => {
  const labelAndPadding = vertical.value ? 60 : 52
  const usableHeight = Math.max(0, meterHeight.value - labelAndPadding)
  const cellHeight = indicatorHeightPx + indicatorGapPx
  const count = Math.floor((usableHeight + indicatorGapPx) / cellHeight)
  return Math.max(4, Math.min(48, count))
})

const colorForDb = (db: number): IndicatorColor => {
  if (db >= -6) return 'red'
  if (db >= -12) return 'orange'
  if (db >= -24) return 'yellow'
  if (db >= -36) return 'lime'
  return 'green'
}

const channelIndicators = computed(() => {
  return levelsDb.value.map((level) => {
    return Array.from({ length: indicatorCount.value }, (_, index) => {
      const ratio = (indicatorCount.value - index) / indicatorCount.value
      const thresholdDb = minDb + ratio * -minDb
      return {
        id: `seg-${index}`,
        color: colorForDb(thresholdDb),
        on: level >= thresholdDb,
      }
    })
  })
})

const clipPeaks = ref<boolean[]>([])
const stoppedSinceLastPlay = ref(true)

const syncClipPeaks = (channelCount: number) => {
  if (clipPeaks.value.length === channelCount) return
  clipPeaks.value = Array.from(
    { length: channelCount },
    (_, index) => clipPeaks.value[index] ?? false,
  )
}

const clearAllClipPeaks = () => {
  clipPeaks.value = clipPeaks.value.map(() => false)
}

watch(
  rawLevelsDb,
  (levels) => {
    syncClipPeaks(levels.length)
    if (!audio.isPlaying) return
    clipPeaks.value = levels.map((level, index) => (clipPeaks.value[index] ?? false) || level > 0)
  },
  { immediate: true },
)

const isStopped = computed(() => {
  return !audio.isPlaying && rawLevelsDb.value.every((level) => level <= minDb)
})

watch(
  isStopped,
  (stopped) => {
    if (!stopped) return
    clearAllClipPeaks()
    stoppedSinceLastPlay.value = true
  },
  { immediate: true },
)

watch(
  () => audio.isPlaying,
  (playing) => {
    if (!playing) return
    if (!stoppedSinceLastPlay.value) return
    clearAllClipPeaks()
    stoppedSinceLastPlay.value = false
  },
)
</script>
