import { describe, expect, it } from 'vitest'

import { localDate, localHour } from '@/time'

describe("today's date", () => {
  it('a session just before midnight stays on that day', () => {
    const lateNight = new Date(2026, 9, 21, 23, 59, 59)
    expect(localDate(lateNight)).toBe('2026-10-21')
    expect(localHour(lateNight)).toBe(23)
  })

  it('just after midnight is the next day', () => {
    expect(localDate(new Date(2026, 9, 22, 0, 0, 1))).toBe('2026-10-22')
  })
})
