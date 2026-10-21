<script setup lang="ts">
import { computed, ref } from 'vue'

import DaySlots from '@/components/DaySlots.vue'
import { calendarMonth, type Slot } from '@/core'
import { useAppStore } from '@/stores/app'

const app = useAppStore()

const [thisYear, thisMonth] = app.today.split('-').map(Number) as [number, number]
const year = ref(thisYear)
const month = ref(thisMonth)

const cal = computed(() =>
  calendarMonth({ records: app.data.sessions, year: year.value, month: month.value }),
)
const title = computed(() =>
  new Date(year.value, month.value - 1, 1).toLocaleDateString(undefined, {
    month: 'long',
    year: 'numeric',
  }),
)
const weekdays = ['S', 'M', 'T', 'W', 'T', 'F', 'S']

function shift(by: number) {
  const m = month.value - 1 + by
  year.value += Math.floor(m / 12)
  month.value = (((m % 12) + 12) % 12) + 1
}

/** Only today and past days can be ticked off by hand. */
function toggle(date: string, slot: Slot) {
  if (date <= app.today) void app.toggleByHand(date, slot)
}
</script>

<template>
  <v-card class="pa-3">
    <div class="d-flex align-center mb-2">
      <v-btn
        icon="mdi-chevron-left"
        variant="text"
        aria-label="Previous month"
        @click="shift(-1)"
      />
      <div class="text-title-large flex-1-1 text-center">{{ title }}</div>
      <v-btn icon="mdi-chevron-right" variant="text" aria-label="Next month" @click="shift(1)" />
    </div>
    <div class="grid">
      <div
        v-for="(d, i) in weekdays"
        :key="i"
        class="text-body-small text-center text-medium-emphasis"
      >
        {{ d }}
      </div>
      <div v-for="i in cal.leadingBlanks" :key="`blank-${i}`" />
      <div
        v-for="day in cal.days"
        :key="day.date"
        class="day pa-1 text-center"
        :class="{ today: day.date === app.today, future: day.date > app.today }"
      >
        <div class="text-body-small">{{ Number(day.date.slice(8)) }}</div>
        <DaySlots
          :morning="day.morning"
          :afternoon="day.afternoon"
          :clickable="day.date <= app.today"
          @toggle="(slot) => toggle(day.date, slot)"
        />
      </div>
    </div>
    <div class="text-body-small text-medium-emphasis mt-3">
      Left circle: morning. Right circle: afternoon. Tap an empty circle to mark a session you did
      without the app; tap it again to undo.
    </div>
  </v-card>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}
.day {
  border-radius: 8px;
}
.day.today {
  outline: 2px solid rgb(var(--v-theme-secondary));
}
.day.future {
  opacity: 0.4;
}
</style>
