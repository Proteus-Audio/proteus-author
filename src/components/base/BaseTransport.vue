<template>
  <div class="transport">
    <el-button v-if="!audio.isPlaying" :icon="VideoPlay" @click="play" text>play</el-button>
    <el-button v-else :icon="VideoPause" @click="audio.pause" text>pause</el-button>
    <el-button id="BaseTransportStop" :icon="Close" @click="audio.stop" text>stop</el-button>
    <el-button id="BaseTransportShuffle" :icon="Refresh" @click="track.shuffle" text
      >shuffle</el-button
    >
    <el-button :icon="ZoomIn" @click="zoomIn" text :disabled="audio.zoom.x === 20"></el-button>
    <el-button :icon="ZoomOut" @click="zoomOut" text :disabled="audio.zoom.x === 1"></el-button>
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
import { invoke } from '@tauri-apps/api'

const audio = useAudioStore()
const track = useTrackStore()
const volumeRef = ref(toneMaster.volume)

const volume = computed({
  get: () => volumeRef.value * 75,
  set: async (value: number) => {
    volumeRef.value = value / 75
    await invoke('set_volume', { volume: (value / 100) * 3 })
    // toneMaster.setGain(value / 75)
  },
})

const play = () => {
  audio.play()
  // if (!track.initialised) {
  //   window.dispatchEvent(new Event('resize'))
  //   track.initialised = true
  // }
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
