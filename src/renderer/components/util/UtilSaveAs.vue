<template>
  <button id="saveAsButton" class="hiddenButton" @click="save"></button>
</template>

<script setup lang="ts">
import { ipcRenderer } from '../../electron'
import { useHeadStore } from '../../stores/head'
import { useTrackStore } from '../../stores/tracks'
import { ProjectSkeleton } from '../../typings/proteus'

const track = useTrackStore()
const head = useHeadStore()

const save = async () => {
  const tracks = track.tracks.map((t) => ({
    id: t.id,
    name: t.name,
    files: t.files.map((f) => ({ id: f.id, path: f.path, name: f.name })),
  }))
  const update: ProjectSkeleton = await ipcRenderer.invoke('save', { tracks })

  head.load(update)
}
</script>
