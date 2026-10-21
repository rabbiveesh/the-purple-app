<script setup lang="ts">
import { computed, ref } from 'vue'

import type { SessionSurvey } from '@/data/schema'

type Answers = Omit<SessionSurvey, 'date' | 'slot'>

const emit = defineEmits<{ save: [answers: Answers]; skip: [] }>()

const difficulty = ref(5)
const heldFullCount = ref<SessionSurvey['heldFullCount']>()
const lastAsStrongAsFirst = ref<SessionSurvey['lastAsStrongAsFirst']>()
const fullyRelaxed = ref<SessionSurvey['fullyRelaxed']>()

const complete = computed(() => heldFullCount.value && lastAsStrongAsFirst.value)

function save() {
  if (!heldFullCount.value || !lastAsStrongAsFirst.value) return
  emit('save', {
    difficulty: difficulty.value,
    heldFullCount: heldFullCount.value,
    lastAsStrongAsFirst: lastAsStrongAsFirst.value,
    ...(fullyRelaxed.value ? { fullyRelaxed: fullyRelaxed.value } : {}),
  })
}
</script>

<template>
  <v-card class="pa-4">
    <div class="text-title-large mb-2">Quick check</div>

    <div class="text-title-medium">How hard was that?</div>
    <v-slider
      v-model="difficulty"
      :min="1"
      :max="10"
      :step="1"
      show-ticks="always"
      thumb-label="always"
      color="primary"
      class="mt-8"
    />
    <div class="d-flex justify-space-between text-body-small text-medium-emphasis mt-n4 mb-4">
      <span>easy</span><span>very hard</span>
    </div>

    <div class="text-title-medium">Could you hold the full 10 seconds?</div>
    <v-chip-group v-model="heldFullCount" selected-class="bg-primary" column>
      <v-chip value="all">All</v-chip>
      <v-chip value="most">Most</v-chip>
      <v-chip value="some">Some</v-chip>
      <v-chip value="few">Few</v-chip>
    </v-chip-group>

    <div class="text-title-medium mt-3">Were the last squeezes as strong as the first?</div>
    <v-chip-group v-model="lastAsStrongAsFirst" selected-class="bg-primary" column>
      <v-chip value="yes">Yes</v-chip>
      <v-chip value="bitWeaker">A bit weaker</v-chip>
      <v-chip value="muchWeaker">Much weaker</v-chip>
    </v-chip-group>

    <div class="text-title-medium mt-3">
      Could you fully relax in between?
      <span class="text-body-small text-medium-emphasis">(optional)</span>
    </div>
    <v-chip-group v-model="fullyRelaxed" selected-class="bg-primary" column>
      <v-chip value="yes">Yes</v-chip>
      <v-chip value="notReally">Not really</v-chip>
    </v-chip-group>

    <div class="d-flex ga-2 mt-4">
      <v-btn variant="text" @click="emit('skip')">Skip</v-btn>
      <v-spacer />
      <v-btn color="primary" :disabled="!complete" @click="save">Done</v-btn>
    </div>
  </v-card>
</template>
