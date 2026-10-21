<script setup lang="ts">
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'

import { useAppStore } from '@/stores/app'

const app = useAppStore()
const route = useRoute()
const icon = `${import.meta.env.BASE_URL}favicon.svg`
onMounted(() => app.load())
</script>

<template>
  <v-app>
    <v-app-bar color="primary" density="comfortable">
      <template #prepend>
        <v-btn v-if="route.path !== '/'" icon="mdi-arrow-left" aria-label="Home" to="/" />
        <img v-else :src="icon" alt="" width="32" height="32" class="ml-3" />
      </template>
      <v-app-bar-title>Pretzel</v-app-bar-title>
      <template #append>
        <v-btn icon="mdi-calendar-month" aria-label="Calendar" to="/calendar" />
        <v-btn icon="mdi-content-save-outline" aria-label="Backup" to="/backup" />
      </template>
    </v-app-bar>

    <v-main class="pretzel-bg">
      <v-alert v-if="app.loadError" type="error" class="ma-4" :text="app.loadError" />
      <v-container v-else-if="app.loaded" class="pa-3" style="max-width: 600px">
        <router-view />
      </v-container>
      <div v-else class="d-flex justify-center pa-8"><v-progress-circular indeterminate /></div>
    </v-main>
  </v-app>
</template>

<style>
.pretzel-bg {
  background: linear-gradient(
    160deg,
    rgba(var(--v-theme-primary), 0.08),
    rgba(var(--v-theme-secondary), 0.08) 50%,
    rgba(var(--v-theme-accent), 0.1)
  );
  min-height: 100vh;
}
</style>
