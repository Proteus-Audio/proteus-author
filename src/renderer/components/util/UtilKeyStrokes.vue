<template>
  <div>KEY STROKES</div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { useAudioStore } from "../../stores/audio";

const audio = useAudioStore();

const keyListener = (e: KeyboardEvent) => {
  if ((e.target as HTMLElement).localName === "body") {
    if (e.key === " ") {
      e.preventDefault();
      console.log(e);
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
