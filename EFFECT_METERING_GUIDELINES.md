# Effect Metering Guidelines

Integration reference for applications consuming the per-effect metering and
analysis API exposed by `proteus-lib`.

This guide covers all three tiers:

- Tier 1: per-effect input/output levels
- Tier 2: analytical frequency-response curves
- Tier 3: FFT-based spectral analysis

For the CLI-specific live/offline harness, see `EFFECTS_METERING_CLI.md`.

---

## Compile-Time Setup

Effect metering is opt-in. Enable the features your application needs on the
`proteus-lib` dependency:

```toml
[dependencies]
# Tiers 1 and 2
proteus-lib = { path = "../proteus-lib", features = ["effect-meter"] }

# Or, if you also need Tier 3:
# proteus-lib = { path = "../proteus-lib", features = ["effect-meter-spectral"] }
```

| Feature                 | What it unlocks                                                        | Notes                                 |
| ----------------------- | ---------------------------------------------------------------------- | ------------------------------------- |
| `effect-meter`          | Tier 1 levels, Tier 1 audible-time accessors, Tier 2 analytical curves | No extra dependency                   |
| `effect-meter-spectral` | Tier 3 spectral analysis and audible-time spectral accessors           | Implies `effect-meter` and `real-fft` |

All public metering types and `Player` methods exist regardless of feature
flags. Disabled builds degrade as follows:

| API surface                                                              | Without the feature compiled in                      |
| ------------------------------------------------------------------------ | ---------------------------------------------------- |
| `set_effect_level_metering_enabled`, `set_effect_level_meter_refresh_hz` | no-op                                                |
| `effect_levels`, `effect_levels_audible`                                 | `None`                                               |
| `effect_frequency_responses`                                             | chain-aligned `Vec<Option<_>>` with all slots `None` |
| `set_spectral_analysis_enabled`, `set_spectral_analysis_refresh_hz`      | no-op                                                |
| `spectral_analysis_enabled`                                              | `false`                                              |
| `effect_band_levels`, `effect_band_levels_audible`                       | `None`                                               |

---

## Choose The Right Timing Model

Tier 1 and Tier 3 now expose two timing semantics:

| Accessor family                                           | Timing                                                    | Best for                            |
| --------------------------------------------------------- | --------------------------------------------------------- | ----------------------------------- |
| `effect_levels()`, `effect_band_levels()`                 | Latest processing-time snapshot from the mix thread       | offline tooling, diagnostics, tests |
| `effect_levels_audible()`, `effect_band_levels_audible()` | Snapshot aligned to the current audible playback boundary | live meters, animated UI            |

The audible-time accessors resolve snapshot timestamps against the current
`Player::get_time()` clock. They intentionally lag behind the latest mix-thread
snapshot so the UI tracks what is being heard rather than what has merely been
processed.

Use the latest processing-time accessors for deterministic inspection. Use the
audible-time accessors for live playback surfaces.

---

## Tier 1: Per-Effect Input/Output Levels

### Enable at runtime

Level metering is runtime-disabled by default even when compiled in. Enable it
when the UI needs it and disable it when the view is hidden:

```rust
player.set_effect_level_metering_enabled(true);

// Optional: change the refresh cadence (default 30 Hz).
player.set_effect_level_meter_refresh_hz(60.0);

player.set_effect_level_metering_enabled(false);
```

### Read snapshots

```rust
// Latest processing-time snapshot from the mix thread.
if let Some(latest) = player.effect_levels() {
    for (index, snap) in latest.iter().enumerate() {
        // snap.input.peak  -> Vec<f32>, one entry per channel
        // snap.input.rms   -> Vec<f32>
        // snap.output.peak -> Vec<f32>
        // snap.output.rms  -> Vec<f32>
    }
}

// Audible-time-aligned snapshot for live UI.
if let Some(audible) = player.effect_levels_audible() {
    for (index, snap) in audible.iter().enumerate() {
        // Same data shape as effect_levels()
    }
}
```

Important behavior differences:

- `effect_levels()` returns a zeroed layout immediately after
  `set_effect_level_metering_enabled(true)`, even before playback has produced a
  metered chunk.
- `effect_levels_audible()` stays `None` until playback has produced at least
  one timestamped snapshot.
- `Some([])` is valid and means the active chain currently has zero effects.

### Data types

```rust
pub struct LevelSnapshot {
    pub peak: Vec<f32>,  // absolute peak per channel
    pub rms: Vec<f32>,   // RMS per channel
}

pub struct EffectLevelSnapshot {
    pub input: LevelSnapshot,   // measured before the effect
    pub output: LevelSnapshot,  // measured after the effect
}
```

Each channel slot in `peak` and `rms` corresponds to the interleaved channel
index, for example `0 = left`, `1 = right` in stereo.

### Return conditions

| Call                      | Returns `None` when                                                         |
| ------------------------- | --------------------------------------------------------------------------- |
| `effect_levels()`         | `effect-meter` is not compiled or runtime level metering is disabled        |
| `effect_levels_audible()` | same as `effect_levels()`, or no timestamped snapshot has been produced yet |

### Refresh cadence

Refresh is scheduled by accumulated sample frames, not by chunk count. This
keeps the cadence stable regardless of internal buffering or convolution batch
sizes.

| Refresh Hz | Approximate period at 48 kHz |
| ---------- | ---------------------------- |
| 15         | ~67 ms                       |
| 30         | ~33 ms (default)             |
| 60         | ~17 ms                       |

Higher rates increase mix-thread work. `30 Hz` is a sensible default for most
meter UIs. `60 Hz` is appropriate for faster peak-style displays.

### Chain changes and inline transitions

During a full-chain inline transition such as `set_effects_inline`, the mix
thread does not publish new per-effect snapshots for the crossfade window.

- `effect_levels()` keeps the last stable latest snapshot during the transition.
- When the new chain becomes active, the latest layout is reinitialized with
  zeroed slots matching the new chain length and channel count.
- `effect_levels_audible()` continues to show the old audible snapshot until
  newly produced snapshots from the new chain have reached audible time.

Do not assume snapshot vector length is stable across reads. Structural changes,
sample-rate changes, and channel-layout changes can resize the snapshot vector.

---

## Tier 2: Analytical Frequency-Response Curves

### Query on demand

Tier 2 is computed on demand from effect settings. It does not require a runtime
enable step.

```rust
let responses: Vec<Option<FilterResponseCurve>> =
    player.effect_frequency_responses(128);
```

Each slot corresponds to an effect in the active chain. Non-filter effects
return `None`. Supported filter effects return a `FilterResponseCurve`:

```rust
pub struct FrequencyResponsePoint {
    pub freq_hz: f32,   // probe frequency in Hz
    pub gain_db: f32,   // gain at that frequency in dB
}

pub struct FilterResponseCurve {
    pub composite: Vec<FrequencyResponsePoint>,      // total response
    pub per_band: Vec<Vec<FrequencyResponsePoint>>,  // per-section breakdown
}
```

### Supported effect types

| Effect           | `composite` | `per_band`                       |
| ---------------- | ----------- | -------------------------------- |
| `LowPassFilter`  | yes         | empty                            |
| `HighPassFilter` | yes         | empty                            |
| `MultibandEq`    | yes         | one curve per configured section |

For multiband EQ, `per_band` is ordered as:

- optional low edge
- each parametric EQ point in configured order
- optional high edge

If a multiband EQ has no configured sections, `composite` is the identity curve
and `per_band` is empty.

### Choosing `num_points`

Probe frequencies are logarithmically spaced from `20 Hz` to Nyquist.

| `num_points` | Use case                           |
| ------------ | ---------------------------------- |
| `32-64`      | thumbnail or minimap preview       |
| `128`        | standard EQ editor overlay         |
| `256-512`    | high-resolution full-width display |

`num_points <= 1` yields a single probe point at the low end of the range.

### Computation details

- Curves are analytical evaluations of the configured filter settings. They are
  not derived from runtime audio.
- Curves reflect target settings, not smoothed internal coefficients.
- Curves do not encode bypass state. A disabled `LowPassFilter`,
  `HighPassFilter`, or `MultibandEq` still returns its configured curve. If your
  UI shows bypass, style or suppress the curve separately.
- No mix-thread work is performed. The query walks the control-path copy of the
  chain under a short lock.

### When to re-query

Re-query when:

- the user changes a filter parameter
- the effect chain is structurally modified
- the sample rate changes

There is no need to poll continuously. Cache the result and invalidate it on
parameter or topology changes.

---

## Tier 3: FFT-Based Spectral Analysis

### Enable at runtime

Spectral analysis is disabled by default and requires both compile-time and
runtime activation:

```rust
player.set_spectral_analysis_enabled(true);

// Optional: change the refresh cadence (default 15 Hz).
player.set_spectral_analysis_refresh_hz(30.0);

if player.spectral_analysis_enabled() {
    // UI can rely on Tier 3 accessors.
}

player.set_spectral_analysis_enabled(false);
```

### Read snapshots

```rust
// Latest processing-time spectral snapshot.
if let Some(latest) = player.effect_band_levels() {
    for (index, slot) in latest.iter().enumerate() {
        match slot {
            Some(snap) => {
                // snap.input.bands_db
                // snap.input.band_centers_hz
                // snap.output.bands_db
                // snap.output.band_centers_hz
            }
            None => {
                // Unsupported effect, or supported effect still warming up.
            }
        }
    }
}

// Audible-time-aligned spectral snapshot for live UI.
if let Some(audible) = player.effect_band_levels_audible() {
    for maybe_snap in audible {
        // Same slot semantics as effect_band_levels()
    }
}
```

Important behavior differences:

- `effect_band_levels()` returns a chain-sized vector immediately after
  `set_spectral_analysis_enabled(true)`.
- Every slot initially starts as `None`, including supported filter effects.
- Supported filter effects become `Some(EffectBandSnapshot)` only after the
  analyzer has captured enough audio to publish its first FFT snapshot.
- Unsupported effects remain `None` permanently.
- `effect_band_levels_audible()` stays `None` until the first timestamped
  spectral snapshot has been produced.

### Data types

```rust
pub struct BandLevels {
    pub bands_db: Vec<f32>,         // energy per bucket in dB
    pub band_centers_hz: Vec<f32>,  // center frequency per bucket in Hz
}

pub struct EffectBandSnapshot {
    pub input: BandLevels,
    pub output: BandLevels,
}
```

### Return conditions

| Call                           | Returns `None` when                                                                       |
| ------------------------------ | ----------------------------------------------------------------------------------------- |
| `effect_band_levels()`         | `effect-meter-spectral` is not compiled or runtime spectral analysis is disabled          |
| `effect_band_levels_audible()` | same as `effect_band_levels()`, or no timestamped spectral snapshot has been produced yet |

Remember that the outer `Option` and the per-slot `Option` mean different
things:

- outer `None`: feature missing or runtime-disabled
- inner `None`: unsupported effect, or a supported effect before first FFT
  publication after enable or rebuild

### Bucket semantics

Spectral buckets are analysis buckets, not exact isolated per-filter
contributions.

| Effect           | Bucket strategy                                                                                                                              |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `LowPassFilter`  | 12 full-spectrum buckets spanning `0 Hz` to Nyquist. They are logarithmically spaced when Nyquist is high enough, otherwise linearly spaced. |
| `HighPassFilter` | same as `LowPassFilter`                                                                                                                      |
| `MultibandEq`    | one bucket per visible control frequency, using `0 Hz`, midpoints between adjacent controls, and Nyquist as boundaries                       |

Additional notes:

- For low-pass and high-pass filters, `band_centers_hz` are bucket centers, not
  the cutoff frequency.
- For multiband EQ, centers are the sorted control frequencies: optional low
  edge, EQ points, optional high edge.
- Buckets describe average spectral power in the analyzed range, not "how much
  this one filter section contributed".

### FFT window and channel handling

The analyzer window size is derived from the refresh cadence:

1. compute `round(sample_rate / refresh_hz)` frames per channel
2. round that up to the next power of two for the FFT size

Tradeoff:

- lower refresh rates cost less often, but each analysis uses a larger window
- higher refresh rates feel more responsive, but use a smaller FFT window and
  more frequent analysis work

Snapshots are channel-aggregated. Each effect yields one spectral snapshot, not
one snapshot per channel. Power is averaged across bins inside a bucket and
across channels before conversion to dB.

### Chain changes and inline transitions

During a full-chain inline transition, spectral publication pauses just like
Tier 1.

- `effect_band_levels()` keeps the last stable latest snapshot during the
  transition, then returns a new chain-sized vector whose slots start as `None`.
- `effect_band_levels_audible()` continues to show the old audible snapshot
  until new-chain spectral snapshots have reached audible time.
- Rebuilding the chain or changing sample rate rebuilds the analyzers. Expect a
  warmup gap before supported filter slots become `Some(...)` again.

### Performance considerations

FFT analysis is the most expensive tier.

- Disable it when hidden.
- `15 Hz` is a good default for compact animated graphs.
- `30 Hz` is reasonable for smoother motion.
- Going above `60 Hz` is usually not worth the extra cost.
- Disabling spectral analysis clears both the latest spectral snapshots and the
  audible-time spectral ring. Re-enabling starts warmup from scratch.

---

## Threading Model And Safety

Tier 1 and Tier 3 use the same shared-state pattern:

- a latest snapshot store for processing-time accessors
- a bounded timestamped ring for audible-time accessors

### Latest processing-time path

```text
Mix thread                             Control thread
----------                             --------------
measure local snapshot                 player.effect_levels()
  |                                      |
  +-> try_lock(latest store)             +-> lock(latest store)
      copy latest snapshot                   clone latest snapshot
      or skip if contended                   return
```

### Audible-time path

```text
Mix thread                             Control thread
----------                             --------------
measure local snapshot                 player.effect_levels_audible()
tag with mix_time_secs                   |
  |                                      +-> lock(audible ring)
  +-> try_lock(audible ring)                 retire entries older than audible time
      push timestamped snapshot              clone front entry
      or skip if contended                   return
```

Key guarantees:

- The mix thread never blocks on metering publication. Both latest publication
  and audible-ring publication use `try_lock()`.
- If a publication tick is skipped because of lock contention, the previous
  snapshot remains visible until the next successful publication.
- The control thread may block briefly while cloning the latest snapshot or
  draining the audible ring.
- Enabling, disabling, and refresh-rate changes use relaxed atomics and may
  take effect on the next mix-thread chunk.
- The audible ring is explicitly bounded to `256` snapshots. When it is full,
  the newest entry is dropped so older entries remain available for audible-time
  lookup. At `30 Hz`, that covers about `8.5 s` of producer-to-consumer delay.

---

## Handling `None` And Empty Data

Use this as the general UI policy:

| Value                                 | Meaning                                                                                                                      | Suggested UI response                     |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| outer `None`                          | feature missing or runtime-disabled; audible-time accessors also return `None` until their first timestamped snapshot exists | hide the panel or show a placeholder      |
| `Some([])`                            | active chain currently has zero effects                                                                                      | show an empty state                       |
| `Some(vec)` with level snapshots      | normal Tier 1 data                                                                                                           | render meters                             |
| `Some(vec)` with spectral slot `None` | unsupported effect or supported effect still warming up                                                                      | render a placeholder or omit spectral row |

---

## Effect Chain Changes

When the effect chain changes structurally by adding, removing, or reordering
effects:

- snapshot indices still correspond to effect indices in the current chain
- latest Tier 1 snapshots are resized and zeroed to the new layout
- latest Tier 3 snapshots are resized to the new layout and start as `None`
- audible-time accessors may continue to reflect the old chain briefly, because
  they intentionally follow what is audible rather than what has just been
  processed

If your UI keeps effect widgets by index, re-read the effect list and treat the
meter snapshot length as dynamic.

---

## Quick Reference

### Player API

```rust
// Tier 1: latest and audible-time levels
player.set_effect_level_metering_enabled(enabled: bool);
player.set_effect_level_meter_refresh_hz(hz: f32);
player.effect_levels() -> Option<Vec<EffectLevelSnapshot>>;
player.effect_levels_audible() -> Option<Vec<EffectLevelSnapshot>>;

// Tier 2: analytical curves
player.effect_frequency_responses(num_points: usize) -> Vec<Option<FilterResponseCurve>>;

// Tier 3: latest and audible-time spectral analysis
player.set_spectral_analysis_enabled(enabled: bool);
player.set_spectral_analysis_refresh_hz(hz: f32);
player.spectral_analysis_enabled() -> bool;
player.effect_band_levels() -> Option<Vec<Option<EffectBandSnapshot>>>;
player.effect_band_levels_audible() -> Option<Vec<Option<EffectBandSnapshot>>>;
```

### Defaults

| Setting                   | Default |
| ------------------------- | ------- |
| Level metering enabled    | `false` |
| Level refresh Hz          | `30`    |
| Spectral analysis enabled | `false` |
| Spectral refresh Hz       | `15`    |
