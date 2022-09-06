<template>
  <div class="track">
    <div
      :style="`width:${width}; background-color: #fff;`"
      :id="`overview-container-${identifier}`"
      class="overview-container"
    ></div>
    <!-- <audio v-if="track" :class="`player ${selected ? 'playable' : 'non-playable'}`" :id="`audio-${identifier}`">
      <source :src="`file://${track.path}`" type="audio/mp3" />
    </audio> -->
  </div>
</template>

<script setup lang="ts">
// import AudioPeaks from "vue-peaks";
import { onMounted, computed, onUpdated, ref, onBeforeMount, onBeforeUnmount, watch } from "vue";

import { useAudioStore } from "../../stores/audio";
import { TrackFileSkeleton } from "../../typings/tracks";
import WaveSurfer from "wavesurfer.js";
import { useAlertStore } from '../../stores/alerts';

// import playMaster from '../../public/playmaster';

interface Props {
  track: TrackFileSkeleton;
  selected: boolean;
}

const audio = useAudioStore();
const alertStore = useAlertStore();

const props = defineProps<Props>();

const width = ref("100%");
const duration = ref(0);

const identifier = computed(() => `${props.track.parentId}-${props.track.id}`);

const wavesurfer = ref(null as WaveSurfer | null);

const clearContainer = () => {
  const container = document.getElementById(`overview-container-${identifier.value}`);
  if (container) container.innerHTML = "";
};

const initialisePeaks = () => {
  clearContainer();
  wavesurfer.value = WaveSurfer.create({
    container: `#overview-container-${identifier.value}`,
    audioContext: audio.audioContext,
    cursorWidth: 0,
    autoCenter: false,
    barHeight: audio.zoom
    // barHeight: 20,
    // barWidth: 1,
  });

  wavesurfer.value.load(`file://${props.track.path}`);

  wavesurfer.value.on("ready", () => {
    duration.value = wavesurfer.value?.getDuration() || duration.value;
    resizeWave();
  });
};

const resizeWave = () => {
  if (wavesurfer.value) {
      width.value = `${duration.value * audio.getScale}px`;
      wavesurfer.value.zoom(audio.getScale - .1);
      // wavesurfer.value.setHeight(audio.zoom * 128);
      // console.log(wavesurfer.value.set())
    }
}

const onResize = () => {
  resizeWave();
  // initialisePeaks();
};

onUpdated(() => {
  resizeWave();
  // initialisePeaks();
});

onBeforeMount(() => {
  window.addEventListener("resize", onResize);
});

onMounted(() => {
  audio.addFile(props.track.path);
  initialisePeaks();
});

// watch(audio.scale, resizeWave);
// watch(audio.zoom, resizeWave);

onBeforeUnmount(() => {
  window.removeEventListener("resize", onResize);
});
</script>

<style lang="scss" scoped>
.track {
  max-width: calc(100% - 44px);
  background-color: rgba(0, 0, 0, 0.1);
  border-radius: 0.5em;
  padding: 0 0.5em;

  .folder-button {
    margin-top: auto;
  }

  .overview-container {
    min-height: 75px;
    width: 100%;
  }
}
</style>
