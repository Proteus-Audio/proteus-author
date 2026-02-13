<template>
  <div class="transport">
    <el-button v-if="!audio.isPlaying" :icon="VideoPlay" @click="play" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="pause" text>pause</el-button>
    <el-button id="BaseTransportStop" :icon="Close" @click="stop" text>stop</el-button>
    <el-button id="BaseTransportShuffle" :icon="Refresh" @click="shuffle" text>shuffle</el-button>
    <el-button :icon="ZoomIn" @click="zoomIn" text :disabled="zoomInDisabled"></el-button>
    <el-button :icon="ZoomOut" @click="zoomOut" text :disabled="zoomOutDisabled"></el-button>
    <el-button :icon="Back" @click="panLeft" text>left</el-button>
    <el-button :icon="Right" @click="panRight" text>right</el-button>
  </div>
</template>

<script setup lang="ts">
import { useAudioStore } from '../../stores/audio'
import { useTrackStore } from '../../stores/track'
import {
  VideoPlay,
  VideoPause,
  Close,
  Refresh,
  ZoomOut,
  ZoomIn,
  Back,
  Right,
} from '@element-plus/icons-vue'
import { computed } from 'vue'

const audio = useAudioStore()
const track = useTrackStore()

const zoomInDisabled = computed(() => {
  return audio.getViewDuration <= 0.51
})

const zoomOutDisabled = computed(() => {
  if (audio.duration <= 0) return false
  return audio.getViewDuration >= audio.duration - 0.01
})

const play = () => {
  void audio.play()
  // if (!track.initialised) {
  //   window.dispatchEvent(new Event('resize'))
  //   track.initialised = true
  // }
}

const pause = () => {
  void audio.pause()
}

const stop = () => {
  void audio.stop()
}

const shuffle = () => {
  void track.shuffle()
}

const zoomIn = () => audio.zoomIn()
const zoomOut = () => audio.zoomOut()
const panLeft = () => audio.panViewLeft(0.2)
const panRight = () => audio.panViewRight(0.2)
</script>

<style lang="scss" scoped>
.transport {
  /* margin-bottom: 1em; */
  background-color: white;
  padding: 1em 0;
  text-align: right;
  display: grid;
  grid-template-columns: 100px 100px 100px 50px 50px 70px 70px;
  gap: 1em;
}
</style>
