<template>
  <div class="playhead-holder">
    <!-- <div class="playhead" :style="`left:${leftPos}px`"></div> -->
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useAudioStore } from "../../stores/audio";

const audio = useAudioStore();

const leftPos = computed(() => audio.getCurrentTime * audio.getScale);
</script>

<style lang="scss" scoped>
.playhead-holder {
  width: 100%;
  padding: 0 0.5em;

  .playhead {
    width: 2px;
    height: 128px;
    background: black;
    position: relative;
    z-index: 18;
    // transition: left 0.2s;
  }
}
</style>
