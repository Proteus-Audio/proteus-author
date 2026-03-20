# ST-04 Create Effect-Specific Dialogs With Metered Visual Feedback

Replace the current monolithic analog-style effect editor with effect-specific dialogs that match the flatter main app and digital control language, add a dedicated multiband EQ editor, and use the effect metering APIs already exposed by `proteus-lib` to drive visual feedback.

## Problem

The current effect editing UI is concentrated in a single conditional dialog component and is still styled around the older analog treatment. That causes four problems:

- the editor does not visually match the flatter main app or the existing digital controls
- each effect is mostly a collection of parameter controls, with little or no visual feedback about what those controls are doing
- `MultibandEq` exists in the library-side metering and response guidance, but it is not yet surfaced in the author UI at all
- the current ticket direction underspecifies which visuals should be analytical and always available versus live and audio-driven during playback

For filters and EQ in particular, editing without a visible response curve makes the UI harder to trust and slower to use. For dynamics, gain, and ambience effects, editing without input/output feedback makes it harder to understand the audible consequence of a change.

## Findings

- [`src/components/effects/EffectsDialog.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/effects/EffectsDialog.vue) currently handles all effect editors in one large conditional component and uses `AnalogIndicator`, `AnalogToggle`, and `AnalogKnob`.
- [`src/components/effects/EffectRack.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/effects/EffectRack.vue) exposes the current author-side effect list from [`src/utils/effects.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/utils/effects.ts).
- [`src/typings/effects.d.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/typings/effects.d.ts) currently includes:
  - `BasicReverb`
  - `DiffusionReverb`
  - `ConvolutionReverb`
  - `Compressor`
  - `Limiter`
  - `LowPassFilter`
  - `HighPassFilter`
  - `Distortion`
  - `Gain`
- [`src-tauri/Cargo.toml`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/Cargo.toml) already compiles `proteus-lib` with `output-meter`, `effect-meter`, and `effect-meter-spectral`.
- [`EFFECT_METERING_GUIDELINES.md`](/Users/innocentsmith/Dev/proteus/proteus-author/EFFECT_METERING_GUIDELINES.md) documents three useful metering tiers already available from `proteus-lib`:
  - Tier 1: per-effect input/output peak and RMS levels
  - Tier 2: analytical frequency-response curves for `LowPassFilter`, `HighPassFilter`, and `MultibandEq`
  - Tier 3: FFT-based input/output spectral analysis for supported spectral-shaping effects
- [`src-tauri/src/player/commands.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/commands.rs), [`src-tauri/src/player/api.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/api.rs), and [`src-tauri/src/player/runtime.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/runtime.rs) currently expose output meter polling, effect-chain mutation, and inline playback control, but they do **not** yet expose author-side commands for effect level snapshots, analytical response curves, or effect spectral snapshots.
- [`src/stores/audio.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/audio.ts) already contains a polling pattern for output levels via `get_levels_db`, which is a reasonable model for effect-meter polling.

## Recommendation

Refactor the effect editor into one shared shell plus one dedicated editor component per effect type. Move the styling toward the flatter digital system used elsewhere in the app, and treat visual feedback as a first-class part of the editor instead of an optional embellishment.

Use the metering work in two layers:

- required, always-available editor visuals that do **not** depend on playback
- optional live overlays and meters that use audible-time snapshots while playback is active

Recommended default policy:

- Tier 2 analytical response curves are required for `LowPassFilter`, `HighPassFilter`, and `MultibandEq`
- Tier 1 input/output meters are the default live feedback mechanism for all effect dialogs
- Tier 3 spectral overlays are optional in the first pass, but the ticket should preserve a clean integration path for them because the backend support already exists in `proteus-lib`
- live UI should prefer the audible-time accessors described in [`EFFECT_METERING_GUIDELINES.md`](/Users/innocentsmith/Dev/proteus/proteus-author/EFFECT_METERING_GUIDELINES.md), not the latest processing-time snapshots

At minimum:

- every supported effect gets its own editor component
- `MultibandEq` is added to the author-side types, effect menu, defaults, serialization, and editor surface
- filter and EQ effects render a frequency response chart with labeled frequencies and a clear target curve
- every dialog surfaces some effect-appropriate visual feedback beyond raw knobs and inputs
- effect metering is only enabled while the relevant UI is visible

## Full Plan

### 1. Split the current monolithic dialog into per-effect editors

Replace the large conditional structure in [`src/components/effects/EffectsDialog.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/effects/EffectsDialog.vue) with a shared wrapper and dedicated components such as:

- `BasicReverbDialog`
- `DiffusionReverbDialog`
- `ConvolutionReverbDialog`
- `CompressorDialog`
- `LimiterDialog`
- `LowPassFilterDialog`
- `HighPassFilterDialog`
- `DistortionDialog`
- `GainDialog`
- `MultibandEqDialog`

Keep the shared shell responsible for dialog framing, title, effect enable/disable state, common actions, and layout slots. Keep effect-specific parameter and visualization logic inside the dedicated dialog components.

### 2. Move the design language from analog to flat digital

Update the effect dialogs to align with the main app and the digital controls already present in:

- [`src/components/digital/DigitalPot.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/digital/DigitalPot.vue)
- [`src/components/digital/DigitalIndicator.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/digital/DigitalIndicator.vue)
- [`src/components/digital/DigitalFader.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/digital/DigitalFader.vue)

The new dialogs should favor:

- flatter surfaces
- sharper spacing and hierarchy
- digital-style labels and states
- less ornamental framing
- consistent density between desktop and narrower widths

This is a visual redesign, not only a component split.

### 3. Add author-side `MultibandEq` support

Extend the author UI so `MultibandEq` is a first-class effect:

- add the type and settings model in [`src/typings/effects.d.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/typings/effects.d.ts)
- add label/defaults/type mapping in [`src/utils/effects.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/utils/effects.ts)
- expose it in the add-effect menu in [`src/components/effects/EffectRack.vue`](/Users/innocentsmith/Dev/proteus/proteus-author/src/components/effects/EffectRack.vue)
- update backend serialization and hydration paths wherever the effect chain is passed through `proteus-author`

The exact shape of the settings model should match the `proteus-lib` contract already expected by the backend side of the stack.

### 4. Add an author-side Tauri bridge for effect metering and response data

Do **not** start by changing `proteus-lib`. The core library support is already present and compiled in.

Instead, extend the author-side player bridge in:

- [`src-tauri/src/player/types.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/types.rs)
- [`src-tauri/src/player/api.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/api.rs)
- [`src-tauri/src/player/runtime.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/runtime.rs)
- [`src-tauri/src/player/commands.rs`](/Users/innocentsmith/Dev/proteus/proteus-author/src-tauri/src/player/commands.rs)

Expose the small command surface the UI actually needs:

- enable or disable Tier 1 per-effect level metering
- optionally adjust Tier 1 refresh rate
- fetch per-effect level snapshots, preferably the audible-time variant for live UI
- query Tier 2 frequency-response curves on demand
- enable or disable Tier 3 spectral analysis
- optionally adjust Tier 3 refresh rate
- fetch per-effect spectral snapshots, again preferring the audible-time variant for live UI

This should mirror the existing `get_levels_db` pattern: the frontend talks only to `proteus-author`, and `proteus-author` decides which player calls to make.

### 5. Add frontend state and lifecycle for dialog-scoped effect metering

Add frontend state, either in [`src/stores/audio.ts`](/Users/innocentsmith/Dev/proteus/proteus-author/src/stores/audio.ts) or in a dedicated effect-visualization store, to manage:

- which effect dialog is currently open
- whether Tier 1 effect metering should be enabled
- whether Tier 3 spectral analysis should be enabled
- cached Tier 2 response curves keyed to the current effect chain
- per-effect live meter and spectral snapshots for the active dialog

Recommended lifecycle:

- when an effect dialog opens, enable Tier 1 effect metering
- when a spectral-shaping dialog opens, fetch its Tier 2 curve immediately
- only enable Tier 3 when a supported spectral-shaping dialog is open and playback is active
- when the dialog closes, disable Tier 1 and Tier 3 again

Recommended refresh policy from the metering guide:

- Tier 1 level meters: `30 Hz` default, optionally `60 Hz` for faster peak motion
- Tier 3 spectral overlays: `15 Hz` default
- Tier 2 response curves: query on demand only when settings, chain topology, or sample rate change

### 6. Define the visual-feedback contract per effect family

Every effect dialog should provide more than raw parameter controls, but the visual aid should match the effect type.

Required first-pass visuals:

- `LowPassFilter`, `HighPassFilter`, `MultibandEq`: logarithmic response chart using Tier 2 analytical curves, with marked frequencies and live parameter updates
- `Compressor`, `Limiter`, `Distortion`: transfer-curve visualization driven from current settings
- `Gain`: input/output level presentation and visible gain delta
- `BasicReverb`, `DiffusionReverb`, `ConvolutionReverb`: parameter-focused spatial or decay visualization, plus dry/wet emphasis and IR state where applicable

Recommended live overlays:

- Tier 1 input/output peak and RMS meters for every effect dialog
- Tier 3 input/output spectral overlays for `LowPassFilter`, `HighPassFilter`, and `MultibandEq`

The key distinction is:

- Tier 2 shows the intended target shape even while stopped
- Tier 1 and Tier 3 show what is actually being heard during playback

### 7. Use the metering APIs with the correct timing semantics and fallback behavior

Follow the behavior documented in [`EFFECT_METERING_GUIDELINES.md`](/Users/innocentsmith/Dev/proteus/proteus-author/EFFECT_METERING_GUIDELINES.md):

- use audible-time accessors for live animated UI
- do not poll Tier 2 continuously
- treat snapshot vector length as dynamic when effects are added, removed, or reordered
- handle `None`, `Some([])`, and per-slot `None` states explicitly in the UI

Suggested UI rules:

- if Tier 1 or Tier 3 is runtime-disabled, hide or placeholder the live panel instead of showing stale data
- if a supported spectral slot is still warming up, show a loading or warming state rather than a broken graph
- if an effect is bypassed, style the curve or meter as bypassed instead of assuming the backend will suppress the data

### 8. Keep the rack flow intact while improving edit affordances

The effect rack should continue to support:

- adding effects
- reordering effects
- opening an editor quickly from a rack item
- clear enabled/disabled state

If needed, update effect mini-cards so their summaries and affordances match the flatter redesign and preview the most important parameter values or live status.

### 9. Verify responsiveness, metering behavior, and regressions

Manual verification should cover:

- opening every effect dialog from the rack
- editing every parameter and seeing the UI update immediately
- reordering effects without breaking editor state or meter-to-effect alignment
- adding and editing `MultibandEq`
- filter and EQ response charts updating correctly as controls move while stopped
- live input/output effect meters updating while playback is active
- live spectral overlays updating correctly for supported spectral effects if Tier 3 is included in the first pass
- metering disabling when dialogs are closed
- acceptable layout and readability on narrow widths

## When To Touch `proteus-lib`

Do **not** make `proteus-lib` changes up front.

Only open a follow-up there if implementation in `proteus-author` shows one of these is genuinely missing:

- a required effect type or settings schema for `MultibandEq`
- a missing serialization or decode path that blocks the author app
- a metering API gap that contradicts [`EFFECT_METERING_GUIDELINES.md`](/Users/innocentsmith/Dev/proteus/proteus-author/EFFECT_METERING_GUIDELINES.md)
- a performance problem severe enough that polling from the author app is not acceptable

The first implementation step here is an author-side integration task, not a library feature request.

## Acceptance Criteria

- [ ] Each supported effect type has a dedicated dialog or editor component instead of a single monolithic conditional editor
- [ ] The effect dialog styling matches the flatter main app and digital UI direction
- [ ] `BasicReverb`, `DiffusionReverb`, `ConvolutionReverb`, `Compressor`, `Limiter`, `LowPassFilter`, `HighPassFilter`, `Distortion`, and `Gain` all have dedicated editors
- [ ] `MultibandEq` is available in the author UI and has its own editor dialog
- [ ] `MultibandEq` can be added, edited, persisted, and reloaded through the author app
- [ ] The author-side Tauri layer exposes the effect metering and response queries needed by the new UI without requiring an upfront `proteus-lib` change
- [ ] `LowPassFilter`, `HighPassFilter`, and `MultibandEq` show a Tier 2 frequency-response visualization with marked frequencies and a visible target curve
- [ ] Tier 2 response curves update immediately when filter or EQ parameters change, including while playback is stopped
- [ ] Effect dialogs provide immediate visual feedback beyond raw controls, with Tier 1 input/output metering available while the dialog is open
- [ ] If Tier 3 is included in the first pass, supported spectral-shaping effects show audible-time spectral overlays and degrade gracefully during warmup or when disabled
- [ ] Effect metering is enabled only while the relevant editor UI is visible
- [ ] The rack/editor flow remains usable for adding, opening, and reordering effects
- [ ] `bun run build` passes
- [ ] `bun run lint` passes
- [ ] Manual UI verification is complete

Backlog
