<template>
  <button id="loadButton" class="hiddenButton" @click="load"></button>
</template>

<script setup lang="ts">
// import { ipcRenderer } from '../../electron.js'
import { useHeadStore } from '../../stores/head.js'
// import { ProjectSkeleton } from '../../typings/proteus.js'

const head = useHeadStore()

const load = () => {
  // const update: ProjectSkeleton = await ipcRenderer.invoke('load')
  // head.load(update)
  console.log(head)
}
</script>
