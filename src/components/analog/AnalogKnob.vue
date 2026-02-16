<template>
  <div class="grid justify-items-center gap-1.5" :style="{ '--knob-size': `${props.size}px` }">
    <label v-if="label" class="analog-label">{{ label }}</label>

    <div
      class="relative h-[var(--knob-size)] w-[var(--knob-size)] rounded-[var(--knob-size)] border-2 border-[#151311] bg-[radial-gradient(circle_at_30%_30%,rgba(255,255,255,0.15),transparent_55%),radial-gradient(circle_at_70%_70%,rgba(0,0,0,0.5),transparent_50%),linear-gradient(145deg,#3a3530,#1e1b18)] shadow-[inset_0_2px_4px_rgba(255,255,255,0.08),inset_0_-6px_10px_rgba(0,0,0,0.6),0_6px_14px_rgba(0,0,0,0.5)]"
    >
      <button
        type="button"
        class="relative size-full cursor-pointer touch-none"
        :style="{ transform: `rotate(${rotation}deg)` }"
        @pointerdown="onKnobPointerDown"
      >
        <span
          class="absolute top-[10px] left-1/2 h-[18px] w-1 -translate-x-1/2 rounded-[2px] bg-[linear-gradient(180deg,var(--color-analog-accent),var(--color-analog-accent-deep))] shadow-[0_0_4px_var(--color-analog-glow)]"
        ></span>
      </button>
    </div>

    <div
      v-if="showValue"
      class="flex items-baseline gap-1.5 text-[0.7rem] uppercase tracking-[0.08em] text-[var(--color-analog-muted)]"
    >
      <input
        v-if="allowNumericInput"
        class="w-[4.2rem] appearance-none rounded border border-[#3f372d] bg-[#1f1b18] px-1.5 py-0.5 text-right text-[0.75rem] tracking-[0.04em] text-[var(--color-analog-text)] outline-[1px] outline-transparent focus:border-[var(--color-analog-accent-deep)] focus:outline-[var(--color-analog-accent)]"
        type="number"
        :min="min"
        :max="max"
        :step="step"
        :value="inputValue"
        @focus="onValueFocus"
        @input="onValueInput"
        @blur="commitTypedValue"
        @keydown.enter.prevent="commitTypedValue"
      />
      <span v-else class="text-[var(--color-analog-text)]">{{ displayValue }}</span>
      <span v-if="units" class="opacity-70">{{ units }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'

interface Props {
  modelValue: number
  min?: number
  max?: number
  step?: number
  label?: string
  size?: number
  units?: string
  showValue?: boolean
  allowNumericInput?: boolean
  dragPixelsPerStep?: number
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 100,
  step: 1,
  label: '',
  size: 82,
  units: '',
  showValue: true,
  allowNumericInput: true,
  dragPixelsPerStep: 2,
})

const emit = defineEmits(['update:modelValue'])

const isEditing = ref(false)
const inputValue = ref('')
const dragStartY = ref(0)
const dragStartValue = ref(0)
const isDragging = ref(false)

const stepPrecision = computed(() => {
  const stepAsText = String(props.step)
  const decimalIndex = stepAsText.indexOf('.')
  if (decimalIndex === -1) return 0
  return stepAsText.length - decimalIndex - 1
})

const normalizeValue = (value: number) => {
  const clamped = Math.min(props.max, Math.max(props.min, value))
  if (props.step <= 0) return clamped
  const stepped = Math.round((clamped - props.min) / props.step) * props.step + props.min
  return Number(stepped.toFixed(stepPrecision.value))
}

const formatValue = (value: number) => {
  if (stepPrecision.value === 0) return String(Math.round(value))
  return Number(value).toFixed(stepPrecision.value)
}

const onPointerMove = (event: PointerEvent) => {
  if (!isDragging.value) return
  event.preventDefault()
  const deltaY = dragStartY.value - event.clientY
  const pixelsPerStep = Math.max(1, props.dragPixelsPerStep)
  const stepDelta = deltaY / pixelsPerStep
  const next = dragStartValue.value + stepDelta * props.step
  emit('update:modelValue', normalizeValue(next))
}

const stopDragging = () => {
  isDragging.value = false
  window.removeEventListener('pointermove', onPointerMove)
  window.removeEventListener('pointerup', stopDragging)
  window.removeEventListener('pointercancel', stopDragging)
}

const onKnobPointerDown = (event: PointerEvent) => {
  event.preventDefault()
  dragStartY.value = event.clientY
  dragStartValue.value = props.modelValue
  isDragging.value = true
  window.addEventListener('pointermove', onPointerMove)
  window.addEventListener('pointerup', stopDragging)
  window.addEventListener('pointercancel', stopDragging)
}

const onValueFocus = () => {
  isEditing.value = true
}

const onValueInput = (event: Event) => {
  inputValue.value = (event.target as HTMLInputElement).value
}

const commitTypedValue = () => {
  isEditing.value = false
  const parsed = Number(inputValue.value)
  if (Number.isNaN(parsed)) {
    inputValue.value = formatValue(props.modelValue)
    return
  }

  const normalized = normalizeValue(parsed)
  emit('update:modelValue', normalized)
  inputValue.value = formatValue(normalized)
}

const rotation = computed(() => {
  const range = props.max - props.min
  if (range === 0) return -135
  const ratio = (props.modelValue - props.min) / range
  return -135 + ratio * 270
})

watch(
  () => props.modelValue,
  (value) => {
    if (!isEditing.value) inputValue.value = formatValue(value)
  },
  { immediate: true },
)

watch(
  () => props.dragPixelsPerStep,
  () => {
    if (props.dragPixelsPerStep <= 0) stopDragging()
  },
)

onBeforeUnmount(() => {
  stopDragging()
})

const displayValue = computed(() => formatValue(props.modelValue))
</script>
