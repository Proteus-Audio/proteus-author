import { Compressor, Distortion, Limiter, Player, Reverb } from 'tone'

// interface SelectionMap {
//   ids: [number, number][];
// }

type SelectionMap = [number, string | undefined][]

interface ToneTrackPlayer {
  id: number
  name: string
  selected: boolean
  tone: Player
}

interface ToneTrack {
  id: number
  name: string
  players: ToneTrackPlayer[]
}

type Effect = Compressor | Reverb | Distortion | Limiter

interface ExposedBuffer {
  _buffer: AudioBuffer
}

export { SelectionMap, ToneTrackPlayer, ToneTrack, Effect, ExposedBuffer }
