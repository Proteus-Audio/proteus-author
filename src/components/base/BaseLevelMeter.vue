<template>
  <div
    ref="meterRef"
    class="level-meter"
    :class="{ inactive: !audio.isPlaying, vertical }"
    :style="{ gridTemplateColumns: `repeat(${levelsDb.length}, 1fr)` }"
  >
    <div v-for="(level, index) in levelsDb" :key="index" class="meter-column">
      <!-- <div class="meter-channel peak-indicator">
        <DigitalIndicator
          :state="indicator.on"
          color="dark-red"
          size="small"
        />
      </div> -->
      <div class="meter-channel">
        <DigitalIndicator
          v-for="indicator in channelIndicators[index]"
          :key="indicator.id"
          :state="indicator.on"
          :color="indicator.color"
          :frozen="!audio.isPlaying"
          size="medium"
        />
      </div>
      <div class="meter-label">{{ indicatorCount }} {{ `${level.toFixed(1)} dB` }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useElementSize } from '@vueuse/core'
import { computed, ref } from 'vue'
import { DigitalIndicator } from '../digital'
import { useAudioStore } from '../../stores/audio'

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

const levelsDb = computed(() => {
  const levels = audio.getLevelsDb.length ? audio.getLevelsDb : [minDb, minDb]
  return levels.map((value) => {
    const safe = Number.isFinite(value) ? value : minDb
    return Math.max(minDb, Math.min(0, safe))
  })
})

const indicatorCount = computed(() => {
  const labelAndPadding = vertical.value ? 38 : 30
  const usableHeight = Math.max(0, meterHeight.value - labelAndPadding)
  console.log('usableHeight', usableHeight)
  const cellHeight = indicatorHeightPx + indicatorGapPx
  const count = Math.floor((usableHeight + indicatorGapPx) / cellHeight)
  return Math.max(4, Math.min(48, count))
})

const colorForDb = (db: number): IndicatorColor => {
  if (db >= -1) return 'dark-red'
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
        // peak: level > thresholdDb,
      }
    })
  })
})
</script>

<style scoped lang="scss">
.level-meter {
  height: 40px;
  width: 100%;
  display: grid;
  gap: 6px;
  padding: 4px;
  border-radius: 6px;
  background: #f6f6f6;
  border: 1px solid #d8d8d8;
  align-items: stretch;
}

.level-meter.vertical {
  height: 100%;
  border-radius: 0;
  border-left: 1px solid #d8d8d8;
  border-right: 0;
  border-top: 0;
  border-bottom: 0;
  padding: 8px 6px;
}

.meter-column {
  display: flex;
  flex-direction: column;
  gap: 6px;
  height: 100%;
}

.meter-channel {
  display: grid;
  grid-auto-rows: minmax(0, 1fr);
  gap: 3px;
  flex: 1 1 auto;
  border-radius: 4px;
  padding: 4px;
}

.meter-label {
  font-size: 11px;
  text-align: center;
  color: #616161;
  line-height: 1;
  font-variant-numeric: tabular-nums;
}

.inactive .meter-label {
  color: #8a8a8a;
}
</style>
