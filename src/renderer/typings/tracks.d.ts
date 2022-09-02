interface Track {
  id: number;
  name: string;
  selection?: number;
  files: TrackFileSkeleton[];
}

interface TrackFileSkeleton {
  id: number;
  parentId: number;
  name: string;
  path: string;
}

interface TrackFile extends File {
  id: number;
  parentId: number;
}

export { Track, TrackFile, TrackFileSkeleton };
