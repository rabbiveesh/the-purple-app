// The shape of everything the app stores. Bump SCHEMA_VERSION and add a step to
// migrations.ts whenever it changes.
import type { SessionRecord, Slot } from '@/core'

export const SCHEMA_VERSION = 2

/** Dates are local calendar days, `YYYY-MM-DD`. */
export type IsoDate = string

export type CoachMode = 'silent' | 'voiceAndScreen' | 'voiceOnly'

/** The quick questions after a coached session. */
export type SessionSurvey = {
  date: IsoDate
  slot: Slot
  /** 1 (easy) to 10 (very hard). */
  difficulty: number
  heldFullCount: 'all' | 'most' | 'some' | 'few'
  lastAsStrongAsFirst: 'yes' | 'bitWeaker' | 'muchWeaker'
  fullyRelaxed?: 'yes' | 'notReally'
}

export type LeakSituation = 'coughSneeze' | 'laugh' | 'jumpRun' | 'lifting' | 'suddenUrge'

export type WeeklyCheckIn = {
  date: IsoDate
  leaks: 'none' | 'onceOrTwice' | 'aFewTimes' | 'mostDays'
  situations: LeakSituation[]
  /** 0 (not at all) to 10 (a lot). */
  bother: number
  heaviness?: 'never' | 'sometimes' | 'often'
}

/** "Compared with when you started, how are things?" from much worse (-3) to much better (3). */
export type MonthlyCheckIn = {
  date: IsoDate
  comparedWithStart: -3 | -2 | -1 | 0 | 1 | 2 | 3
}

export type AppData = {
  version: number
  sessions: SessionRecord[]
  sessionSurveys: SessionSurvey[]
  weeklyCheckIns: WeeklyCheckIn[]
  monthlyCheckIns: MonthlyCheckIn[]
  lastCoachMode?: CoachMode
}
