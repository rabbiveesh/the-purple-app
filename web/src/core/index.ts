// The app's only entry point into the Rust exercise core (../../core).
// Components and stores import from here, never from 'purple-core' directly,
// so the core can be wrapped, mocked or moved behind a server without touching callers.
export * from 'purple-core'
