<template>
  <div
    class="grid h-full w-[54px] grid-rows-[24px_minmax(0,1fr)_22px] gap-2 border-l-2 border-zinc-400 bg-zinc-200 px-1.5 py-2"
  >
    <div class="text-center text-[10px] tracking-[0.04em] text-zinc-600">GAIN</div>

    <div class="relative overflow-hidden rounded-[2px] border-2 border-zinc-500 bg-zinc-300">
      <div
        ref="trackRef"
        class="absolute top-8 right-0 bottom-3 left-0 cursor-ns-resize"
        @pointerdown="onTrackPointerDown"
      >
        <div
          class="pointer-events-none absolute inset-0 -top-5 bg-[repeating-linear-gradient(to_right,transparent_0,transparent_17px,rgba(80,80,80,0.18)_17px,rgba(80,80,80,0.18)_19px,transparent_19px,transparent_36px)]"
        ></div>
        <div
          class="pointer-events-none absolute right-0 bottom-0 left-0"
          :style="{ height: `${fillPercent}%` }"
        ></div>

        <button
          type="button"
          class="absolute right-[3px] left-[3px] h-[42px] cursor-ns-resize border-2 border-zinc-600 bg-zinc-100 p-0"
          :style="{ bottom: capBottom }"
          aria-label="Volume fader"
          @pointerdown.stop="onCapPointerDown"
        >
          <span
            class="absolute inset-0 bg-[repeating-linear-gradient(to_bottom,white_0,white_3px,#969696_3px,transparent_20px,white_34px,#969696_34px,#969696_42px)]"
          ></span>
          <span
            class="relative block h-full w-full bg-[repeating-linear-gradient(to_bottom,transparent_0,transparent_3px,#969696_3px,#969696_5px)]"
          ></span>
        </button>
      </div>
    </div>

    <div class="text-center text-[10px] tracking-[0.04em] text-zinc-600">{{ dbReadout }}</div>
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
  removePlayerChangedListener = await listen('PLAYER_CHANGED', () => {
    void refreshFromBackend()
  })
})

onBeforeUnmount(() => {
  clearDragging()
  if (removePlayerChangedListener) {
    removePlayerChangedListener()
  }
})
</script>
