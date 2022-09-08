<template>
  <div class="effects-controls">
    <h2>COMPRESSOR</h2>
    <div class="control-bin">
      <div>threshold</div>
      <el-slider v-model="threshold" :show-tooltip="false" :min="-100" :max="0" :step="0.1" size="small" />

      <div>ratio</div>
      <el-slider v-model="ratio" :show-tooltip="false" :min="1" :max="20" :step="0.005" size="small" />

      <div>knee</div>
      <el-slider v-model="knee" :show-tooltip="false" :max="40" :step="0.01" size="small" />

      <div>attack</div>
      <el-slider v-model="attack" :show-tooltip="false" :max="1" :step="0.001" size="small" />

      <div>release</div>
      <el-slider v-model="release" :show-tooltip="false" :max="1" :step="0.001" size="small" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Compressor } from "tone";
import { computed, onMounted } from "vue";
import { toneMaster } from "../../public/toneMaster";
import { useAudioStore } from "../../stores/audio";

const audio = useAudioStore();

let compressor: Compressor | undefined;

const threshold = computed({
  get() {
    return audio.compressor.threshold;
  },
  set(threshold: number) {
    if (compressor) compressor.threshold.value = threshold;
    else getcompressor();
    audio.compressor.threshold = threshold;
  },
});

const ratio = computed({
  get() {
    return audio.compressor.ratio;
  },
  set(ratio: number) {
    if (compressor) compressor.ratio.value = ratio;
    else getcompressor();
    audio.compressor.ratio = ratio;
  },
});

const knee = computed({
  get() {
    return audio.compressor.knee;
  },
  set(knee: number) {
    if (compressor) compressor.knee.value = knee;
    else getcompressor();
    audio.compressor.knee = knee;
  },
});

const attack = computed({
  get() {
    return audio.compressor.attack;
  },
  set(attack: number) {
    if (compressor) compressor.attack.value = attack;
    else getcompressor();
    audio.compressor.attack = attack;
  },
});

const release = computed({
  get() {
    return audio.compressor.release;
  },
  set(release: number) {
    if (compressor) compressor.release.value = release;
    else getcompressor();
    audio.compressor.release = release;
  },
});

const getcompressor = () => {
  if (!compressor) {
    const tentativeCompressor = toneMaster.getEffect("Compressor");
    if (tentativeCompressor instanceof Compressor) compressor = tentativeCompressor;
  }
};

onMounted(() => {
  getcompressor();
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
