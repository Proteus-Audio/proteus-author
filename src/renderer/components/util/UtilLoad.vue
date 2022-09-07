<template>
  <button id="loadButton" class="hiddenButton" @click="load"></button>
</template>

<script setup lang="ts">
import { ipcRenderer } from "../../electron";
import { useHeadStore } from '../../stores/head';
import { useTrackStore } from '../../stores/tracks';
import { ProjectSkeleton } from "../../typings/proteus";

const head = useHeadStore();
const track = useTrackStore();

const load = async () => {
  const update: ProjectSkeleton = await ipcRenderer.invoke("load");
  console.log(update);

  if (update.tracks) {
    !update.location || head.setFileLocation(update.location);
    track.replaceTracksFromLoad(update.tracks);
    track.setSelections();
    !update.location || head.setPath(update.location);
    !update.name || head.setName(update.name);
    console.log(track);
  }
};
</script>
