<template>
  <div class="transport">
    <el-button v-if="!audio.isPlaying" :icon="VideoPlay" @click="audio.playPizz" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="audio.pause" text>pause</el-button>
    <el-button :icon="Close" @click="audio.stopPizz" text>stop</el-button>
    <el-button :icon="Refresh" @click="shuffle" text>shuffle</el-button>
    <el-button :icon="InfoFilled" @click="test" text>test</el-button>
    <el-button :icon="ZoomIn" @click="zoom" text>zoom</el-button>
  </div>
</template>

<script setup lang="ts">
import { useAudioStore } from '../../stores/audio';
import { useTrackStore } from '../../stores/tracks';
import { VideoPlay, VideoPause, Close, Refresh, InfoFilled, ZoomIn } from "@element-plus/icons-vue";

const audio = useAudioStore();
const track = useTrackStore();

const shuffle = () => {
  const playing = audio.isPlaying;
  if (playing) audio.pause();
  track.setSelections();
  audio.pause();
  if (playing)
    setTimeout(() => {
      audio.play();
    }, 300);
};

const test = () => {
  audio.setScale(audio.getScale + 5);
}
const zoom = () => {
  audio.setZoom(audio.zoom + 5);
}
</script>

<style scoped>
.transport {
  /* margin-bottom: 1em; */
  background-color: white;
  padding: 1em;
}
</style>
