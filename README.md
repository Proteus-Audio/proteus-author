Proteus (.prot) file authoring **Vue3** + **Electron** TypeScript based application based on [Deluze's electron-vue-template](https://github.com/Deluze/electron-vue-template). This repo is moving from the same idea as the [multiplay mixer](https://github.com/howardah/multiplay_mixer) flutter application.

# Tauri + Vue 3 + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).

## About

> “It’s possible that our grandchildren will look at us and say ‘You mean people used to listen to the same thing over and over again?’” - Brian Eno

I attended a lecture in 2014 by Dr. Andy Farnell on Procedural Audio who spoke, in part, about the distinction between fixed and performance mediums (ie film vs stage, album vs concert). Making note of the fact that while a theatre performance has a fixed structure and the story envokes a mood, it also adapts itself to the space and time of the specific performance.

Though, undoutably, much of the draw of performance art is owed to community and social connection, I think there’s a case to be made that some of the power of perfomance is in its subtle unpredictability.

While the world of popular cinematic storytelling is, at least in part, beginning to push itself out of a fixed format ([_Black Mirror: Bandersnatch_](https://www.npr.org/2018/12/28/680671691/black-mirror-bandersnatch-makes-you-choose-your-own-adventure)  /  [Neflix’s growing library of interactive content](https://help.netflix.com/en/node/62526)) and the world of video gaming, which has long-touted interactive storytelling, is  [approaching cinematic realism](https://youtu.be/d8B1LNrBpqc), popular recorded music is still very much fixed.

Procedural music itself is not a new thing, the video game and contemporary composition communities have been exploring it for a long while (Steve Reich’s  [_It’s Gonna Rain_](https://www.npr.org/sections/deceptivecadence/2015/01/27/381575433/fifty-years-of-steve-reichs-its-gonna-rain)  was recorded in 1965). But, as of yet, examples of procedural music in the realm of song are sparse.

The, possibly obvious, solution that I would like to explore would be to record a song in such a way that you have some number (say 10) of each individual part (ie, 10 takes of the vocal, 10 of the drums, 10 of the guitar, etc). Then on play back, you choose a random selection of each part. On a simple song with 5 parts (Guitar, Vocals, Drums, Bass, Synth) this would yield 100,000 unique combinations.

Widespread internet accessibility and the popularity of streaming music could make this potentially very achievable.

My first proof of concept of this variable playback format ( [hosted here](https://multiplay-wnabuuzq2q-uc.a.run.app/?ref=ath) ) used [SoX](http://sox.sourceforge.net/)  to simply combine the parts of a short piece into a new random composite file. In early 2021, I started to work on expanding the idea out with two [Flutter](https://flutter.dev/)-based desktop applications (  [here](https://github.com/howardah/multiplay)  &  [here](https://github.com/howardah/multiplay_mixer)  ) which read and write  [Matroska](https://www.matroska.org/index.html)  Audio files. Using a streamable container file format like Matroska, it is possible to hold all the parts in one distinct package and stream different sets together as well has include additional data which can serve as a guide for how to process each part of the recording.

In mid-2022, I decided to replace the flutter applications with an  [ElectronJS](https://www.electronjs.org/)  application ( the repository which you’re currently looking at ) in order to make use of the flexibility of CSS styling and, at the same time, decided to name the project after the Greek sea-god  [Proteus](https://en.wikipedia.org/wiki/Proteus)  who represents mutability and is the root of the adjective ‘protean’.

Shortly after beginning to write the electron application, I realised that the resulting file size and performance of the build was far from ideal for a, relatively, simple application. I did some additional research and found  [Tauri](https://tauri.app/)  which offers nearly everything that I was looking for with electron but with  _significantly_  improved performance. I'm currently transitioning this repo from Electron to Tauri ( Tauri branch [here](https://github.com/howardah/proteus-author/tree/migrate-to-tauri) ).

There's still much to do with the project so, if you would like to follow along, you can keep tabs on this repo its  [issues page](https://github.com/howardah/proteus-author/issues). If you’d like talk about the idea, feel free to give me a shout at [adam.thomas.howard@gmail.com](mailto:adam.thomas.howard@gmail.com)!
