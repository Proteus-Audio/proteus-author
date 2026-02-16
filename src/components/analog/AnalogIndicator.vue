<template>
  <div class="grid justify-items-center gap-1.5">
    <span
      class="block rounded-full border border-[#0f0d0b] bg-[#2a2622] shadow-[inset_0_2px_3px_rgba(0,0,0,0.7)] transition-all duration-150"
      :class="[sizeClass, state ? activeClass : '']"
    ></span>
    <span v-if="label" class="analog-label">{{ label }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  state: boolean
  label?: string
  size?: 'small' | 'medium' | 'large'
  color?: 'red' | 'green' | 'amber'
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
  color: 'amber',
  size: 'medium',
})

const sizeClass = computed(() => {
  if (props.size === 'small') return 'size-[10px]'
  if (props.size === 'large') return 'size-5'
  return 'size-4'
})

const activeClass = computed(() => {
  if (props.color === 'green') {
    return 'bg-[#7dd36c] shadow-[0_0_10px_rgba(125,211,108,0.8),0_0_20px_rgba(125,211,108,0.4)]'
  }

  if (props.color === 'red') {
    return 'bg-[#e36a6a] shadow-[0_0_10px_rgba(227,106,106,0.8),0_0_20px_rgba(227,106,106,0.4)]'
  }

  return 'bg-[#f2c94c] shadow-[0_0_10px_rgba(242,201,76,0.8),0_0_20px_rgba(242,201,76,0.4)]'
})
</script>
