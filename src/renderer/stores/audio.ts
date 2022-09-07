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
  const duration = ref(0);
  const zoom = ref({ y: 1, x: 20 });
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

  const setXScale = (x: number): void => {
    zoom.value.x = x;
  };

  const setYScale = (y: number): void => {
    zoom.value.y = pageYOffset;
  };

  const setDuration = async () => {
    await Tone.loaded();
    duration.value = toneMaster.duration;
  }

  return {
    scale,
    zoom,
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
    setPlaying,
    setCurrentTime,
    togglePlaying,
    setDuration
  };
});
