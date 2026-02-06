<template>
  <div class="effects-controls">
    <h2>REVERB</h2>
    <div v-if="exists" class="control-bin">
      <div>decay</div>
      <el-slider v-model="decay" :show-tooltip="false" :max="70" :step="0.1" size="small" />
      <el-input type="text" v-model="decay" disabled="true" />

      <div>pre delay</div>
      <el-slider v-model="preDelay" :show-tooltip="false" :max="2" :step="0.005" size="small" />
      <el-input type="text" v-model="preDelay" disabled="true" />

      <div>mix</div>
      <el-slider v-model="mix" :show-tooltip="false" :max="1" :step="0.001" size="small" />
      <el-input type="text" v-model="mix" disabled="true" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Reverb } from 'tone'
import { computed, onMounted } from 'vue'
import { ReverbSettings } from '../../assets/effects'
import { toneMaster } from '../../assets/toneMaster'
import { useAudioStore } from '../../stores/audio'

interface Props {
  effectIndex: number
}

const audio = useAudioStore()
const props = defineProps<Props>()
const exists = computed((): boolean => audio.effects[props.effectIndex] !== undefined)

let reverb: Reverb | undefined

const decay = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as ReverbSettings).decay
  },
  set(decay: number) {
    if (reverb) reverb.decay = decay
    else getReverb()
    ;(audio.effects[props.effectIndex].effect as ReverbSettings).decay = decay
  },
})

const preDelay = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as ReverbSettings).preDelay
  },
  set(preDelay: number) {
    if (reverb) reverb.preDelay = preDelay
    else getReverb()
    ;(audio.effects[props.effectIndex].effect as ReverbSettings).preDelay = preDelay
  },
})

const mix = computed({
  get() {
    return (audio.effects[props.effectIndex].effect as ReverbSettings).mix
  },
  set(mix: number) {
    if (reverb) reverb.wet.value = mix
    else getReverb()
    ;(audio.effects[props.effectIndex].effect as ReverbSettings).mix = mix
  },
})

const getReverb = () => {
  if (!reverb) {
    const tentativeReverb = toneMaster.getEffect('Reverb')
    if (tentativeReverb instanceof Reverb) reverb = tentativeReverb
  }
}

onMounted(() => {
  getReverb()
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
