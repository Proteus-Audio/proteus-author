<template>
  <div
    class="grid h-full w-[84px] grid-rows-[1fr_1fr] gap-2 border-2 border-zinc-400/50 bg-zinc-100/50 p-1.5"
  >
    <div class="flex flex-col items-center">
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">LVL</div>
      <DigitalPot
        v-model="levelDb"
        :min="minDb"
        :max="maxDb"
        :drag-axis="dragAxis"
        :drag-step="0.12"
        :wheel-step="0.5"
        :reset-value="0"
        size="medium"
        aria-label="Track level"
      />
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">{{ levelReadout }}</div>
    </div>

    <div class="flex flex-col items-center">
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">PAN</div>
      <DigitalPot
        v-model="panModel"
        :min="-1"
        :max="1"
        :drag-axis="dragAxis"
        :drag-step="0.004"
        :wheel-step="0.02"
        :reset-value="0"
        size="medium"
        aria-label="Track pan"
      />
      <div class="text-[10px] tracking-[0.04em] text-zinc-600">{{ panReadout }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import DigitalPot from './DigitalPot.vue'

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

const minDb = -10
const maxDb = 10

const clampDb = (db: number) => Math.min(maxDb, Math.max(minDb, db))
const clampPan = (pan: number) => Math.min(1, Math.max(-1, pan))

const dbToLinear = (db: number) => 10 ** (db / 20)
const linearToDb = (linear: number): number => {
  const safe = Math.max(linear, 0.0001)
  return clampDb(20 * Math.log10(safe))
}

const levelDb = computed({
  get: () => linearToDb(props.level),
  set: (value: number) => {
    emit('update:level', dbToLinear(clampDb(value)))
  },
})

const panModel = computed({
  get: () => clampPan(props.pan),
  set: (value: number) => {
    emit('update:pan', clampPan(value))
  },
})

const levelReadout = computed(
  () => `${levelDb.value >= 0 ? '+' : ''}${levelDb.value.toFixed(1)} dB`,
)

const panReadout = computed(() => {
  if (Math.abs(panModel.value) < 0.02) return 'C'
  const side = panModel.value < 0 ? 'L' : 'R'
  return `${side}${Math.round(Math.abs(panModel.value) * 100)}`
})
</script>
