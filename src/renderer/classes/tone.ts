import EventEmitter from 'events'
import { Destination, Gain, getContext, Limiter, loaded, Player } from 'tone'
import { Clock } from './clock'
import { ToneTrack, Effect, SelectionMap, ToneTrackPlayer } from '../typings/tone'
import { Track } from '../typings/tracks'

type Command = 'start' | 'stop' | 'play' | 'clockSync'
type PlayCallback = (currentTime: number, iteration: number) => void

class ToneMaster {
  playing: boolean
  seeking: boolean
  currentTime: number
  currentPlayers: HTMLCollectionOf<HTMLAudioElement> | undefined
  tracks: ToneTrack[]
  effects: Effect[]
  gain: Gain
  lastGain: number
  clock: Clock
  cachedCallback: PlayCallback

  constructor() {
    this.playing = false
    this.seeking = false
    this.currentTime = 0
    this.tracks = []
    this.effects = []
    this.gain = new Gain(1).toDestination()
    this.lastGain = 1
    this.clock = new Clock()
    this.cachedCallback = () => {}
    this.connectEffects()
  }

  private _initTonePlayer(player: ToneTrackPlayer) {
    player.tone.connect(this.gain)
    player.tone.mute = !player.selected
    player.tone.sync()
  }

  private _allPLayers(): ToneTrackPlayer[] {
    const players: ToneTrackPlayer[] = []
    this.tracks.forEach((track) => {
      players.push(...track.players)
    })
    return players
  }

  private _executeOnAllPLayers(command: Command): void {
    this._allPLayers().forEach((player) => {
      if (command === 'stop' || command === 'start') player.tone[command]()
      if (command === 'play') player.tone.start()
      if (command === 'play' || command === 'clockSync') player.tone.seek(this.clock.seconds)
    })
  }

  get context(): AudioContext {
    return getContext().rawContext as AudioContext
  }

  get duration(): number {
    let duration = 0
    this._allPLayers().forEach((player) => {
      const d = player.tone.buffer.duration
      if (d > duration) duration = d
    })
    return duration
  }

  get volume() {
    return this.gain.gain.value
  }

  setGain(gain: number) {
    this.gain.gain.value = gain
  }

  async rampGain(gain: number, time?: number): Promise<void> {
    const milli = time || 100
    const waitForTheRamp: Promise<void> = new Promise((resolve) => {
      this.gain.gain.rampTo(gain, milli / 1000)
      setTimeout(() => {
        resolve()
      }, milli)
    })
    await waitForTheRamp
  }

  async initPeaks(eventEmitter: EventEmitter, logEmitter?: boolean) {
    if (logEmitter) console.log(eventEmitter)
  }

  clear() {
    this.tracks = []
    this.effects = []
    this.playing = false
    this.currentTime = 0
    this.connectEffects()
  }

  async seek(time: number) {
    this.seeking = true
    this.clock.seek(time)
    if (this.playing) {
      await this.pause()
      await this.play()
    }
    this.seeking = false
  }

  trackFromId(trackId: number): ToneTrack | undefined {
    return this.tracks.find((track) => track.id === trackId)
  }

  playerFromIds(trackId: number, playerId: number): Player | undefined {
    const track = this.trackFromId(trackId)
    if (!track) return
    for (let i = 0; i < track.players.length; i++) {
      const player = track.players[i]
      if (playerId === player.id) return player.tone
    }
  }

  setSelections(selections: SelectionMap) {
    selections.forEach((s) => {
      this.setTrackSelection(s[0], s[1])
    })
  }

  setTrackSelection(trackId: number, selection: number) {
    const track = this.trackFromId(trackId)
    if (track) {
      track.players.forEach((player) => {
        player.selected = player.id === selection
        if (player.selected) player.tone.mute = false
        else player.tone.mute = true
      })
    }
  }

  addTrack(track: ToneTrack) {
    track.players.forEach((player) => {
      this._initTonePlayer(player)
    })
    this.tracks.push(track)
  }

  addToneTrackFromTrack(track: Track) {
    const players: ToneTrackPlayer[] = []
    track.files.forEach((f) => {
      players.push({
        id: f.id,
        name: f.name,
        selected: false,
        tone: new Player(`file://${f.path}`),
      })
    })

    this.addTrack({ id: track.id, name: track.name, players })
  }

  addPlayer(trackId: number, player: ToneTrackPlayer) {
    const track = this.trackFromId(trackId)
    if (!track) return

    this._initTonePlayer(player)
    const trackIndex = track.players.findIndex((p) => p.id === player.id)
    trackIndex === -1 ? track.players.push(player) : (track.players[trackIndex] = player)
  }

  removeEffect(index: number) {
    if (index !== -1) this.effects.splice(index, 1)
    this.connectEffects()
  }

  getEffect(effectName: string): Effect | void {
    const index = this.effects.findIndex((effect) => effect.name === effectName)
    if (index !== -1) return this.effects[index]
  }

  async addEffect(effect: Effect) {
    this.effects.push(effect)
    this.connectEffects()
  }

  connectEffects() {
    Destination.chain(...this.effects, new Limiter(-5))
  }

  async play(callback?: PlayCallback) {
    if (this.playing) return
    this.playing = true

    if (callback) this.cachedCallback = callback
    const update = (iteration?: number) => {
      const i = iteration || 0
      this.cachedCallback(this.clock.seconds, i)
      if (this.clock.seconds >= this.duration) {
        this.cachedCallback(0, i)
        this.stop()
      }
      if (this.playing === true)
        setTimeout(() => {
          update(i + 1)
        }, 50)
    }

    await loaded()

    this._executeOnAllPLayers('play')
    this.clock.play()
    update()

    this.gain.gain.value = 0
    await this.rampGain(this.lastGain)
  }

  async pause() {
    this.playing = false
    this.clock.pause()

    this.lastGain = this.gain.gain.value
    await this.rampGain(0)
    this._executeOnAllPLayers('stop')
  }

  async stop() {
    this.playing = false
    this.clock.stop()
    this.cachedCallback = () => {}

    this.lastGain = this.gain.gain.value
    await this.rampGain(0)
  }
}

export default ToneMaster
