<template>
  <button id="saveButton" class="hiddenButton" @click="save"></button>
</template>

<script setup lang="ts">
const save = () => {
  // const tracks = track.tracks.map((t) => ({
  //   id: t.id,
  //   name: t.name,
  //   files: t.files.map((f) => ({ id: f.id, path: f.path, name: f.name, extension: f.extension })),
  // }))
  // const projectToSave: ProjectSkeleton = {
  //   location: head.path,
  //   name: head.name,
  //   tracks,
  //   effects: cloneDeep(audio.effects),
  // }
  // console.log(projectToSave)
  // const update: ProjectSkeleton = await ipcRenderer.invoke('save', projectToSave)
  // head.load(update)
}
</script>
