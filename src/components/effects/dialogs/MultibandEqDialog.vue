<template>
  <EffectDialogShell
    title="Multiband EQ"
    description="Parametric EQ with optional edge filters and analytical response guidance."
    :enabled="settings.enabled"
    :is-playing="isPlaying"
    :metering-enabled="meteringEnabled"
    :spectral-enabled="spectralEnabled"
    @update:enabled="settings.enabled = $event"
  >
    <template #visual>
      <div class="grid gap-4">
        <EffectResponseChart
          :response="responseCurve"
          :spectral="spectralSnapshot"
          :enabled="settings.enabled"
          :is-playing="isPlaying"
          :spectral-enabled="spectralEnabled"
        />

        <div class="grid gap-3 md:grid-cols-3">
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Bands</div>
            <div class="mt-2 text-2xl font-semibold text-zinc-950">
              {{ settings.points.length }}
            </div>
          </div>
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Low Edge</div>
            <div class="mt-2 text-lg font-semibold text-zinc-950">{{ lowEdgeLabel }}</div>
          </div>
          <div class="rounded-[18px] border border-zinc-200 bg-zinc-50 p-4">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">High Edge</div>
            <div class="mt-2 text-lg font-semibold text-zinc-950">{{ highEdgeLabel }}</div>
          </div>
        </div>
      </div>
    </template>

    <template #controls>
      <div class="grid gap-4">
        <div class="grid gap-2 rounded-[18px] border border-zinc-300 bg-white p-3">
          <div class="flex items-center justify-between gap-3">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Band Stack</div>
            <UButton color="neutral" variant="outline" size="sm" @click="addPoint"
              >Add Band</UButton
            >
          </div>
        </div>

        <div class="grid gap-3 rounded-[18px] border border-zinc-300 bg-white p-3">
          <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">Low Edge</div>
          <select
            class="rounded-xl border border-zinc-300 bg-zinc-50 px-3 py-2 text-sm text-zinc-950 outline-none"
            :value="settings.low_edge?.type ?? ''"
            @change="setLowEdgeType(($event.target as HTMLSelectElement).value)"
          >
            <option value="">None</option>
            <option value="high_pass">High-Pass</option>
            <option value="low_shelf">Low Shelf</option>
          </select>

          <template v-if="settings.low_edge">
            <EffectDialControl
              v-model="settings.low_edge.freq_hz"
              label="Frequency"
              :min="20"
              :max="4000"
              :step="1"
              units="Hz"
              scale="log"
            />
            <EffectDialControl
              v-model="settings.low_edge.q"
              label="Q"
              :min="0.1"
              :max="10"
              :step="0.01"
              :precision="2"
            />
            <EffectDialControl
              v-if="settings.low_edge.type === 'low_shelf'"
              v-model="settings.low_edge.gain_db"
              label="Gain"
              :min="-24"
              :max="24"
              :step="0.5"
              units="dB"
              :precision="1"
            />
          </template>
        </div>

        <div
          v-for="(point, index) in settings.points"
          :key="`${index}-${point.freq_hz}`"
          class="grid gap-3 rounded-[18px] border border-zinc-300 bg-white p-3"
        >
          <div class="flex items-center justify-between gap-3">
            <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">
              Band {{ index + 1 }}
            </div>
            <UButton
              color="error"
              variant="ghost"
              size="sm"
              :disabled="settings.points.length === 0"
              @click="removePoint(index)"
            >
              Remove
            </UButton>
          </div>

          <EffectDialControl
            v-model="point.freq_hz"
            label="Frequency"
            :min="20"
            :max="20000"
            :step="1"
            units="Hz"
            scale="log"
          />
          <EffectDialControl
            v-model="point.q"
            label="Q"
            :min="0.1"
            :max="10"
            :step="0.01"
            :precision="2"
          />
          <EffectDialControl
            v-model="point.gain_db"
            label="Gain"
            :min="-24"
            :max="24"
            :step="0.5"
            units="dB"
            :precision="1"
          />
        </div>

        <div class="grid gap-3 rounded-[18px] border border-zinc-300 bg-white p-3">
          <div class="text-[10px] uppercase tracking-[0.18em] text-zinc-500">High Edge</div>
          <select
            class="rounded-xl border border-zinc-300 bg-zinc-50 px-3 py-2 text-sm text-zinc-950 outline-none"
            :value="settings.high_edge?.type ?? ''"
            @change="setHighEdgeType(($event.target as HTMLSelectElement).value)"
          >
            <option value="">None</option>
            <option value="low_pass">Low-Pass</option>
            <option value="high_shelf">High Shelf</option>
          </select>

          <template v-if="settings.high_edge">
            <EffectDialControl
              v-model="settings.high_edge.freq_hz"
              label="Frequency"
              :min="500"
              :max="20000"
              :step="1"
              units="Hz"
              scale="log"
            />
            <EffectDialControl
              v-model="settings.high_edge.q"
              label="Q"
              :min="0.1"
              :max="10"
              :step="0.01"
              :precision="2"
            />
            <EffectDialControl
              v-if="settings.high_edge.type === 'high_shelf'"
              v-model="settings.high_edge.gain_db"
              label="Gain"
              :min="-24"
              :max="24"
              :step="0.5"
              units="dB"
              :precision="1"
            />
          </template>
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
import type {
  EffectBandSnapshot,
  EffectLevelSnapshot,
  FilterResponseCurve,
  MultibandEqSettings,
} from '../../../typings/effects'
import {
  createEqPoint,
  createHighEdgeFilter,
  createLowEdgeFilter,
  formatFrequencyValue,
} from '../../../utils/effects'
import EffectDialControl from './EffectDialControl.vue'
import EffectDialogShell from './EffectDialogShell.vue'
import EffectLiveMeters from './EffectLiveMeters.vue'
import EffectResponseChart from './EffectResponseChart.vue'

interface Props {
  settings: MultibandEqSettings
  responseCurve: FilterResponseCurve | null
  spectralSnapshot: EffectBandSnapshot | null
  liveLevels: EffectLevelSnapshot | null
  isPlaying: boolean
  meteringEnabled: boolean
  spectralEnabled: boolean
}

const props = defineProps<Props>()

const addPoint = () => {
  const last = props.settings.points[props.settings.points.length - 1]
  const nextFrequency = last ? Math.min(18_000, Math.round(last.freq_hz * 1.6)) : 1_000
  props.settings.points.push(createEqPoint(nextFrequency, 0, 0.8))
}

const removePoint = (index: number) => {
  props.settings.points.splice(index, 1)
}

const setLowEdgeType = (type: string) => {
  props.settings.low_edge = type ? createLowEdgeFilter(type as 'high_pass' | 'low_shelf') : null
}

const setHighEdgeType = (type: string) => {
  props.settings.high_edge = type ? createHighEdgeFilter(type as 'low_pass' | 'high_shelf') : null
}

const lowEdgeLabel = computed(() => {
  const edge = props.settings.low_edge
  if (!edge) return 'None'
  return edge.type === 'high_pass' ? `HP ${formatFrequencyValue(edge.freq_hz)}` : 'Low Shelf'
})

const highEdgeLabel = computed(() => {
  const edge = props.settings.high_edge
  if (!edge) return 'None'
  return edge.type === 'low_pass' ? `LP ${formatFrequencyValue(edge.freq_hz)}` : 'High Shelf'
})
</script>
