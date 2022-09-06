import { defineStore } from "pinia";
import { computed, ref } from "vue";
import PlayMaster from "../typings/playMaster";
import { useAlertStore } from "./alerts";
import { useTrackStore } from "./tracks";
import * as Tone from "tone";
import { Player, Players } from "tone";
import ToneMaster from '../typings/tone';

export const useAudioStore = defineStore("prot", () => {
  const alert = useAlertStore();
  const track = useTrackStore();

  /////////////
  //  STORE  //
  /////////////

  const playing = ref(false);
  const currentTime = ref(0);
  const master = ref(new ToneMaster());
  // const master = ref(new PlayMaster());
  const context = ref(new AudioContext());
  const scale = ref(20 as number);
  const zoom = ref(1 as number);
  //   const group = ref(new Pizzicato.Group());

  /////////////
  // GETTERS //
  /////////////

  const isPlaying = computed((): boolean => playing.value);
  const getCurrentTime = computed((): number => currentTime.value);
  const getScale = computed((): number => scale.value);
  const audioContext = computed((): AudioContext => context.value);
  const players = computed((): Player[] => {
    const players = [] as Player[];
    track.selectedTracks.forEach((track) => {
      const player = new Player(`file://${track.path}`).toDestination();
      //   player.sync();
      players.push(player);
    });
    return players;
  });

  const player = computed((): Players => {
    const players = new Tone.Players();
    track.allTracks.forEach((track) => {
      track.files.forEach((file) => {
        players.add(file.name, `file://${file.path}`);
      });
    });
    return players;
  });

  /////////////
  // SETTERS //
  /////////////

  const setCurrentTime = (time: number): void => {
    currentTime.value = time;
  };

  const addFile = async (path: string) => {
    // const reverb = new Tone.Reverb(20).toDestination();
    // reverb.wet.value = 1;
    // await reverb.ready;
    // console.log(path);
    // const sound = new Tone.Player("file://" + path).toDestination();
    // await Tone.loaded();
    // sound.connect(reverb);
    // // const waveform = new Tone.Waveform();
    // // console.log(sound.);
    // console.log(sound.buffer.toArray());
    // sound.start();
  };

  const play = () => {
    if (!track.trackFilesExists) {
      alert.addAlert("There are no tracks to play", "warning");
      return;
    }

    master.value.play();
    setPlaying(true);
  };

  const pause = () => {
    master.value.pause();
    setPlaying(false);
  };

  const playPause = () => {
    isPlaying.value ? pause() : play();
  };

  const stop = () => {
    master.value.stop();
    setPlaying(false);
  };

  const setPlaying = (playingVal: boolean): void => {
    playing.value = playingVal;
  };

  const togglePlaying = (): void => {
    playing.value = !playing.value;
  };

  const refreshContext = (): void => {
    context.value = new AudioContext();
  };

  const setScale = (newScale: number): void => {
    scale.value  = newScale;
  };

  const setZoom = (newZoom: number): void => {
    zoom.value  = newZoom;
  };

  return {
    master,
    scale,
    zoom,
    isPlaying,
    getCurrentTime,
    getScale,
    audioContext,
    player,
    addFile,
    play,
    pause,
    playPause,
    stop,
    setScale,
    setZoom,
    setPlaying,
    setCurrentTime,
    togglePlaying,
    refreshContext,
  };
});
