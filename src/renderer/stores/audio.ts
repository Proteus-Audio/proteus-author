import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { useAlertStore } from "./alerts";
import { useTrackStore } from "./tracks";
import * as Tone from "tone";
import { toneMaster } from "../public/toneMaster";
import { CompressorSettings, ReverbSettings } from "../typings/effects";

export const useAudioStore = defineStore("prot", () => {
  const alert = useAlertStore();
  const track = useTrackStore();

  /////////////
  //  STORE  //
  /////////////

  const playing = ref(false);
  const currentTime = ref(0);
  const scale = ref(20 as number);
  const duration = ref(0);
  const zoom = ref({ y: 1, x: 20 });
  const reverb = ref({
    active: false,
    decay: 20,
    preDelay: 0,
    mix: 0.2,
    ready: false,
  } as ReverbSettings);
  const compressor = ref({
    active: false,
    threshold: -15,
    attack: 0.2,
    knee: 0,
    ratio: 2,
    release: 0.1,
  } as CompressorSettings);
  //   const group = ref(new Pizzicato.Group());

  /////////////
  // GETTERS //
  /////////////

  const isPlaying = computed((): boolean => playing.value);
  const watch = computed(() => ({ playing: playing.value, zoom: zoom.value, scale: scale.value }));
  const getCurrentTime = computed((): number => currentTime.value);
  const getXScale = computed((): number => zoom.value.x);
  const getYScale = computed((): number => zoom.value.y);
  const context = computed((): AudioContext => toneMaster.context);
  const audioContext = computed((): AudioContext => context.value);

  /////////////
  // SETTERS //
  /////////////

  const setCurrentTime = (time: number): void => {
    currentTime.value = time;
  };

  const play = async () => {
    if (!track.trackFilesExists) {
      alert.addAlert("There are no tracks to play", "warning");
      return;
    }

    setPlaying(true);
    await toneMaster.play((time: number, i?: number) => {
      if (time === 0 && i !== 0) stop();
      else currentTime.value = time;
    });
  };

  const pause = async () => {
    setPlaying(false);
    await toneMaster.pause();
  };

  const playPause = async () => {
    isPlaying.value ? await pause() : await play();
  };

  const stop = async () => {
    await toneMaster.stop();
    currentTime.value = 0;
    setPlaying(false);
  };

  const setPlaying = (playingVal: boolean): void => {
    playing.value = playingVal;
  };

  const togglePlaying = (): void => {
    playing.value = !playing.value;
  };

  const setXScale = (x: number): void => {
    zoom.value.x = x;
  };

  const setYScale = (y: number): void => {
    zoom.value.y = pageYOffset;
  };

  const zoomIn = (axis?: "x" | "y" | "both", degree?: number) => {
    axis = axis || "x";
    const amount = degree ? degree / 100 : 1;
    if (axis === "x" || axis === "both") setXScale(getXScale.value + 5 * amount);
    if (axis === "y" || axis === "both") setYScale(getXScale.value + 1 * amount);
  };

  const zoomOut = (axis?: "x" | "y" | "both", degree?: number) => {
    axis = axis || "x";
    const amount = degree ? degree / 100 : 1;
    if (axis === "x" || axis === "both") setXScale(getXScale.value - 5 * amount);
    if (axis === "y" || axis === "both") setYScale(getXScale.value - 1 * amount);
  };

  const setDuration = async () => {
    await Tone.loaded();
    duration.value = toneMaster.duration;
  };

  return {
    scale,
    zoom,
    reverb,
    compressor,
    duration,
    watch,
    isPlaying,
    getCurrentTime,
    getXScale,
    getYScale,
    audioContext,
    play,
    pause,
    playPause,
    stop,
    setXScale,
    setYScale,
    zoomIn,
    zoomOut,
    setPlaying,
    setCurrentTime,
    togglePlaying,
    setDuration,
  };
});
