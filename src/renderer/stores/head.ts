import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { ProjectHead, ProjectSkeleton } from '../typings/proteus'
import { useAudioStore } from './audio'
import { useTrackStore } from './tracks'

export const useHeadStore = defineStore('head', () => {
  const track = useTrackStore()
  const audio = useAudioStore()

  /////////////
  //  STORE  //
  /////////////

  const head = ref({ name: 'untitled', path: '' } as ProjectHead)

  /////////////
  // GETTERS //
  /////////////

  const name = computed(() => head.value.name)
  const path = computed(() => head.value.path)

  /////////////
  // SETTERS //
  /////////////

  const setFileLocation = (location: string) => {
    head.value.name = (location.match(/[^/\\]*\.\w+$/) || ['.jpg'])[0].replace(/\.\w+$/, '')
    head.value.path = location
  }

  const setName = (name: string) => {
    head.value.name = name
  }
  const setPath = (location: string) => {
    head.value.path = location
  }

  const load = (project: ProjectSkeleton) => {
    if (project.tracks.length > 0) {
      !project.location || setFileLocation(project.location)
      track.replaceTracksFromLoad(project.tracks)
      track.setSelections()
      !project.location || setPath(project.location)
      !project.name || setName(project.name)
      if (project.effects.length > 0) audio.replaceEffects(project.effects)
    }
  }

  return {
    name,
    path,
    setFileLocation,
    setName,
    setPath,
    load,
  }
})
