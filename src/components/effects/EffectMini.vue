<template>
  <div
    class="relative flex h-full min-w-max w-max cursor-grab flex-col items-center justify-center gap-2 overflow-hidden rounded-lg bg-zinc-700 px-8 pr-10 py-3 text-white transition-[height,margin] duration-300 hover:opacity-85"
    @click.stop="toggleEdit"
  >
    <button
      class="effect-drag-handle absolute top-[0.65rem] right-[0.65rem] cursor-grab border-0 bg-transparent px-1 py-0.5 text-[0.8rem] leading-none tracking-[1px] text-white/75 active:cursor-grabbing"
      type="button"
      aria-label="Drag effect"
      @click.stop
    >
      |||
    </button>

    <div class="absolute top-3 left-3">
      <AnalogIndicator size="small" :state="true" :color="enabled ? 'green' : 'red'" />
    </div>

    <div class="grid items-center whitespace-nowrap text-center font-semibold">{{ label }}</div>

    <UModal v-model:open="editOpen" :ui="{ content: 'max-w-[calc(100%-4em)]' }">
      <template #content>
        <div class="p-4" @click.stop>
          <EffectDialog :effectIndex="index" />
          <div class="mt-4 grid grid-cols-[auto_auto] justify-end gap-3">
            <UButton icon="i-lucide-x" variant="outline" color="neutral" @click="toggleEdit"
              >Close</UButton
            >
            <UButton icon="i-lucide-trash-2" variant="outline" color="error" @click="removeEffect">
              Remove Effect
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent, ref } from 'vue'
import type { EffectChainItem } from '../../assets/effects'
import { useAudioStore } from '../../stores/audio'
import type { EffectSettings } from '../../typings/effects'
import { AnalogIndicator } from '../analog'

const EffectDialog = defineAsyncComponent(() => import('./EffectsDialog.vue'))

interface Props {
  item: EffectChainItem
  index: number
}

const audio = useAudioStore()
const props = defineProps<Props>()

const editOpen = ref(false)

const label = computed(() => audio.effectLabel(props.item.effect))
const toggleEdit = () => {
  editOpen.value = !editOpen.value
}

const effect = computed((): EffectSettings | undefined => {
  return Object.values(props.item.effect)[0]
})

const enabled = computed({
  get() {
    return effect.value?.enabled ?? false
  },
  set(value) {
    if (effect.value) {
      effect.value.enabled = value
    }
  },
})

const removeEffect = () => {
  audio.removeEffect(props.item.id)
}
</script>
