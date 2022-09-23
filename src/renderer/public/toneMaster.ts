import EventEmitter from 'events'
import ToneMaster from '../classes/tone'

const toneMaster = new ToneMaster()
class PeaksPlayer {
  init: (eventEmitter: EventEmitter) => Promise<void>
  destroy: () => void
  play: () => void
  pause: () => void
  seek: (time: number) => void
  isPlaying: () => boolean
  isSeeking: () => boolean
  getCurrentTime: () => number
  getDuration: () => number

  constructor() {
    this.init = async (eventEmitter: EventEmitter) => await toneMaster.initPeaks(eventEmitter)
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
