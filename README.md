# Pretzel (the-purple-app)

A pelvic floor (kegel) exercise app, with enough gamification that the exercises actually happen. It presents itself as "Pretzel".

What it does and why: [`docs/requirements.md`](docs/requirements.md).

## Layout

| Path | What |
|---|---|
| `core/` | Rust crate with the exercise logic, compiled to WebAssembly. Pure functions, no I/O. |
| `web/` | Vue 3 + Vuetify app (installable PWA). Data is stored in the browser through `web/src/storage`. |

## Development

Requirements: Rust with the `wasm32-unknown-unknown` target, [`wasm-pack`](https://github.com/rustwasm/wasm-pack), Node 22.18+ or 24.

```sh
# Build the core (re-run after changing core/)
cd core && wasm-pack build --release --target bundler --out-dir pkg --out-name purple_core

# Run the app
cd ../web && npm install && npm run dev
```

Tests: `cargo test` in `core/`, `npx vitest run` in `web/`.

## Deploying

Pushes to `master` build and deploy to GitHub Pages through `.github/workflows/ci.yml`. In the repository settings, set **Pages > Source** to **GitHub Actions** once.

See `CLAUDE.md` for the house rules.
