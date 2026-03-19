# ST-02 Project Structure Review

A review of the current directory structure against the STYLE_GUIDE.md, with recommendations for better organization.

## Problem

The project structure is mostly sound and follows the style guide well. However, there are several areas where the current layout diverges from the guide's own principles, contains dead weight, or could be reorganized for better maintainability as the project grows.

The issues fall into three categories: **dead artifacts** that should be removed, **misplaced files** that live in the wrong directory, and **structural improvements** that would better align with the style guide's "group by domain" principle.

---

## 1. Dead Artifacts (Remove)

### 1a. Empty directories in `src/`

Three empty directories exist with no files:

- `src/classes/` -- listed in the style guide as "if needed" but currently unused
- `src/components/playground/` -- not listed in the style guide at all
- `src/components/util/` -- not listed in the style guide at all

**Recommendation:** Delete all three. They add noise to the tree and can be recreated if/when needed. The style guide already documents where `classes/` would go.

### 1b. `src-tauri/src/test.rs`

This file is 18 lines of entirely commented-out code (a sample `ProjectSkeleton` struct). It violates the style guide's rule: _"Remove commented-out code. It lives in version control if you need it later."_

**Recommendation:** Delete `test.rs` and remove the `mod test;` declaration from `main.rs` (if present -- verify before deleting).

---

## 2. Misplaced Files

### 2a. `src/assets/effects.ts` should move to `src/utils/`

`effects.ts` contains pure TypeScript logic: type conversion maps, factory functions for default effect settings, dB conversion math, and serialization helpers. It imports from `../typings/effects` and exports interfaces and functions. It has no static asset characteristics (no CSS, no images, no fonts).

The style guide defines `assets/` as _"Static assets, CSS, and shared logic"_ and explicitly lists `effects.ts` there. However, the style guide also defines `utils/` as _"Pure utility functions"_ -- which is exactly what `effects.ts` contains. The file was likely placed in `assets/` early on before `utils/` existed, and the style guide was written to describe reality rather than prescribe the ideal.

**Recommendation:** Move `effects.ts` to `src/utils/effects.ts`. Update all imports (grep for `assets/effects` to find them). Update the style guide's project structure diagram to reflect the move.

### 2b. Style guide documents `file/mod.rs` but code uses `file.rs`

The style guide's backend structure diagram shows `file/mod.rs` as the module root, but the actual code uses `file.rs` (the alternative Rust module layout). Both are valid Rust, but they should match.

**Recommendation:** Update the style guide diagram to show `file.rs` instead of `file/mod.rs` to match the actual code. No code change needed.

---

## 3. Structural Improvements

### 3a. Create a `player/` submodule in the backend

`player.rs` (469 lines) and `player_runtime.rs` (518 lines) are both over the style guide's hard limit of 500 lines for `.rs` files, and they are closely coupled -- `player.rs` sends commands via the channel that `player_runtime.rs` listens on.

**Current:**

```
src-tauri/src/
├── player.rs              # 469 lines - Tauri command handlers
├── player_runtime.rs      # 518 lines - Actor thread + mpsc dispatch
```

**Proposed:**

```
src-tauri/src/
├── player.rs              # Module root (re-exports, like file.rs does)
├── player/
│   ├── commands.rs         # Tauri command handlers (from current player.rs)
│   ├── runtime.rs          # Actor thread (from current player_runtime.rs)
│   ├── types.rs            # PlayerCommand enum, shared types
│   └── mix.rs              # Track mix and shuffle point helpers
```

This mirrors the pattern already established by `file.rs` + `file/`. Extracting `PlayerCommand` and related types into `types.rs`, and pulling shuffle-point and `build_paths_tracks` helpers into `mix.rs`, would bring both main files well under the 350-line soft limit.

**Recommendation:** Refactor into a `player/` submodule. This also addresses the style guide's note: _"player.rs -- extract `build_paths_tracks` and shuffle-point helpers into separate modules."_

### 3b. Fold `helpers.rs` into `file/utils.rs`

`helpers.rs` contains a single 23-line function: `get_cache_dir()`. This function is file-system-related (resolving and creating the app cache directory) and would fit naturally in `file/utils.rs`, which already contains file-system utility functions.

Having a top-level `helpers.rs` with one function is not harmful, but it adds a module to `main.rs` for minimal benefit and makes the top-level module list less informative.

**Recommendation:** Move `get_cache_dir()` into `file/utils.rs` and re-export it from `file.rs`. Remove `helpers.rs` and its `mod` declaration in `main.rs`. Update all call sites (grep for `helpers::get_cache_dir`).

### 3c. Consider the `analog/` and `digital/` component grouping

The style guide says _"Group by domain, not by technical role."_ The `analog/` and `digital/` directories group by visual metaphor (skeuomorphic knob vs. digital fader), not by feature domain. For example, `DigitalTrackMix.vue` is in `digital/` but is functionally a track-level control.

However, this is a deliberate design system choice -- analog and digital represent two distinct UI paradigms that the app explicitly offers. A `DigitalTrackMix` uses the same visual language as `DigitalFader` and `DigitalPot`, so grouping them together makes sense from a design-system perspective.

**Recommendation:** Keep `analog/` and `digital/` as-is. They represent coherent design-system categories, which is a valid interpretation of "domain." If the app grows and these directories get large, consider nesting them under `controls/` (e.g., `components/controls/analog/`, `components/controls/digital/`) to make the hierarchy clearer. No action needed now.

### 3d. `input/` directory has a single file

`src/components/input/` contains only `InputAutoSizedText.vue`. A directory for a single component is lightweight overhead, but it looks sparse.

**Recommendation:** Leave it for now. If more input components are added (likely as the app grows), the directory will earn its keep. If no new input components appear, consider moving `InputAutoSizedText.vue` to `base/` during a future cleanup.

---

## Summary of Recommended Actions

| Priority | Action                                                              | Effort                                |
| -------- | ------------------------------------------------------------------- | ------------------------------------- |
| Low      | Delete empty dirs: `classes/`, `playground/`, `util/`               | Trivial                               |
| Low      | Delete `test.rs` (commented-out code)                               | Trivial                               |
| Low      | Fix style guide: `file/mod.rs` -> `file.rs`                         | Trivial                               |
| Medium   | Move `assets/effects.ts` -> `utils/effects.ts`                      | Small (update imports)                |
| Medium   | Fold `helpers.rs` into `file/utils.rs`                              | Small (move function, update imports) |
| High     | Refactor `player.rs` + `player_runtime.rs` into `player/` submodule | Medium (module restructure)           |

The **Low** items are cleanup with no risk. The **Medium** items are straightforward moves. The **High** item (player submodule) is the most impactful improvement and directly addresses two of the style guide's noted size violations.

## Acceptance Criteria

- [ ] Empty directories `src/classes/`, `src/components/playground/`, `src/components/util/` are deleted
- [ ] `src-tauri/src/test.rs` is deleted and its mod declaration removed
- [ ] `src/assets/effects.ts` is moved to `src/utils/effects.ts` with all imports updated
- [ ] `STYLE_GUIDE.md` project structure diagrams are updated to reflect: `effects.ts` in `utils/`, `file.rs` instead of `file/mod.rs`, and the new `player/` submodule
- [ ] `src-tauri/src/helpers.rs` is folded into `src-tauri/src/file/utils.rs`
- [ ] `player.rs` and `player_runtime.rs` are refactored into a `player/` submodule with files under the soft limit
- [ ] `bun run build` passes (frontend type check)
- [ ] `bun run lint` passes
- [ ] `cargo build` succeeds from `src-tauri/`
- [ ] App runs and plays audio correctly (manual verification)

Backlog
