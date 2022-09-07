import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { useAlertStore } from "./alerts";
import { useTrackStore } from "./tracks";
import * as Tone from "tone";
import { toneMaster } from "../public/toneMaster";

export const useAudioStore = defineStore("prot", () => {
  const alert = useAlertStore();
  const track = useTrackStore();

  /////////////
  //  STORE  //
  /////////////

  const playing = ref(false);
  const currentTime = ref(0);
  const scale = ref(20 as number);
  const zoom = ref(1 as number);
  const duration = ref(0);
  const znS = ref({ zoom: 1, scale: 20 });
  //   const group = ref(new Pizzicato.Group());

  /////////////
  // GETTERS //
  /////////////

  const isPlaying = computed((): boolean => playing.value);
  const watch = computed(() => ({ playing: playing.value, zoom: zoom.value, scale: scale.value }));
  const getCurrentTime = computed((): number => currentTime.value);
  const getScale = computed((): number => znS.value.scale);
  const context = computed((): AudioContext => toneMaster.context);
  const audioContext = computed((): AudioContext => context.value);

  /////////////
  // SETTERS //
  /////////////

  const setCurrentTime = (time: number): void => {
    currentTime.value = time;
  };

  const play = () => {
    if (!track.trackFilesExists) {
      alert.addAlert("There are no tracks to play", "warning");
      return;
    }

    toneMaster.play((time: number, i?: number) => {
      if (time === 0 && i !== 0) stop();
      else currentTime.value = time;
    });
    setPlaying(true);
  };

  const pause = () => {
    toneMaster.pause();
    setPlaying(false);
  };

  const playPause = () => {
    isPlaying.value ? pause() : play();
  };

  const stop = () => {
    toneMaster.stop();
    currentTime.value = 0;
    setPlaying(false);
  };

  const setPlaying = (playingVal: boolean): void => {
    playing.value = playingVal;
  };

  const togglePlaying = (): void => {
    playing.value = !playing.value;
  };

  const setScale = (newScale: number): void => {
    znS.value.scale = newScale;
  };

  const setZoom = (newZoom: number): void => {
    znS.value.zoom = newZoom;
  };

  const setDuration = async () => {
    await Tone.loaded();
    duration.value = toneMaster.duration;
  }

  return {
    scale,
    zoom,
    znS,
    duration,
    watch,
    isPlaying,
    getCurrentTime,
    getScale,
    audioContext,
    play,
    pause,
    playPause,
    stop,
    setScale,
    setZoom,
    setPlaying,
    setCurrentTime,
    togglePlaying,
    setDuration
  };
});
