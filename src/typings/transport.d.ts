import PlayMaster from './playMaster'

interface Transport {
  playing: boolean
  currentTime: number
  master: PlayMaster
}

export { Transport }
