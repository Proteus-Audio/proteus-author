import { Compressor, Destination, Distortion, Limiter, loaded, Player, Players, Reverb, Transport } from "tone";
import {ToneTrack, Effect, SelectionMap, ToneTrackPlayer} from "./tone.d";
import { Track } from './tracks';

class ToneMaster {
  playing: boolean;
  currentTime: number;
  currentPlayers: HTMLCollectionOf<HTMLAudioElement> | undefined;
  tracks: ToneTrack[];
  effects: Effect[];

  constructor() {
    Transport.cancel();
    Transport.stop();
    this.playing = false;
    this.currentTime = 0;
    this.tracks = [];
    this.effects = [];
    // this.effects = [new Limiter(-5), new Reverb(20), new Compressor(-100, 20)];
    this.connectEffects();
  }

  _initTonePlayer(tone:Player) {
    tone.volume.value = -20;
    tone.toDestination();
    tone.sync();
  }

  clear() {
    this.tracks = [];
    this.effects = [];
    this.playing = false;
    this.currentTime = 0;
    this.connectEffects();
  }

  trackFromId(trackId: number): ToneTrack | undefined {
    return this.tracks.find((track) => track.id === trackId);
  }

  setSelections(selections: SelectionMap) {
    selections.forEach((s) => {
      const track = this.trackFromId(s[0]);
      if (track)
        track.players.forEach((player) => {
          if (player.id === s[1]) player.tone.mute = false;
          else player.tone.mute = true;
        });
    });
  }

  async startAllPlayers() {
    await loaded();
    this.tracks.forEach(track => {
      track.players.forEach(player => {
        console.log(player.tone.state);
       if(player.tone.state === 'stopped') player.tone.start();
      });
    })
  }

  addTrack(track: ToneTrack) {
    track.players.forEach((player) => {
      this._initTonePlayer(player.tone);
    });
    this.tracks.push(track);
    this.startAllPlayers();
  }

  addToneTrackFromTrack(track: Track) {
    const players:ToneTrackPlayer[] = [];
    track.files.forEach(f => {
      players.push({
        id: f.id,
        name: f.name,
        selected: false,
        tone: new Player(`file://${f.path}`),
      });
    });

    this.addTrack({id: track.id, name: track.name, players})
  }

  addPlayer(trackId: number, player: ToneTrackPlayer) {
    const track = this.trackFromId(trackId);
    if (!track) return;

    this._initTonePlayer(player.tone);
    const trackIndex = track.players.findIndex((p) => p.id === player.id);
    trackIndex === -1 ? track.players.push(player) : (track.players[trackIndex] = player);
    this.startAllPlayers();
  }

  async addEffect(effect: Effect) {
    this.effects.push(effect);
  }

  connectEffects() {
    Destination.chain(...this.effects, new Limiter(-5));
  }

  playOne() {
    const player = this.tracks[0]?.players[0];
    // const player = this.tracks[0]?.players[0]?.tone;
    player.tone.sync();
    console.log(player);
    player.tone.toDestination();
    player.tone.start();
    Transport.start();
  }

  async play() {
    this.connectEffects();
    await loaded();
    console.log(this.tracks);
    Transport.start();
  }

  pause() {
    Transport.pause();
  }

  stop() {
    Transport.stop();
  }
}

export default ToneMaster;
