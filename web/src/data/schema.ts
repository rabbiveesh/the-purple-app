// The shape of everything the app stores. Bump SCHEMA_VERSION and add a step to
// migrations.ts whenever it changes.

export const SCHEMA_VERSION = 1

// Empty until the first feature: the requirements are still being gathered.
export type AppData = {
  version: number
}
