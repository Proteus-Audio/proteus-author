<template>
  <div class="analog-indicator" :class="colorClass">
    <span class="light" :class="{ on: state }"></span>
    <span v-if="label" class="analog-label">{{ label }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  state: boolean
  label?: string
  size?: 'small' | 'medium' | 'large'
  color?: 'red' | 'green' | 'lime' | 'amber' | 'yellow' | 'orange' | 'dark-red'
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
  color: 'amber',
  size: 'medium',
})

const colorClass = computed(() => `color-${props.color} ${props.size}`)
</script>

<style lang="scss" scoped>
.analog-indicator {
  display: grid;
  gap: 0.35rem;
  justify-items: center;

  .light {
    width: 100%;
    height: 1rem;
    background: #2a2622;

    border: 4px solid #0f0d0b;
    transition:
      box-shadow 0.15s ease-out,
      background 0.15s ease-out;
  }

  &.small .light {
    width: 10px;
    height: 10px;
  }

  &.color-green .light.on {
    background: oklch(72.3% 0.219 149.579);
  }

  &.color-lime .light.on {
    background: oklch(76.8% 0.233 130.85);
  }

  &.color-yellow .light.on {
    background: oklch(79.5% 0.184 86.047);
  }

  &.color-amber .light.on {
    background: oklch(76.9% 0.188 70.08);
  }

  &.color-orange .light.on {
    background: oklch(64.6% 0.222 41.116);
  }

  &.color-red .light.on {
    background: oklch(57.7% 0.245 27.325);
  }

  &.color-dark-red .light.on {
    background: oklch(44.4% 0.177 26.899);
  }
}
</style>
