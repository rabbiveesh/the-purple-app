# The Purple App

A pelvic floor (kegel) exercise app for one user, with enough gamification that the exercises actually happen. Product requirements live in the project's shared files (`requirements/requirements.md`). Until they exist, don't build features: ask.

## How work arrives

Most requests come from a non-technical user. For her requests:

- **Pure vibe coding: commit straight to `master`.** No branches, no pull requests, no review. CI deploys `master` to GitHub Pages, so a pushed change is live a few minutes later.
- Make every technical decision yourself, following the rules below. Never ask her about code, libraries, data models or git. Ask only about what the app should do or look like, in plain words, and show her the result.
- CI is the safety net, so keep it green: run the checks below before pushing, and if a push breaks CI, fix it right away.
- Tell her in plain language what changed and where to see it. No jargon, file names or diffs.

Requests from the developer follow normal practice (a branch and a pull request) unless they say otherwise.

## No personal names

Never put anyone's name in the repository: not in code, data, docs, tests, commit messages or UI copy. Say "the user". Anything personal (a name to greet, a goal) is user data entered in the app, never code.

## Golden rule: never reinvent wheels

Kittens die when you do that. Before writing any non-trivial utility, look for a well-maintained library that already does it and use that. Hand-rolled code is only for logic that is genuinely specific to this app (the exercise and progress rules).

## Do things the right way, not the short way

The short way leads to more dead kittens. Prefer the properly typed, tested, maintainable approach even when a shortcut would work today. For example, TypeScript types for the Rust core are generated from the Rust structs (tsify), not hand-copied. Values cross the wasm boundary as typed objects (tsify `Ts<T>`), not JSON strings. CI runs the tests and lints. Vibe coding sets the pace, not the quality bar.

## Stack

- **Core logic: Rust** (`core/`), compiled to WebAssembly with `wasm-pack`. Pure functions over data: exercise routines, session timing, streaks, levels and rewards. No I/O, so it can be lifted directly into a future backend.
- **Frontend: Vue 3 + TypeScript SPA** (`web/`), built with Vite.
  - Component framework: **Vuetify** (mobile-first, large touch targets).
  - State: **Pinia**. Routing: **vue-router** (hash history, for GitHub Pages).
  - **vite-plugin-wasm** loads the Rust core. **vite-plugin-pwa** makes it installable and offline. Shipping as a PWA is deliberate: it's the simplest way to ship.
- **Storage: unstorage**, behind the app's own storage module (`web/src/storage`). Today it uses the localStorage driver. Moving to a backend later means swapping the driver, not rewriting callers. Keep every read and write going through that module, via `web/src/data/repository.ts`.
- **No backend.** Data lives in the browser.
- **Hosting: GitHub Pages**, deployed by GitHub Actions on every push to `master`.
- **AI (Claude API): keep it to a minimum**, since every call costs money.

## Checks before every push

```sh
cd core && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
cd core && wasm-pack build --release --target bundler --out-dir pkg --out-name purple_core
cd web && npx prettier --check src/ && npx oxlint . && npx eslint . && npx vitest run && npm run build
```

## Product principles

Provisional until the requirements are written; the requirements win where they differ.

- Private and discreet: nothing about the exercises shows outside the app (the name, icon and any notifications stay neutral).
- Gamification serves the habit. It should make doing today's session the easy choice, never shame a missed day.
- Big, clear views; one obvious thing to do on each screen.

## Assume you're wrong

When you confidently assume you're right, you kill kittens. Assume you're wrong about the requirements, about how the user interacts with the app, and about the data model. Shape the code so that being wrong has the smallest possible blast radius. A simple change that turns into a big one also kills kittens. In practice:

- Keep modules small, with narrow interfaces, so one wrong idea is fixed in one place.
- Version stored data and migrate it explicitly (`web/src/data/migrations.ts`), because the data model will change. Never break data already on her phone.
- Keep the app's UI decoupled from the exercise rules and from storage.
- Prefer configuration and data over hard-coded behaviour (routines, durations, rewards).
- Don't embed things in the wrong place. Exercise and progress rules live in the Rust core, never in Vue components. Persistence goes only through the storage module. Putting something where it's convenient instead of where it belongs is a shortcut, and shortcuts kill kittens.

## Abstractions follow the data, and compose cleanly

Abstractions that aren't about the data make programming harder, not easier, and they kill kittens. So do things that don't compose cleanly. Name and shape modules after the domain data (exercises, sessions, streaks, rewards), not after generic machinery. Prefer small pure functions that take data and return data, and combine them, over frameworks, managers, wrappers or class hierarchies. If a piece can't be reused without dragging its surroundings along, it's shaped wrong.

Interfaces must not lie. A name, type or function must do exactly what it claims, no more and no less. Jumping through interfaces that pretend to be something they aren't is the worst kitten killer of all.

## Tests express intent

Test names say what the user would notice ("finishing today's session extends the streak"), not which function was called.

## Kitten death toll

Finish every piece of work for the developer (each PR, each reply that reports finished work) with a **kitten death toll**: an honest count of the shortcuts taken and wheels reinvented, with each one named, why it happened, and what would fix it. Zero is the goal. Never round down. For the non-technical user, keep the toll in the commit message instead of the reply.
