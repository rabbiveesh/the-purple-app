<script setup lang="ts">
import { useRouter } from 'vue-router'

import type { MonthlyCheckIn } from '@/data/schema'
import { useAppStore } from '@/stores/app'

const app = useAppStore()
const router = useRouter()

const choices: { value: MonthlyCheckIn['comparedWithStart']; label: string }[] = [
  { value: 3, label: 'Much better' },
  { value: 2, label: 'Better' },
  { value: 1, label: 'A little better' },
  { value: 0, label: 'About the same' },
  { value: -1, label: 'A little worse' },
  { value: -2, label: 'Worse' },
  { value: -3, label: 'Much worse' },
]

async function pick(value: MonthlyCheckIn['comparedWithStart']) {
  await app.saveMonthlyCheckIn({ date: app.today, comparedWithStart: value })
  await router.push('/')
}
</script>

<template>
  <v-card class="pa-4">
    <div class="text-title-large">Monthly check-in</div>
    <div class="text-title-medium mb-3">Compared with when you started, how are things now?</div>
    <div class="d-flex flex-column ga-2">
      <v-btn
        v-for="c in choices"
        :key="c.value"
        variant="tonal"
        color="primary"
        @click="pick(c.value)"
      >
        {{ c.label }}
      </v-btn>
    </div>
    <v-btn variant="text" to="/" class="mt-3">Later</v-btn>
  </v-card>
</template>
