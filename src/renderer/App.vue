<template>
  <div id="app">
    <BaseContainer>
      <!-- <Hello /> -->
      <h1 class="center">Proteus.play</h1>
      <a class="shuffler" @click="prot.setSelections">SHUFFLE!</a>

      <el-affix :offset="20">
        <BaseTransport />
      </el-affix>

      <TrackBin v-for="track in prot.tracks" :track-id="track.id" :key="track.id" />
      <div class="padding"></div>
    </BaseContainer>

    <EffectRack />
  </div>
</template>

<script setup lang="ts">
import Hello from "./components/Hello.vue";
import { onMounted } from "vue";
import EffectRack from "./components/effects/EffectRack.vue";
import BaseContainer from "./components/base/BaseContainer.vue";
import TrackBin from "./components/track/TrackBin.vue";

import { useProteusStore } from "./stores/proteus";
import BaseTransport from "./components/base/BaseTransport.vue";

const prot = useProteusStore();

onMounted(() => {
  prot.addEmptyTrackIfNone();
});
</script>

<style lang="scss">
body {
  margin: 0;
  font-family: "Silkscreen", "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}

.center {
  text-align: center;
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
