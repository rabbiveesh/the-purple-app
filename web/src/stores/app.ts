import { defineStore } from 'pinia'
import { ref } from 'vue'

import { emptyData, loadAppData, saveAppData } from '@/data/repository'
import type { AppData } from '@/data/schema'

export const useAppStore = defineStore('app', () => {
  const data = ref<AppData>(emptyData())
  const loaded = ref(false)
  const loadError = ref<string | null>(null)

  async function load() {
    try {
      const saved = await loadAppData()
      if (saved) {
        data.value = saved
      } else {
        await saveAppData(data.value)
      }
      loaded.value = true
    } catch (e) {
      loadError.value = e instanceof Error ? e.message : String(e)
    }
  }

  return { data, loaded, loadError, load }
})
