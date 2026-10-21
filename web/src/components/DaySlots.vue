<script setup lang="ts">
// The morning and afternoon circles for one day.
import type { Slot } from '@/core'

defineProps<{ morning: boolean; afternoon: boolean; size?: number; clickable?: boolean }>()
const emit = defineEmits<{ toggle: [slot: Slot] }>()

const slots: { slot: Slot; label: string }[] = [
  { slot: 'morning', label: 'Morning' },
  { slot: 'afternoon', label: 'Afternoon' },
]
</script>

<template>
  <div class="d-flex ga-1 justify-center">
    <component
      :is="clickable ? 'button' : 'span'"
      v-for="s in slots"
      :key="s.slot"
      class="slot"
      :class="{ done: s.slot === 'morning' ? morning : afternoon }"
      :style="{ width: `${size ?? 14}px`, height: `${size ?? 14}px` }"
      :aria-label="`${s.label}: ${(s.slot === 'morning' ? morning : afternoon) ? 'done' : 'not done'}`"
      :type="clickable ? 'button' : undefined"
      @click="clickable && emit('toggle', s.slot)"
    />
  </div>
</template>

<style scoped>
.slot {
  display: inline-block;
  border-radius: 50%;
  border: 2px solid rgb(var(--v-theme-primary));
  background: transparent;
  padding: 0;
}
.slot.done {
  background: rgb(var(--v-theme-primary));
  box-shadow: 0 0 6px rgba(var(--v-theme-secondary), 0.8);
}
</style>
