<template>
  <div class="analog-knob" :style="knobStyle">
    <label v-if="label" class="analog-label">{{ label }}</label>
    <div class="knob-wrap">
      <div class="knob">
        <div class="knob-notch"></div>
      </div>
      <input
        class="knob-input"
        type="range"
        :min="min"
        :max="max"
        :step="step"
        :value="modelValue"
        @input="onKnobInput"
      />
    </div>
    <div v-if="showValue" class="knob-readout">
      <input
        v-if="allowNumericInput"
        class="value-input"
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
      <span v-else class="value">{{ displayValue }}</span>
      <span v-if="units" class="units">{{ units }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'

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
})

const emit = defineEmits(['update:modelValue'])

const isEditing = ref(false)
const inputValue = ref('')

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

const onKnobInput = (event: Event) => {
  const next = Number((event.target as HTMLInputElement).value)
  emit('update:modelValue', normalizeValue(next))
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

const knobStyle = computed(() => ({
  '--knob-size': `${props.size}px`,
  '--knob-rotation': `${rotation.value}deg`,
}))

watch(
  () => props.modelValue,
  (value) => {
    if (!isEditing.value) inputValue.value = formatValue(value)
  },
  { immediate: true },
)

const displayValue = computed(() => formatValue(props.modelValue))
</script>

<style lang="scss" scoped>
.analog-knob {
  display: grid;
  gap: 0.4rem;
  justify-items: center;
}

.knob-wrap {
  position: relative;
  width: var(--knob-size);
  height: var(--knob-size);
}

.knob {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background:
    radial-gradient(circle at 30% 30%, rgba(255, 255, 255, 0.35), transparent 55%),
    radial-gradient(circle at 70% 70%, rgba(0, 0, 0, 0.5), transparent 50%),
    linear-gradient(145deg, #3a3530, #1e1b18);
  border: 2px solid #151311;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.08),
    inset 0 -6px 10px rgba(0, 0, 0, 0.6),
    0 6px 14px rgba(0, 0, 0, 0.5);
  position: relative;
  transform: rotate(var(--knob-rotation));
  transition: transform 0.08s ease-out;
}

.knob-notch {
  position: absolute;
  top: 10px;
  left: 50%;
  width: 4px;
  height: 18px;
  border-radius: 2px;
  transform: translateX(-50%);
  background: linear-gradient(180deg, var(--analog-accent), var(--analog-accent-deep));
  box-shadow: 0 0 4px var(--analog-glow);
}

.knob-input {
  position: absolute;
  inset: 0;
  opacity: 0;
  cursor: pointer;
}

.knob-readout {
  display: flex;
  align-items: baseline;
  gap: 0.35rem;
  font-size: 0.7rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--analog-muted);
}

.knob-readout .value {
  color: var(--analog-text);
  font-variant-numeric: tabular-nums;
}

.value-input {
  appearance: textfield;
  width: 4.2rem;
  border: 1px solid #3f372d;
  border-radius: 4px;
  background: #1f1b18;
  color: var(--analog-text);
  font: inherit;
  font-size: 0.75rem;
  letter-spacing: 0.04em;
  text-align: right;
  padding: 0.2rem 0.35rem;
}

.value-input::-webkit-outer-spin-button,
.value-input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.value-input:focus {
  outline: 1px solid var(--analog-accent);
  border-color: var(--analog-accent-deep);
}

.knob-readout .units {
  opacity: 0.7;
}
</style>
