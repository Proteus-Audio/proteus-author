<template>
  <button
    type="button"
    class="relative rounded-full border-2 border-zinc-600 bg-zinc-100"
    :style="knobStyle"
    :aria-label="ariaLabel"
    @pointerdown="onPointerDown"
    @wheel="onWheel"
  >
    <span
      class="absolute rounded-full border border-zinc-500 bg-zinc-200"
      :style="innerStyle"
    ></span>
    <span
      class="pointer-events-none absolute top-1/2 left-1/2 h-0 w-0 -translate-x-1/2 -translate-y-1/2"
    >
      <span class="absolute rounded bg-zinc-700" :style="needleStyle"></span>
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'

interface Props {
  modelValue: number
  min: number
  max: number
  dragAxis?: 'horizontal' | 'vertical'
  size?: 'small' | 'medium' | 'large' | number
  dragStep?: number
  wheelStep?: number
  ariaLabel?: string
}

const props = withDefaults(defineProps<Props>(), {
  dragAxis: 'horizontal',
  size: 'medium',
  ariaLabel: 'Digital potentiometer',
})

const emit = defineEmits<{
  (event: 'update:modelValue', value: number): void
}>()

const knobAngleMin = -135
const knobAngleMax = 135

const sizeMap: Record<'small' | 'medium' | 'large', number> = {
  small: 28,
  medium: 34,
  large: 42,
}

const knobSizePx = computed(() => {
  if (typeof props.size === 'number') return Math.max(20, props.size)
  return sizeMap[props.size]
})

const innerInsetPx = computed(() => Math.max(3, Math.round(knobSizePx.value * 0.12)))
const needleLengthPx = computed(() => Math.max(8, Math.round(knobSizePx.value * 0.32)))
const defaultDragStep = computed(() => (props.max - props.min) / 300)
const defaultWheelStep = computed(() => (props.max - props.min) / 100)

const knobStyle = computed(() => ({
  width: `${knobSizePx.value}px`,
  height: `${knobSizePx.value}px`,
}))

const innerStyle = computed(() => ({
  inset: `${innerInsetPx.value}px`,
}))

const localValue = ref(props.modelValue)
const lastPointer = ref<{ x: number; y: number } | null>(null)

const clamp = (value: number) => Math.min(props.max, Math.max(props.min, value))

watch(
  () => props.modelValue,
  (value) => {
    localValue.value = clamp(value)
  },
  { immediate: true },
)

const angle = computed(() => {
  if (props.max <= props.min) return knobAngleMin
  const ratio = (localValue.value - props.min) / (props.max - props.min)
  return knobAngleMin + ratio * (knobAngleMax - knobAngleMin)
})

const needleStyle = computed(() => ({
  top: `-${needleLengthPx.value}px`,
  left: '-1px',
  width: '2px',
  height: `${needleLengthPx.value}px`,
  transform: `rotate(${angle.value}deg)`,
  transformOrigin: `center ${needleLengthPx.value}px`,
}))

const emitClamped = (value: number) => {
  const next = clamp(value)
  if (Math.abs(next - localValue.value) < 0.0001) return
  localValue.value = next
  emit('update:modelValue', next)
}

const onMove = (event: PointerEvent) => {
  const previous = lastPointer.value
  if (!previous) {
    lastPointer.value = { x: event.clientX, y: event.clientY }
    return
  }

  const delta =
    props.dragAxis === 'vertical' ? previous.y - event.clientY : event.clientX - previous.x
  lastPointer.value = { x: event.clientX, y: event.clientY }

  const step = typeof props.dragStep === 'number' ? props.dragStep : defaultDragStep.value
  emitClamped(localValue.value + delta * step)
}

const clearDrag = () => {
  lastPointer.value = null
  window.removeEventListener('pointermove', onMove)
  window.removeEventListener('pointerup', clearDrag)
}

const onPointerDown = (event: PointerEvent) => {
  event.preventDefault()
  lastPointer.value = { x: event.clientX, y: event.clientY }
  window.addEventListener('pointermove', onMove)
  window.addEventListener('pointerup', clearDrag)
}

const onWheel = (event: WheelEvent) => {
  event.preventDefault()
  const direction = event.deltaY < 0 ? 1 : -1
  const step = typeof props.wheelStep === 'number' ? props.wheelStep : defaultWheelStep.value
  emitClamped(localValue.value + direction * step)
}

onBeforeUnmount(() => {
  clearDrag()
})
</script>
