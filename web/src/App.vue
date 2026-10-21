<script setup lang="ts">
import { onMounted } from 'vue'
import { useAppStore } from '@/stores/app'

const app = useAppStore()
onMounted(() => app.load())
</script>

<template>
  <v-app>
    <v-app-bar color="primary" density="comfortable">
      <v-app-bar-title>The Purple App</v-app-bar-title>
    </v-app-bar>

    <v-main>
      <v-alert v-if="app.loadError" type="error" class="ma-4" :text="app.loadError" />
      <v-container v-else-if="app.loaded" class="pa-3" style="max-width: 900px">
        <router-view />
      </v-container>
      <div v-else class="d-flex justify-center pa-8"><v-progress-circular indeterminate /></div>
    </v-main>
  </v-app>
</template>
