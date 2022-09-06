import { Compressor, Distortion, Limiter, Player, Reverb } from "tone";

// interface SelectionMap {
//   ids: [number, number][];
// }

type SelectionMap = [number, number][];

interface ToneTrackPlayer {
  id: number;
  name: string;
  selected: boolean;
  tone: Player;
}

interface ToneTrack {
  id: number;
  name: string;
  players: ToneTrackPlayer[];
}

type Effect = Compressor | Reverb | Distortion | Limiter;

export { SelectionMap, ToneTrackPlayer, ToneTrack, Effect };
