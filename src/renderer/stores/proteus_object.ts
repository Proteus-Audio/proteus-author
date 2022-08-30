import { defineStore } from "pinia";
import { Track, TrackFile } from "../typings/tracks";
import _ from "lodash";

export const useProteusStore = defineStore("prot", {
  state: () => {
    return { tracks: [] as Track[] };
  },
  // could also be defined as
  // state: () => ({ count: 0 })
  getters: {
    nextTrackId(state): number {
      let highest = 0;
      state.tracks.forEach((track) => {
        if (track.id > highest) highest = track.id;
      });
      return highest + 1;
    },
    emptyTrackExists: (state) => state.tracks.some((t) => t.files.length === 0),
  },
  actions: {
    nextFileId(track: number | Track): number {
      const files = typeof track === "number" ? this.getTrackFromId(track)?.files : track.files;
      let highest = 0;
      (files || []).forEach((file) => {
        if (file.id > highest) highest = file.id;
      });
      return highest + 1;
    },
    getTrackFromId(trackId: number): Track | undefined {
      return this.tracks.find((v) => v.id === trackId);
    },
    getOrCreateTrackFromId(trackId: number): Track {
      return this.getTrackFromId(trackId) || this.addTrack({ id: this.nextTrackId, files: [] });
    },
    addTrack(track: Track): Track {
      if (this.tracks.some((t) => t.id === track.id)) {
        track.id = this.nextTrackId;
      }
      this.tracks.push(track);
      return track;
    },
    addFileToTrack(files: File | File[], trackId: number) {
      const index = this.tracks.findIndex((v) => v.id === trackId);
      if (!Array.isArray(files)) files = [files];
      files.forEach((file) => {
        const trackFile: TrackFile = _.assignIn(file, {
          id: this.nextFileId(this.tracks[index]),
          parentId: trackId,
        });
        this.tracks[index].files.push(trackFile);
      });
    },
  },
});
