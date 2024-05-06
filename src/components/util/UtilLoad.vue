<template>
  <button id="loadButton" class="hiddenButton" @click="load"></button>
</template>

<script setup lang="ts">
// import { ipcRenderer } from '../../electron'
import { useHeadStore } from '../../stores/head'
// import { ProjectSkeleton } from '../../typings/proteus'

const head = useHeadStore()

const load = async () => {
  // const update: ProjectSkeleton = await ipcRenderer.invoke('load')
  // head.load(update)
  console.log(head)
}
</script>
