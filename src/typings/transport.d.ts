import type PlayMaster from './playMaster.ts'

export interface Transport {
  playing: boolean
  currentTime: number
  master: PlayMaster
}
