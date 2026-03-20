# ST-04 Create Effect-Specific Dialogs With Flat Visual Feedback

Replace the current monolithic analog-style effect editor with effect-specific dialogs that match the flatter main app and digital control language, and add a dedicated multiband EQ editor for the effect already documented in `proteus-lib`.

## Problem

The current effect editing UI is concentrated in a single conditional dialog component and is still styled around the older analog treatment. That causes three problems:

- the editor does not visually match the flatter main app or the existing digital controls
- each effect has only parameter controls, with little or no visual feedback about what those controls are doing
- `MultibandEq` exists in the library-side metering/response guidance, but it is not yet surfaced in the author UI at all

For filters and EQ in particular, editing without a visible response curve makes the UI harder to trust and slower to use.

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
- [`EFFECT_METERING_GUIDELINES.md`](/Users/innocentsmith/Dev/proteus/proteus-author/EFFECT_METERING_GUIDELINES.md) already documents response-curve and spectral-analysis support for `LowPassFilter`, `HighPassFilter`, and `MultibandEq`, including guidance for a standard EQ overlay and per-band curve rendering.

## Recommendation

Refactor the effect editor into one shared shell plus one dedicated editor component per effect type. Move the styling toward the flatter digital system used elsewhere in the app, and treat visual feedback as a first-class part of the editor instead of an optional embellishment.

At minimum:

- every supported effect gets its own editor component
- `MultibandEq` is added to the author-side types, effect menu, defaults, serialization, and editor surface
- filter and EQ effects render a frequency response chart with labeled frequencies and an emulated change curve
- the new dialogs reuse a common shell, spacing system, and digital component vocabulary so they feel cohesive

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

### 4. Add chart-based visual feedback for spectral shaping effects

For `LowPassFilter`, `HighPassFilter`, and `MultibandEq`, add an editor chart that shows:

- a logarithmic frequency axis
- marked control frequencies
- the intended response curve
- live updates while parameters change

Prefer using the guidance already documented in [`EFFECT_METERING_GUIDELINES.md`](/Users/innocentsmith/Dev/proteus/proteus-author/EFFECT_METERING_GUIDELINES.md):

- response-curve overlays for the target shape
- optional spectral snapshots when the view is open and playback is active

If runtime spectral analysis is not wired in the first pass, the initial version should still render an emulated or queried target curve so the user sees the expected shape immediately while editing.

### 5. Add effect-appropriate visual feedback beyond filters

Each dialog should include a lightweight visual aid that matches the effect:

- reverb dialogs: decay/pre-delay/dry-wet space or envelope cues
- compressor/limiter dialogs: threshold, knee, and ratio transfer visualization
- distortion: drive vs threshold transfer curve
- gain: output delta / level indication
- convolution reverb: IR source state, tail trimming, and dry/wet balance emphasis

These do not all need the same chart, but every effect dialog should expose more than raw knobs and inputs.

### 6. Keep the rack flow intact while improving edit affordances

The effect rack should continue to support:

- adding effects
- reordering effects
- opening an editor quickly from a rack item
- clear enabled/disabled state

If needed, update effect mini-cards so their summaries and affordances match the flatter redesign and preview the most important parameter values.

### 7. Verify responsiveness and layout behavior

Manual verification should cover:

- opening every effect dialog from the rack
- editing every parameter and seeing the UI update immediately
- reordering effects without breaking editor state
- adding and editing `MultibandEq`
- filter/EQ response charts updating correctly as controls move
- acceptable layout and readability on narrow widths

## Acceptance Criteria

- [ ] Each supported effect type has a dedicated dialog/editor component instead of a single monolithic conditional editor
- [ ] The effect dialog styling matches the flatter main app and digital UI direction
- [ ] `BasicReverb`, `DiffusionReverb`, `ConvolutionReverb`, `Compressor`, `Limiter`, `LowPassFilter`, `HighPassFilter`, `Distortion`, and `Gain` all have dedicated editors
- [ ] `MultibandEq` is available in the author UI and has its own editor dialog
- [ ] `MultibandEq` can be added, edited, persisted, and reloaded through the author app
- [ ] `LowPassFilter`, `HighPassFilter`, and `MultibandEq` show a frequency response visualization with marked frequencies and a visible change curve
- [ ] Effect controls provide immediate visual feedback while being adjusted
- [ ] The rack/editor flow remains usable for adding, opening, and reordering effects
- [ ] `bun run build` passes
- [ ] `bun run lint` passes
- [ ] Manual UI verification is complete

Backlog
