<template>
  <div class="track">
    <div :id="`overview-container-${identifier}`" class="overview-container"></div>
    <audio v-if="track" :class="`player ${selected ? 'playable' : 'non-playable'}`" :id="`audio-${identifier}`">
      <source :src="`file://${track.path}`" type="audio/mp3" />
    </audio>
  </div>
</template>

<script setup lang="ts">
// import AudioPeaks from "vue-peaks";
import Peaks from "peaks.js";
import type PeaksOptions from "peaks.js";
import { onMounted, computed, onUpdated, ref } from "vue";
import { v4 as uuidv4 } from "uuid";

import { useProteusStore } from "../../stores/proteus";
import { TrackFile } from "../../typings/tracks";

interface Props {
  track: TrackFile;
  selected: boolean;
}

const prot = useProteusStore();

const props = defineProps<Props>();

const uuid = uuidv4();

const audioContext = new AudioContext();

const peaks = ref([]);

// const peaks = computed(() => {
//   reinitialisePeaks();
//   return { name: props.track.name };
// });

const identifier = computed(() => `${props.track.parentId}-${props.track.id}`);

const initialisePeaks = () => {
  const container = document.getElementById(`overview-container-${identifier.value}`);
//   if(container) container.innerHTML = "";
  let options: PeaksOptions = {
    overview: {
      container: container,
      waveformColor: "#848484",
      playedWaveformColor: "#5d5d5d",
      playheadColor: "white",
      showAxisLabels: false,
    },
    mediaElement: document.getElementById(`audio-${identifier.value}`),
    //   mediaElement: document.querySelector("audio"),
    webAudio: {
      audioContext: new AudioContext(),
    },
  };
  
  Peaks.init(options, function (err, peaks) {
    // console.log(peaks?.views.getView('overview'));
    // console.log(uuid);
    // Do something when the waveform is displayed and ready
  });
};

onUpdated(() => {
    initialisePeaks();
});

onMounted(() => {
  initialisePeaks();
});
</script>

<style lang="scss" scoped>
.track {
  max-width: calc(100% - 44px);

  .folder-button {
    margin-top: auto;
  }

  .overview-container {
    min-height: 75px;
    width: 100%;
  }
}
</style>
