// The user's local calendar day and hour. Sessions belong to the day on her phone's clock.
import type { IsoDate } from '@/data/schema'

/** `YYYY-MM-DD` in local time (the en-CA locale formats dates that way). */
export function localDate(at: Date = new Date()): IsoDate {
  return at.toLocaleDateString('en-CA')
}

export function localHour(at: Date = new Date()): number {
  return at.getHours()
}
