<template>
  <div
    class="absolute z-20 flex flex-col items-center justify-center gap-2 bg-zinc-500/60"
    :class="inset === -4 ? '-inset-4' : 'inset-0'"
  >
    <div class="grid grid-cols-3 gap-1">
      <span
        v-for="cell in cells"
        :key="cell.id"
        class="size-2 bg-white"
        :class="cell.hidden ? 'invisible' : 'animate-pixel-fade'"
        :style="cell.hidden ? undefined : { animationDelay: `${cell.delay}s` }"
      ></span>
    </div>
    <div v-if="message" class="text-center text-xs text-white">{{ message }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  message?: string
  inset?: 0 | -4
}

withDefaults(defineProps<Props>(), {
  inset: 0,
})

const cells = computed(() => {
  return Array.from({ length: 9 }, (_, i) => ({
    id: i,
    hidden: i === 4,
    delay: -(i * 0.1),
  }))
})
</script>
