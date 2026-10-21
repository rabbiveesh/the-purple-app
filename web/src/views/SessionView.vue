<script setup lang="ts">
import { useWakeLock } from '@vueuse/core'
import { computed, onBeforeUnmount, ref } from 'vue'
import { useRouter } from 'vue-router'

import CelebrationDance from '@/components/CelebrationDance.vue'
import SessionSurveyForm from '@/components/SessionSurveyForm.vue'
import { defaultPlanOptions, sessionPlan, type Slot } from '@/core'
import type { CoachMode, SessionSurvey } from '@/data/schema'
import { usePlayer } from '@/session/player'
import { hush, speak, voiceAvailable } from '@/session/voice'
import { movementName, phaseLabel } from '@/session/words'
import { useAppStore } from '@/stores/app'

const app = useAppStore()
const router = useRouter()
const wakeLock = useWakeLock()

const options = defaultPlanOptions()
const plan = sessionPlan(app.routine, options)
const getReadySeconds = Math.round(options.getReadyMs / 1000)

type Stage = 'choose' | 'running' | 'survey' | 'celebrate'
const stage = ref<Stage>('choose')
const mode = ref<CoachMode>(app.data.lastCoachMode ?? 'silent')
let startedAt = new Date()
const finishedSlot = ref<{ date: string; slot: Slot }>()
const pointsBefore = ref(0)
/** The coach's latest count, shown during quick squeezes where seconds don't apply. */
const lastCount = ref<number>()

const player = usePlayer(plan, {
  onCue: (cue) => {
    if (cue.say.kind === 'count') lastCount.value = cue.say.n
    else if (cue.say.kind === 'getReady' || cue.say.kind === 'rest') lastCount.value = undefined
    if (mode.value !== 'silent') speak(cue.say, getReadySeconds)
  },
  onFinish: async () => {
    void wakeLock.release()
    pointsBefore.value = app.summary.pretzelPoints
    finishedSlot.value = await app.finishSession(startedAt)
    stage.value = 'survey'
  },
})

const modes: { mode: CoachMode; title: string; subtitle: string; icon: string }[] = [
  { mode: 'silent', title: 'Silent', subtitle: 'Screen only, no sound', icon: 'mdi-volume-off' },
  {
    mode: 'voiceAndScreen',
    title: 'Voice + screen',
    subtitle: 'The coach counts out loud',
    icon: 'mdi-account-voice',
  },
  { mode: 'voiceOnly', title: 'Voice only', subtitle: 'For the car', icon: 'mdi-car' },
]

/** Keeping the screen on is a nicety: if the phone says no, the session still runs. */
function keepScreenOn() {
  wakeLock.request('screen').catch(() => {})
}

function begin(chosen: CoachMode) {
  mode.value = chosen
  void app.rememberCoachMode(chosen)
  startedAt = new Date()
  stage.value = 'running'
  keepScreenOn()
  player.play()
}

function togglePause() {
  if (player.paused.value) {
    keepScreenOn()
    player.play()
  } else {
    player.pause()
    hush()
  }
}

function stopForNow() {
  player.pause()
  hush()
  void router.push('/')
}

onBeforeUnmount(() => {
  player.pause()
  hush()
  void wakeLock.release()
})

const exercise = computed(() => {
  const n = player.step.value?.exercise ?? 1
  return { n, ...app.routine.exercises[n - 1]! }
})

const phase = computed(() => player.step.value?.phase ?? 'getReady')
/** The bar rises while squeezing and falls while releasing. */
const bar = computed(() => {
  const p = player.stepProgress.value
  if (phase.value === 'squeeze') return p * 100
  if (phase.value === 'release') return (1 - p) * 100
  return 0
})
/** Counted up for squeeze and release, like the coach; counted down for waits. */
const bigNumber = computed(() => {
  if (exercise.value.movement.kind === 'quickSqueezes' && phase.value !== 'rest') {
    return phase.value === 'getReady' ? player.stepSecondsLeft.value : (lastCount.value ?? '')
  }
  return phase.value === 'squeeze' || phase.value === 'release'
    ? player.stepSecond.value
    : player.stepSecondsLeft.value
})

const pointsEarned = computed(() => app.summary.pretzelPoints - pointsBefore.value)

async function saveSurvey(answers: Omit<SessionSurvey, 'date' | 'slot'>) {
  if (finishedSlot.value) await app.saveSessionSurvey({ ...finishedSlot.value, ...answers })
  stage.value = 'celebrate'
}
</script>

<template>
  <div v-if="stage === 'choose'" class="d-flex flex-column ga-3">
    <div class="text-headline-small text-center my-2">Hey, ready to start?</div>
    <v-card
      v-for="m in modes"
      :key="m.mode"
      :color="m.mode === mode ? 'primary' : undefined"
      :variant="m.mode === mode ? 'tonal' : 'elevated'"
      :disabled="m.mode !== 'silent' && !voiceAvailable()"
      class="pa-2"
      @click="begin(m.mode)"
    >
      <v-card-item :prepend-icon="m.icon" :title="m.title" :subtitle="m.subtitle" />
    </v-card>
    <div class="text-body-small text-center text-medium-emphasis">
      Tap one to start. Exercises follow on by themselves.
    </div>
  </div>

  <div v-else-if="stage === 'running'" class="d-flex flex-column ga-4 align-center text-center">
    <template v-if="mode !== 'voiceOnly'">
      <div class="text-label-large">
        Exercise {{ exercise.n }} of {{ plan.exerciseCount }} ·
        {{ movementName(exercise.movement) }}
      </div>
      <div class="text-title-medium text-medium-emphasis" style="min-height: 1.75em">
        <template v-if="player.step.value && player.step.value.rep > 0">
          Rep {{ player.step.value.rep }} of {{ exercise.reps }}
        </template>
      </div>
      <div class="phase text-primary" :class="phase">{{ phaseLabel[phase] }}</div>
      <div class="d-flex align-center ga-8">
        <div class="big-number">{{ bigNumber }}</div>
        <div class="meter" aria-hidden="true">
          <div class="meter-fill" :style="{ height: `${bar}%` }" />
        </div>
      </div>
    </template>
    <template v-else>
      <v-icon icon="mdi-car" size="64" color="primary" class="mt-8" />
      <div class="text-title-large">Voice only. Eyes on the road!</div>
    </template>

    <v-btn
      icon
      :aria-label="player.paused.value ? 'Carry on' : 'Pause'"
      color="primary"
      size="128"
      class="mt-6"
      @click="togglePause"
    >
      <v-icon :icon="player.paused.value ? 'mdi-play' : 'mdi-pause'" size="64" />
    </v-btn>
    <v-btn v-if="player.paused.value" variant="text" @click="stopForNow">Stop for now</v-btn>
  </div>

  <SessionSurveyForm
    v-else-if="stage === 'survey'"
    @save="saveSurvey"
    @skip="stage = 'celebrate'"
  />

  <div v-else class="d-flex flex-column align-center">
    <CelebrationDance :message="`You did it! +${pointsEarned} pretzel points`" />
    <v-btn to="/" color="primary" size="x-large" rounded="xl" class="mt-4">Yay!</v-btn>
  </div>
</template>

<style scoped>
.meter {
  position: relative;
  width: 40px;
  height: 160px;
  border-radius: 20px;
  overflow: hidden;
  background: rgba(var(--v-theme-secondary), 0.15);
}
.meter-fill {
  position: absolute;
  bottom: 0;
  width: 100%;
  background: rgb(var(--v-theme-secondary));
}
.phase {
  font-size: 3.5rem;
  font-weight: 800;
  line-height: 1.1;
}
.big-number {
  font-size: 6rem;
  font-weight: 800;
  line-height: 1;
  min-height: 6rem;
  font-variant-numeric: tabular-nums;
}
</style>
