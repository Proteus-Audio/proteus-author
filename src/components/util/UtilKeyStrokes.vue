<template>
  <div>KEY STROKES</div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from 'vue'
import { useAppShortcuts } from '../../composables/useAppShortcuts'

const { registerShortcuts, unregisterShortcuts } = useAppShortcuts()

onMounted(() => {
  registerShortcuts()
})

onBeforeUnmount(() => {
  unregisterShortcuts()
})
</script>
