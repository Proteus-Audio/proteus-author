# ST-03 Add Mute/Solo Controls For Track Bins

Add per-bin mute and solo controls that work both while stopped and during active playback, with minimal audible disruption.

## Problem

Each track bin currently exposes level and pan controls, but there is no fast way to isolate or silence a bin without moving the fader. That makes auditioning combinations slower and also loses the user's intended level when they want a temporary mute.

This needs to work smoothly during playback. Rebuilding the player on every mute/solo toggle would be the wrong implementation path because `init_player` replaces the player instance, seeks back, and resumes playback. That is heavier than necessary for a simple audibility change.

## Findings

The current app already has the core primitive needed for a smooth implementation:

- [`src/stores/track.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/track.ts) debounces per-track mix updates and calls the Tauri command `set_track_mix`.
- [`src-tauri/src/player/commands.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/commands.rs) maps `track_id` to the active playback slot and forwards the update to the player.
- [`src-tauri/src/player/runtime.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/runtime.rs) applies the change inline with `player.set_track_mix_inline(slot_index, level, pan)`.
- [`src-tauri/src/player/mix.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/mix.rs) already builds player tracks from project state at init time.

Because inline level changes already exist, this does **not** require a `proteus-lib` change as the first implementation step.

## Recommendation

Implement mute/solo in `proteus-author` by storing mute/solo state in the project model and deriving an **effective playback level** from:

- the track's stored `level`
- the track's `muted` state
- whether any track is currently `soloed`

Recommended effective-level rule:

- If any playable track is soloed, only soloed tracks are audible.
- A muted track remains silent even if it is also soloed.
- If no track is soloed, non-muted tracks play at their stored level.

This preserves the user's fader level, avoids player rebuilds, and keeps mute/solo responsive during playback.

## Full Plan Without Touching `proteus-lib`

### 1. Extend the track schema and defaults

Add `muted?: boolean` and `soloed?: boolean` to:

- [`src/typings/tracks.d.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/typings/tracks.d.ts)
- [`src/typings/proteus.d.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/typings/proteus.d.ts)
- [`src-tauri/src/project.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/project.rs)

Use serde defaults on the Rust side so older project files load with `muted = false` and `soloed = false`.

Also update new-track creation paths in:

- [`src/stores/track.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/track.ts)
- [`src-tauri/src/file/registry.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/file/registry.rs)

### 2. Add track-store state helpers

In [`src/stores/track.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/track.ts):

- Add getters for `anySoloed` and `effectiveTrackLevel(trackId)`.
- Add actions like `setTrackMuted(trackId, muted)` and `setTrackSoloed(trackId, soloed)`.
- Keep `level` as the user's saved fader value. Do not overwrite it when muting.
- When mute/solo changes, log project changes and trigger a live mix sync immediately.

The important distinction is:

- `track.level` = persisted user level
- `effective level` = what gets sent to the player right now

### 3. Split persistence from live playback updates

The current `set_track_mix` command persists `level` and `pan` and also updates the player inline. That is correct for fader/pan moves, but mute/solo needs a way to update playback without destroying the stored fader value.

Add app/backend helpers so mute/solo can update playback using derived levels:

- Option A: add dedicated commands like `set_track_mute`, `set_track_solo`, and `sync_track_mix_state`
- Option B: keep one command, but have it compute effective levels from project state instead of assuming the passed `level` is always the stored level

Recommended approach: keep persistence and live-sync explicit.

- `set_track_mix` should continue to persist raw `level` and `pan`
- new mute/solo commands should persist `muted` / `soloed`
- a shared backend helper should recompute and push effective levels for all playable tracks inline

### 4. Add a backend helper that applies effective levels inline

In [`src-tauri/src/player/mix.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/mix.rs) and [`src-tauri/src/player/commands.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/commands.rs):

- Add a helper that walks project tracks in playback order
- Detect whether any playable track is soloed
- Compute each playable track's effective level
- Push that level and the stored pan to the player with inline updates

This same helper should be reused after:

- changing level
- changing pan
- toggling mute
- toggling solo

That keeps the runtime behavior consistent and avoids mismatches between stored state and current playback state.

### 5. Respect mute/solo on player init and reload

Update [`src-tauri/src/player/mix.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/mix.rs) so `build_paths_tracks()` uses the effective level, not just the raw stored level.

That ensures mute/solo still applies after:

- loading a project
- reinitializing the player
- reshuffling / selection changes

### 6. Add the UI controls to each bin

Update [`src/components/track/TrackBin.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/track/TrackBin.vue):

- Add a `Mute` button and a `Solo` button near the name / bin controls
- Show clear active states for muted and soloed bins
- Disable nothing during playback; toggles should remain live
- Consider styling the waveform/bin shell to reflect muted or soloed state

The existing [`DigitalTrackMix`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/digital/DigitalTrackMix.vue) control can stay unchanged if mute/solo is handled as adjacent bin actions.

### 7. Persist new fields in the project head/save path

Update [`src/stores/head.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/head.ts) so `projectState()` includes `muted` and `soloed`.

Also confirm project load/sync in [`src/stores/track.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/track.ts) hydrates missing values back to `false`.

### 8. Make export honor mute/solo state

Update [`src-tauri/src/file/export.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/file/export.rs) so exported play settings use effective levels, not raw levels.

Otherwise a muted bin could play in exports even though it was silent in the editor.

### 9. Verify smoothness and regressions

Manual verification should cover:

- mute during playback
- unmute during playback
- solo one bin during playback
- solo multiple bins during playback
- mute a soloed bin
- changing level/pan while mute/solo is active
- saving and reopening a project with mute/solo states
- export honoring mute/solo state

## When To Touch `proteus-lib`

Do **not** make `proteus-lib` changes up front.

Only open a follow-up in `proteus-lib` if testing shows that repeated inline gain changes produce audible zipper noise, clicks, or other artifacts that cannot be addressed from `proteus-author`.

If that happens, the `proteus-lib` follow-up should be narrowly scoped to smoothing track-level gain transitions during `set_track_mix_inline` updates, not to adding mute/solo concepts directly to the core library.

## Acceptance Criteria

- [ ] Each populated track bin has visible mute and solo controls
- [ ] Mute and solo state persist in project files
- [ ] Toggling mute or solo during playback does not rebuild the player
- [ ] Toggling mute or solo during playback uses inline mix updates and produces minimal disruption
- [ ] Stored fader level is preserved when muting and restored when unmuting
- [ ] Soloing one or more bins silences all other playable bins
- [ ] A muted bin stays silent even if it is soloed
- [ ] Loading a saved project restores mute/solo state correctly
- [ ] Exported play settings honor mute/solo state
- [ ] `bun run build` passes
- [ ] `bun run lint` passes
- [ ] Manual playback verification is complete

Backlog
