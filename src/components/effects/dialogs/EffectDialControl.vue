<template>
  <div class="grid gap-3 rounded-[18px] border border-zinc-300 bg-white p-3">
    <div class="flex items-start justify-between gap-3">
      <div class="min-w-0">
        <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">{{ label }}</div>
        <div v-if="helper" class="mt-1 text-xs leading-5 text-zinc-500">{{ helper }}</div>
      </div>

      <div class="shrink-0 text-right text-sm font-medium text-zinc-950">{{ readout }}</div>
    </div>

    <div class="grid place-items-center gap-3">
      <DigitalPot
        :model-value="dialValue"
        :min="0"
        :max="1000"
        :reset-value="
          typeof props.resetValue === 'number' ? toDialRatio(props.resetValue) * 1000 : undefined
        "
        :drag-step="4"
        :wheel-step="8"
        size="large"
        :aria-label="label"
        @update:model-value="updateFromDial"
      />

      <input
        class="w-full accent-zinc-950"
        type="range"
        min="0"
        max="1000"
        step="1"
        :value="dialValue"
        :disabled="disabled"
        @input="updateFromDial(Number(($event.target as HTMLInputElement).value))"
      />

      <div class="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-2">
        <input
          class="w-full rounded-xl border border-zinc-300 bg-zinc-50 px-3 py-2 text-sm text-zinc-950 outline-none transition focus:border-zinc-500"
          type="number"
          :min="min"
          :max="max"
          :step="step"
          :disabled="disabled"
          :value="numberInputValue"
          @change="updateFromInput(($event.target as HTMLInputElement).value)"
        />
        <span v-if="units" class="text-xs uppercase tracking-[0.16em] text-zinc-500">{{
          units
        }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import DigitalPot from '../../digital/DigitalPot.vue'

interface Props {
  label: string
  modelValue: number
  min: number
  max: number
  step?: number
  units?: string
  helper?: string
  disabled?: boolean
  precision?: number
  resetValue?: number
  scale?: 'linear' | 'log'
}

const props = withDefaults(defineProps<Props>(), {
  step: 1,
  units: '',
  helper: '',
  disabled: false,
  scale: 'linear',
})

const emit = defineEmits<{
  (event: 'update:modelValue', value: number): void
}>()

const clamp = (value: number) => Math.min(props.max, Math.max(props.min, value))

const decimals = computed(() => {
  if (typeof props.precision === 'number') return props.precision
  const normalized = `${props.step}`
  return normalized.includes('.') ? normalized.split('.')[1].length : 0
})

const toDialRatio = (value: number) => {
  if (props.scale === 'log' && props.min > 0 && props.max > 0) {
    const minLog = Math.log10(props.min)
    const maxLog = Math.log10(props.max)
    return (Math.log10(clamp(value)) - minLog) / Math.max(maxLog - minLog, 1e-6)
  }

  return (clamp(value) - props.min) / Math.max(props.max - props.min, 1e-6)
}

const fromDialRatio = (ratio: number) => {
  const safeRatio = Math.min(1, Math.max(0, ratio))
  if (props.scale === 'log' && props.min > 0 && props.max > 0) {
    const minLog = Math.log10(props.min)
    const maxLog = Math.log10(props.max)
    return Math.pow(10, minLog + safeRatio * (maxLog - minLog))
  }

  return props.min + safeRatio * (props.max - props.min)
}

const quantize = (value: number) => {
  const stepped = Math.round(value / props.step) * props.step
  return Number(clamp(stepped).toFixed(decimals.value))
}

const dialValue = computed(() => Math.round(toDialRatio(props.modelValue) * 1000))
const numberInputValue = computed(() => Number(clamp(props.modelValue).toFixed(decimals.value)))
const readout = computed(() => {
  const value = Number(clamp(props.modelValue).toFixed(decimals.value))
  return props.units ? `${value} ${props.units}` : `${value}`
})

const updateFromDial = (rawValue: number) => {
  if (props.disabled) return
  emit('update:modelValue', quantize(fromDialRatio(rawValue / 1000)))
}

const updateFromInput = (rawValue: string) => {
  if (props.disabled) return
  const parsed = Number(rawValue)
  if (!Number.isFinite(parsed)) return
  emit('update:modelValue', quantize(parsed))
}
</script>
