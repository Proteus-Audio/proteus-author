import { defineStore } from "pinia";
import { Track, TrackFile } from "../typings/tracks";
import _ from "lodash";
import { computed, ref } from "vue";

export const useProteusStore = defineStore("prot", () => {
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

  /////////////
  // SETTERS //
  /////////////

  function getTrackFromId(trackId: number): Track | undefined {
    return tracks.value.find((v) => v.id === trackId);
  }

  function getOrCreateTrackFromId(trackId: number): Track {
    return getTrackFromId(trackId) || addTrack({ id: nextTrackId.value, files: [] });
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

  function setSelections() {
    tracks.value.forEach((track, i) => {
      setTrackSelection(track.id, i);
    });
  }

  function getTrackSelection(trackId: number): TrackFile | undefined {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    const selectionId = tracks.value[index].selection;
    return tracks.value[index].files.find((file) => file.id === selectionId);
  }

  function setTrackSelection(trackId: number, index?: number) {
    index = index || tracks.value.findIndex((v) => v.id === trackId);
    const options = tracks.value[index].files.map((f) => f.id);
    tracks.value[index].selection = _.sample(options);
  }

  function addFileToTrack(files: File | File[], trackId: number) {
    const index = tracks.value.findIndex((v) => v.id === trackId);
    if (!Array.isArray(files)) files = [files];
    files.forEach((file) => {
      const trackFile: TrackFile = _.assignIn(file, {
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
      const fileIndex = tracks.value[index].files.findIndex(file => file.id === id);
      if(fileIndex !== -1) tracks.value[index].files.splice(fileIndex, 1);
      if(fileIndex === tracks.value[index].selection) {
        console.log(tracks.value[index])
        setTrackSelection(tracks.value[index].id, index);
        console.log(tracks.value[index])
      }
    });
  }

  return {
    tracks,
    nextTrackId,
    emptyTrackExists,
    getTrackFromId,
    getOrCreateTrackFromId,
    nextFileId,
    addTrack,
    addFileToTrack,
    setSelections,
    getTrackSelection,
    setTrackSelection,
    removeFileFromTrack
  };
});
