<template>
  <button id="saveButton" class="hiddenButton" @click="save"></button>
</template>

<script setup lang="ts">
import { ipcRenderer } from "../../electron";
import { useProteusStore } from "../../stores/proteus";
import { ProjectSkeleton } from '../../typings/proteus';

const prot = useProteusStore();

const save = async () => {
  const tracks = prot.tracks.map((t) => ({
    id: t.id,
    name: t.name,
    files: t.files.map((f) => ({ id: f.id, path: f.path, name: f.name })),
  }));
  const update:ProjectSkeleton = await ipcRenderer.invoke("save", { location: prot.head.path, tracks });
  console.log(update)

  if(update.tracks) {
    !update.location || prot.setFileLocation(update.location);
    prot.replaceTracksFromLoad(update.tracks);
  }
};
</script>
