<template>
  <div class="effects-controls">
    <h2>REVERB</h2>
    <div class="control-bin">
      <div>decay</div>
      <el-slider v-model="decay" :show-tooltip="false" :max="70" :step="0.1" size="small" />

      <div>pre delay</div>
      <el-slider v-model="preDelay" :show-tooltip="false" :max="2" :step="0.005" size="small" />

      <div>mix</div>
      <el-slider v-model="mix" :show-tooltip="false" :max="1" :step="0.001" size="small" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Reverb } from "tone";
import { computed, onMounted } from "vue";
import { toneMaster } from "../../public/toneMaster";
import { useAudioStore } from "../../stores/audio";

const audio = useAudioStore();

let reverb: Reverb | undefined;

const decay = computed({
  get() {
    return audio.reverb.decay;
  },
  set(decay: number) {
    if (reverb) reverb.decay = decay;
    else getReverb();
    audio.reverb.decay = decay;
  },
});

const preDelay = computed({
  get() {
    return audio.reverb.preDelay;
  },
  set(preDelay: number) {
    if (reverb) reverb.preDelay = preDelay;
    else getReverb();
    audio.reverb.preDelay = preDelay;
  },
});

const mix = computed({
  get() {
    return audio.reverb.mix;
  },
  set(mix: number) {
    if (reverb) reverb.wet.value = mix;
    else getReverb();
    audio.reverb.mix = mix;
  },
});

const getReverb = () => {
  if (!reverb) {
    const tentativeReverb = toneMaster.getEffect("Reverb");
    if (tentativeReverb instanceof Reverb) reverb = tentativeReverb;
  }
};

onMounted(() => {
  getReverb();
});
</script>

<style lang="scss" scoped>
.effects-controls {
  padding-bottom: 3em;

  h2 {
    margin-top: 0;
  }
  .control-bin {
    display: grid;
    grid-template-columns: 100px 1fr;
    column-gap: 1em;
    row-gap: 1em;
    text-align: right;
  }
}
</style>
