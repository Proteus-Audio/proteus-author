<template>
  <button id="saveAsButton" class="hiddenButton" @click="save"></button>
</template>

<script setup lang="ts">
// import { ipcRenderer } from '../../electron.js'
const save = () => {
  // const tracks = track.tracks.map((t) => ({
  //   id: t.id,
  //   name: t.name,
  //   files: t.files.map((f) => ({ id: f.id, path: f.path, name: f.name })),
  // }))
  // console.log(head, tracks)
  // const update: ProjectSkeleton = await ipcRenderer.invoke('save', { tracks })
  // head.load(update)
}
</script>
