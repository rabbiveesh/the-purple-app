// Plays a session plan against the clock: the current step, the cues as their moment passes,
// and pausing. All timing rules come from the core's plan; this only follows it.
import { useRafFn } from '@vueuse/core'
import { computed, ref } from 'vue'

import type { Cue, SessionPlan, Step } from '@/core'

/** The step running at `ms` into the session (steps are back to back and sorted). */
export function stepAt(plan: SessionPlan, ms: number): Step | undefined {
  return plan.steps.findLast((s) => s.startMs <= ms)
}

/** The cues from index `next` on whose moment has come by `ms`, and the index after them. */
export function cuesDue(plan: SessionPlan, next: number, ms: number): { due: Cue[]; next: number } {
  let end = next
  while (end < plan.cues.length && plan.cues[end]!.atMs <= ms) end++
  return { due: plan.cues.slice(next, end), next: end }
}

export function usePlayer(
  plan: SessionPlan,
  handlers: { onCue: (cue: Cue) => void; onFinish: () => void },
) {
  const elapsedMs = ref(0)
  const paused = ref(false)
  const finished = ref(false)
  let nextCue = 0
  let lastFrame: number | null = null

  function advance(now: number) {
    const delta = lastFrame === null ? 0 : now - lastFrame
    lastFrame = now
    elapsedMs.value = Math.min(plan.totalMs, elapsedMs.value + delta)

    const { due, next } = cuesDue(plan, nextCue, elapsedMs.value)
    nextCue = next
    due.forEach(handlers.onCue)

    if (elapsedMs.value >= plan.totalMs && !finished.value) {
      finished.value = true
      raf.pause()
      handlers.onFinish()
    }
  }

  const raf = useRafFn(({ timestamp }) => advance(timestamp), { immediate: false })

  /** Starts, or carries on after a pause. */
  function play() {
    if (finished.value) return
    paused.value = false
    lastFrame = null
    raf.resume()
  }

  function pause() {
    paused.value = true
    raf.pause()
  }

  const step = computed(() => stepAt(plan, elapsedMs.value))
  const msIntoStep = computed(() => (step.value ? elapsedMs.value - step.value.startMs : 0))
  /** 0..1 through the current step. */
  const stepProgress = computed(() =>
    step.value ? Math.min(1, msIntoStep.value / step.value.durationMs) : 0,
  )
  /** Counting up from 1, as the coach counts out loud. */
  const stepSecond = computed(() => Math.floor(msIntoStep.value / 1000) + 1)
  const stepSecondsLeft = computed(() =>
    step.value ? Math.ceil((step.value.durationMs - msIntoStep.value) / 1000) : 0,
  )

  return {
    elapsedMs,
    paused,
    finished,
    step,
    stepProgress,
    stepSecond,
    stepSecondsLeft,
    play,
    pause,
  }
}
