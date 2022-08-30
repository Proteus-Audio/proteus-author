<template>
  <div id="app">
    <BaseContainer>
      <Hello />
      <a class="shuffler" @click="prot.setSelections">SHUFFLE!</a>
      <BaseTransport />
      <TrackBin
        v-for="track in prot.tracks"
        @add-tracks="addAudioFiles"
        :track-id="track.id"
        :key="track.id"
      />
      <div class="padding"></div>
    </BaseContainer>

    <EffectRack />
  </div>
</template>

<script setup lang="ts">
import "primevue/resources/primevue.min.css";
import "primeicons/primeicons.css";
import Hello from "./components/Hello.vue";
import { onMounted, ref } from "vue";
import type { Ref } from "vue";
import EffectRack from "./components/effects/EffectRack.vue";
import BaseContainer from "./components/base/BaseContainer.vue";
import TrackBin from "./components/track/TrackBin.vue";
import { Track } from "./typings/tracks";

import { useProteusStore } from "./stores/proteus";
import BaseTransport from './components/base/BaseTransport.vue';

const prot = useProteusStore();

const addAudioFiles = (input: any) => {
  const key = input.id;
  prot.addFileToTrack(input.files, key);
  addEmptyTrackIfNone();
};

const addEmptyTrackIfNone = () => {
  if (!prot.emptyTrackExists) {
    prot.addTrack({ id: prot.nextTrackId, files: [] });
  }
};

onMounted(() => {
  addEmptyTrackIfNone();
});
</script>

<style lang="scss">
body {
  margin: 0;
  font-family: "Silkscreen", "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}

// img.file {
//   width: 100%;
//   min-height: 300px;
// }

.shuffler {
  cursor: pointer;
  margin-bottom: 1em;
  display: block;

  &:hover {
    opacity: 0.7;
  }

}

.padding {
  display: inline-block;
  width: 1em;
  height: 1em;
}
</style>
