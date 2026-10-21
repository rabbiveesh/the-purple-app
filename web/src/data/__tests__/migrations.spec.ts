import { describe, expect, it } from 'vitest'

import { migrate, UnsupportedDataError } from '../migrations'
import { emptyData } from '../repository'
import { SCHEMA_VERSION } from '../schema'

describe('saved data', () => {
  it('loads data saved by the current version unchanged', () => {
    const saved = {
      ...emptyData(),
      sessions: [{ date: '2026-10-21', slot: 'morning' as const, markedByHand: false }],
    }
    expect(migrate(structuredClone(saved))).toEqual(saved)
  })

  it('upgrades the first, empty version to an empty exercise history', () => {
    expect(migrate({ version: 1 })).toEqual(emptyData())
  })

  it('refuses data from a newer version of the app instead of guessing', () => {
    expect(() => migrate({ version: SCHEMA_VERSION + 1 })).toThrow(UnsupportedDataError)
  })

  it('refuses data with no version number', () => {
    expect(() => migrate({})).toThrow(UnsupportedDataError)
  })
})
