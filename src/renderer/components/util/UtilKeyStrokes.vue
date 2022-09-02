<template>
  <div>KEY STROKES</div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import { useProteusStore } from "../../stores/proteus";

const prot = useProteusStore();

const keyListener = (e: KeyboardEvent) => {
  if ((e.target as HTMLElement).localName === "body") {
    if (e.key === " ") {
      e.preventDefault();
      console.log(e);
      prot.playPause();
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
