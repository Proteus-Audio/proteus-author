<template>
  <div class="digital-fader">
    <div class="fader-label">GAIN</div>

    <div class="fader-container">
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

    <div class="fader-readout">{{ `${Math.ceil((value / 100) * 30) / 10}` }}</div>
    <!-- <div class="fader-readout">{{ `${Math.round(fillPercent)}%` }}</div> -->
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onBeforeUnmount, ref } from 'vue'
import { toneMaster } from '../../assets/toneMaster'

const minValue = 0
const maxValue = 75
const capHeight = 22

const trackRef = ref<HTMLElement | null>(null)
const value = ref(Math.min(maxValue, Math.max(minValue, Math.round(toneMaster.volume * 75))))

const fillPercent = computed(() => (value.value / maxValue) * 100)
const capBottom = computed(() => `calc(${fillPercent.value}% - ${capHeight / 2}px)`)

const clamp = (next: number) => Math.min(maxValue, Math.max(minValue, next))

const commitVolume = () => {
  void invoke('set_volume', { volume: (value.value / 100) * 3 })
}

const setFromPointer = (clientY: number) => {
  const track = trackRef.value
  if (!track) return
  const rect = track.getBoundingClientRect()
  const ratio = (rect.bottom - clientY) / Math.max(rect.height, 1)
  const next = clamp(Math.round(ratio * maxValue))
  if (next === value.value) return
  value.value = next
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

onBeforeUnmount(() => {
  clearDragging()
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
