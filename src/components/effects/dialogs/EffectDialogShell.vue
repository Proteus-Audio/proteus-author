<template>
  <div class="grid gap-4">
    <header
      class="grid gap-4 rounded-[22px] border border-zinc-300 bg-[linear-gradient(135deg,#f8fafc_0%,#eef2f7_100%)] p-5 shadow-[0_18px_40px_rgba(15,23,42,0.08)]"
    >
      <div class="flex flex-wrap items-start justify-between gap-4">
        <div class="min-w-0 space-y-2">
          <div class="text-[10px] uppercase tracking-[0.22em] text-zinc-500">Effect Editor</div>
          <h2 class="text-[1.3rem] leading-tight font-semibold text-zinc-950">{{ title }}</h2>
          <p class="max-w-3xl text-sm leading-6 text-zinc-600">{{ description }}</p>
        </div>

        <div class="grid gap-2 justify-items-end">
          <div class="flex items-center gap-3">
            <DigitalIndicator
              size="small"
              :state="enabled"
              :color="enabled ? 'green' : 'red'"
              :blue-scale="false"
              :label="enabled ? 'Enabled' : 'Bypassed'"
            />

            <button
              type="button"
              class="rounded-full border px-3 py-1.5 text-[11px] font-medium uppercase tracking-[0.16em] transition-colors"
              :class="
                enabled
                  ? 'border-cyan-700 bg-cyan-950 text-cyan-100 hover:bg-cyan-900'
                  : 'border-zinc-300 bg-zinc-100 text-zinc-700 hover:bg-zinc-200'
              "
              @click="$emit('update:enabled', !enabled)"
            >
              {{ enabled ? 'Disable' : 'Enable' }}
            </button>
          </div>

          <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
            {{ liveStatus }}
          </div>
        </div>
      </div>
    </header>

    <div class="grid gap-4 xl:grid-cols-[minmax(0,1.3fr)_minmax(290px,0.85fr)]">
      <section
        class="rounded-[22px] border border-zinc-300 bg-white p-4 shadow-[0_12px_32px_rgba(15,23,42,0.06)]"
      >
        <slot name="visual" />
      </section>

      <aside
        class="rounded-[22px] border border-zinc-300 bg-zinc-50 p-4 shadow-[0_12px_32px_rgba(15,23,42,0.05)]"
      >
        <slot name="controls" />
      </aside>
    </div>

    <section class="rounded-[22px] border border-zinc-300 bg-zinc-950 p-4 text-zinc-100">
      <slot name="meters" />
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import DigitalIndicator from '../../digital/DigitalIndicator.vue'

interface Props {
  title: string
  description: string
  enabled: boolean
  isPlaying: boolean
  meteringEnabled: boolean
  spectralEnabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  spectralEnabled: false,
})

defineEmits<{
  (event: 'update:enabled', value: boolean): void
}>()

const liveStatus = computed(() => {
  if (!props.meteringEnabled) return 'Live meters offline'
  if (!props.isPlaying) return 'Awaiting playback'
  if (props.spectralEnabled) return 'Live metering + spectral active'
  return 'Live metering active'
})
</script>
