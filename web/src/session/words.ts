// Everything the coach says or shows, in one place. The word for the exercises never appears:
// the app is "Pretzel" to anyone glancing at the phone.
import type { Movement, Phase, Say } from '@/core'

export function spokenText(say: Say, getReadySeconds: number): string {
  switch (say.kind) {
    case 'getReady':
      return `Exercise ${say.exercise} starts in ${getReadySeconds} seconds`
    case 'squeeze':
      return 'Squeeze'
    case 'release':
      return 'Release'
    case 'rest':
      return 'Rest'
    case 'count':
      return String(say.n)
    case 'exerciseDone':
      return 'Done'
    case 'sessionDone':
      return 'All done. Pretzel time!'
  }
}

export const phaseLabel: Record<Phase, string> = {
  getReady: 'Get ready',
  squeeze: 'Squeeze',
  release: 'Release',
  rest: 'Rest',
  exerciseDone: 'Done!',
}

export function movementName(movement: Movement): string {
  switch (movement.kind) {
    case 'slowSqueeze':
      return 'Slow squeezes'
    case 'hold':
      return 'Long holds'
    case 'quickSqueezes':
      return 'Quick squeezes'
  }
}
