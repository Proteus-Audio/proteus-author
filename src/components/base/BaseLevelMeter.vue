<template>
  <div
    class="level-meter"
    :class="{ inactive: !audio.isPlaying, vertical }"
    :style="{ gridTemplateColumns: `repeat(${meterLevels.length}, 1fr)` }"
  >
    <div v-for="(level, index) in meterLevels" :key="index" class="meter-channel">
      <div class="meter-bar" :style="{ height: `${level}%` }"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAudioStore } from '../../stores/audio'

const props = defineProps<{
  vertical?: boolean
}>()

const audio = useAudioStore()
const minDb = -60

const vertical = computed(() => props.vertical ?? false)

const meterLevels = computed(() => {
  const levels = audio.getLevelsDb.length ? audio.getLevelsDb : [minDb, minDb]
  return levels.map((value) => {
    const safe = Number.isFinite(value) ? value : minDb
    const clamped = Math.max(minDb, Math.min(0, safe))
    return ((clamped - minDb) / -minDb) * 100
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

.meter-channel {
  position: relative;
  height: 100%;
  background: #e3e3e3;
  border-radius: 4px;
  overflow: hidden;
}

.meter-bar {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 0%;
  background: linear-gradient(180deg, #e94b4b 0%, #f3d55a 35%, #5ac47a 100%);
  transition: height 60ms linear;
}

.inactive .meter-bar {
  background: #bcbcbc;
}
</style>
