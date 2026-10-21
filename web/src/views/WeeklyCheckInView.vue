<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import type { LeakSituation, WeeklyCheckIn } from '@/data/schema'
import { useAppStore } from '@/stores/app'

const app = useAppStore()
const router = useRouter()

const leaks = ref<WeeklyCheckIn['leaks']>()
const situations = ref<LeakSituation[]>([])
const bother = ref(0)
const heaviness = ref<WeeklyCheckIn['heaviness']>()

const situationChoices: { value: LeakSituation; label: string }[] = [
  { value: 'coughSneeze', label: 'Cough or sneeze' },
  { value: 'laugh', label: 'Laughing' },
  { value: 'jumpRun', label: 'Jumping or running' },
  { value: 'lifting', label: 'Lifting the kids' },
  { value: 'suddenUrge', label: 'Sudden urge' },
]

async function save() {
  if (!leaks.value) return
  await app.saveWeeklyCheckIn({
    date: app.today,
    leaks: leaks.value,
    situations: leaks.value === 'none' ? [] : situations.value,
    bother: leaks.value === 'none' ? 0 : bother.value,
    ...(heaviness.value ? { heaviness: heaviness.value } : {}),
  })
  await router.push('/')
}
</script>

<template>
  <v-card class="pa-4">
    <div class="text-title-large mb-2">Weekly check-in</div>

    <div class="text-title-medium">Any leaks this week?</div>
    <v-chip-group v-model="leaks" selected-class="bg-primary" column>
      <v-chip value="none">None</v-chip>
      <v-chip value="onceOrTwice">Once or twice</v-chip>
      <v-chip value="aFewTimes">A few times</v-chip>
      <v-chip value="mostDays">Most days</v-chip>
    </v-chip-group>

    <template v-if="leaks && leaks !== 'none'">
      <div class="text-title-medium mt-3">When? (pick any)</div>
      <v-chip-group v-model="situations" multiple selected-class="bg-primary" column>
        <v-chip v-for="c in situationChoices" :key="c.value" :value="c.value">{{ c.label }}</v-chip>
      </v-chip-group>

      <div class="text-title-medium mt-3">How much did it bother you?</div>
      <v-slider
        v-model="bother"
        :min="0"
        :max="10"
        :step="1"
        show-ticks="always"
        thumb-label="always"
        color="primary"
        class="mt-8"
      />
      <div class="d-flex justify-space-between text-body-small text-medium-emphasis mt-n4">
        <span>not at all</span><span>a lot</span>
      </div>
    </template>

    <div class="text-title-medium mt-3">
      Any heaviness or dragging feeling down there?
      <span class="text-body-small text-medium-emphasis">(optional)</span>
    </div>
    <v-chip-group v-model="heaviness" selected-class="bg-primary" column>
      <v-chip value="never">Never</v-chip>
      <v-chip value="sometimes">Sometimes</v-chip>
      <v-chip value="often">Often</v-chip>
    </v-chip-group>

    <div class="d-flex mt-4">
      <v-btn variant="text" to="/">Later</v-btn>
      <v-spacer />
      <v-btn color="primary" :disabled="!leaks" @click="save">Save</v-btn>
    </div>
  </v-card>
</template>
