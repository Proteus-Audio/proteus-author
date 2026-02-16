<template>
  <div class="grid justify-items-stretch gap-1.5">
    <span
      class="block w-full border-4 border-sky-100 bg-slate-100 transition-colors duration-150"
      :class="[sizeClass, state ? activeColorClass : '', frozen && state ? 'bg-slate-400!' : '']"
    ></span>
    <span v-if="label" class="text-center text-[10px] leading-none text-zinc-500">{{ label }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  state: boolean
  label?: string
  size?: 'small' | 'medium' | 'large'
  color?: 'red' | 'green' | 'lime' | 'amber' | 'yellow' | 'orange' | 'dark-red'
  blueScale?: boolean
  frozen?: boolean
  peak?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
  color: 'amber',
  size: 'medium',
  blueScale: true,
})

const sizeClass = computed(() => {
  if (props.size === 'small') return 'h-3'
  if (props.size === 'large') return 'h-8'
  return 'h-5'
})

const activeColorClass = computed(() => {
  if (props.frozen) return 'bg-slate-400'

  const blueScaleMap: Record<NonNullable<Props['color']>, string> = {
    green: 'bg-sky-300',
    lime: 'bg-sky-400',
    yellow: 'bg-sky-500',
    amber: 'bg-sky-600',
    orange: 'bg-sky-700',
    red: 'bg-sky-800',
    'dark-red': 'bg-red-800',
  }

  const standardMap: Record<NonNullable<Props['color']>, string> = {
    green: 'bg-green-600',
    lime: 'bg-lime-500',
    yellow: 'bg-yellow-400',
    amber: 'bg-amber-400',
    orange: 'bg-orange-500',
    red: 'bg-red-600',
    'dark-red': 'bg-red-800',
  }

  return props.blueScale ? blueScaleMap[props.color] : standardMap[props.color]
})
</script>
