<template>
  <div
    class="level-meter"
    :class="{ inactive: !audio.isPlaying, vertical }"
    :style="{ gridTemplateColumns: `repeat(${meterLevels.length}, 1fr)` }"
  >
    <div v-for="(level, index) in meterLevels" :key="index" class="meter-column">
      <div class="meter-channel">
        <div class="meter-mask" :style="{ height: `${meterMasks[index]}%` }"></div>
        <div class="meter-peak" :style="{ bottom: `${peakLevels[index]}%` }"></div>
      </div>
      <div class="meter-label">{{ meterLabels[index] }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useAudioStore } from '../../stores/audio'

const props = defineProps<{
  vertical?: boolean
}>()

const audio = useAudioStore()
const minDb = -60

const vertical = computed(() => props.vertical ?? false)

const levelsDb = computed(() => {
  const levels = audio.getLevelsDb.length ? audio.getLevelsDb : [minDb, minDb]
  return levels.map((value) => {
    const safe = Number.isFinite(value) ? value : minDb
    return Math.max(minDb, Math.min(0, safe))
  })
})

const meterLevels = computed(() => {
  return levelsDb.value.map((value) => ((value - minDb) / -minDb) * 100)
})

const meterMasks = computed(() => {
  return meterLevels.value.map((level) => Math.max(0, Math.min(100, 100 - level)))
})

const meterLabels = computed(() => {
  return levelsDb.value.map((value) => `${value.toFixed(1)} dB`)
})

const peakDb = ref<number[]>([])

const syncPeaks = (levels: number[]) => {
  if (levels.length !== peakDb.value.length) {
    peakDb.value = levels.map(() => minDb)
  }
}

watch(
  levelsDb,
  (levels) => {
    syncPeaks(levels)
    if (!audio.isPlaying) return
    peakDb.value = levels.map((level, index) => {
      const currentPeak = peakDb.value[index] ?? minDb
      return Math.max(level, currentPeak)
    })
  },
  { immediate: true },
)

watch(
  () => audio.isPlaying,
  (playing) => {
    if (!playing) return
    const levels = levelsDb.value.length ? levelsDb.value : [minDb, minDb]
    peakDb.value = levels.slice()
  },
)

const peakLevels = computed(() => {
  const values = peakDb.value.length === meterLevels.value.length ? peakDb.value : levelsDb.value
  return values.map((value) => ((value - minDb) / -minDb) * 100)
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
  align-items: end;
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
  position: relative;
  flex: 1 1 auto;
  background: #e3e3e3;
  border-radius: 4px;
  overflow: hidden;
  background: linear-gradient(180deg, #e94b4b 0%, #f3d55a 35%, #5ac47a 100%);
}

.meter-mask {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  height: 100%;
  background: #e3e3e3;
  transition: height 60ms linear;
}

.meter-peak {
  position: absolute;
  left: 0;
  right: 0;
  height: 2px;
  background: #222;
  transform: translateY(1px);
}

.meter-label {
  font-size: 11px;
  text-align: center;
  color: #616161;
  line-height: 1;
  font-variant-numeric: tabular-nums;
}

.inactive .meter-channel {
  background: #bcbcbc;
}

.inactive .meter-mask {
  background: #bcbcbc;
}

.inactive .meter-peak {
  background: #8a8a8a;
}
</style>
