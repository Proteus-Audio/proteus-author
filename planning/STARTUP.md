# Startup Optimization Plan

## Baseline (from latest trace)

Current observed startup timeline:

```text
[startup][rust][app] +158.4ms tauri builder initialized
[startup][rust][window:main-window-1] +323.6ms (Δ165.2ms) webview window built
[startup][rust][app] +323.7ms (Δ0.0ms) main window requested
[startup][rust][player] +2486.1ms (Δ2162.5ms) set_effects_chain called before player init (No player found)
```

Working interpretation:

- Rust + window creation is relatively fast (~324ms).
- The largest gap is after window creation before JS entry starts (~1.68s).
- Most delay is likely frontend boot + parse/eval + initial mount + immediate watchers.

Additional measured milestones from latest run:

- `window built` -> `main.ts:start`: ~1678ms
- `main.ts:start` -> `first-paint-approx`: ~140ms
- `first-paint-approx` -> `App.vue:first-frame-after-mounted-work`: ~55ms
- `main.ts:start` -> `App.vue:first-frame-after-mounted-work`: ~196ms

## Targets

- [ ] Reduce "window built -> first paint" to under 1000ms on a cold start.
- [ ] Reduce "window built -> first backend startup command" to under 1200ms.
- [ ] Ensure only one startup-time effects sync attempt happens.
- [ ] Keep behavior unchanged for project loading/playback.

## Phase 0: Fix Measurement Reliability

- [x] Fix startup trace command payload shape so web marks are logged in Rust timeline.
- [x] Add a distinct `first-paint` mark (`requestAnimationFrame` after mount is acceptable first approximation).
- [x] Capture and store one "before changes" log sample and one "after each phase" sample.
- [x] Confirm logs include both:
  - [x] `[startup][rust] ...`
  - [x] `[startup][web] ...`

Phase 0 status: Complete.

## Phase 1: Remove Known Startup Duplication

- [x] Remove duplicate immediate `audio.effects` sync watcher in `App.vue`.
- [x] Remove immediate startup sync for effects watcher in `audio.ts`.
- [ ] Verify `set_effects_chain` is called once during startup (or zero if no effects/project).
- [x] Gate `set_effects_chain` call when effects are empty and no player exists (avoid unnecessary IPC noise).
- [x] Suppress startup hydration writes from `head.logChanges()` until initial sync completes.
- [ ] Ensure no repeated `new_project` initialization logs during empty startup.

## Phase 2: Improve First Paint (UI-First Boot)

- [ ] Render a tiny shell immediately (title/header/loading placeholder).
- [x] Defer non-critical setup until after first paint:
  - [x] register all non-essential event listeners after first frame
  - [x] `get_play_state` after first frame
  - [x] `trackStore.sync()` after first frame (or split into minimal + deferred portions)
- [x] Keep controls responsive while deferred tasks run.

Notes:

- Startup shell experiment was reverted due UX regression (visible grey flash) without startup time benefit.
- Keep deferral improvements, but use existing full UI as the initial render path.

## Phase 3: Split Initial Frontend Payload

Current build output shows large initial assets:

- JS: ~1.36MB (`dist/assets/index-*.js`)
- CSS: ~486KB (`dist/assets/index-*.css`)

Checklist:

- [ ] Introduce route/component-level code splitting for heavy non-critical UI:
  - [x] waveform-heavy components
  - [x] effects dialog/tooling
- [x] Resolve the `@tauri-apps/plugin-dialog` static+dynamic import conflict:
  - [x] keep it dynamic-only where possible
  - [x] avoid static import path that forces it into main chunk
- [x] Add Vite manual chunking for large vendor groups (`element-plus`, draggable, etc.).
- [x] Re-run build and record new chunk sizes.
- [ ] Target: reduce main startup JS chunk by at least 35%.

Migration note:

- Planned UI library migration away from `element-plus` is expected.
- Phase 3 changes are intentionally migration-safe (chunk strategy + lazy loading) and should remain useful after migration.

Latest build snapshot (after migration-safe Phase 3 pass):

- `index-*.js`: 41.95kB (gzip 15.25kB)
- `vendor-element-plus-*.js`: 761.95kB (gzip 241.30kB)
- `vendor-vue-*.js`: 202.35kB (gzip 76.45kB)
- `vendor-misc-*.js`: 171.75kB (gzip 60.56kB)
- `index-*.css`: 485.38kB (gzip 63.90kB)
- `EffectsDialog-*.js` (lazy): 16.81kB

Latest build snapshot (after waveform/track lazy loading):

- `index-*.js`: 30.79kB (gzip 11.29kB)
- `TrackBin-*.js` (lazy): 5.95kB
- `TrackWaveform-*.js` (lazy): 6.59kB
- `index-*.css`: 480.11kB (gzip 63.01kB)
- `vendor-element-plus-*.js`: 761.95kB (gzip 241.30kB)

Latest build snapshot (after removing global `@nuxt/ui/vue-plugin` in entry):

- transformed modules: `1540 -> 667`
- `index-*.css`: `480.11kB -> 196.52kB`
- `vendor-misc-*.js`: `~336kB -> 313.08kB`
- `index-*.js`: `30.79kB -> 33.42kB` (roughly flat)

Notes:

- Kept `@nuxt/ui/vite` plugin and component usage intact.
- Removed only the global runtime install (`app.use(ui)`), which appears to be unnecessary in this Vite setup and expensive on startup.
- Added Vite dev warmup + dependency pre-bundling for hot startup modules to reduce first-request transform latency in `tauri dev`.

Additional migration-safe cleanup applied:

- Removed `lodash` and `@types/lodash` dependencies.
- Replaced `assignIn` and `sample` usage in `track` store with native equivalents.

## Phase 4: CSS/Font Startup Cost

- [x] Add `font-display: swap` to `@font-face` definitions.
- [x] Limit startup font variants to only required weights for first paint.
- [ ] Defer or prune non-essential global CSS.
- [ ] Re-measure first paint and text render timing.

Latest CSS/font pass:

- Removed unused `theme.css` import from entry.
- Reduced bundled font files from many Inter/Silkscreen variants to:
  - Inter 400 (`woff2`)
  - Inter 600 (`woff2`)
  - Silkscreen 400 (`woff2`)
- CSS bundle reduced to `194.68kB` (gzip `27.37kB`).

## Phase 5: Validate and Guard

- [ ] Run 5 cold-start runs and collect median timings.
- [ ] Run 5 warm-start runs and collect median timings.
- [ ] Confirm no functional regressions:
  - [ ] open/save/export still work
  - [ ] playback and effects still initialize correctly
  - [ ] menu/shortcut events still register
- [ ] Keep startup tracing in repo (non-invasive) for future regressions.

## Execution Order

1. Phase 0
2. Phase 1
3. Phase 2
4. Phase 3
5. Phase 4
6. Phase 5

## Notes

- Prioritize reducing the ~2.16s post-window gap first; that is currently the dominant delay.
- Do not optimize `init_player` startup path yet unless traces prove it is on the critical path before first paint.
