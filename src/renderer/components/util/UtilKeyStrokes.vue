<template>
  <div>KEY STROKES</div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { useAudioStore } from "../../stores/audio";
import { useTrackStore } from '../../stores/tracks';

const audio = useAudioStore();
const track = useTrackStore();

const keyListener = (e: KeyboardEvent) => {
  if ((e.target as HTMLElement).localName === "body") {
    if (e.key === " ") {
      e.preventDefault();
      if (!audio.isPlaying && !track.initialised) {
        window.dispatchEvent(new Event("resize"));
        track.initialised = true;
      }
      audio.playPause();
    }
  }
};

onMounted(() => {
  window.addEventListener("keydown", keyListener);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", keyListener);
});
</script>
