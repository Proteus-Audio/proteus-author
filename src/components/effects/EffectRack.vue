<template>
  <div
    class="fixed right-0 bottom-0 left-0 z-20 h-[var(--effect-rack-height)] w-full border-t border-zinc-300 bg-[linear-gradient(180deg,#f8fafc_0%,#eef2f7_100%)] px-4 py-3 transition-[height] duration-300"
  >
    <div class="grid h-full grid-rows-[auto_minmax(0,1fr)] gap-3">
      <div class="flex items-center justify-between gap-3">
        <div>
          <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Master Chain</div>
          <div class="mt-1 text-sm font-semibold text-zinc-950">Effect Rack</div>
        </div>

        <UDropdownMenu
          id="effect-rack"
          :items="effectMenuItems"
          class="z-20"
          :content="{ align: 'end', side: 'top' }"
          :ui="{ content: 'z-[70]' }"
        >
          <UButton icon="i-lucide-plus" color="neutral" variant="outline" size="sm">
            Add Effect
          </UButton>
        </UDropdownMenu>
      </div>

      <div
        v-if="noEffects"
        class="flex h-full items-center justify-center rounded-[18px] border border-dashed border-zinc-300 bg-white/70 text-center text-xs uppercase tracking-[0.18em] text-zinc-500"
      >
        There are no effects yet. Add one to start shaping the master chain.
      </div>

      <div v-else class="pointer-events-none z-30 min-h-0">
        <Draggable
          class="pointer-events-auto flex h-full w-full flex-nowrap items-stretch gap-4 overflow-x-auto overflow-y-hidden pb-1"
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
import { effectTypeLabels, effectTypes } from '../../utils/effects'
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
      class: 'cursor-pointer text-xs uppercase tracking-[0.16em] text-zinc-600 hover:bg-zinc-100',
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
  box-shadow: 0 18px 32px rgba(15, 23, 42, 0.18);
  @apply rounded-[18px];
}

.effect-rack-item.effect-drag-active {
  transition-property: none;
}
</style>
