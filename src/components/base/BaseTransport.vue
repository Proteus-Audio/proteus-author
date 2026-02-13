<template>
  <div class="transport">
    <el-button v-if="!audio.isPlaying" :icon="VideoPlay" @click="play" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="pause" text>pause</el-button>
    <el-button id="BaseTransportStop" :icon="Close" @click="stop" text>stop</el-button>
    <el-button id="BaseTransportShuffle" :icon="Refresh" @click="shuffle" text>shuffle</el-button>
    <el-button :icon="ZoomIn" @click="zoomIn" text :disabled="zoomInDisabled"></el-button>
    <el-button :icon="ZoomOut" @click="zoomOut" text :disabled="zoomOutDisabled"></el-button>
    <div class="volume-bin">
      <el-slider v-model="volume" :show-tooltip="false" size="small" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAudioStore } from '../../stores/audio'
import { useTrackStore } from '../../stores/track'
import { VideoPlay, VideoPause, Close, Refresh, ZoomOut, ZoomIn } from '@element-plus/icons-vue'
import { toneMaster } from '../../assets/toneMaster'
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const audio = useAudioStore()
const track = useTrackStore()
const volumeRef = ref(toneMaster.volume)

const volume = computed({
  get: () => volumeRef.value * 75,
  set: (value: number) => {
    volumeRef.value = value / 75
    void invoke('set_volume', { volume: (value / 100) * 3 })
    // toneMaster.setGain(value / 75)
  },
})

const zoomInDisabled = computed(() => {
  if (audio.duration <= 0) return true
  return audio.getViewDuration <= 0.51
})

const zoomOutDisabled = computed(() => {
  if (audio.duration <= 0) return true
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
</script>

<style lang="scss" scoped>
.transport {
  /* margin-bottom: 1em; */
  background-color: white;
  padding: 1em 0;
  text-align: right;
  display: grid;
  grid-template-columns: 100px 100px 100px 50px 50px 1fr;
  gap: 1em;
  .volume-bin {
    // width: calc(100% - 500px);
    // display: inline-block;
  }
}
</style>
