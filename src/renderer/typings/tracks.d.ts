interface Track {
  id: number;
  selection?: number;
  files: TrackFile[];
}

interface TrackFile extends File {
  id: number;
  parentId: number;
}

export { Track, TrackFile };
