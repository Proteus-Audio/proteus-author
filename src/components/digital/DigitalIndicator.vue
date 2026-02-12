<template>
  <div
    class="digital-indicator"
    :class="{ [colorClass]: true, 'blue-scale': blueScale, frozen, peak }"
  >
    <span class="light" :class="{ on: state }"></span>
    <span v-if="label" class="digital-label">{{ label }}</span>
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

const colorClass = computed(() => `color-${props.color} ${props.size}`)
</script>

<style lang="scss" scoped>
.digital-indicator {
  display: grid;
  gap: 0.35rem;
  justify-items: stretch;

  .light {
    width: 100%;
    height: 2rem;
    background: oklch(92.9% 0.013 255.508);

    border: 4px solid oklch(86.9% 0.022 252.894);
    transition:
      box-shadow 0.15s ease-out,
      background 0.15s ease-out;
  }

  &.small .light {
    height: 0.75rem;
  }

  &.medium .light {
    height: 1.25rem;
  }

  &.large .light {
    height: 2rem;
  }

  &.color-green .light.on {
    background: oklch(52.7% 0.154 150.069);
  }

  &.color-lime .light.on {
    background: oklch(64.8% 0.2 131.684);
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

  &.frozen .light.on {
    background: oklch(70.4% 0.04 256.788) !important;
  }

  &.blue-scale {
    &.color-green .light.on {
      background: oklch(82.8% 0.111 230.318);
    }

    &.color-lime .light.on {
      background: oklch(74.6% 0.16 232.661);
    }

    &.color-yellow .light.on {
      background: oklch(68.5% 0.169 237.323);
    }

    &.color-amber .light.on {
      background: oklch(58.8% 0.158 241.966);
    }

    &.color-orange .light.on {
      background: oklch(50% 0.134 242.749);
    }

    &.color-red .light.on {
      background: oklch(44.3% 0.11 240.79);
    }

    &.color-dark-red .light.on {
      background: oklch(44.4% 0.177 26.899);
    }
  }
}
</style>
