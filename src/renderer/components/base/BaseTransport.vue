<template>
    <div class="transport">
        <el-button v-if="!prot.isPlaying" :icon="VideoPlay" @click="play" text>play</el-button>
        <el-button v-else :icon="VideoPause" @click="pause" text>pause</el-button>
        <el-button :icon="Close" @click="stop" text>stop</el-button>
    </div>
</template>

<script setup lang="ts">
import { useProteusStore } from "../../stores/proteus";
import playMaster from "../../public/playmaster";
import { VideoPlay, VideoPause, Close } from "@element-plus/icons-vue";

const prot = useProteusStore();

const play = () => {
    playMaster.play();
    prot.setPlaying(true);
};

const pause = () => {
  playMaster.pause();
  prot.setPlaying(false);
};

const stop = () => {
  playMaster.stop();
  prot.setPlaying(false);
};
</script>

<style scoped>
.transport {
  margin-bottom: 1em;
}
</style>
