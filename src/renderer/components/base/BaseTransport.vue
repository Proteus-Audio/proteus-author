<template>
  <div class="transport">
    <el-button v-if="!prot.isPlaying" :icon="VideoPlay" @click="prot.play" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="prot.pause" text>pause</el-button>
    <el-button :icon="Close" @click="prot.stop" text>stop</el-button>
    <el-button :icon="Refresh" @click="shuffle" text>shuffle</el-button>
    <el-button :icon="InfoFilled" @click="test" text>test</el-button>
  </div>
</template>

<script setup lang="ts">
import { useProteusStore } from "../../stores/proteus";
import { VideoPlay, VideoPause, Close, Refresh, InfoFilled } from "@element-plus/icons-vue";

const prot = useProteusStore();

const shuffle = () => {
  const playing = prot.isPlaying;
  if (playing) prot.pause();
  prot.setSelections();
  prot.pause();
  if (playing)
    setTimeout(() => {
      prot.play();
    }, 300);
};

const test = () => {
  const audio = new AudioContext();

  console.log(audio);
}
</script>

<style scoped>
.transport {
  /* margin-bottom: 1em; */
  background-color: white;
  padding: 1em;
}
</style>
