Yes. Based on your current code path, this is what happens between window open and UI paint.

  1. Tauri starts and creates the webview window in Rust at src-tauri/src/main.rs:99 and src-tauri/src/
     windows.rs:20.
  2. The webview loads the frontend entry (index.html -> /src/main.ts) from index.html:12.
  3. main.ts imports all global CSS and app code before mount (src/main.ts:2 to src/main.ts:10), then mounts
     Vue at src/main.ts:16.
  4. Until that mount completes, the window is basically just an empty #app container (index.html:11), so you
     see a blank window.
  5. After mount, App.vue renders shell UI (src/App.vue:1 onward), then onMounted runs and registers many
     window listeners + backend calls (src/App.vue:135 to src/App.vue:263).
  6. Additional components fire startup IPC:

  - DigitalFader requests volume (src/components/digital/DigitalFader.vue:98 to src/components/digital/
    DigitalFader.vue:103).
  - trackStore.sync() requests project state and combinations (src/stores/track.ts:191 to src/stores/
    track.ts:200).
  - waveform components request peaks on mount for selected files (src/components/track/TrackWaveform.vue:482,
    src/components/track/TrackWaveform.vue:345).

  Big contributors I see right now:

  - Initial payload is large: dist/assets/index-CoJ6nUbK.js is ~1.3 MB and dist/assets/index--qJOLb0K.css is
    ~475 KB.
  - You load many font files with no font-display (src/assets/fonts.css:2 onward), which can delay text
    appearance.
  - Startup does extra IPC immediately:
      - audio.effects sync is triggered in both App.vue (src/App.vue:89) and audio.ts (src/stores/
        audio.ts:349), so you’re doing duplicate effect-sync work at startup.

  If this delay is mainly in tauri dev, it will also vary because dev mode uses devUrl (src-tauri/
  tauri.conf.json) and depends on Vite startup/warm cache.

  If you want, I can implement a startup trace next (performance marks + Rust timestamps) so you get exact
  timings for:

  - window created
  - JS entry start
  - Vue mounted
  - first animation frame
  - first backend sync done.
