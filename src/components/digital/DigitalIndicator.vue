<template>
  <div
    class="digital-indicator"
    :class="[
      sizeClass,
      `color-${props.color}`,
      { on: state, frozen: frozen && state, 'blue-scale': blueScale },
    ]"
  >
    <span class="light"></span>
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
  if (props.size === 'small') return 'small'
  if (props.size === 'large') return 'large'
  return 'medium'
})
</script>

<style scoped>
@reference "../../assets/index.css";

.digital-indicator {
  @apply grid justify-items-stretch gap-1.5;

  .light {
    @apply block w-full border-4 border-gray-200 bg-slate-100/50 transition-colors duration-150;
  }

  &.small .light {
    @apply h-3;
  }

  &.medium .light {
    @apply h-5;
  }

  &.large .light {
    @apply h-8;
  }

  &.on {
    &.color-green .light {
      @apply bg-green-600;
    }

    &.color-lime .light {
      @apply bg-lime-500;
    }

    &.color-yellow .light {
      @apply bg-yellow-400;
    }

    &.color-amber .light {
      @apply bg-amber-400;
    }

    &.color-orange .light {
      @apply bg-orange-500;
    }

    &.color-red .light {
      @apply bg-red-600;
    }

    &.color-dark-red .light {
      @apply bg-red-800;
    }
  }

  &.blue-scale.on {
    &.color-green .light {
      @apply bg-cyan-500;
    }

    &.color-lime .light {
      @apply bg-cyan-600;
    }

    &.color-yellow .light {
      @apply bg-cyan-700;
    }

    &.color-amber .light {
      @apply bg-cyan-800;
    }

    &.color-orange .light {
      @apply bg-cyan-900;
    }

    &.color-red .light {
      @apply bg-cyan-950;
    }

    &.color-dark-red .light {
      @apply bg-red-800;
    }
    &.frozen.on .light {
      @apply bg-slate-400;
    }
  }

  &.frozen.on .light {
    @apply bg-slate-400;
  }
}
</style>
