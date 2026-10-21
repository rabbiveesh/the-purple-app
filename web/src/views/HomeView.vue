<script setup lang="ts">
import { coreVersion } from '@/core'
</script>

<template>
  <!-- Placeholder until the first feature: the requirements are still being gathered. -->
  <v-card class="pa-6 text-center">
    <v-icon icon="mdi-flower-tulip" size="64" color="primary" />
    <h1 class="text-h5 mt-4">Coming soon</h1>
    <p class="text-medium-emphasis mt-2">Core v{{ coreVersion() }}</p>
  </v-card>
</template>
