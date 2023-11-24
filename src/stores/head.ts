import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { ProjectHead, ProjectSkeleton, TrackSkeleton } from '../typings/proteus'
import { useAudioStore } from './audio'
import { useTrackStore } from './track'
import { invoke } from '@tauri-apps/api/tauri'

export const useHeadStore = defineStore('head', () => {
  const track = useTrackStore()
  const audio = useAudioStore()

  /////////////
  //  STORE  //
  /////////////

  const head = ref({ name: 'untitled', path: undefined } as ProjectHead)

  /////////////
  // GETTERS //
  /////////////

  const name = computed({
    get: () => head.value.name,
    set: (name: string) => {
      head.value.name = name
    },
  })
  const path = computed({
    get: () => head.value.path,
    set: (location: string | undefined) => {
      head.value.path = location
    },
  })

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

  const load = async (project: ProjectSkeleton) => {
    if (project.tracks.length > 0) {
      !project.location || setFileLocation(project.location)
      await track.replaceTracksFromLoad(project.tracks)
      track.setSelections()
      !project.location || setPath(project.location)
      !project.name || setName(project.name)
      if (project.effects.length > 0) audio.replaceEffects(project.effects)
    }
  }

  const projectState = (): ProjectSkeleton => {
    const tracks = track.tracks.map((t) => ({
      id: t.id,
      name: t.name,
      file_ids: t.files.map((f) => f.id),
    })) as TrackSkeleton[]

    const project = {
      name: head.value.name,
      location: head.value.path,
      tracks: tracks,
      effects: audio.effects,
      files: [],
    } as ProjectSkeleton

    return project
  }

  const logChanges = async (): Promise<boolean> => {
    const project = projectState()

    console.log(project)

    return await invoke('project_changes', { newProject: project })
  }

  const save = (): ProjectSkeleton => {
    const project = projectState()

    // invoke('auto_save', { newProject: JSON.stringify(project) })

    return project
  }

  return {
    name,
    path,
    setFileLocation,
    setName,
    setPath,
    load,
    save,
    projectState,
    logChanges,
  }
})
