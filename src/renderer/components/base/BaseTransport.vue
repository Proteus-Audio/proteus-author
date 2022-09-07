<template>
  <div class="transport">
    <el-button v-if="!audio.isPlaying" :icon="VideoPlay" @click="play" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="audio.pause" text>pause</el-button>
    <el-button :icon="Close" @click="audio.stop" text>stop</el-button>
    <el-button :icon="Refresh" @click="shuffle" text>shuffle</el-button>
    <el-button :icon="ZoomIn" @click="zoomIn" text></el-button>
    <el-button :icon="ZoomOut" @click="zoomOut" text></el-button>
    <div class="volume-bin">
      <el-slider v-model="volume" :show-tooltip="false" size="small" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAudioStore } from "../../stores/audio";
import { useTrackStore } from "../../stores/tracks";
import { VideoPlay, VideoPause, Close, Refresh, ZoomOut, ZoomIn } from "@element-plus/icons-vue";
import { toneMaster } from "../../public/toneMaster";
import { computed, ref } from "vue";

const audio = useAudioStore();
const track = useTrackStore();
const volumeRef = ref(toneMaster.volume);

const volume = computed({
  get: () => volumeRef.value * 75,
  set: (value: number) => {
    volumeRef.value = value / 75;
    toneMaster.setGain(value / 75);
  },
});

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

const play = () => {
  audio.play();
  if (!track.initialised) {
    window.dispatchEvent(new Event("resize"));
    track.initialised = true;
  }
};

const zoomIn = () => {
  audio.setXScale(audio.getXScale + 5);
};
const zoomOut = () => {
  audio.setXScale(audio.getXScale - 5);
};
</script>

<style lang="scss" scoped>
.transport {
  /* margin-bottom: 1em; */
  background-color: white;
  padding: 1em;
  .volume-bin {
    width: calc(100% - 500px);
    display: inline-block;
  }
}
</style>
