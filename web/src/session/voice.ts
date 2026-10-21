// The audio coach. For now it uses the phone's built-in voice; recorded clips are planned.
import type { Say } from '@/core'

import { spokenText } from './words'

export function voiceAvailable(): boolean {
  return typeof window !== 'undefined' && 'speechSynthesis' in window
}

/** Says a cue, cutting off anything still being said so the coach never falls behind the clock. */
export function speak(say: Say, getReadySeconds: number): void {
  if (!voiceAvailable()) return
  const utterance = new SpeechSynthesisUtterance(spokenText(say, getReadySeconds))
  utterance.rate = 1.1
  window.speechSynthesis.cancel()
  window.speechSynthesis.speak(utterance)
}

export function hush(): void {
  if (voiceAvailable()) window.speechSynthesis.cancel()
}
