# Effect Metering Guidelines

Integration reference for authoring applications consuming the per-effect metering
API exposed by `proteus-lib`. Covers all three tiers: time-domain levels,
analytical frequency-response curves, and FFT-based spectral analysis.

---

## Compile-Time Setup

Effect metering is **off by default**. Enable the features your application needs
in your `Cargo.toml` dependency on `proteus-lib`:

```toml
[dependencies]
proteus-lib = { path = "../proteus-lib", features = ["effect-meter"] }

# If you also need animated spectral analysis:
proteus-lib = { path = "../proteus-lib", features = ["effect-meter-spectral"] }
```

| Feature                 | What it unlocks                                | Dependencies               |
| ----------------------- | ---------------------------------------------- | -------------------------- |
| `effect-meter`          | Tier 1 (levels) and Tier 2 (analytical curves) | —                          |
| `effect-meter-spectral` | Tier 3 (FFT spectral analysis)                 | `effect-meter`, `real-fft` |

All public types and `Player` methods exist regardless of feature flags. Without
the feature compiled in, setters are no-ops and getters return `None`.

---

## Tier 1: Per-Effect Input/Output Levels

### Enable at runtime

Level metering is runtime-disabled by default even when compiled in. Enable it
when your UI needs it and disable it when the metering view is hidden:

```rust
// Turn on when the user opens the level-meter panel.
player.set_effect_level_metering_enabled(true);

// Optional: change the refresh cadence (default 30 Hz).
player.set_effect_level_meter_refresh_hz(60.0);

// Turn off when the panel is closed.
player.set_effect_level_metering_enabled(false);
```

### Read snapshots

```rust
if let Some(snapshots) = player.effect_levels() {
    // One EffectLevelSnapshot per effect in the active chain.
    for (index, snap) in snapshots.iter().enumerate() {
        // snap.input.peak  — Vec<f32>, one entry per channel
        // snap.input.rms   — Vec<f32>, one entry per channel
        // snap.output.peak — Vec<f32>, one entry per channel
        // snap.output.rms  — Vec<f32>, one entry per channel
    }
}
```

`effect_levels()` returns `None` when:

- the `effect-meter` feature is not compiled
- runtime level metering is disabled
- playback has not started

The `Vec` length matches the number of effects in the active chain. Each
channel slot in `peak` / `rms` corresponds to the interleaved channel index
(e.g. index 0 = left, index 1 = right for stereo).

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

### Refresh cadence

Refresh is scheduled by **accumulated sample frames**, not by chunk count. This
makes the cadence stable regardless of internal buffering or convolution batch
sizes.

| Refresh Hz | Approximate latency at 48 kHz |
| ---------- | ----------------------------- |
| 15         | ~67 ms                        |
| 30         | ~33 ms (default)              |
| 60         | ~17 ms                        |

Higher rates increase mix-thread measurement work. 30 Hz is a sensible default
for most meter UIs; 60 Hz is appropriate for fast-response peak meters.

### Inline-transition behaviour

During a full-chain inline transition (e.g. `set_effects_inline`), level
snapshots are **frozen** at the last stable value. Publication resumes with
zeroed snapshots once the new chain becomes active. Do not treat a frozen
snapshot as an error — display the last known values until fresh data arrives.

---

## Tier 2: Analytical Frequency-Response Curves

### Query on demand

Tier 2 is computed **on demand** from effect settings and does not require
runtime enabling. Call it whenever the UI needs to draw filter curves:

```rust
let responses: Vec<Option<FilterResponseCurve>> =
    player.effect_frequency_responses(128);  // 128 log-spaced points
```

Each slot corresponds to an effect in the active chain. Slots for non-filter
effects (gain, delay, compressor, etc.) are `None`. Slots for filter-type
effects contain a `FilterResponseCurve`:

```rust
pub struct FrequencyResponsePoint {
    pub freq_hz: f32,   // probe frequency in Hz
    pub gain_db: f32,   // gain at that frequency in dB
}

pub struct FilterResponseCurve {
    pub composite: Vec<FrequencyResponsePoint>,        // total response
    pub per_band: Vec<Vec<FrequencyResponsePoint>>,    // per-section breakdown
}
```

### Supported effect types

| Effect           | `composite` | `per_band`                       |
| ---------------- | ----------- | -------------------------------- |
| `LowPassFilter`  | yes         | empty (single-filter)            |
| `HighPassFilter` | yes         | empty (single-filter)            |
| `MultibandEq`    | yes         | one curve per configured section |

For multiband EQ, `per_band` contains one curve for each configured section in
order: optional low edge, each parametric point, optional high edge. The
`composite` curve is the sum of all per-band curves in dB.

### Choosing `num_points`

Points are **logarithmically spaced** from 20 Hz to Nyquist. More points give
smoother curves but cost more to compute:

| `num_points` | Use case                           |
| ------------ | ---------------------------------- |
| 32–64        | Thumbnail or minimap preview       |
| 128          | Standard EQ editor overlay         |
| 256–512      | High-resolution full-width display |

### Computation details

- Curves are pure analytical evaluations of the biquad transfer function
  H(e^jω) — they reflect the **target settings**, not the real-time smoothed
  state.
- No mix-thread work is performed. The query reads the control-path copy of the
  effect chain under a short lock.
- During parameter ramps, the curve shows where the filter is heading, not where
  it currently is. This is the expected behaviour for an EQ editor overlay.

### When to re-query

Re-query when:

- the user changes a filter parameter (frequency, Q, gain)
- the effect chain is structurally modified (effects added/removed/reordered)
- the sample rate changes

There is no need to poll continuously. Caching the result and invalidating on
parameter change is the recommended approach.

---

## Tier 3: FFT-Based Spectral Analysis

### Enable at runtime

Spectral analysis is disabled by default and requires both compile-time and
runtime activation:

```rust
// Turn on when the user opens the spectral view.
player.set_spectral_analysis_enabled(true);

// Optional: change the refresh cadence (default 15 Hz).
player.set_spectral_analysis_refresh_hz(30.0);

// Turn off when the view is closed.
player.set_spectral_analysis_enabled(false);
```

### Read snapshots

```rust
if let Some(snapshots) = player.effect_band_levels() {
    // One Option<EffectBandSnapshot> per effect in the active chain.
    // Non-filter effects are None.
    for (index, maybe_snap) in snapshots.iter().enumerate() {
        if let Some(snap) = maybe_snap {
            // snap.input.bands_db        — Vec<f32>, energy per bucket in dB
            // snap.input.band_centers_hz  — Vec<f32>, center freq per bucket in Hz
            // snap.output.bands_db        — Vec<f32>
            // snap.output.band_centers_hz — Vec<f32>
        }
    }
}
```

`effect_band_levels()` returns `None` when:

- the `effect-meter-spectral` feature is not compiled
- runtime spectral analysis is disabled
- playback has not started

### Data types

```rust
pub struct BandLevels {
    pub bands_db: Vec<f32>,         // energy per bucket in dB
    pub band_centers_hz: Vec<f32>,  // center frequency label per bucket
}

pub struct EffectBandSnapshot {
    pub input: BandLevels,
    pub output: BandLevels,
}
```

### Bucket semantics

Spectral buckets are **analysis buckets aligned to visible controls**, not exact
isolated per-filter contributions:

| Effect           | Bucket strategy                                                                                        |
| ---------------- | ------------------------------------------------------------------------------------------------------ |
| `LowPassFilter`  | Two buckets: below cutoff and above cutoff                                                             |
| `HighPassFilter` | Two buckets: below cutoff and above cutoff                                                             |
| `MultibandEq`    | Boundaries at midpoints between adjacent control frequencies, with edge-filter cutoffs as outer bounds |

For multiband EQ, the number of buckets equals the number of configured control
points (low edge + parametric points + high edge). Each bucket's center
frequency and boundary are derived from the sorted control frequencies.

### Channel handling

Spectral snapshots contain **channel-aggregated power** — one snapshot per
effect, not one per channel. This matches typical EQ-editor display conventions.

### Inline-transition behaviour

Same as Tier 1: spectral snapshots are frozen during a full-chain inline
transition and rebuild when the new chain becomes active. Display the last known
values during the transition window.

### Performance considerations

FFT analysis is the most expensive tier. Keep these guidelines in mind:

- **Disable when hidden.** Always call `set_spectral_analysis_enabled(false)`
  when the spectral view is not visible.
- **Use a moderate refresh rate.** 15 Hz (default) is sufficient for animated
  bar displays. 30 Hz is appropriate for smooth spectrum visualizations. Avoid
  going above 60 Hz.
- **Expect lazy startup.** FFT plans and ring buffers are created on first
  enable (or after a chain rebuild). The first snapshot may arrive slightly later
  than subsequent ones.

---

## Threading Model and Safety

All metering data flows from the **mix thread** to the **control thread** via
a shared `Arc<Mutex<_>>` store:

```
Mix thread                          Control thread
──────────                          ──────────────
measure → local snapshot            player.effect_levels()
  ↓                                   ↓
try_lock() → publish ────────────→ lock() → clone → return
  │                                   │
  └ if contended: skip silently       └ always succeeds (may block briefly)
```

**Key guarantees:**

- The mix thread **never blocks**. Publication uses `try_lock()` — if the
  control thread is reading, the mix thread skips that publication tick.
- The control thread may block briefly while the mix thread publishes, but
  publications are infrequent (15–60 Hz) and fast (memcpy of pre-sized vectors).
- Enabling/disabling metering uses `Relaxed` atomics. State changes may be
  delayed by one mix-thread chunk (~1–5 ms). This is intentional and correct.

### Polling pattern

The recommended control-thread polling pattern:

```rust
// In your UI render loop or timer callback:
fn update_meters(&self) {
    // Tier 1
    if let Some(levels) = self.player.effect_levels() {
        self.render_level_meters(&levels);
    }

    // Tier 3
    if let Some(bands) = self.player.effect_band_levels() {
        self.render_spectral_bars(&bands);
    }
}

// For Tier 2, query only on parameter change:
fn on_effect_parameter_changed(&self) {
    let responses = self.player.effect_frequency_responses(128);
    self.render_eq_curves(&responses);
}
```

### Handling `None`

All metering getters can return `None`. Handle this gracefully:

| Return value | Meaning                                                    | UI action                    |
| ------------ | ---------------------------------------------------------- | ---------------------------- |
| `None`       | Feature not compiled, runtime-disabled, or not yet started | Show placeholder or hide     |
| `Some([])`   | Active chain has zero effects                              | Show empty state             |
| `Some([..])` | Normal snapshot data                                       | Render meters/curves/spectra |

---

## Effect Chain Changes

When the effect chain changes structurally (effects added, removed, or
reordered):

- **Tier 1:** Snapshot vector resizes automatically. Newly created slots start
  at zero. The UI should expect the snapshot length to change between reads.
- **Tier 2:** Re-query `effect_frequency_responses()` to get curves for the new
  chain layout.
- **Tier 3:** Spectral analyzers rebuild for the new chain. Expect a brief gap
  in spectral data while ring buffers refill.

Match snapshot indices to effect indices in your chain model. If your UI
maintains a list of effect widgets, the snapshot at index `i` corresponds to the
effect at index `i` in `player.effects()`.

---

## Quick Reference

### Player API

```rust
// Tier 1: Level metering
player.set_effect_level_metering_enabled(enabled: bool);
player.set_effect_level_meter_refresh_hz(hz: f32);
player.effect_levels() -> Option<Vec<EffectLevelSnapshot>>;

// Tier 2: Analytical curves (on-demand, no runtime toggle needed)
player.effect_frequency_responses(num_points: usize) -> Vec<Option<FilterResponseCurve>>;

// Tier 3: Spectral analysis
player.set_spectral_analysis_enabled(enabled: bool);
player.set_spectral_analysis_refresh_hz(hz: f32);
player.effect_band_levels() -> Option<Vec<Option<EffectBandSnapshot>>>;
```

### Feature flags

```toml
# Levels + analytical curves
effect-meter = ["proteus-lib/effect-meter"]

# Levels + analytical curves + spectral analysis
effect-meter-spectral = ["proteus-lib/effect-meter-spectral"]
```

### Defaults

| Setting                   | Default |
| ------------------------- | ------- |
| Level metering enabled    | `false` |
| Level refresh Hz          | 30      |
| Spectral analysis enabled | `false` |
| Spectral refresh Hz       | 15      |
