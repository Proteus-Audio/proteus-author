import { defineStore } from "pinia";
import { DropFile, DropFileSkeleton, Track, TrackFile, TrackFileSkeleton } from "../typings/tracks";
import { sample, assignIn } from "lodash";
import { computed, ref } from "vue";
import { TrackSkeleton } from "../typings/proteus";
import { useAudioStore } from "./audio";
import { SelectionMap, ToneTrackPlayer } from "../typings/tone";
import { Player, context } from "tone";
import { toneMaster } from "../assets/toneMaster";

export const useTrackStore = defineStore("track", () => {
  const audio = useAudioStore();

  /////////////
  //  STORE  //
  /////////////

  const tracks = ref([] as Track[]);
  const initialised = ref(true);

  /////////////
  // GETTERS //
  /////////////

  const nextTrackId = computed((): number => {
    let highest = 0;
    tracks.value.forEach((track) => {
      if (track.id > highest) highest = track.id;
    });
    return highest + 1;
  });

  const emptyTrackExists = computed((): boolean => tracks.value.some((t) => t.files.length === 0));
  const trackFilesExists = computed((): boolean => tracks.value.some((t) => t.files.length > 0));
  const selectedTracks = computed((): TrackFileSkeleton[] => {
    const selectedTracks = [] as TrackFileSkeleton[];
    tracks.value.forEach((track) => {
      const selection = track.selection;
      track.files.forEach((file) => {
        if (file.id === selection) selectedTracks.push(file);
      });
    });
    return selectedTracks;
  });

  const allTracks = computed((): Track[] => tracks.value);

  /////////////
  // SETTERS //
  /////////////

  function getTrackFromId(trackId: number): Track | undefined {
    return tracks.value.find((v) => v.id === trackId);
  }

  function getTrackIndexFromId(trackId: number): number {
    return tracks.value.findIndex((v) => v.id === trackId);
  }

  function getOrCreateTrackFromId(trackId: number): Track {
    return getTrackFromId(trackId) || addTrack({ id: nextTrackId.value, name: "", files: [] });
  }

  function clearTracks(): void {
    tracks.value = [];
  }

  async function replaceTracksFromLoad(trackSkeletons: TrackSkeleton[]) {
    const buildTracks: Track[] = [];
    toneMaster.clear();

    for (let i = 0; i < trackSkeletons.length; i++) {
      const skeleton = trackSkeletons[i];
      const track: Track = { id: skeleton.id, name: skeleton.name, files: [] };

      const players: ToneTrackPlayer[] = [];
      for (let j = 0; j < skeleton.files.length; j++) {
        const f = skeleton.files[j];
        track.files.push({ ...f, parentId: track.id });
        players.push({
          id: f.id,
          name: f.name,
          selected: f.id === track.selection,
          tone: new Player(`file://${f.path}`),
        });
      }

      toneMaster.addTrack({ id: track.id, name: track.name, players });
      buildTracks.push(track);
    }

    audio.setDuration();
    tracks.value = buildTracks;
  }

  function nextFileId(track: number | Track): number {
    const files = typeof track === "number" ? getTrackFromId(track)?.files : track.files;
    let highest = 0;
    (files || []).forEach((file) => {
      if (file.id > highest) highest = file.id;
    });
    return highest + 1;
  }

  function addTrack(track: Track): Track {
    if (tracks.value.some((t) => t.id === track.id)) {
      track.id = nextTrackId.value;
    }

    toneMaster.addToneTrackFromTrack(track);
    tracks.value.push(track);

    audio.setDuration();
    return track;
  }

  const addEmptyTrackIfNone = () => {
    if (!emptyTrackExists.value) {
      addTrack({ id: nextTrackId.value, name: "", files: [] });
    }
  };

  const shuffle = async () => {
    const playing = audio.isPlaying;
    if (playing) await audio.pause();
    setSelections();
    if (playing) await audio.play();
  };

  const shuffleTrackBin = async (trackId: number, index?: number) => {
    const playing = audio.isPlaying;
    if (playing) await audio.pause();
    const selection = setTrackSelection(trackId, index);
    toneMaster.setTrackSelection(trackId, selection);
    if (playing) await audio.play();
  };

  const setSelections = () => {
    const selectionMap: SelectionMap = [];
    tracks.value.forEach((track, i) => {
      selectionMap.push([track.id, setTrackSelection(track.id, i)]);
    });
    toneMaster.setSelections(selectionMap);
  };

  const getTrackSelection = (trackId: number): TrackFileSkeleton | undefined => {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    const selectionId = tracks.value[index].selection;
    return tracks.value[index].files.find((file) => file.id === selectionId);
  };

  const setTrackSelection = (trackId: number, index?: number): number => {
    index = index || tracks.value.findIndex((v) => v.id === trackId);
    const options = tracks.value[index].files.map((f) => f.id);
    const selection = sample(options);
    tracks.value[index].selection = selection;
    return selection || -1;
  };

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
  };

  const addFileToTrackBinary = async (
    files: DropFileSkeleton | DropFileSkeleton[],
    trackId: number
  ) => {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    if (!Array.isArray(files)) files = [files];

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      const trackFile: TrackFile = assignIn(file, {
        id: nextFileId(tracks.value[index]),
        parentId: trackId,
      });
      const audioBuffer = await context.decodeAudioData(file.data.buffer);
      console.log(audioBuffer, trackFile.id);
      toneMaster.addPlayer(trackId, {
        id: trackFile.id,
        selected: false,
        name: trackFile.name,
        tone: new Player(audioBuffer),
      });
      tracks.value[index].files.push(trackFile);
    }
  };

  const removeFileFromTrack = (fileIds: number | number[], trackId: number) => {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    if (!Array.isArray(fileIds)) fileIds = [fileIds];
    fileIds.forEach((id) => {
      const fileIndex = tracks.value[index].files.findIndex((file) => file.id === id);
      if (fileIndex !== -1) tracks.value[index].files.splice(fileIndex, 1);
      if (fileIndex === tracks.value[index].selection) {
        setTrackSelection(tracks.value[index].id, index);
      }
    });
  };

  return {
    tracks,
    initialised,
    allTracks,
    nextTrackId,
    emptyTrackExists,
    trackFilesExists,
    selectedTracks,
    getTrackFromId,
    getTrackIndexFromId,
    getOrCreateTrackFromId,
    clearTracks,
    replaceTracksFromLoad,
    nextFileId,
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
  };
});
