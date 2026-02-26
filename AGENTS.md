# Repository Guidelines

## Project Structure & Module Organization
The app is a Vue 3 + TypeScript + Vite frontend with a Tauri backend.
- `src/` contains the UI and client logic.
- `src/App.vue` is the main shell; `src/main.ts` bootstraps the app.
- `src/components/`, `src/classes/`, `src/stores/`, and `src/utils/` hold UI, domain logic, state, and helpers.
- `src/assets/` holds static assets.
- `src-tauri/` contains the Rust/Tauri desktop shell.
- `scripts/` contains repo utilities (e.g., version bumps).

## Build, Test, and Development Commands
Use Bun (preferred) or npm.
- `bun install` installs dependencies.
- `bun run dev` starts the Vite dev server.
- `bun run build` type-checks (`vue-tsc --noEmit`) and builds the app.
- `bun run preview` serves the production build locally.
- `bun run lint` runs ESLint on `src/`.
- `bun run tauri` runs Tauri CLI commands (see `src-tauri/` for config).
- `bun run bump:patch|bump:minor|bump:major` updates the app version.

## Coding Style & Naming Conventions
- TypeScript + Vue SFCs (`.vue`) are the primary source files.
- Indentation follows Prettier defaults (2 spaces) and is enforced via ESLint + Prettier.
- Use `PascalCase` for Vue components and classes, `camelCase` for functions/variables, and `kebab-case` for file names where appropriate.
- Use Nuxt UI for frontend components. When needed, reference Nuxt UI docs in `nuxtui-docs/content`.
- Run `bun run lint` before submitting changes.

## Testing Guidelines
There is no dedicated JS test framework configured. Manual testing and type-checking are expected.
- Run `bun run build` to validate TypeScript types.
- Rust tests may exist in `src-tauri/src/` (run via `cargo test` from `src-tauri/` if needed).

## Commit & Pull Request Guidelines
Recent commits use short, imperative summaries (e.g., “Update App.vue”). Keep messages concise and action-focused.
For PRs:
- Include a clear description of changes and rationale.
- Link related issues when applicable.
- Provide screenshots or screen recordings for UI changes.

## Security & Configuration Tips
- Tauri settings live in `src-tauri/` and `tauri.conf.json` (if edited, note in your PR).
- Avoid committing secrets; use environment variables or local config files instead.
