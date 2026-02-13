<template>
  <div class="digital-fader">
    <div class="fader-label">GAIN</div>

    <div class="fader-container">
      <!-- <div class="scale-label scale-top">+10</div> -->
      <!-- <div class="scale-label scale-mid">-25</div> -->
      <!-- <div class="scale-label scale-bottom">-60</div> -->

      <div ref="trackRef" class="fader-track" @pointerdown="onTrackPointerDown">
        <div class="fader-grid"></div>
        <div class="fader-fill" :style="{ height: `${fillPercent}%` }"></div>
        <button
          type="button"
          class="fader-cap"
          :style="{ bottom: capBottom }"
          aria-label="Volume fader"
          @pointerdown.stop="onCapPointerDown"
        >
          <span class="cap-grip"></span>
        </button>
      </div>
    </div>

    <div class="fader-readout">{{ dbReadout }}</div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

const minDb = -60
const maxDb = 10
const capHeight = 42

const trackRef = ref<HTMLElement | null>(null)
const dbValue = ref(0)
let removePlayerChangedListener: (() => void) | null = null

const clampDb = (db: number) => Math.min(maxDb, Math.max(minDb, db))
const dbToLinear = (db: number) => Math.pow(10, db / 20)
const linearToDb = (linear: number) => {
  const safe = Math.max(linear, 0.0001)
  return clampDb(20 * Math.log10(safe))
}

const fillPercent = computed(() => ((dbValue.value - minDb) / (maxDb - minDb)) * 100)
const capBottom = computed(() => `calc(${fillPercent.value}% - ${capHeight / 2}px)`)
const dbReadout = computed(() => `${dbValue.value >= 0 ? '+' : ''}${dbValue.value.toFixed(1)} dB`)

const commitVolume = () => {
  const linear = dbToLinear(dbValue.value)
  void invoke('set_volume', { volume: linear })
}

const refreshFromBackend = async () => {
  try {
    const linear = await invoke<number>('get_volume')
    dbValue.value = linearToDb(linear)
  } catch {
    // Keep current UI value on failures.
  }
}

const setFromPointer = (clientY: number) => {
  const track = trackRef.value
  if (!track) return
  const rect = track.getBoundingClientRect()
  const ratio = (rect.bottom - clientY) / Math.max(rect.height, 1)
  const nextDb = clampDb(minDb + ratio * (maxDb - minDb))
  if (Math.abs(nextDb - dbValue.value) < 0.05) return
  dbValue.value = nextDb
  commitVolume()
}

const onTrackPointerDown = (event: PointerEvent) => {
  event.preventDefault()
  setFromPointer(event.clientY)
}

const onPointerMove = (event: PointerEvent) => {
  setFromPointer(event.clientY)
}

const clearDragging = () => {
  window.removeEventListener('pointermove', onPointerMove)
  window.removeEventListener('pointerup', clearDragging)
}

const onCapPointerDown = (event: PointerEvent) => {
  event.preventDefault()
  window.addEventListener('pointermove', onPointerMove)
  window.addEventListener('pointerup', clearDragging)
}

onMounted(async () => {
  await refreshFromBackend()
  removePlayerChangedListener = await listen('PLAYER_CHANGED', async () => {
    await refreshFromBackend()
  })
})

onBeforeUnmount(() => {
  clearDragging()
  if (removePlayerChangedListener) {
    removePlayerChangedListener()
  }
})
</script>

<style scoped lang="scss">
.digital-fader {
  width: 54px;
  height: 100%;
  display: grid;
  grid-template-rows: 24px minmax(0, 1fr) 22px;
  gap: 8px;
  padding: 8px 6px;
  background: #e5e5e5;
  border-left: 2px solid #b9b9b9;
}

.fader-container {
  position: relative;
  border: 2px solid #8f8f8f;
  border-radius: 2px;
  background: #d2d2d2;
  overflow: hidden;
}

.scale-label {
  position: absolute;
  right: 2px;
  font-size: 8px;
  color: #5d5d5d;
  z-index: 2;
  line-height: 1;
  pointer-events: none;
}

.scale-top {
  top: 3px;
}

.scale-mid {
  top: 50%;
  transform: translateY(-50%);
}

.scale-bottom {
  bottom: 3px;
}

.fader-label,
.fader-readout {
  font-size: 10px;
  text-align: center;
  color: #4b4b4b;
  letter-spacing: 0.04em;
}

.fader-track {
  position: absolute;
  top: 2rem;
  left: 0;
  right: 0;
  bottom: 0.8rem;
  cursor: ns-resize;
}

.fader-grid {
  position: absolute;
  inset: 0;
  top: -1.2rem;
  background-image: repeating-linear-gradient(
    to right,
    transparent 0,
    transparent 17px,
    rgba(80, 80, 80, 0.18) 17px,
    rgba(80, 80, 80, 0.18) 19px,
    transparent 19px,
    transparent 36px
  );
  pointer-events: none;
}

.fader-fill {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
}

.fader-cap {
  position: absolute;
  left: 3px;
  right: 3px;
  height: 42px;
  border: 2px solid #616161;
  background: #f0f0f0;
  padding: 0;
  cursor: ns-resize;

  &:before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 100%;
    background: repeating-linear-gradient(
      to bottom,
      white 0,
      white 3px,
      #969696 3px,
      transparent 20px,
      white 34px,
      #969696 34px,
      #969696 42px
    );
  }
}

.cap-grip {
  display: block;
  width: 100%;
  height: 100%;
  background-image: repeating-linear-gradient(
    to bottom,
    transparent 0,
    transparent 3px,
    #969696 3px,
    #969696 5px
  );
}
</style>
