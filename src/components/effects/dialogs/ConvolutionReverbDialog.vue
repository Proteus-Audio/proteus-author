<template>
  <EffectDialogShell
    title="Convolution Reverb"
    description="Impulse-response ambience with file-backed loading and dry/wet blending."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <div class="grid gap-3 md:grid-cols-[minmax(0,1fr)_220px]">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-950 p-4 text-zinc-100">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-400">
              Impulse Response
            </div>
            <div class="mt-3 text-lg font-semibold">
              {{ impulseResponseLoaded ? 'Loaded' : 'Waiting For IR' }}
            </div>
            <div class="mt-2 break-all text-sm leading-6 text-zinc-300">
              {{ impulseResponseLabel }}
            </div>
            <div class="mt-4 h-2 rounded-full bg-white/10">
              <div
                class="h-full rounded-full bg-cyan-500"
                :style="{ width: `${Math.round(settings.dry_wet * 100)}%` }"
              ></div>
            </div>
            <div class="mt-2 text-xs uppercase tracking-[0.16em] text-zinc-400">
              {{ Math.round(settings.dry_wet * 100) }}% wet
            </div>
          </div>

          <div class="grid gap-3 rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div>
              <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Tail Floor</div>
              <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ tailDbReadout }}</div>
              <div class="mt-1 text-sm text-zinc-600">Read-only from the decoded IR metadata.</div>
            </div>

            <div
              class="rounded-[16px] border border-zinc-200 bg-white p-3 text-sm leading-6 text-zinc-600"
            >
              Clearer IR content shortens the visual tail. Longer or noisier responses keep the wet
              field denser even at the same mix amount.
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.dry_wet"
          label="Dry / Wet"
          :min="0"
          :max="1"
          :step="0.01"
          :precision="2"
        />

        <div class="grid gap-2 rounded-[18px] border border-zinc-300 bg-white p-3">
          <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
            Impulse Response Source
          </div>
          <UInput
            v-model="impulseResponseValue"
            color="neutral"
            variant="outline"
            placeholder="attachment:ir.wav or /path/to/ir.wav"
          />
          <div class="grid grid-cols-2 gap-2">
            <UButton color="neutral" variant="outline" @click="() => void pickImpulseResponse()">
              Choose File
            </UButton>
            <UButton color="neutral" variant="ghost" @click="clearImpulseResponse">Clear</UButton>
          </div>
        </div>
      </div>
    </template>

    <template #meters>
      <EffectLiveMeters
        :snapshot="liveLevels"
        :enabled="meteringEnabled"
        :is-playing="isPlaying"
        :bypassed="!settings.enabled"
      />
    </template>
  </EffectDialogShell>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ConvolutionReverbSettings, EffectLevelSnapshot } from '../../../typings/effects'
import { formatDbValue } from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'

interface Props {
  settings: ConvolutionReverbSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const impulseResponseValue = computed({
  get: () =>
    props.settings.impulse_response_path ||
    props.settings.impulse_response_attachment ||
    props.settings.impulse_response ||
    '',
  set: (value: string) => {
    const next = value || null
    props.settings.impulse_response = next
    props.settings.impulse_response_attachment = null
    props.settings.impulse_response_path = null
  },
})

const impulseResponseLoaded = computed(() => Boolean(impulseResponseValue.value))
const impulseResponseLabel = computed(() => impulseResponseValue.value || 'No file selected')
const tailDbReadout = computed(() =>
  props.settings.impulse_response_tail_db == null
    ? 'Unknown'
    : formatDbValue(props.settings.impulse_response_tail_db, 1),
)

const pickImpulseResponse = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const result = await open({
    multiple: false,
    filters: [
      {
        name: 'Impulse Response',
        extensions: ['wav', 'aif', 'aiff', 'flac', 'ogg'],
      },
    ],
  })

  if (!result) return
  const path = Array.isArray(result) ? result[0] : result
  if (!path) return

  props.settings.impulse_response = null
  props.settings.impulse_response_attachment = null
  props.settings.impulse_response_path = path
}

const clearImpulseResponse = () => {
  props.settings.impulse_response = null
  props.settings.impulse_response_attachment = null
  props.settings.impulse_response_path = null
}
</script>
