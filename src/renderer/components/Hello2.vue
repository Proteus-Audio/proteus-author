<template>
  <div>
    <button v-on:click="playpause">{{ playing ? "Pause" : "Play" }}</button>
    <button v-on:click="stop">Stop</button>
    <el-slider v-model="slider" :show-tooltip="false" />
    <div id="visualise1" class="visualise"></div>
    <div id="visualise2" class="visualise"></div>
    <div id="visualise3" class="visualise"></div>
    <div id="visualise4" class="visualise"></div>
  </div>
</template>

<script setup lang="ts">
import * as Tone from "tone";
// import p5, {Element} from "p5";
import { computed, onMounted, ref } from "vue";
import Peaks, { PeaksInstance, PeaksOptions } from "peaks.js";
import ToneMaster from "../typings/tone";
import { EventEmitter } from "stream";
import { cloneDeep } from "lodash";
import { cloneAudioBuffer } from "../public/tools";
import { Player } from "tone";
const path1 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_piano1.mp3";
const path2 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_rythmn3.mp3";
const path3 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_clar2.mp3";
const path4 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_bgclar3.mp3";
const toneMaster = new ToneMaster();

const playing = ref(false);
const players: Player[] = [];
const peaksPlayers: PeaksInstance[] = [];

const sliderRef = ref(toneMaster.volume);

const slider = computed({
  get: () => sliderRef.value * 75,
  set: (value: number) => {
    sliderRef.value = value / 75;
    toneMaster.setGain(value / 75);
  },
});

const gain = new Tone.Gain(1).toDestination();

const playpause = () => {
  toneMaster.playing ? toneMaster.pause() : play();
  // playing.value ? pause() : play();
  playing.value = !playing.value;
};

const play = () => {
  toneMaster.play();
  peaksPlayers.forEach((pp) => pp.player.play());
};

const stop = () => {
  toneMaster.stop();
  playing.value = false;
  // transport.stop();
  // players.forEach((player) => {
  //   player.stop();
  // });
};

const initialize = () => {
  players.push(
    new Tone.Player(path1),
    new Tone.Player(path2),
    new Tone.Player(path3),
    new Tone.Player(path4)
  );

  players.forEach((player) => {
    player.connect(gain);
    // player.sync();
    // player.start(2);
  });
};

onMounted(async () => {
  toneMaster.clear();

  toneMaster.addTrack({
    id: 1,
    name: "piano",
    players: [{ id: 1, selected: true, name: "op_piano1", tone: new Tone.Player(path1) }],
  });

  toneMaster.addTrack({
    id: 2,
    name: "clar",
    players: [{ id: 1, selected: true, name: "op_clar2", tone: new Tone.Player(path3) }],
  });

  toneMaster.addTrack({
    id: 3,
    name: "bgclar",
    players: [{ id: 1, selected: true, name: "op_bgclar3", tone: new Tone.Player(path4) }],
  });

  toneMaster.addTrack({
    id: 4,
    name: "rythmn",
    players: [{ id: 1, selected: true, name: "op_rythmn3", tone: new Tone.Player(path2) }],
  });

  await Tone.loaded();
  const reverb = new Tone.Reverb(20);
  reverb.wet.value = 0.2;
  await reverb.ready;
  const compressor = new Tone.Compressor(-40, 2);
  toneMaster.addEffect(compressor);
  toneMaster.addEffect(reverb);

  // ====================================================================================================== //
  // ====================================================================================================== //
  // ====================================================================================================== //
  // ====================================================================================================== //

  // sound.connect(reverb);
  // const buf = sound.buffer.toArray();

  // const gain = new Tone.Gain(0.8);
  // sound.chain(reverb, compressor, gain, Tone.Destination);
  // // sound.start();

  // const analyser = new Tone.Analyser("waveform", 128);
  // const waveform = new Tone.Waveform(128);
  // analyser.smoothing = 0.5;
  // sound.connect(analyser);
  // sound.connect(waveform);
  // sound.connect(compressor);
  // sound.sync();

  // const options = {
  //   overview: {
  //     container: document.getElementById('visualise')
  //   },
  //   mediaElement: document.querySelector('audio'),
  //   webAudio: {
  //     AudioBuffer: (toneMaster.tracks[0].players[0].tone.buffer as any)._buffer
  //   }
  // };

  // Peaks.init(options as PeaksOptions, function(err, peaks) {
  //   if (err) {
  //     console.error('Failed to initialize Peaks instance: ' + err.message);
  //     return;
  //   }

  //   // Do something when the waveform is displayed and ready
  // });

  // ====================================================================================================== //
  // ====================================================================================================== //
  // ====================================================================================================== //
  // ====================================================================================================== //

  class PeaksPlayer {
    init: (eventEmitter: EventEmitter) => Promise<void>;
    destroy: () => void;
    play: () => void;
    pause: () => void;
    seek: (time: number) => void;
    isPlaying: () => boolean;
    isSeeking: () => boolean;
    getCurrentTime: () => number;
    getDuration: () => number;

    constructor() {
      this.init = async (eventEmitter: EventEmitter) => await toneMaster.initPeaks(eventEmitter);
      this.destroy = () => toneMaster.clear();
      this.play = () => toneMaster.play();
      this.pause = () => toneMaster.pause();
      this.seek = (time: number) => toneMaster.seek(time);
      this.isPlaying = () => toneMaster.playing;
      this.isSeeking = () => toneMaster.seeking;
      this.getCurrentTime = () => toneMaster.clock.seconds;
      this.getDuration = () => toneMaster.duration;
    }
  }

  console.log((toneMaster.tracks[0].players[0].tone.buffer as any)._buffer);
  console.log(cloneDeep((toneMaster.tracks[0].players[0].tone.buffer as any)._buffer));

  const buffers = [
    cloneAudioBuffer((toneMaster.tracks[0].players[0].tone.buffer as any)._buffer),
    cloneAudioBuffer((toneMaster.tracks[1].players[0].tone.buffer as any)._buffer),
    cloneAudioBuffer((toneMaster.tracks[2].players[0].tone.buffer as any)._buffer),
    cloneAudioBuffer((toneMaster.tracks[3].players[0].tone.buffer as any)._buffer),
  ];

  buffers.forEach((buffer, index) => {
    const options = {
      overview: {
        container: document.getElementById("visualise" + (index + 1)),
      },
      player: new PeaksPlayer(),
      webAudio: {
        audioBuffer: buffer,
      },
    };

    Peaks.init(options as PeaksOptions, function (err, peaks) {
      if (err) {
        console.error("Failed to initialize Peaks instance: " + err.message);
        return;
      }

      if (peaks) peaksPlayers.push(peaks);
      // Do something when the waveform is displayed and ready
    });
  });
});
</script>

<style lang="scss">
.visualise {
  min-height: 200px;
  min-width: 100%;

  .waveform {
    width: 100%;
    height: 100%;
  }
}
</style>
