<template>
  <button id="saveButton" class="hiddenButton" @click="save"></button>
</template>

<script setup lang="ts">
import { cloneDeep } from 'lodash'
import { ipcRenderer } from '../../electron'
import { useAudioStore } from '../../stores/audio'
import { useHeadStore } from '../../stores/head'
import { useTrackStore } from '../../stores/tracks'
import { ProjectSkeleton } from '../../typings/proteus'

const head = useHeadStore()
const track = useTrackStore()
const audio = useAudioStore()

const save = async () => {
  const tracks = track.tracks.map((t) => ({
    id: t.id,
    name: t.name,
    files: t.files.map((f) => ({ id: f.id, path: f.path, name: f.name })),
  }))

  const projectToSave: ProjectSkeleton = {
    location: head.path,
    name: head.name,
    tracks,
    effects: cloneDeep(audio.effects),
  }

  console.log(projectToSave)

  const update: ProjectSkeleton = await ipcRenderer.invoke('save', { projectToSave })

  head.load(update)
}
</script>
