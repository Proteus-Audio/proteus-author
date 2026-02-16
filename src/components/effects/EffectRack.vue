<template>
  <UDropdownMenu
    id="effect-rack"
    :items="effectMenuItems"
    class="z-20"
    :content="{ align: 'center', side: 'top' }"
    :ui="{ content: 'z-[70]' }"
  >
    <div
      class="fixed right-0 bottom-0 left-0 z-20 h-[var(--effect-rack-height)] w-full cursor-pointer border-t-2 border-zinc-500 bg-zinc-200 p-4 transition-[height] duration-300"
      :class="noEffects ? 'grid place-items-center' : 'block'"
    >
      <div v-if="noEffects" class="text-center text-xs uppercase text-zinc-500">
        There are no effects, click to add one
      </div>

      <Draggable
        v-else
        class="flex h-full w-full flex-nowrap items-stretch gap-4 overflow-x-auto overflow-y-hidden pb-1"
        :list="effects"
        item-key="id"
        handle=".effect-drag-handle"
        ghost-class="opacity-40"
        chosen-class="opacity-85 scale-[0.98]"
        drag-class="opacity-90 shadow-[0_12px_20px_rgba(0,0,0,0.35)]"
        :force-fallback="true"
        :fallback-on-body="true"
        :animation="160"
        @end="onDragEnd"
      >
        <template #item="{ element, index }">
          <div class="w-max shrink-0 transition-transform duration-150 ease-out">
            <EffectMini class="effect" :item="element" :index="index" />
          </div>
        </template>
      </Draggable>
    </div>
  </UDropdownMenu>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Draggable from 'vuedraggable'
import { effectTypeLabels, effectTypes } from '../../assets/effects'
import { useAudioStore } from '../../stores/audio'
import type { AudioEffectType } from '../../typings/effects'
import EffectMini from './EffectMini.vue'

const audio = useAudioStore()

const effects = computed(() => audio.effects)
const noEffects = computed(() => effects.value.length <= 0)

const addEffect = (toAdd: AudioEffectType) => {
  audio.addEffect(toAdd)
}

const effectMenuItems = computed(() => {
  return [
    effectTypes.map((type) => ({
      label: effectTypeLabels[type],
      class: 'cursor-pointer text-xs uppercase text-zinc-500 hover:bg-zinc-200',
      onSelect: () => addEffect(type),
    })),
  ]
})

const onDragEnd = () => {
  audio.scheduleSyncEffects()
}
</script>
