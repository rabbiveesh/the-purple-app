<script setup lang="ts">
import { computed, onMounted } from 'vue'

import DaySlots from '@/components/DaySlots.vue'
import PretzelJar from '@/components/PretzelJar.vue'
import { useAppStore } from '@/stores/app'

const app = useAppStore()
onMounted(() => app.refreshToday())

const s = computed(() => app.summary)

const cheer = computed(() => {
  const days = s.value.streakDays
  if (days >= 30) return `${days} days in a row. Legendary pretzel!`
  if (days >= 7) return `${days} days in a row. You're on fire!`
  if (days >= 2) return `${days} days in a row. Go you!`
  if (days === 1) return 'Day one of a new streak.'
  return 'Every pretzel starts with one twist.'
})
</script>

<template>
  <div class="d-flex flex-column ga-4">
    <v-card class="pa-4 text-center" color="primary" variant="tonal">
      <PretzelJar :points="s.jarPoints" :capacity="s.jarCapacity" :jars-filled="s.jarsFilled" />
      <div class="text-body-large mt-3">{{ cheer }}</div>
    </v-card>

    <v-card class="pa-4 text-center">
      <div class="text-label-large">Today</div>
      <DaySlots :morning="s.today.morning" :afternoon="s.today.afternoon" :size="28" />
      <div class="d-flex justify-center ga-8 text-body-small text-medium-emphasis mt-1">
        <span>morning</span><span>afternoon</span>
      </div>
      <v-btn
        to="/session"
        color="secondary"
        size="x-large"
        block
        rounded="xl"
        class="mt-4"
        prepend-icon="mdi-play"
      >
        Pretzel time!
      </v-btn>
    </v-card>

    <v-card v-if="app.dueCheckIns.weekly || app.dueCheckIns.monthly" class="pa-2">
      <v-list density="comfortable">
        <v-list-item
          v-if="app.dueCheckIns.weekly"
          to="/check-in/weekly"
          prepend-icon="mdi-calendar-week"
          title="Weekly check-in"
          subtitle="Four quick questions"
        />
        <v-list-item
          v-if="app.dueCheckIns.monthly"
          to="/check-in/monthly"
          prepend-icon="mdi-star-shooting"
          title="Monthly check-in"
          subtitle="One question"
        />
      </v-list>
    </v-card>

    <div class="d-flex ga-4">
      <v-card class="pa-3 flex-1-1 text-center">
        <div class="text-headline-small">{{ s.rotomies }}</div>
        <div class="text-body-small text-medium-emphasis">pretzelometer rotomies</div>
      </v-card>
      <v-card class="pa-3 flex-1-1 text-center" to="/calendar">
        <div class="text-headline-small">{{ s.sessions }}</div>
        <div class="text-body-small text-medium-emphasis">sessions · see calendar</div>
      </v-card>
    </div>
  </div>
</template>
