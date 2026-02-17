<template>
  <div
    class="fixed right-0 bottom-0 left-0 z-20 h-[var(--effect-rack-height)] w-full cursor-pointer border-t-2 border-zinc-500 bg-zinc-200 p-4 transition-[height] duration-300"
  >
    <UDropdownMenu
      id="effect-rack"
      :items="effectMenuItems"
      class="z-20"
      :content="{ align: 'center', side: 'top' }"
      :ui="{ content: 'z-[70]' }"
    >
      <div class="absolute inset-0 flex items-center justify-center w-full h-full">
        <div v-if="noEffects" class="text-center text-xs uppercase text-zinc-500">
          There are no effects, click to add one
        </div>
      </div>
    </UDropdownMenu>
    <div
      v-if="!noEffects"
      class="absolute inset-0 p-2 flex items-center justify-start pointer-events-none z-30"
    >
      <div>
        <Draggable
          class="flex h-full w-full flex-nowrap items-stretch gap-4 overflow-x-auto overflow-y-hidden pb-1 pointer-events-auto"
          :list="effects"
          item-key="id"
          handle=".effect-drag-handle"
          ghost-class="effect-drag-ghost"
          chosen-class="effect-drag-chosen"
          drag-class="effect-drag-active"
          :force-fallback="true"
          :fallback-on-body="true"
          :animation="160"
          @end="onDragEnd"
        >
          <template #item="{ element, index }">
            <div class="effect-rack-item w-max shrink-0 transition-transform duration-150 ease-out">
              <EffectMini class="effect" :item="element" :index="index" />
            </div>
          </template>
        </Draggable>
      </div>
    </div>
  </div>
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

<style scoped>
@reference "../../assets/index.css";

.effect-drag-ghost {
  opacity: 0.4;
}

.effect-drag-chosen {
  opacity: 0.85;
  transform: scale(0.98);
}

.effect-drag-active {
  opacity: 0.9;
  box-shadow: 0 12px 20px rgba(0, 0, 0, 0.35);
  @apply rounded-lg;
}

.effect-rack-item.effect-drag-active {
  transition-property: none;
}
</style>
