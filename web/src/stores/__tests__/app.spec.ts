import { beforeEach, describe, expect, it } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import unstorageMemory from 'unstorage/drivers/memory'

import { loadAppData } from '@/data/repository'
import { useDriver } from '@/storage'
import { useAppStore } from '@/stores/app'
import { localDate } from '@/time'

beforeEach(async () => {
  useDriver(unstorageMemory())
  setActivePinia(createPinia())
  await useAppStore().load()
})

describe('progress', () => {
  it('finishing a morning session fills the morning circle and earns pretzel points', async () => {
    const app = useAppStore()
    const morning = new Date()
    morning.setHours(9, 0, 0, 0)
    await app.finishSession(morning)

    expect(app.summary.today).toMatchObject({ morning: true, afternoon: false })
    expect(app.summary.pretzelPoints).toBe(10)
    expect(app.summary.streakDays).toBe(1)
  })

  it('is saved on the phone', async () => {
    const app = useAppStore()
    await app.finishSession(new Date())
    expect((await loadAppData())?.sessions).toHaveLength(1)
  })

  it('a session done without the app can be ticked off by hand, and unticked', async () => {
    const app = useAppStore()
    const today = localDate()
    await app.toggleByHand(today, 'afternoon')
    expect(app.isDone(today, 'afternoon')).toBe(true)
    await app.toggleByHand(today, 'afternoon')
    expect(app.isDone(today, 'afternoon')).toBe(false)
  })

  it('a coached session cannot be unticked by accident', async () => {
    const app = useAppStore()
    const afternoon = new Date()
    afternoon.setHours(16, 0, 0, 0)
    const { date, slot } = await app.finishSession(afternoon)
    await app.toggleByHand(date, slot)
    expect(app.isDone(date, slot)).toBe(true)
  })

  it('asks the weekly check-in once there is something to check in about', async () => {
    const app = useAppStore()
    expect(app.dueCheckIns.weekly).toBe(false)
    await app.finishSession(new Date())
    expect(app.dueCheckIns.weekly).toBe(true)
    await app.saveWeeklyCheckIn({ date: app.today, leaks: 'none', situations: [], bother: 0 })
    expect(app.dueCheckIns.weekly).toBe(false)
  })
})
