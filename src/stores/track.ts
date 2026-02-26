import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { ProjectSkeleton } from '../typings/proteus'
import type { DropFileSkeleton, Track, TrackFile } from '../typings/tracks'
import { startupMark } from '../utils/startup-trace'
import { useAudioStore } from './audio'
import { useHeadStore } from './head'

export const useTrackStore = defineStore('track', () => {
  const audio = useAudioStore()
  const head = useHeadStore()
  const mixSyncTimers = new Map<number, ReturnType<typeof setTimeout>>()
  const maxTrackLevel = 10 ** (10 / 20)

  /////////////
  //  STORE  //
  /////////////

  const tracks = ref([] as Track[])
  const files = ref([] as DropFileSkeleton[])
  const possibleCombinations = ref('0')
  const startupSyncTraced = ref(false)

  const sampleOne = <T>(items: T[]): T | undefined => {
    if (items.length === 0) return undefined
    const index = Math.floor(Math.random() * items.length)
    return items[index]
  }

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
    return (
      getTrackFromId(trackId) ||
      addTrack({ id: nextTrackId.value, name: '', file_ids: [], level: 1, pan: 0 })
    )
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
    if (typeof track.level !== 'number') track.level = 1
    if (typeof track.pan !== 'number') track.pan = 0
    if (!track.shuffle_points) track.shuffle_points = []

    tracks.value.push(track)

    void audio.setDuration()
    void head.logChanges()
    return track
  }

  const addEmptyTrackIfNone = () => {
    if (!emptyTrackExists.value) {
      addTrack({ id: nextTrackId.value, name: '', file_ids: [], level: 1, pan: 0 })
    }
  }

  const removeTrack = async (trackId: number) => {
    const index = tracks.value.findIndex((t) => t.id === trackId)
    if (index === -1) return false

    const existing = mixSyncTimers.get(trackId)
    if (existing) {
      clearTimeout(existing)
      mixSyncTimers.delete(trackId)
    }

    tracks.value.splice(index, 1)
    addEmptyTrackIfNone()

    await refreshPossibleCombinations()
    await head.logChanges()
    void audio.setDuration()
    return true
  }

  const clampLevel = (level: number) => {
    if (!Number.isFinite(level)) return 1
    return Math.min(maxTrackLevel, Math.max(0, level))
  }

  const clampPan = (pan: number) => {
    if (!Number.isFinite(pan)) return 0
    return Math.min(1, Math.max(-1, pan))
  }

  const scheduleTrackMixSync = (trackId: number) => {
    const existing = mixSyncTimers.get(trackId)
    if (existing) {
      clearTimeout(existing)
    }

    const timer = setTimeout(() => {
      const track = getTrackFromId(trackId)
      if (!track) return
      void invoke('set_track_mix', {
        trackId,
        level: track.level ?? 1,
        pan: track.pan ?? 0,
      })
      mixSyncTimers.delete(trackId)
    }, 120)

    mixSyncTimers.set(trackId, timer)
  }

  const setTrackLevel = (trackId: number, level: number) => {
    const track = getTrackFromId(trackId)
    if (!track) return 1
    const next = clampLevel(level)
    const current = track.level ?? 1
    if (Math.abs(next - current) < 0.0001) return current
    track.level = next
    void head.logChanges()
    scheduleTrackMixSync(trackId)
    return next
  }

  const setTrackPan = (trackId: number, pan: number) => {
    const track = getTrackFromId(trackId)
    if (!track) return 0
    const next = clampPan(pan)
    const current = track.pan ?? 0
    if (Math.abs(next - current) < 0.0001) return current
    track.pan = next
    void head.logChanges()
    scheduleTrackMixSync(trackId)
    return next
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
    const selection = sampleOne(options)
    tracks.value[index].selection = selection
    return selection
  }

  const addFileToTrackBinary = (files: DropFileSkeleton | DropFileSkeleton[], trackId: number) => {
    const index = tracks.value.findIndex((v) => v.id === trackId)
    if (!Array.isArray(files)) files = [files]

    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      const trackFile: TrackFile = {
        ...file,
        parentId: trackId,
      }
      tracks.value[index].file_ids.push(trackFile.id)
    }

    void head.logChanges()
    void refreshPossibleCombinations()
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
    void refreshPossibleCombinations()
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
    await refreshPossibleCombinations()
  }

  const removeShufflePoint = async (trackId: number, seconds: number, toleranceSeconds: number) => {
    const index = tracks.value.findIndex((track) => track.id === trackId)
    if (index === -1) return

    const shufflePoints = await invoke<string[]>('remove_shuffle_point', {
      trackId,
      seconds,
      toleranceSeconds,
    })

    tracks.value[index].shuffle_points = shufflePoints
    void head.logChanges()
    await refreshPossibleCombinations()
  }

  const refreshPossibleCombinations = async () => {
    const count = await invoke<string | null>('get_possible_combinations')
    possibleCombinations.value = count || 'overflow'
  }

  const sync = async () => {
    const shouldTraceStartup = !startupSyncTraced.value
    if (shouldTraceStartup) startupMark('track.sync:start')

    const projectState = await invoke<ProjectSkeleton>('get_project_state')
    if (shouldTraceStartup) startupMark('track.sync:after-get-project-state')
    console.log(projectState)

    files.value = projectState.files
    tracks.value = projectState.tracks.map((track) => ({
      ...track,
      level: track.level ?? 1,
      pan: track.pan ?? 0,
      shuffle_points: track.shuffle_points || [],
    }))

    addEmptyTrackIfNone()
    await refreshPossibleCombinations()
    if (shouldTraceStartup) {
      startupMark('track.sync:after-refresh-possible-combinations')
      startupSyncTraced.value = true
    }
  }

  return {
    tracks,
    files,
    possibleCombinations,
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
    removeTrack,
    addFileToTrackBinary,
    shuffle,
    shuffleTrackBin,
    setTrackSelection,
    setTrackLevel,
    setTrackPan,
    addShufflePoint,
    removeShufflePoint,
    removeFileFromTrack,
    refreshPossibleCombinations,
    sync,
  }
})
