<template>
  <div id="proteus-author">
    <Teleport to="head">
      <title>Proteus Author - {{windowTitle}}</title>
    </Teleport>
    <Util />
    <BaseContainer>
      <Hello2 />
      <BaseAlertBox />
      <h1 class="center">Proteus.play</h1>
      <el-affix :offset="0">
        <BaseTransport />
      </el-affix>

      <TrackBin v-for="track in trackStore.tracks" :track-id="track.id" :key="track.id" />
      <div class="padding"></div>
    </BaseContainer>

    <EffectRack />
  </div>
</template>

<script setup lang="ts">
import Hello from "./components/Hello.vue";
import { computed, onMounted } from "vue";
import EffectRack from "./components/effects/EffectRack.vue";
import BaseContainer from "./components/base/BaseContainer.vue";
import TrackBin from "./components/track/TrackBin.vue";

import { useProteusStore } from "./stores/proteus";
import BaseTransport from "./components/base/BaseTransport.vue";
import BaseAlertBox from "./components/base/BaseAlertBox.vue";
import Util from "./components/util/Util.vue";
import { useHeadStore } from './stores/head';
import { useTrackStore } from './stores/tracks';
import Hello2 from './components/Hello2.vue';

const head = useHeadStore();
const trackStore = useTrackStore();

const windowTitle = computed(() => {
  return head.name;
})

onMounted(() => {
  trackStore.addEmptyTrackIfNone();
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
