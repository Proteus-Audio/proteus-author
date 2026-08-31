<template>
  <EffectDialogShell
    title="Compressor"
    description="Dynamic range control with threshold, ratio, timing, and makeup gain."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <div
          class="overflow-hidden rounded-[20px] border border-zinc-200 bg-[radial-gradient(circle_at_top,rgba(34,197,94,0.08),transparent_38%),linear-gradient(180deg,#ffffff_0%,#f5f7fb_100%)] p-4"
        >
          <div class="flex flex-wrap items-start justify-between gap-4">
            <div>
              <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Compression</div>
              <div class="mt-2 text-4xl font-semibold" :class="compressionTone">
                {{ compressionReadout }}
              </div>
              <div class="mt-2 max-w-2xl text-sm leading-6 text-zinc-600">
                {{ compressionText }}
              </div>
            </div>

            <div
              class="rounded-[16px] border border-zinc-200 bg-white/80 px-3 py-2 text-right shadow-[0_10px_20px_rgba(15,23,42,0.05)]"
            >
              <div class="text-[10px] uppercase tracking-[0.16em] text-zinc-500">Formula</div>
              <div class="mt-1 text-sm font-medium text-zinc-950">Output - Input</div>
            </div>
          </div>

          <div
            class="mt-4 flex flex-wrap items-center gap-4 text-[10px] uppercase tracking-[0.18em] text-zinc-500"
          >
            <div class="flex items-center gap-2">
              <span class="h-2.5 w-2.5 rounded-full bg-cyan-400"></span>
              Input Peak
            </div>
            <div class="flex items-center gap-2">
              <span class="h-2.5 w-2.5 rounded-full bg-emerald-400"></span>
              Output Peak
            </div>
            <div
              class="rounded-full border border-amber-200 bg-amber-50 px-3 py-1 text-amber-700"
            >
              Threshold {{ thresholdReadout }}
            </div>
          </div>

          <div class="mt-6 grid gap-4 lg:grid-cols-[52px_minmax(0,1fr)_130px]">
            <div
              class="flex h-[280px] flex-col justify-between text-right text-xs tabular-nums text-zinc-500"
            >
              <span v-for="tick in meterTicks" :key="`tick-${tick}`">
                {{ formatMeterTick(tick) }}
              </span>
            </div>

            <div
              class="relative h-[280px] rounded-[22px] border border-white/10 bg-zinc-950 p-4 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)]"
            >
              <div
                class="relative h-full overflow-hidden rounded-[18px] bg-[linear-gradient(180deg,rgba(244,63,94,0.18)_0%,rgba(245,158,11,0.12)_40%,rgba(16,185,129,0.08)_100%)]"
              >
                <div
                  v-for="tick in meterTicks"
                  :key="`grid-${tick}`"
                  class="absolute inset-x-0 border-t border-white/10"
                  :style="{ bottom: `${toMeterPercent(tick)}%` }"
                ></div>

                <div
                  class="absolute inset-x-0 border-t border-dashed border-amber-300/90"
                  :style="{ bottom: `${thresholdPercent}%` }"
                ></div>

                <div
                  class="absolute right-3 flex items-center gap-2"
                  :style="{ bottom: `calc(${thresholdPercent}% - 10px)` }"
                >
                  <span class="text-[10px] uppercase tracking-[0.18em] text-amber-200">
                    Threshold
                  </span>
                  <span
                    class="h-0 w-0 border-y-[7px] border-y-transparent border-r-[10px] border-r-amber-400"
                  ></span>
                </div>

                <div class="absolute inset-0 flex items-end justify-center gap-8 px-8 py-4">
                  <div
                    class="flex h-full w-16 items-end rounded-[18px] border border-white/10 bg-white/5 p-2"
                  >
                    <div
                      class="w-full rounded-[12px] bg-gradient-to-t from-cyan-500 via-cyan-400 to-sky-200 shadow-[0_0_20px_rgba(34,211,238,0.28)] transition-[height] duration-150"
                      :style="{ height: `${inputPeakPercent}%` }"
                    ></div>
                  </div>

                  <div
                    class="flex h-full w-16 items-end rounded-[18px] border border-white/10 bg-white/5 p-2"
                  >
                    <div
                      class="w-full rounded-[12px] bg-gradient-to-t from-emerald-600 via-emerald-400 to-emerald-200 shadow-[0_0_20px_rgba(16,185,129,0.28)] transition-[height] duration-150"
                      :style="{ height: `${outputPeakPercent}%` }"
                    ></div>
                  </div>
                </div>
              </div>
            </div>

            <div class="grid h-[280px] content-between gap-3">
              <div class="rounded-[16px] border border-zinc-200 bg-white/80 p-3">
                <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Input Peak</div>
                <div class="mt-2 text-xl font-semibold text-zinc-950">{{ inputPeakReadout }}</div>
              </div>

              <div class="rounded-[16px] border border-zinc-200 bg-white/80 p-3">
                <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
                  Output Peak
                </div>
                <div class="mt-2 text-xl font-semibold text-zinc-950">{{ outputPeakReadout }}</div>
              </div>

              <div class="rounded-[16px] border border-zinc-200 bg-white/80 p-3">
                <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Threshold</div>
                <div class="mt-2 text-xl font-semibold text-zinc-950">{{ thresholdReadout }}</div>
              </div>
            </div>
          </div>
        </div>

        <div class="grid gap-3 md:grid-cols-3">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Ratio</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ ratioReadout }}</div>
          </div>

          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
              Attack / Release
            </div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ timingReadout }}</div>
          </div>

          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Makeup Gain</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">{{ makeupReadout }}</div>
          </div>
        </div>

      </div>
    </template>

    <template #controls>
      <div class="grid gap-3">
        <EffectDialControl
          v-model="settings.threshold_db"
          label="Threshold"
          :min="-60"
          :max="0"
          :step="0.5"
          units="dB"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.ratio"
          label="Ratio"
          :min="1"
          :max="20"
          :step="0.1"
          :precision="1"
        />
        <EffectDialControl
          v-model="settings.attack_ms"
          label="Attack"
          :min="1"
          :max="200"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.release_ms"
          label="Release"
          :min="1"
          :max="500"
          :step="1"
          units="ms"
        />
        <EffectDialControl
          v-model="settings.makeup_gain_db"
          label="Makeup Gain"
          :min="-30"
          :max="30"
          :step="0.5"
          units="dB"
          :precision="1"
        />
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
import type { CompressorSettings, EffectLevelSnapshot } from '../../../typings/effects'
import { maxChannelDb, normalizeDb } from '../../../utils/effect-visuals'
import { formatDbValue } from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'

interface Props {
  settings: CompressorSettings
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
}

const props = defineProps<Props>()

const MIN_DB = -60
const MAX_DB = 12
const meterTicks = [12, 0, -12, -24, -36, -48, -60]

const inputPeakDb = computed(() => maxChannelDb(props.liveLevels?.input.peak ?? [], MIN_DB, MAX_DB))
const outputPeakDb = computed(() => maxChannelDb(props.liveLevels?.output.peak ?? [], MIN_DB, MAX_DB))
const compressionDeltaDb = computed(() => outputPeakDb.value - inputPeakDb.value)
const meterAvailable = computed(() => props.meteringEnabled && props.isPlaying && props.liveLevels)

const toMeterPercent = (db: number) => normalizeDb(db, MIN_DB, MAX_DB) * 100
const formatMeterTick = (db: number) => `${db > 0 ? '+' : ''}${db} dB`

const inputPeakPercent = computed(() => toMeterPercent(inputPeakDb.value))
const outputPeakPercent = computed(() => toMeterPercent(outputPeakDb.value))
const thresholdPercent = computed(() => toMeterPercent(props.settings.threshold_db))

const inputPeakReadout = computed(() => formatDbValue(inputPeakDb.value, 1))
const outputPeakReadout = computed(() => formatDbValue(outputPeakDb.value, 1))
const thresholdReadout = computed(() => formatDbValue(props.settings.threshold_db, 1))
const ratioReadout = computed(() => `${props.settings.ratio.toFixed(1)}:1`)
const timingReadout = computed(
  () => `${Math.round(props.settings.attack_ms)} / ${Math.round(props.settings.release_ms)} ms`,
)
const makeupReadout = computed(() => formatDbValue(props.settings.makeup_gain_db, 1))

const compressionReadout = computed(() => {
  if (!meterAvailable.value) return '--'
  return formatDbValue(compressionDeltaDb.value, 1)
})

const compressionTone = computed(() => {
  if (!meterAvailable.value) return 'text-zinc-500'
  if (compressionDeltaDb.value <= -0.5) return 'text-emerald-600'
  if (compressionDeltaDb.value >= 0.5) return 'text-amber-600'
  return 'text-zinc-950'
})

const compressionText = computed(() => {
  if (!props.meteringEnabled) {
    return 'Enable live metering to calculate the compressor delta from output minus input.'
  }
  if (!props.isPlaying) {
    return 'Start playback to watch peaks rise into the threshold and update the live delta.'
  }
  if (!props.liveLevels) {
    return 'Waiting for the next live level snapshot.'
  }
  if (!props.settings.enabled) {
    return 'The effect is bypassed, so the meter is only showing pre and post level context.'
  }
  if (compressionDeltaDb.value <= -0.5) {
    return 'Output is below input, so the compressor is actively holding the signal down.'
  }
  if (compressionDeltaDb.value >= 0.5) {
    return 'Makeup gain is pushing the post-compression level above the incoming peak.'
  }
  return 'Input and output are tracking closely at the current threshold and ratio.'
})
</script>
