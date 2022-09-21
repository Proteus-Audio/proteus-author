<template>
  <div class="effects-controls">
    <h2>COMPRESSOR</h2>
    <div v-if="exists" class="control-bin">
      <div>threshold</div>
      <el-slider
        v-model="threshold"
        :show-tooltip="false"
        :min="-100"
        :max="0"
        :step="0.1"
        size="small"
      />
      <el-input type="text" v-model="threshold" disabled="true" />

      <div>ratio</div>
      <el-slider
        v-model="ratio"
        :show-tooltip="false"
        :min="1"
        :max="20"
        :step="0.005"
        size="small"
      />
      <el-input type="text" v-model="ratio" disabled="true" />

      <div>knee</div>
      <el-slider v-model="knee" :show-tooltip="false" :max="40" :step="0.01" size="small" />
      <el-input type="text" v-model="knee" disabled="true" />

      <div>attack</div>
      <el-slider v-model="attack" :show-tooltip="false" :max="1" :step="0.001" size="small" />
      <el-input type="text" v-model="attack" disabled="true" />

      <div>release</div>
      <el-slider v-model="release" :show-tooltip="false" :max="1" :step="0.001" size="small" />
      <el-input type="text" v-model="release" disabled="true" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Compressor } from 'tone'
import { computed, onMounted } from 'vue'
import { CompressorSettings } from '../../public/effects'
import { toneMaster } from '../../public/toneMaster'
import { useAudioStore } from '../../stores/audio'

interface Props {
  effectIndex: number
}

const audio = useAudioStore()
const props = defineProps<Props>()
const exists = computed((): boolean => audio.effects[props.effectIndex] !== undefined)

let compressor: Compressor | undefined

const threshold = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as CompressorSettings).threshold
  },
  set(threshold: number) {
    if (compressor) compressor.threshold.value = threshold
    else getcompressor()
    ;(audio.effects[props.effectIndex].effect as CompressorSettings).threshold = threshold
  },
})

const ratio = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as CompressorSettings).ratio
  },
  set(ratio: number) {
    if (compressor) compressor.ratio.value = ratio
    else getcompressor()
    ;(audio.effects[props.effectIndex].effect as CompressorSettings).ratio = ratio
  },
})

const knee = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as CompressorSettings).knee
  },
  set(knee: number) {
    if (compressor) compressor.knee.value = knee
    else getcompressor()
    ;(audio.effects[props.effectIndex].effect as CompressorSettings).knee = knee
  },
})

const attack = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as CompressorSettings).attack
  },
  set(attack: number) {
    if (compressor) compressor.attack.value = attack
    else getcompressor()
    ;(audio.effects[props.effectIndex].effect as CompressorSettings).attack = attack
  },
})

const release = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as CompressorSettings).release
  },
  set(release: number) {
    if (compressor) compressor.release.value = release
    else getcompressor()
    ;(audio.effects[props.effectIndex].effect as CompressorSettings).release = release
  },
})

const getcompressor = () => {
  if (!compressor) {
    const tentativeCompressor = toneMaster.getEffect('Compressor')
    if (tentativeCompressor instanceof Compressor) compressor = tentativeCompressor
  }
}

onMounted(() => {
  getcompressor()
})
</script>

<style lang="scss" scoped>
.effects-controls {
  padding-bottom: 3em;

  h2 {
    margin-top: 0;
  }
  .control-bin {
    display: grid;
    grid-template-columns: 100px 1fr 100px;
    column-gap: 1em;
    row-gap: 1em;
    text-align: right;
  }
}
</style>
