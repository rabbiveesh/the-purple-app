// The user's local calendar day and hour. Sessions belong to the day on her phone's clock.
import { formatISO } from 'date-fns'

import type { IsoDate } from '@/data/schema'

/** `YYYY-MM-DD` of the local calendar day. */
export function localDate(at: Date = new Date()): IsoDate {
  return formatISO(at, { representation: 'date' })
}

export function localHour(at: Date = new Date()): number {
  return at.getHours()
}
