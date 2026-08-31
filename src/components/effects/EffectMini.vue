<template>
  <div
    class="relative flex h-full min-w-[220px] w-[220px] cursor-pointer flex-col justify-between gap-3 overflow-hidden rounded-xl border border-zinc-300 bg-zinc-50 px-4 py-3 text-zinc-950 shadow-sm transition-transform duration-150 mt-0.5 hover:-translate-y-0.5"
    @click.stop="toggleEdit"
  >
    <button
      class="effect-drag-handle absolute top-2 right-3 cursor-grab! rounded-full border border-zinc-300 bg-white px-2 py-1 text-[0.7rem] leading-none tracking-[0.2em] text-zinc-500 active:cursor-grabbing"
      type="button"
      aria-label="Drag effect"
      @click.stop
    >
      |||
    </button>

    <div class="grid gap-3 pr-10">
      <div class="flex items-start gap-3">
        <DigitalIndicator
          size="small"
          class="w-6"
          :state="true"
          :color="enabled ? 'green' : 'red'"
          :blue-scale="false"
        />
        <div class="min-w-0">
          <div class="-mt-1 text-sm font-semibold leading-5 text-zinc-950">{{ label }}</div>
        </div>
      </div>

      <!-- <div class="min-h-[2.5rem] text-xs leading-5 text-zinc-600">{{ summary }}</div> -->
    </div>

    <UModal
      v-model:open="editOpen"
      :ui="{
        wrapper: 'z-[80]',
        overlay: 'z-[80]',
        content:
          'z-[81] max-w-[min(1120px,calc(100%-2rem))] rounded-[28px] border border-zinc-300 bg-zinc-100 overflow-scroll',
      }"
    >
      <template #content>
        <div class="p-4 md:p-5" @click.stop>
          <EffectDialog :effect-id="item.id" />
          <div class="mt-4 flex flex-wrap justify-end gap-3">
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
import type { EffectChainItem } from '../../utils/effects'
import { useAudioStore } from '../../stores/audio'
import type { EffectSettings } from '../../typings/effects'
import { getEffectSummary } from '../../utils/effects'
import DigitalIndicator from '../digital/DigitalIndicator.vue'

const EffectDialog = defineAsyncComponent(() => import('./EffectsDialog.vue'))

interface Props {
  item: EffectChainItem
  index: number
}

const audio = useAudioStore()
const props = defineProps<Props>()

const editOpen = ref(false)

const label = computed(() => audio.effectLabel(props.item.effect))
const summary = computed(() => getEffectSummary(props.item.effect))
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
