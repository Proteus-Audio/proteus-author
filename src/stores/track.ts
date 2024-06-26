import { defineStore } from 'pinia'
import { DropFile, DropFileSkeleton, Track, TrackFile, TrackFileSkeleton } from '../typings/tracks'
import { sample, assignIn } from 'lodash'
import { computed, ref } from 'vue'
import { ProjectSkeleton, TrackSkeleton } from '../typings/proteus'
import { useAudioStore } from './audio'
import { SelectionMap } from '../typings/tone'
import { toneMaster } from '../assets/toneMaster'
import { useHeadStore } from './head'
import { invoke } from '@tauri-apps/api'

export const useTrackStore = defineStore('track', () => {
  const audio = useAudioStore()
  const head = useHeadStore()

  /////////////
  //  STORE  //
  /////////////

  const tracks = ref([] as Track[])
  const files = ref([] as DropFileSkeleton[])
  const initialised = ref(true)

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
    if (track) {
      track.name = name
      head.logChanges()
    }

    return name
  }

  function clearTracks(): void {
    tracks.value = []
  }

  function getFileFromId(fileId: string): DropFileSkeleton | undefined {
    return files.value.find((file) => file.id === fileId)
  }

  async function replaceTracksFromLoad(trackSkeletons: TrackSkeleton[]) {
    const buildTracks: Track[] = []
    toneMaster.clear()

    for (let i = 0; i < trackSkeletons.length; i++) {
      const skeleton = trackSkeletons[i]
      const track: Track = { id: skeleton.id, name: skeleton.name, file_ids: [] }

      // const players: ToneTrackPlayer[] = []
      // for (let j = 0; j < skeleton.files.length; j++) {
      //   const f = skeleton.files[j]
      //   track.files.push({ ...f, parentId: track.id })

      //   const fileSrc = convertFileSrc(f.path)

      //   const buffer = await getAudioBuffer(fileSrc)

      //   players.push({
      //     id: f.id,
      //     name: f.name,
      //     selected: f.id === track.selection,
      //     tone: new Player(buffer),
      //   })
      // }

      // toneMaster.addTrack({ id: track.id, name: track.name, players })
      buildTracks.push(track)
    }

    audio.setDuration()
    tracks.value = buildTracks
  }

  // function nextFileId(track: number | Track): number {
  //   const files = typeof track === 'number' ? getTrackFromId(track)?.files : track.files
  //   let highest = 0
  //   ;(files || []).forEach((file) => {
  //     if (file.id > highest) highest = file.id
  //   })
  //   return highest + 1
  // }

  function addTrack(track: Track): Track {
    if (tracks.value.some((t) => t.id === track.id)) {
      track.id = nextTrackId.value
    }

    toneMaster.addToneTrackFromTrack(track)
    tracks.value.push(track)

    audio.setDuration()
    head.logChanges()
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
    sync()
  }

  const shuffleTrackBin = async (trackId: number, index?: number) => {
    const playing = audio.isPlaying
    if (playing) await audio.pause()
    const selection = setTrackSelection(trackId, index) || ''
    toneMaster.setTrackSelection(trackId, selection)
    if (playing) await audio.play()
  }

  const setSelections = () => {
    const selectionMap: SelectionMap = []
    tracks.value.forEach((track, i) => {
      selectionMap.push([track.id, setTrackSelection(track.id, i)])
    })
    toneMaster.setSelections(selectionMap)
  }

  const getTrackSelection = (trackId: number): string | undefined => {
    const index = tracks.value.findIndex((v) => v.id === trackId)
    const selectionId = tracks.value[index].selection
    return tracks.value[index].file_ids.find((id) => id === selectionId)
  }

  const setTrackSelection = (trackId: number, index?: number): string | undefined => {
    index = index || tracks.value.findIndex((v) => v.id === trackId)
    const options = tracks.value[index].file_ids.map((id) => id)
    const selection = sample(options)
    tracks.value[index].selection = selection
    return selection
  }

  const addFileToTrack = (files: DropFile | DropFile[], trackId: number) => {
    // const index = tracks.value.findIndex((v) => v.id === trackId)
    // if (!Array.isArray(files)) files = [files]
    // files.forEach((file) => {
    //   const trackFile: TrackFile = assignIn(file, {
    //     id: nextFileId(tracks.value[index]),
    //     parentId: trackId,
    //   })
    //   tracks.value[index].files.push(trackFile)
    //   toneMaster.addPlayer(trackId, {
    //     id: trackFile.id,
    //     selected: false,
    //     name: trackFile.name,
    //     tone: new Player(`file://${trackFile.path}`),
    //   })
    // })
  }

  const addFileToTrackBinary = async (
    files: DropFileSkeleton | DropFileSkeleton[],
    trackId: number,
  ) => {
    const index = tracks.value.findIndex((v) => v.id === trackId)
    if (!Array.isArray(files)) files = [files]

    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      const trackFile: TrackFile = assignIn(file, {
        parentId: trackId,
      })
      // const audioBuffer = await context.decodeAudioData(file.data.buffer)
      // console.log(audioBuffer, trackFile.id)
      // toneMaster.addPlayer(trackId, {
      //   id: trackFile.id,
      //   selected: false,
      //   name: trackFile.name,
      //   tone: new Player(audioBuffer),
      // })
      tracks.value[index].file_ids.push(trackFile)
    }

    head.logChanges()
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

    head.logChanges()
  }

  const sync = async () => {
    const projectState = (await invoke('get_project_state')) as ProjectSkeleton
    console.log(projectState)

    files.value = projectState.files
    tracks.value = projectState.tracks

    addEmptyTrackIfNone()
  }

  return {
    tracks,
    files,
    initialised,
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
    replaceTracksFromLoad,
    addTrack,
    addEmptyTrackIfNone,
    addFileToTrack,
    addFileToTrackBinary,
    shuffle,
    shuffleTrackBin,
    setSelections,
    getTrackSelection,
    setTrackSelection,
    removeFileFromTrack,
    sync,
  }
})
