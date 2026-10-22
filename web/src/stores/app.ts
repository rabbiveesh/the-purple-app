import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

import {
  checkInsDue,
  defaultRoutine,
  progressSummary,
  slotForHour,
  withRestBetweenReps,
  type Slot,
} from '@/core'
import { emptyData, loadAppData, saveAppData } from '@/data/repository'
import type {
  AppData,
  CoachMode,
  IsoDate,
  MonthlyCheckIn,
  SessionSurvey,
  WeeklyCheckIn,
} from '@/data/schema'
import { localDate, localHour } from '@/time'

export const useAppStore = defineStore('app', () => {
  const data = ref<AppData>(emptyData())
  const loaded = ref(false)
  const loadError = ref<string | null>(null)
  // The current day; refreshed when the home screen opens and when a session finishes.
  const today = ref<IsoDate>(localDate())
  const prescribed = defaultRoutine()
  /** The prescribed routine with the user's own rest between reps. */
  const routine = computed(() =>
    withRestBetweenReps(prescribed, data.value.settings.restBetweenRepsMs),
  )

  async function load() {
    try {
      const saved = await loadAppData()
      if (saved) {
        data.value = saved
      } else {
        await saveAppData(data.value)
      }
      loaded.value = true
    } catch (e) {
      loadError.value = e instanceof Error ? e.message : String(e)
    }
  }

  async function update(change: (d: AppData) => void) {
    change(data.value)
    await saveAppData(data.value)
  }

  function refreshToday() {
    today.value = localDate()
  }

  const summary = computed(() =>
    progressSummary({ records: data.value.sessions, today: today.value, routine: routine.value }),
  )

  const dueCheckIns = computed(() =>
    checkInsDue({
      records: data.value.sessions,
      lastWeekly: data.value.weeklyCheckIns.at(-1)?.date,
      lastMonthly: data.value.monthlyCheckIns.at(-1)?.date,
      today: today.value,
    }),
  )

  /** Records a coached session that just finished, in the slot it started in. */
  async function finishSession(startedAt: Date): Promise<{ date: IsoDate; slot: Slot }> {
    const done = { date: localDate(startedAt), slot: slotForHour(localHour(startedAt)) }
    refreshToday()
    await update((d) => d.sessions.push({ ...done, markedByHand: false }))
    return done
  }

  function isDone(date: IsoDate, slot: Slot) {
    return data.value.sessions.some((s) => s.date === date && s.slot === slot)
  }

  /** Marks a slot done by hand, or takes back a slot that was marked by hand. */
  async function toggleByHand(date: IsoDate, slot: Slot) {
    const byHand = data.value.sessions.findIndex(
      (s) => s.date === date && s.slot === slot && s.markedByHand,
    )
    if (byHand >= 0) {
      await update((d) => d.sessions.splice(byHand, 1))
    } else if (!isDone(date, slot)) {
      await update((d) => d.sessions.push({ date, slot, markedByHand: true }))
    }
  }

  const saveSessionSurvey = (s: SessionSurvey) => update((d) => d.sessionSurveys.push(s))
  const saveWeeklyCheckIn = (c: WeeklyCheckIn) => update((d) => d.weeklyCheckIns.push(c))
  const saveMonthlyCheckIn = (c: MonthlyCheckIn) => update((d) => d.monthlyCheckIns.push(c))
  const setRestBetweenReps = (ms: number) => update((d) => (d.settings.restBetweenRepsMs = ms))
  const rememberCoachMode = (mode: CoachMode) => update((d) => (d.lastCoachMode = mode))

  /** Replaces everything with a backup (already migrated by the caller). */
  async function restore(backup: AppData) {
    data.value = backup
    await saveAppData(data.value)
  }

  return {
    data,
    loaded,
    loadError,
    today,
    routine,
    summary,
    dueCheckIns,
    load,
    refreshToday,
    finishSession,
    isDone,
    toggleByHand,
    saveSessionSurvey,
    saveWeeklyCheckIn,
    saveMonthlyCheckIn,
    rememberCoachMode,
    setRestBetweenReps,
    restore,
  }
})
