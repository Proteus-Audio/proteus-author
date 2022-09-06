import { defineStore } from "pinia";
import { Track, TrackFile, TrackFileSkeleton } from "../typings/tracks";
import { sample, assignIn } from "lodash";
import { computed, ref } from "vue";
import { TrackSkeleton } from "../typings/proteus";
import { useAudioStore } from "./audio";
import { SelectionMap, ToneTrackPlayer } from "../typings/tone.d";
import { Player } from "tone";

export const useTrackStore = defineStore("track", () => {
  const audio = useAudioStore();

  /////////////
  //  STORE  //
  /////////////

  const tracks = ref([] as Track[]);

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
    audio.master.clear();

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
          selected: false,
          tone: new Player(`file://${f.path}`),
        });
      }
      
      audio.master.addTrack({id: track.id, name: track.name, players});
      buildTracks.push(track);
    }

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

    audio.master.addToneTrackFromTrack(track);
    tracks.value.push(track);
    return track;
  }

  const addEmptyTrackIfNone = () => {
    if (!emptyTrackExists.value) {
      addTrack({ id: nextTrackId.value, name: "", files: [] });
    }
  };

  function setSelections() {
    const selectionMap:SelectionMap = [];
    tracks.value.forEach((track, i) => {
      selectionMap.push([track.id, setTrackSelection(track.id, i)]);
    });
  }

  function getTrackSelection(trackId: number): TrackFileSkeleton | undefined {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    const selectionId = tracks.value[index].selection;
    return tracks.value[index].files.find((file) => file.id === selectionId);
  }

  function setTrackSelection(trackId: number, index?: number):number {
    index = index || tracks.value.findIndex((v) => v.id === trackId);
    const options = tracks.value[index].files.map((f) => f.id);
    const selection = sample(options);
    tracks.value[index].selection = selection;
    return selection || -1;
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
        console.log(tracks.value[index]);
        setTrackSelection(tracks.value[index].id, index);
        console.log(tracks.value[index]);
      }
    });
  }

  return {
    tracks,
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
    setSelections,
    getTrackSelection,
    setTrackSelection,
    removeFileFromTrack,
  };
});
