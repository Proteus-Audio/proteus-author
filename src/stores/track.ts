import { invoke } from '@tauri-apps/api/core'
import { assignIn, sample } from 'lodash'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { ProjectSkeleton } from '../typings/proteus'
import type { DropFileSkeleton, Track, TrackFile } from '../typings/tracks'
import { useAudioStore } from './audio'
import { useHeadStore } from './head'

export const useTrackStore = defineStore('track', () => {
  const audio = useAudioStore()
  const head = useHeadStore()

  /////////////
  //  STORE  //
  /////////////

  const tracks = ref([] as Track[])
  const files = ref([] as DropFileSkeleton[])

  /////////////
  // GETTERS //
  /////////////

  const nextTrackId = computed((): number => {
    let highest = 0
    tracks.value.forEach((track) => {
      if (track.id > highest) highest = track.id
    })
    return highest + 1
  })

  const emptyTrackExists = computed((): boolean =>
    tracks.value.some((t) => t.file_ids.length === 0),
  )
  const trackFilesExists = computed((): boolean => tracks.value.some((t) => t.file_ids.length > 0))
  const selectedTracks = computed((): string[] => {
    const selectedTracks = [] as string[]
    tracks.value.forEach((track) => {
      const selection = track.selection
      track.file_ids.forEach((id) => {
        if (id === selection) selectedTracks.push(id)
      })
    })
    return selectedTracks
  })

  const allTracks = computed((): Track[] => tracks.value)

  /////////////
  // SETTERS //
  /////////////

  function getTrackFromId(trackId: number): Track | undefined {
    return tracks.value.find((v) => v.id === trackId)
  }

  function getTrackIndexFromId(trackId: number): number {
    return tracks.value.findIndex((v) => v.id === trackId)
  }

  function getOrCreateTrackFromId(trackId: number): Track {
    return getTrackFromId(trackId) || addTrack({ id: nextTrackId.value, name: '', file_ids: [] })
  }

  function setTrackName(trackId: number, name: string) {
    const track = getTrackFromId(trackId)
    const oldName = track?.name
    if (oldName === name) return name
    if (track) {
      track.name = name
      void head.logChanges()
    }

    return name
  }

  function clearTracks(): void {
    tracks.value = []
  }

  function getFileFromId(fileId: string): DropFileSkeleton | undefined {
    return files.value.find((file) => file.id === fileId)
  }

  function addTrack(track: Track): Track {
    if (tracks.value.some((t) => t.id === track.id)) {
      track.id = nextTrackId.value
    }

    tracks.value.push(track)

    void audio.setDuration()
    void head.logChanges()
    return track
  }

  const addEmptyTrackIfNone = () => {
    if (!emptyTrackExists.value) {
      addTrack({ id: nextTrackId.value, name: '', file_ids: [] })
    }
  }

  const shuffle = async () => {
    // const now = new Date()
    await invoke('shuffle')
    await sync()
  }

  const shuffleTrackBin = async (trackId: number, index?: number) => {
    const playing = audio.isPlaying
    if (playing) await audio.pause()
    setTrackSelection(trackId, index)
    if (playing) await audio.play()
  }

  const setTrackSelection = (trackId: number, index?: number): string | undefined => {
    index = index || tracks.value.findIndex((v) => v.id === trackId)
    const options = tracks.value[index].file_ids.map((id) => id)
    const selection = sample(options)
    tracks.value[index].selection = selection
    return selection
  }

  const addFileToTrackBinary = (files: DropFileSkeleton | DropFileSkeleton[], trackId: number) => {
    const index = tracks.value.findIndex((v) => v.id === trackId)
    if (!Array.isArray(files)) files = [files]

    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      const trackFile: TrackFile = assignIn(file, {
        parentId: trackId,
      })
      tracks.value[index].file_ids.push(trackFile.id)
    }

    void head.logChanges()
  }

  const removeFileFromTrack = (fileIds: string | string[], trackId: number) => {
    const index = tracks.value.findIndex((v) => v.id === trackId)
    if (!Array.isArray(fileIds)) fileIds = [fileIds]
    fileIds.forEach((id) => {
      const fileIndex = tracks.value[index].file_ids.findIndex((file_id) => file_id === id)
      if (fileIndex !== -1) tracks.value[index].file_ids.splice(fileIndex, 1)
      if (id === tracks.value[index].selection) {
        setTrackSelection(tracks.value[index].id, index)
      }
    })

    void head.logChanges()
  }

  const addShufflePoint = async (trackId: number, seconds: number) => {
    const index = tracks.value.findIndex((track) => track.id === trackId)
    if (index === -1) return

    const shufflePoints = await invoke<string[]>('add_shuffle_point', {
      trackId,
      seconds,
    })

    tracks.value[index].shuffle_points = shufflePoints
    void head.logChanges()
  }

  const sync = async () => {
    const projectState = await invoke<ProjectSkeleton>('get_project_state')
    console.log(projectState)

    files.value = projectState.files
    tracks.value = projectState.tracks

    addEmptyTrackIfNone()
  }

  return {
    tracks,
    files,
    allTracks,
    nextTrackId,
    emptyTrackExists,
    trackFilesExists,
    selectedTracks,
    getTrackFromId,
    getFileFromId,
    getTrackIndexFromId,
    getOrCreateTrackFromId,
    setTrackName,
    clearTracks,
    addTrack,
    addEmptyTrackIfNone,
    addFileToTrackBinary,
    shuffle,
    shuffleTrackBin,
    setTrackSelection,
    addShufflePoint,
    removeFileFromTrack,
    sync,
  }
})
