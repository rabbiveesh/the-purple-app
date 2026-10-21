import { describe, expect, it } from 'vitest'

import { defaultPlanOptions, defaultRoutine, sessionPlan } from '@/core'

import { cuesDue, stepAt } from '../player'
import { spokenText } from '../words'

const plan = sessionPlan(defaultRoutine(), defaultPlanOptions())

describe('following the session plan', () => {
  it('starts with five seconds to get ready', () => {
    expect(stepAt(plan, 0)).toMatchObject({ phase: 'getReady', exercise: 1 })
    expect(stepAt(plan, 4_999)?.phase).toBe('getReady')
    expect(stepAt(plan, 5_000)).toMatchObject({ phase: 'squeeze', rep: 1 })
  })

  it('announces the first exercise the moment the session starts', () => {
    const { due } = cuesDue(plan, 0, 0)
    expect(due.map((c) => spokenText(c.say, 5))).toEqual(['Exercise 1 starts in 5 seconds'])
  })

  it('says each cue once, even across many frames', () => {
    let next = 0
    const said: string[] = []
    for (let ms = 0; ms <= 10_000; ms += 16) {
      const r = cuesDue(plan, next, ms)
      next = r.next
      said.push(...r.due.map((c) => spokenText(c.say, 5)))
    }
    expect(said).toEqual([
      'Exercise 1 starts in 5 seconds',
      'Squeeze',
      '2',
      '3',
      '4',
      '5',
      'Release',
    ])
  })
})
