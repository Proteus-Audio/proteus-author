<template>
  <EffectDialog :effectIndex="effectIndex" />
</template>

<script setup lang="ts">
import EffectDialog from './EffectDialog.vue'

interface Props {
  effectIndex: number
}

defineProps<Props>()
</script>
