import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { ProjectHead, ProjectSkeleton, TrackSkeleton } from '../typings/proteus'
import { useAudioStore } from './audio'
import { useTrackStore } from './track'

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

  const load = async () => {
    const project = await invoke<ProjectSkeleton>('get_project_state')
    if (project) {
      setFileLocation(project.location || '')
      await track.sync()
      setPath(project.location || '')
      setName(project.name || '')
      audio.replaceEffects(project.effects || [])
      await invoke('init_player')
      await invoke('set_selections')
      await track.sync()
      await audio.setDuration()
    }
  }

  const projectState = (): ProjectSkeleton => {
    const tracks = track.tracks.map((t) => ({
      id: t.id,
      name: t.name,
      selection: t.selection || undefined,
      file_ids: t.file_ids,
      shuffle_points: t.shuffle_points || [],
      level: t.level ?? 1,
      pan: t.pan ?? 0,
    })) as TrackSkeleton[]

    const project = {
      name: head.value.name,
      location: head.value.path,
      tracks: tracks,
      effects: audio.effectsChainForBackend as unknown as ProjectSkeleton['effects'],
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

    void invoke('auto_save', { newProject: JSON.stringify(project) })

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
