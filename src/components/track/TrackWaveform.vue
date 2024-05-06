<template>
  <div class="track">
    <div
      :style="`width:${width};`"
      :id="`overview-container-${identifier}`"
      class="overview-container"
    >
      <div class="channels">
        <div class="channel" v-for="(peaks, channel) in simplifiedPeaks" :key="channel">
          <div @click="seek" class="control"></div>
          <div class="playhead"></div>
          <template v-for="annotation in peaksLength" :key="`annotation-${channel}-${annotation}`">
            <div
              v-if="annotate(annotation)"
              :style="{ left: `${(annotation - 1) * 2}px` }"
              class="annotation"
            >
              <div class="timestamp">{{ getAnnotation(annotation) }}</div>
            </div>
          </template>
          <template v-for="(peak, index) in peaks.peaks" :key="`${channel}-${index}`">
            <div :style="{ height: `${calcHeight(peak)}%` }" class="peak"></div>
          </template>
        </div>
      </div>
    </div>
    <!-- <audio v-if="track" :class="`player ${selected ? 'playable' : 'non-playable'}`" :id="`audio-${identifier}`" controls>
      <source :src="`file://${track.path}`" type="audio/mp3" />
    </audio> -->
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref, watch } from 'vue'

import { useAudioStore } from '../../stores/audio'
import { TrackFile } from '../../typings/tracks'
import { invoke } from '@tauri-apps/api'

interface Props {
  track: TrackFile
  selected: boolean
}

interface SimplifiedPeaks {
  peaks: number[]
  zoom: number
  original_length: number
}

const audio = useAudioStore()
const props = defineProps<Props>()

const duration = ref(0)
const identifier = computed(() => `${props.track.parentId}-${props.track.id}`)
const widthVal = computed((): number => duration.value * audio.getXScale)
const width = computed((): string => (widthVal.value > 0 ? `${widthVal.value}px` : '100%'))

const calcHeight = (peak: number) => {
  return peak * 200 + 1
}

const simplifiedPeaks = ref([] as SimplifiedPeaks[])

watch(audio.zoom, () => {
  // console.log('zoom changed')
  getSimplifiedPeaks().then((peaks) => {
    simplifiedPeaks.value = peaks
  })
})

const getSimplifiedPeaks = async () => {
  const simplifiedPeaks = (await invoke('get_simplified_peaks', {
    fileId: props.track.id,
    zoom: audio.zoom.x,
  })) as SimplifiedPeaks[]

  console.log('simplifiedPeaks', simplifiedPeaks)

  return simplifiedPeaks
}

const peaksLength = computed(() => simplifiedPeaks.value[0]?.peaks.length || 0)

const zoomLevel = computed(() => {
  const factor = (simplifiedPeaks.value[0]?.original_length || 100) / peaksLength.value

  console.log('factor', factor)

  return 100 / factor
})

const annotate = (index: number): boolean => {
  const zoom = zoomLevel.value

  let toAnnotate = false

  const setToAnnotate = (division: number) => {
    toAnnotate = Math.floor(index % (zoom * division)) === 0
  }

  if (zoom <= 3) setToAnnotate(30)
  else if (zoom <= 5) setToAnnotate(10)
  else if (zoom <= 15) setToAnnotate(5)
  else if (zoom <= 25) setToAnnotate(3)
  else if (zoom <= 100) setToAnnotate(2)
  else setToAnnotate(1)

  return toAnnotate
}

const getAnnotation = (index: number): string => {
  const seconds = index / zoomLevel.value

  // Return seconds converted to mm:ss
  return new Date(seconds * 1000).toISOString().substr(14, 5)
}

const playheadPosition = computed(() => {
  const factor = audio.clock * zoomLevel.value * 2
  console.log(audio.clock, factor, zoomLevel.value)
  return `${factor}px`
})

const seek = (event: MouseEvent) => {
  const seconds = event.offsetX / zoomLevel.value / 2
  audio.seek(seconds)
}

onMounted(() => {
  console.log('starting file')

  getSimplifiedPeaks().then((peaks) => {
    simplifiedPeaks.value = peaks
  })

  console.log(props.track)
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

.channels {
  position: absolute;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  overflow-x: hidden;
  overflow-y: hidden;
  .channel {
    position: relative;
    height: 100%;
    width: 100%;
    width: calc(2px * v-bind(peaksLength));
    // width: calc((2px + 0.1em) * v-bind(peaksLength));
    display: flex;
    flex-wrap: wrap;
    // gap: 0.1em;
    flex-direction: row;
    // flex-direction: column;
    align-items: center;
    // align-items: flex-start;
    // justify-content: flex-end;
    background-color: white;
    overflow: hidden;

    .playhead {
      position: absolute;
      pointer-events: none;
      top: 0;
      left: v-bind(playheadPosition);
      height: 100%;
      width: 1px;
      background-color: #7474746e;
    }

    .control {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
    }

    .peak {
      position: relative;
      display: block;
      width: 2px;
      background-color: #747474;
      opacity: 0.5;
      // height: calc(10% * attr(top));
      border-radius: 0.1em;
      pointer-events: none;

      // &:nth-of-type(5n) {
      //   &::after {
      //     content: '';
      //     display: block;
      //     width: 1px;
      //     height: 1000px;
      //     background-color: rgba(0, 0, 0, 0.1);
      //     position: absolute;
      //     left: 0;
      //     top: -500px;
      //   }
      // }

      // &:nth-of-type(10n) {
      //   &::after {
      //     content: '';
      //     display: block;
      //     width: 2px;
      //     height: 1000px;
      //     background-color: rgba(0, 0, 0, 0.2);
      //     position: absolute;
      //     left: 0;
      //     top: -500px;
      //   }
      // }
    }

    .annotation {
      position: absolute;
      height: 100%;
      pointer-events: none;
      // width: 1px;
      // background: grey;

      .timestamp {
        position: absolute;
        font-size: 0.7em;
        bottom: 15px;
        left: 0;
        transform: translateX(-50%);
        color: rgba(0, 0, 0, 0.4);
        background: white;
        z-index: 10;
      }

      &::after,
      &::before {
        content: '';
        display: block;
        width: 2px;
        position: absolute;
        top: 0;
        height: 10px;
        background-color: rgba(0, 0, 0, 0.1);
      }

      &::before {
        bottom: 0;
        top: auto;
      }
    }
  }
}
</style>
