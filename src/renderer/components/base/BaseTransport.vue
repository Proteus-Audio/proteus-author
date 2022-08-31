<template>
  <div class="transport">
    <el-button v-if="!prot.isPlaying" :icon="VideoPlay" @click="prot.play" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="prot.pause" text>pause</el-button>
    <el-button :icon="Close" @click="prot.stop" text>stop</el-button>
    <el-button :icon="Refresh" @click="shuffle" text>shuffle</el-button>
  </div>
</template>

<script setup lang="ts">
import { useProteusStore } from "../../stores/proteus";
import { VideoPlay, VideoPause, Close, Refresh } from "@element-plus/icons-vue";

const prot = useProteusStore();

const shuffle = () => {
  const playing = prot.isPlaying;
  if (playing) prot.pause();
  prot.setSelections();
  prot.pause();
  if (playing)
    setTimeout(() => {
      prot.play();
    }, 200);
};
</script>

<style scoped>
.transport {
  /* margin-bottom: 1em; */
  background-color: white;
  padding: 1em;
}
</style>
