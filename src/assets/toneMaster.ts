import type EventEmitter from 'node:events'
import ToneMaster from '../classes/tone'

const toneMaster = new ToneMaster()
class PeaksPlayer {
  init: (eventEmitter: EventEmitter) => Promise<void>
  destroy: () => void
  play: () => Promise<void>
  pause: () => Promise<void>
  seek: (time: number) => Promise<void>
  isPlaying: () => boolean
  isSeeking: () => boolean
  getCurrentTime: () => number
  getDuration: () => number

  constructor() {
    this.init = (eventEmitter: EventEmitter) => Promise.resolve(toneMaster.initPeaks(eventEmitter))
    this.destroy = () => toneMaster.clear()
    this.play = () => toneMaster.play()
    this.pause = () => toneMaster.pause()
    this.seek = (time: number) => toneMaster.seek(time)
    this.isPlaying = () => toneMaster.playing
    this.isSeeking = () => toneMaster.seeking
    this.getCurrentTime = () => toneMaster.seconds
    this.getDuration = () => toneMaster.duration
  }
}

export { toneMaster, PeaksPlayer }
