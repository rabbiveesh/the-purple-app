<script setup lang="ts">
import { useFileDialog } from '@vueuse/core'
import { ref } from 'vue'

import { migrate } from '@/data/migrations'
import { useAppStore } from '@/stores/app'

const app = useAppStore()
const message = ref<{ type: 'success' | 'error'; text: string }>()

function download() {
  const blob = new Blob([JSON.stringify(app.data, null, 2)], { type: 'application/json' })
  const link = document.createElement('a')
  link.href = URL.createObjectURL(blob)
  link.download = `pretzel-backup-${app.today}.json`
  link.click()
  URL.revokeObjectURL(link.href)
}

const { open, onChange } = useFileDialog({ accept: 'application/json', multiple: false })
onChange(async (files) => {
  const file = files?.[0]
  if (!file) return
  try {
    const backup = migrate(JSON.parse(await file.text()))
    await app.restore(backup)
    message.value = { type: 'success', text: `Restored ${backup.sessions.length} sessions.` }
  } catch (e) {
    message.value = {
      type: 'error',
      text: `That file couldn't be restored: ${e instanceof Error ? e.message : e}`,
    }
  }
})
</script>

<template>
  <v-card class="pa-4">
    <div class="text-title-large">Backup</div>
    <p class="text-body-medium my-3">
      Everything lives only on this phone. Save a backup now and then, and keep it somewhere safe.
    </p>
    <div class="d-flex flex-column ga-3">
      <v-btn color="primary" prepend-icon="mdi-download" @click="download">Save a backup</v-btn>
      <v-btn variant="tonal" prepend-icon="mdi-upload" @click="open()">
        Restore from a backup
      </v-btn>
    </div>
    <v-alert v-if="message" :type="message.type" :text="message.text" class="mt-4" />
  </v-card>
</template>
