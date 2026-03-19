# Proteus Author Style Guide

This document defines the coding standards and architectural guidelines for the Proteus Author project. Its purpose is to keep the codebase maintainable, readable, and scaleable as the project grows. New code **must** follow these guidelines; existing code should be brought into compliance opportunistically during related changes.

---

## Table of Contents

1. [Project Structure](#1-project-structure)
2. [File and Function Size Limits](#2-file-and-function-size-limits)
3. [Frontend (Vue + TypeScript)](#3-frontend-vue--typescript)
4. [Backend (Rust + Tauri)](#4-backend-rust--tauri)
5. [State Management](#5-state-management)
6. [IPC Contract (Frontend ↔ Backend)](#6-ipc-contract-frontend--backend)
7. [Naming Conventions](#7-naming-conventions)
8. [Error Handling](#8-error-handling)
9. [Logging and Debug Output](#9-logging-and-debug-output)
10. [Styling and CSS](#10-styling-and-css)
11. [Testing](#11-testing)
12. [Code Hygiene](#12-code-hygiene)

---

## 1. Project Structure

### Frontend (`src/`)

```
src/
├── main.ts                    # App entry point (bootstrap only)
├── App.vue                    # Root shell and global listeners
├── components/
│   ├── analog/                # Physical-style mixer controls
│   ├── base/                  # Layout, transport, alerts, title
│   ├── digital/               # Digital-style controls (faders, pots)
│   ├── effects/               # Effects chain UI
│   ├── input/                 # Form inputs
│   └── track/                 # Track waveform and bin
├── composables/               # Vue composables (useXxx pattern)
├── stores/                    # Pinia stores
├── typings/                   # TypeScript type declarations (.d.ts)
├── assets/                    # Static assets (CSS, fonts, images)
├── utils/                     # Pure utility functions
│   ├── effects.ts             # Effect factories and serialization
│   └── startup-trace.ts       # Startup performance tracing
└── classes/                   # Domain model classes (if needed)
```

**Rules:**

- **One component per file.** Each `.vue` file exports a single component.
- **Group by domain, not by technical role.** Components live in folders named after the feature area they serve (`effects/`, `track/`), not generic buckets (`buttons/`, `modals/`).
- **Barrel exports (`index.ts`) are optional.** Use them only when a folder's components are frequently imported together. Do not create barrels for folders with fewer than three public exports.
- **Composables** go in `src/composables/` and follow the `useXxx` naming convention.
- **Type-only files** go in `src/typings/` as `.d.ts` files. Types that are tightly coupled to a single module may live in that module instead.

### Backend (`src-tauri/src/`)

```
src-tauri/src/
├── main.rs                    # Tauri bootstrap and invoke handler registration
├── project.rs                 # Project data structures and state helpers
├── effects.rs                 # Effect decoding
├── peaks.rs                   # Waveform peak extraction
├── menu.rs                    # App menu construction and event handling
├── windows.rs                 # Window creation
├── startup.rs                 # Startup tracing
├── alerts.rs                  # Alert emission helpers
├── player.rs                  # Player submodule root (re-exports)
├── player/
│   ├── commands.rs            # Tauri command handlers for player operations
│   ├── runtime.rs             # Player actor and worker thread
│   ├── api.rs                 # Channel-based API for sending commands to the worker
│   ├── types.rs               # PlayerCommand enum and shared types
│   └── mix.rs                 # Track building and shuffle-point helpers
├── file.rs                    # File submodule root (re-exports)
└── file/
    ├── export.rs              # .prot export pipeline
    ├── project_io.rs          # Project load/save
    ├── registry.rs            # File registry and missing-file recovery
    ├── waveform.rs            # Waveform peak calculation
    ├── utils.rs               # File and cache utility functions
    └── types.rs               # Shared file types
```

**Rules:**

- **One Tauri command module per domain.** Group related `#[tauri::command]` handlers together (e.g., all player commands in `player.rs`). Do not scatter unrelated commands across files.
- **Separate command handlers from business logic.** Command handlers should be thin — extract, validate, delegate, respond. Heavy logic belongs in dedicated functions or modules.
- **Use submodules (`file/`) for cohesive feature areas** that span multiple files.

---

## 2. File and Function Size Limits

| Metric                    | Soft Limit | Hard Limit | Action                                     |
| ------------------------- | ---------- | ---------- | ------------------------------------------ |
| Lines per `.vue` file     | 300        | 500        | Split into child components or composables |
| Lines per `.ts` file      | 250        | 400        | Extract helpers, split module              |
| Lines per `.rs` file      | 350        | 500        | Extract into submodule                     |
| Lines per function/method | 40         | 60         | Extract subroutines                        |
| Pinia store exports       | 25         | 35         | Split into focused stores                  |

When a file approaches the soft limit, plan to refactor. When it hits the hard limit, refactor before adding more code.

**Currently over limits (track for opportunistic cleanup):**

- `src/components/effects/EffectsDialog.vue` (~640 lines) — extract per-effect-type computed properties into a composable or helper.
- `src/components/track/TrackWaveform.vue` (~527 lines) — extract canvas drawing logic into a utility.
- `src/App.vue` (~517 lines) — extract window listener registration and missing-file logic into composables.
- `src/stores/audio.ts` (~407 lines) — split view/zoom logic from playback/effects.

---

## 3. Frontend (Vue + TypeScript)

### Component Conventions

- Use `<script setup lang="ts">` exclusively. Do not use the Options API.
- Define props with `defineProps<Props>()` using a named interface.
- Define emits with `defineEmits<Emits>()` when a component emits events.
- Prefer `computed()` for derived state. Avoid storing derived values in `ref()`.
- Keep template expressions simple — move complex logic to computed properties or functions in `<script>`.
- Lazy-load heavy components with `defineAsyncComponent()` when they are not needed at first paint.

### TypeScript

- Enable and respect `strict` mode. Do not use `// @ts-ignore` or `// @ts-expect-error` without a comment explaining why.
- Prefer type inference where it is clear. Add explicit annotations on function signatures, exported values, and anywhere the type is not obvious.
- Avoid `as` type assertions. Use type guards or proper generic typing instead.

  ```ts
  // Avoid
  const scale = ref(20 as number)
  const tracks = ref([] as Track[])

  // Prefer
  const scale = ref(20)
  const tracks = ref<Track[]>([])
  ```

- Do not use `any`. Use `unknown` and narrow with type guards when the type is truly dynamic.

### Imports

- Group imports in this order, separated by blank lines:
  1. External libraries (`vue`, `@tauri-apps/*`, `pinia`, etc.)
  2. Internal modules (`../stores/`, `../typings/`, `../utils/`)
  3. Sibling/child imports (`./ChildComponent.vue`)
- Use the existing ESLint + Prettier configuration. Run `bun run lint` before committing.

### Reducing Boilerplate

When multiple effect types (or similar variants) produce near-identical computed getters/setters, extract a factory:

```ts
// Preferred: factory for effect property bindings
function effectProp<T>(settings: () => Record<string, T> | undefined, key: string, fallback: T) {
  return computed({
    get: () => settings()?.[key] ?? fallback,
    set: (value: T) => {
      const s = settings()
      if (s) s[key] = value
    },
  })
}
```

This eliminates the hundreds of lines of repetitive `computed({ get, set })` blocks in files like `EffectsDialog.vue`.

---

## 4. Backend (Rust + Tauri)

### Command Handlers

- Keep `#[tauri::command]` functions thin. They should:
  1. Extract state from `Window` / `State<T>`
  2. Validate inputs
  3. Call a domain function
  4. Return a result
- Avoid deeply nested logic inside command handlers.

### Error Handling

- Prefer `Result<T, E>` returns from command handlers over silent failures and `println!`.
- Use Tauri's `Result` pattern so errors are surfaced to the frontend:
  ```rust
  #[tauri::command]
  fn my_command() -> Result<String, String> {
      do_work().map_err(|e| e.to_string())
  }
  ```
- Minimize `.unwrap()`. Use `.unwrap_or_default()`, `?`, or explicit error handling. Reserve `.unwrap()` for cases where the invariant is truly guaranteed and a panic is acceptable.
- When `.expect()` is used, the message should explain **why** the value is expected to be present, not just restate the operation.

### Patterns

- Use `Arc<Mutex<T>>` sparingly. Prefer message-passing (as in `player/runtime.rs`) for state that is accessed from multiple threads.
- Group related `use` statements. Place `std` imports first, then external crates, then internal modules.
- Avoid wildcard imports (`use crate::player::*`) in files other than `main.rs`. Wildcard re-exports in `main.rs` are acceptable for registering invoke handlers.

---

## 5. State Management

### Pinia Stores

- Use the **Composition API** (`defineStore('name', () => { ... })`) for all stores. Do not mix in Options API stores.
- Organize store internals in this order:
  1. Dependency stores and constants
  2. Reactive state (`ref`, `reactive`)
  3. Computed getters
  4. Actions/mutations
  5. Watchers (if any)
  6. `return` block
- **Do not create trivial wrappers.** If a computed just returns a ref's `.value`, expose the ref directly instead:

  ```ts
  // Avoid
  const playing = ref(false)
  const isPlaying = computed(() => playing.value)

  // Prefer: expose `playing` directly, or rename the ref to `isPlaying`
  const isPlaying = ref(false)
  ```

- **Keep stores focused.** A store should manage one domain concern. If a store exceeds 25 exports, consider splitting it. For example, the audio store could split view/zoom state from playback/effects state.
- **Use consistent function syntax.** Prefer arrow functions for short actions and named functions for complex logic, but be consistent within a single store.

### Tauri Managed State

- Each piece of managed state should have a dedicated constructor (`create_xxx_state()`) called in `main.rs`.
- Window-scoped state uses label-keyed `HashMap`s behind `Mutex`.
- Always clean up window state on `WindowEvent::Destroyed`.

---

## 6. IPC Contract (Frontend ↔ Backend)

- **Type definitions must stay in sync.** When a Rust struct used in IPC changes, update the corresponding TypeScript type in `src/typings/`.
- Use `serde(rename_all = "camelCase")` on Rust IPC structs to match JavaScript conventions, or document the exact casing contract.
- IPC command names use `snake_case` on the Rust side (`get_play_state`). The frontend invokes them with the same snake_case string.
- Define return types explicitly on all `invoke<T>()` calls. Do not use untyped `invoke()`.

---

## 7. Naming Conventions

| Context                        | Convention                                      | Example                                  |
| ------------------------------ | ----------------------------------------------- | ---------------------------------------- |
| Vue components                 | PascalCase filename and tag                     | `TrackWaveform.vue`, `<TrackWaveform />` |
| TypeScript files               | camelCase                                       | `useAppShortcuts.ts`                     |
| TypeScript variables/functions | camelCase                                       | `setTrackLevel`                          |
| TypeScript types/interfaces    | PascalCase                                      | `ProjectSkeleton`                        |
| TypeScript enums               | PascalCase members                              | `AudioEffectType`                        |
| Pinia stores                   | `useXxxStore`                                   | `useAudioStore`                          |
| Composables                    | `useXxx`                                        | `useAppShortcuts`                        |
| Rust files                     | snake_case                                      | `player_runtime.rs`                      |
| Rust types/enums               | PascalCase                                      | `PlayerCommand`                          |
| Rust functions                 | snake_case                                      | `build_paths_tracks`                     |
| Rust constants                 | SCREAMING_SNAKE_CASE                            | `TRACK_LEVEL_MAX`                        |
| CSS classes                    | Tailwind utilities or kebab-case custom classes | `shuffle-point-cursor-add`               |
| Tauri events                   | SCREAMING_SNAKE_CASE                            | `FILE_LOADED`, `UPDATE_PLAYHEAD`         |
| IPC commands                   | snake_case                                      | `get_project_state`                      |

---

## 8. Error Handling

### Frontend

- Wrap `invoke()` calls in try/catch when failure is possible and the user needs feedback.
- Use the alert store (`useAlertStore`) to surface errors to the user. Do not silently swallow failures.
- Prefer early returns for guard clauses over deeply nested conditionals.

### Backend

- Use `Result` returns from command handlers. Use `String` or a custom error type for the error variant.
- Log errors with the `log` crate (`log::error!`, `log::warn!`), not `println!` or `eprintln!`.
- Never `panic!` or `.unwrap()` on user-controlled input (file paths, IPC payloads, etc.).

---

## 9. Logging and Debug Output

### Rules

- **No `println!` in production Rust code.** Use the `log` crate (`info!`, `debug!`, `warn!`, `error!`). The `tauri-plugin-log` is already configured.

  ```rust
  // Avoid
  println!("Setting Effects: {:?}", effects);

  // Prefer
  log::debug!("Setting effects: {:?}", effects);
  ```

- **No `console.log` in committed frontend code** unless it is behind a debug flag or temporary during development. Remove before merging.

  ```ts
  // Avoid
  console.log(projectState)

  // Prefer: remove entirely, or use a debug utility
  ```

- **Use structured logging** when context is important:
  ```rust
  log::info!(target: "player", "init_player completed in {}ms", elapsed);
  ```

### Stale Debug Output (Current Violations)

The following files contain `println!` or `console.log` statements that should be converted or removed:

- `src-tauri/src/file/export.rs` — `println!` for command output and settings
- `src-tauri/src/file/project_io.rs` — `println!` for save/open operations
- `src-tauri/src/main.rs` — `println!("Hello, world!")`
- `src/stores/audio.ts` — `console.log('refreshLevels')`
- `src/stores/track.ts` — `console.log(projectState)`
- `src/App.vue` — `console.log` for save and play state

---

## 10. Styling and CSS

- **Tailwind CSS is the primary styling approach.** Use utility classes in templates.
- **Avoid inline styles** (`style="..."`) except for dynamic values that cannot be expressed as classes (e.g., computed pixel positions).
- **Use CSS custom properties** for theme values and dynamic layout dimensions (e.g., `--meter-width`, `--effect-rack-height`).
- **Scoped styles** (`<style scoped>`) are the default for component-specific CSS.
- **Global styles** go in `src/assets/` (`index.css`, `theme.css`, `analog.css`).
- **Do not use `!important`** unless overriding third-party library styles. Annotate with a comment explaining why.
- **Inline SVG data URIs** (as in cursor definitions) are acceptable for small, self-contained assets. For anything larger than ~500 characters, extract to a file.

---

## 11. Testing

### Current State

There is no JavaScript test framework configured. Rust tests are minimal.

### Target State

- **Type checking is mandatory.** `bun run build` (which runs `vue-tsc --noEmit`) must pass before merging.
- **Linting is mandatory.** `bun run lint` must pass before merging.
- **Rust compilation must succeed.** `cargo build` from `src-tauri/` must complete without warnings treated as errors.
- **Manual testing** is expected for UI changes. Include screenshots or recordings in PRs when applicable.
- When a JS test framework is introduced, prioritize tests for:
  1. Pure utility functions (`src/utils/effects.ts`, serialization logic)
  2. Store logic (Pinia store actions with mocked `invoke`)
  3. Complex computed properties

---

## 12. Code Hygiene

### Commented-Out Code

- **Remove commented-out code.** It lives in version control if you need it later. Do not leave blocks of commented-out imports, functions, or plugin registrations in the source.
  ```rust
  // Avoid leaving these around:
  // .plugin(tauri_plugin_clipboard_manager::init())
  // .plugin(tauri_plugin_http::init())
  ```

### Dead Code

- Remove unused variables, imports, and functions. Configure `#[allow(dead_code)]` only for intentionally-reserved public API surface, with a comment explaining why.

### Magic Numbers

- Extract meaningful constants for values that appear in logic:

  ```ts
  // Avoid
  if (ratio < 0.8) return
  const targetPlayheadRatio = 0.35

  // Prefer
  const FOLLOW_MODE_TRIGGER_RATIO = 0.8
  const FOLLOW_MODE_TARGET_RATIO = 0.35
  ```

- Numeric values in component props (like knob min/max/step) are fine — they are self-documenting in context.

### Duplication

- When the same pattern appears three or more times, extract it. This applies especially to:
  - Effect-type dispatch chains (use lookup tables or factories)
  - `invoke()` + error handling patterns
  - Repetitive computed getter/setter pairs

### Commits

- Use short, imperative commit messages: "Add shuffle point support", "Fix export path handling".
- Reference related issues when applicable.
- Keep commits focused on a single logical change.

---

## Appendix: Tooling Reference

| Tool         | Command                            | Purpose                         |
| ------------ | ---------------------------------- | ------------------------------- |
| Dev server   | `bun run dev`                      | Start Vite dev server           |
| Build        | `bun run build`                    | Type-check and build frontend   |
| Lint         | `bun run lint`                     | ESLint on `src/`                |
| Tauri dev    | `bun run tauri dev`                | Full desktop app in dev mode    |
| Rust check   | `cargo check` (from `src-tauri/`)  | Fast Rust compilation check     |
| Rust build   | `cargo build` (from `src-tauri/`)  | Full Rust build                 |
| Version bump | `bun run bump:patch\|minor\|major` | Update version in all manifests |

### Formatter Configuration

- **Indent:** 2 spaces (TS/Vue/CSS), 4 spaces (Rust via `rustfmt` default)
- **Line width:** 100 characters (TS/Vue), 100 characters (Rust)
- **Semicolons:** None (TypeScript — enforced by Prettier)
- **Quotes:** Single quotes (TypeScript), double quotes (Rust)
- **Trailing commas:** All (TypeScript)
