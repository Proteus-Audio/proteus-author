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
  color?: 'red' | 'green' | 'amber'
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
}

.light {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #2a2622;
  border: 1px solid #0f0d0b;
  box-shadow: inset 0 2px 3px rgba(0, 0, 0, 0.7);
  transition:
    box-shadow 0.15s ease-out,
    background 0.15s ease-out;
}

.small {
  .light {
    width: 10px;
    height: 10px;
  }
}

.color-amber .light.on {
  background: #f2c94c;
  box-shadow:
    0 0 10px rgba(242, 201, 76, 0.8),
    0 0 20px rgba(242, 201, 76, 0.4);
}

.color-green .light.on {
  background: #7dd36c;
  box-shadow:
    0 0 10px rgba(125, 211, 108, 0.8),
    0 0 20px rgba(125, 211, 108, 0.4);
}

.color-red .light.on {
  background: #e36a6a;
  box-shadow:
    0 0 10px rgba(227, 106, 106, 0.8),
    0 0 20px rgba(227, 106, 106, 0.4);
}
</style>
