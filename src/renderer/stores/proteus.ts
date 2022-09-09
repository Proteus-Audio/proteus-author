import { defineStore } from "pinia";
import { Track, TrackFile, TrackFileSkeleton } from "../typings/tracks";
import { sample, assignIn } from "lodash";
import { computed, ref } from "vue";
import { Transport } from "../typings/transport";
import PlayMaster from "../typings/playMaster";
import { Alert, AlertType, ProjectHead, TrackSkeleton } from "../typings/proteus";

export const useProteusStore = defineStore("prot", () => {
  /////////////
  //  STORE  //
  /////////////

  const tracks = ref([] as Track[]);
  const transport = ref({ playing: false, currentTime: 0, master: new PlayMaster() } as Transport);
  const alerts = ref([] as Alert[]);
  const head = ref({name: "untitled", path: ""} as ProjectHead);

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

  const isPlaying = computed((): boolean => transport.value.playing);

  /////////////
  // SETTERS //
  /////////////

  const play = () => {
    if (!trackFilesExists.value) {
      addAlert("There are no tracks to play", "warning");
      return;
    }
    transport.value.master.play();
    setPlaying(true);
  };

  const pause = () => {
    transport.value.master.pause();
    setPlaying(false);
  };

  const playPause = () => {
    isPlaying.value ? pause() : play();
  }

  const stop = () => {
    transport.value.master.stop();
    setPlaying(false);
  };

  const addAlert = (contents: string, type?: AlertType) => {
    type = type || "info";
    alerts.value.push({ contents, type, autoClose: type !== "error" });
  };

  const setFileLocation = (location: string) => {
    head.value.name = (location.match(/[^\/\\]*\.\w+$/) || [".jpg"])[0].replace(/\.\w+$/, '');
    head.value.path = location;
  };

  const setPlaying = (playing: boolean): void => {
    transport.value.playing = playing;
  };

  const togglePlaying = (playing: boolean): void => {
    transport.value.playing = !isPlaying.value;
  };

  function getTrackFromId(trackId: number): Track | undefined {
    return tracks.value.find((v) => v.id === trackId);
  }

  function getTrackIndexFromId(trackId:number):number {
    return tracks.value.findIndex((v) => v.id === trackId);
  }

  function getOrCreateTrackFromId(trackId: number): Track {
    return getTrackFromId(trackId) || addTrack({ id: nextTrackId.value, name: "", files: [] });
  }

  function clearTracks():void {
    tracks.value = [];
  }

  async function replaceTracksFromLoad(trackSkeletons: TrackSkeleton[]) {
    const buildTracks: Track[] = [];
    for (let i = 0; i < trackSkeletons.length; i++) {
      const skeleton = trackSkeletons[i];
      const track: Track = { id: skeleton.id, name: skeleton.name, files: [] };
      
      for (let j = 0; j < skeleton.files.length; j++) {
        const f = skeleton.files[j];
        track.files.push({...f, parentId: track.id});
      }
      const tIndex = tracks.value.findIndex(t => t.id === track.id)
      // if(tIndex !== -1) tracks.value[tIndex] = track;
      buildTracks.push(track);
    }
    
    tracks.value = buildTracks;

    // const forDeletion:string[] = [];
    // tracks.value.forEach(track => {
    //   tracks.value = buildTracks;

    // })

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
    tracks.value.push(track);
    return track;
  }

  const addEmptyTrackIfNone = () => {
    if (!emptyTrackExists.value) {
      addTrack({ id: nextTrackId.value, name: "", files: [] });
    }
  };

  function setSelections() {
    tracks.value.forEach((track, i) => {
      setTrackSelection(track.id, i);
    });
  }

  function getTrackSelection(trackId: number): TrackFileSkeleton | undefined {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    const selectionId = tracks.value[index].selection;
    return tracks.value[index].files.find((file) => file.id === selectionId);
  }

  function setTrackSelection(trackId: number, index?: number) {
    index = index || tracks.value.findIndex((v) => v.id === trackId);
    const options = tracks.value[index].files.map((f) => f.id);
    tracks.value[index].selection = sample(options);
  }

  function addFileToTrack(files: File | File[], trackId: number) {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    if (!Array.isArray(files)) files = [files];
    files.forEach((file) => {
      const trackFile: TrackFile = assignIn(file, {
        id: nextFileId(tracks.value[index]),
        parentId: trackId,
      });
      tracks.value[index].files.push(trackFile);
    });
  }

  function removeFileFromTrack(fileIds: number | number[], trackId: number) {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    if (!Array.isArray(fileIds)) fileIds = [fileIds];
    fileIds.forEach((id) => {
      const fileIndex = tracks.value[index].files.findIndex((file) => file.id === id);
      if (fileIndex !== -1) tracks.value[index].files.splice(fileIndex, 1);
      if (fileIndex === tracks.value[index].selection) {
        setTrackSelection(tracks.value[index].id, index);
      }
    });
  }

  return {
    tracks,
    transport,
    alerts,
    head,
    isPlaying,
    nextTrackId,
    emptyTrackExists,
    trackFilesExists,
    play,
    pause,
    playPause,
    stop,
    addAlert,
    setFileLocation,
    setPlaying,
    togglePlaying,
    getTrackFromId,
    getTrackIndexFromId,
    getOrCreateTrackFromId,
    clearTracks,
    replaceTracksFromLoad,
    nextFileId,
    addTrack,
    addEmptyTrackIfNone,
    addFileToTrack,
    setSelections,
    getTrackSelection,
    setTrackSelection,
    removeFileFromTrack,
  };
});
