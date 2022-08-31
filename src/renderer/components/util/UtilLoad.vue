<template>
  <button id="loadButton" class="hiddenButton" @click="load"></button>
</template>

<script setup lang="ts">
import { ipcRenderer } from "../../electron";
import { useProteusStore } from "../../stores/proteus";
import { ProjectSkeleton, TrackSkeleton } from '../../typings/proteus';

const prot = useProteusStore();

const load = async () => {
  const update:ProjectSkeleton = await ipcRenderer.invoke("load");
  console.log(update)

  if(update.tracks) {
    !update.location || prot.setFileLocation(update.location);
    prot.replaceTracksFromLoad(update.tracks);
    if(update.location) prot.setFileLocation(update.location);
  }
};
</script>
