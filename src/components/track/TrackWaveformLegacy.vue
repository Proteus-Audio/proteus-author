<template>
  <div class="track">
    <div
      :style="`width:${width}; background-color: #fff;`"
      :id="`overview-container-${identifier}`"
      class="overview-container"
    ></div>
    <!-- <audio v-if="track" :class="`player ${selected ? 'playable' : 'non-playable'}`" :id="`audio-${identifier}`" controls>
      <source :src="`file://${track.path}`" type="audio/mp3" />
    </audio> -->
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'

import { useAudioStore } from '../../stores/audio'
import { TrackFileSkeleton } from '../../typings/tracks'
import { ExposedBuffer } from '../../typings/tone.d'
import Peaks, { PeaksOptions } from 'peaks.js'
import { toneMaster, PeaksPlayer } from '../../assets/toneMaster'
import * as Tone from 'tone'
import { cloneAudioBuffer } from '../../assets/tools'
import { useTrackStore } from '../../stores/track'

interface Props {
  track: TrackFileSkeleton
  selected: boolean
}

const audio = useAudioStore()
const trackStore = useTrackStore()
const props = defineProps<Props>()

const duration = ref(0)
const identifier = computed(() => `${props.track.parentId}-${props.track.id}`)
const widthVal = computed((): number => duration.value * audio.getXScale)
const width = computed((): string => (widthVal.value > 0 ? `${widthVal.value}px` : '100%'))

const clearContainer = () => {
  const container = document.getElementById(`overview-container-${identifier.value}`)
  if (container) container.innerHTML = ''
  return container
}

const initialisePeaks = async () => {
  trackStore.initialised = false
  const player = false // toneMaster.playerFromIds(props.track.parentId, props.track.id)
  if (!player) return
  const container = clearContainer()

  await Tone.loaded()
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  duration.value = player.buffer.duration
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const audioBuffer = cloneAudioBuffer((player.buffer as unknown as ExposedBuffer)._buffer)
  console.log(props.track, audioBuffer)
  const options = {
    overview: {
      container: container,
    },
    // mediaElement: document.querySelector("audio"),
    player: new PeaksPlayer(),
    webAudio: {
      // audioContext: new AudioContext(),
      audioBuffer: audioBuffer,
    },
  }

  Peaks.init(options as PeaksOptions, function (err, peaks) {
    if (err) {
      console.error('Failed to initialize Peaks instance: ' + err.message)
      console.log(peaks)
      return
    }
    // Do something when the waveform is displayed and ready
  })
}

onMounted(() => {
  console.log('starting file')
  initialisePeaks()
})
</script>

<style lang="scss" scoped>
.track {
  // max-width: calc(100% - 44px);
  background-color: rgba(0, 0, 0, 0.1);
  // border-radius: 0.5em;
  // padding: 0 0.5em;

  .folder-button {
    margin-top: auto;
  }

  .overview-container {
    min-height: 150px;
    width: 100%;
  }
}
</style>
