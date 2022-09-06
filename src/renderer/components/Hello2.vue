<template>
  <div>
    <div id="visualise"></div>
  </div>
</template>

<script setup lang="ts">
import * as Tone from "tone";
// import p5, {Element} from "p5";
import { onMounted } from "vue";
import { SVG } from "@svgdotjs/svg.js";
import type { PointArrayAlias, PointArray, ArrayXY } from "@svgdotjs/svg.js";
import WaveSurfer from "wavesurfer.js";
import ToneMaster from "../typings/tone";
const path = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_piano1.mp3";
const path2 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_rythmn2.mp3";
const path3 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_clar2.mp3";
const path4 = "file:///Users/innocentsmith/Sites/electron/proteus-author/dev-assets/op_bgclar3.mp3";

onMounted(async () => {
  const toneMaster = new ToneMaster();

  toneMaster.addTrack({
    id: 1,
    name: "piano",
    players: [{ id: 1, selected: true, name: "op_piano1", tone: new Tone.Player(path) }],
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

  const delayedBeats = new Tone.Player(path2);
  // const delay = new Tone.Delay(5);
  // delayedBeats.connect(delay);
  // await Tone.loaded()
  // delayedBeats.start(5);

  toneMaster.addTrack({
    id: 4,
    name: "rythmn",
    players: [{ id: 1, selected: true, name: "op_rythmn2", tone: delayedBeats }],
  });

  await Tone.loaded();
  const reverb = new Tone.Reverb(20);
  reverb.wet.value = 1;
  await reverb.ready;
  const compressor = new Tone.Compressor(-40, 2);
  // toneMaster.addEffect(compressor);
  // toneMaster.addEffect(reverb);

  console.log("hello?");
  toneMaster.play();
  // toneMaster.playOne();

  // Tone.Transport.stop();
  // Tone.Transport.cancel();
  // // Tone.Transport.setLoopPoints(0, 20);

  // // const players = new Tone.Players()
  // console.log(path);
  const sound = new Tone.Player(path);
  await Tone.loaded();
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

  console.log(sound);
  // sound.start();
  // // gain.gain.rampTo(1, 8)
  // // setTimeout(() => {
  // //   gain.gain.rampTo(0, 8)

  // // }, 8000);

  // Tone.Transport.start();

  // console.log(Tone.Transport.state);
  // console.log(Tone.Transport.now());

  // // buffer.connect(sound);

  // const wavesurfer = WaveSurfer.create({
  //   container: "#visualise",
  // });

  // // wavesurfer.load(path);
  // wavesurfer.loadDecodedBuffer((sound.buffer as any)._buffer);

  // wavesurfer.setVolume(0);
  // wavesurfer.play();

  // setTimeout(() => {
  //   gain.gain.rampTo(0, 0.5);
  //   setTimeout(() => {
  //     wavesurfer.pause()
  //     Tone.Transport.pause();
  //     console.log(Tone.Transport.seconds)
  //     setTimeout(() => {
  //       wavesurfer.play();
  //       Tone.Transport.start();
  //       gain.gain.rampTo(0.8, 0.5);
  //     }, 2000);
  //   }, 600);
  // }, 5789);

  // // sound.l
  // wavesurfer.zoom();

  // wavesurfer.on("ready", () => {
  //   wavesurfer.seekTo(0.5);
  // });
});
</script>

<style lang="scss">
#visualise {
  min-height: 200px;
  min-width: 100%;

  .waveform {
    width: 100%;
    height: 100%;
  }
}
</style>
