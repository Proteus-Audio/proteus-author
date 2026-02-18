<template>
  <div
    class="grid h-full w-[84px] grid-rows-[1fr_1fr] gap-2 border-2 border-zinc-500 bg-zinc-200 p-1.5"
  >
    <div class="grid grid-rows-[12px_1fr_14px] justify-items-center">
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">LVL</div>
      <button
        ref="levelKnobRef"
        type="button"
        class="relative size-[34px] rounded-full border-2 border-zinc-600 bg-zinc-100"
        aria-label="Track level"
        @pointerdown="onLevelPointerDown"
        @wheel="onLevelWheel"
      >
        <span class="absolute inset-[4px] rounded-full border border-zinc-500 bg-zinc-200"></span>
        <span
          class="pointer-events-none absolute top-1/2 left-1/2 h-0 w-0 -translate-x-1/2 -translate-y-1/2"
        >
          <span
            class="absolute -top-[11px] -left-[1px] h-[11px] w-[2px] rounded bg-zinc-700"
            :style="{ transform: `rotate(${levelAngle}deg)`, transformOrigin: 'center 11px' }"
          ></span>
        </span>
      </button>
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">{{ levelReadout }}</div>
    </div>

    <div class="grid grid-rows-[12px_1fr_14px] justify-items-center">
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">PAN</div>
      <button
        ref="panKnobRef"
        type="button"
        class="relative size-[34px] rounded-full border-2 border-zinc-600 bg-zinc-100"
        aria-label="Track pan"
        @pointerdown="onPanPointerDown"
        @wheel="onPanWheel"
      >
        <span class="absolute inset-[4px] rounded-full border border-zinc-500 bg-zinc-200"></span>
        <span
          class="pointer-events-none absolute top-1/2 left-1/2 h-0 w-0 -translate-x-1/2 -translate-y-1/2"
        >
          <span
            class="absolute -top-[11px] -left-[1px] h-[11px] w-[2px] rounded bg-zinc-700"
            :style="{ transform: `rotate(${panAngle}deg)`, transformOrigin: 'center 11px' }"
          ></span>
        </span>
      </button>
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">{{ panReadout }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'

interface Props {
  level: number
  pan: number
  dragAxis?: 'horizontal' | 'vertical'
}

const props = withDefaults(defineProps<Props>(), {
  dragAxis: 'horizontal',
})

const emit = defineEmits<{
  (event: 'update:level', value: number): void
  (event: 'update:pan', value: number): void
}>()

const minDb = -60
const maxDb = 10
const knobMinAngle = -135
const knobMaxAngle = 135

const levelKnobRef = ref<HTMLElement | null>(null)
const panKnobRef = ref<HTMLElement | null>(null)
const levelDb = ref(0)
const panValue = ref(0)
const lastLevelPointer = ref<{ x: number; y: number } | null>(null)
const lastPanPointer = ref<{ x: number; y: number } | null>(null)

const clampDb = (db: number) => Math.min(maxDb, Math.max(minDb, db))
const clampPan = (pan: number) => Math.min(1, Math.max(-1, pan))

const dbToLinear = (db: number) => Math.pow(10, db / 20)
const linearToDb = (linear: number) => {
  const safe = Math.max(linear, 0.0001)
  return clampDb(20 * Math.log10(safe))
}

watch(
  () => props.level,
  (level) => {
    levelDb.value = linearToDb(level)
  },
  { immediate: true },
)

watch(
  () => props.pan,
  (pan) => {
    panValue.value = clampPan(pan)
  },
  { immediate: true },
)

const levelAngle = computed(() => {
  const ratio = (levelDb.value - minDb) / (maxDb - minDb)
  return knobMinAngle + ratio * (knobMaxAngle - knobMinAngle)
})

const panAngle = computed(() => {
  const ratio = (panValue.value + 1) / 2
  return knobMinAngle + ratio * (knobMaxAngle - knobMinAngle)
})

const levelReadout = computed(
  () => `${levelDb.value >= 0 ? '+' : ''}${levelDb.value.toFixed(1)} dB`,
)

const panReadout = computed(() => {
  if (Math.abs(panValue.value) < 0.02) return 'C'
  const side = panValue.value < 0 ? 'L' : 'R'
  return `${side}${Math.round(Math.abs(panValue.value) * 100)}`
})

const levelStepDb = 0.5
const panStep = 0.02

const emitLevelDb = (nextDb: number) => {
  const clamped = clampDb(nextDb)
  if (Math.abs(clamped - levelDb.value) < 0.01) return
  levelDb.value = clamped
  emit('update:level', dbToLinear(clamped))
}

const emitPanValue = (nextPan: number) => {
  const clamped = clampPan(nextPan)
  if (Math.abs(clamped - panValue.value) < 0.001) return
  panValue.value = clamped
  emit('update:pan', clamped)
}

const emitLevelFromPointer = (clientX: number, clientY: number) => {
  const previous = lastLevelPointer.value
  if (!previous) {
    lastLevelPointer.value = { x: clientX, y: clientY }
    return
  }
  const delta = props.dragAxis === 'vertical' ? previous.y - clientY : clientX - previous.x
  lastLevelPointer.value = { x: clientX, y: clientY }
  const nextDb = levelDb.value + delta * 0.12
  emitLevelDb(nextDb)
}

const emitPanFromPointer = (clientX: number, clientY: number) => {
  const previous = lastPanPointer.value
  if (!previous) {
    lastPanPointer.value = { x: clientX, y: clientY }
    return
  }
  const delta = props.dragAxis === 'vertical' ? previous.y - clientY : clientX - previous.x
  lastPanPointer.value = { x: clientX, y: clientY }
  const nextPan = panValue.value + delta * 0.004
  emitPanValue(nextPan)
}

const onLevelMove = (event: PointerEvent) => {
  emitLevelFromPointer(event.clientX, event.clientY)
}

const onPanMove = (event: PointerEvent) => {
  emitPanFromPointer(event.clientX, event.clientY)
}

const clearLevelDrag = () => {
  lastLevelPointer.value = null
  window.removeEventListener('pointermove', onLevelMove)
  window.removeEventListener('pointerup', clearLevelDrag)
}

const clearPanDrag = () => {
  lastPanPointer.value = null
  window.removeEventListener('pointermove', onPanMove)
  window.removeEventListener('pointerup', clearPanDrag)
}

const onLevelPointerDown = (event: PointerEvent) => {
  event.preventDefault()
  lastLevelPointer.value = { x: event.clientX, y: event.clientY }
  emitLevelFromPointer(event.clientX, event.clientY)
  window.addEventListener('pointermove', onLevelMove)
  window.addEventListener('pointerup', clearLevelDrag)
}

const onPanPointerDown = (event: PointerEvent) => {
  event.preventDefault()
  lastPanPointer.value = { x: event.clientX, y: event.clientY }
  emitPanFromPointer(event.clientX, event.clientY)
  window.addEventListener('pointermove', onPanMove)
  window.addEventListener('pointerup', clearPanDrag)
}

const onLevelWheel = (event: WheelEvent) => {
  event.preventDefault()
  const direction = event.deltaY < 0 ? 1 : -1
  emitLevelDb(levelDb.value + direction * levelStepDb)
}

const onPanWheel = (event: WheelEvent) => {
  event.preventDefault()
  const direction = event.deltaY < 0 ? 1 : -1
  emitPanValue(panValue.value + direction * panStep)
}

onBeforeUnmount(() => {
  clearLevelDrag()
  clearPanDrag()
})
</script>
