<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{ points: number; capacity: number; jarsFilled: number }>()

const fill = computed(() => Math.min(1, props.points / props.capacity))
// The jar's inside runs from y=30 to y=110 in the drawing.
const fillHeight = computed(() => 80 * fill.value)
</script>

<template>
  <div class="d-flex flex-column align-center">
    <svg
      viewBox="0 0 100 120"
      width="120"
      height="144"
      role="img"
      :aria-label="`Pretzel jar: ${points} of ${capacity} pretzel points`"
    >
      <defs>
        <clipPath id="jar-inside">
          <rect x="14" y="30" width="72" height="80" rx="14" />
        </clipPath>
      </defs>
      <rect x="14" y="30" width="72" height="80" rx="14" fill="rgb(var(--v-theme-surface))" />
      <g clip-path="url(#jar-inside)">
        <rect
          class="jar-fill"
          x="14"
          :y="110 - fillHeight"
          width="72"
          :height="fillHeight"
          fill="rgb(var(--v-theme-accent))"
        />
      </g>
      <rect
        x="14"
        y="30"
        width="72"
        height="80"
        rx="14"
        fill="none"
        stroke="rgb(var(--v-theme-primary))"
        stroke-width="4"
      />
      <rect x="24" y="16" width="52" height="14" rx="4" fill="rgb(var(--v-theme-primary))" />
      <text x="50" y="80" text-anchor="middle" font-size="30">🥨</text>
    </svg>
    <div class="text-title-large">{{ points }} / {{ capacity }}</div>
    <div class="text-body-small text-medium-emphasis">
      pretzel points<template v-if="jarsFilled > 0"> · {{ jarsFilled }} full jars</template>
    </div>
  </div>
</template>

<style scoped>
.jar-fill {
  transition:
    y 0.8s ease,
    height 0.8s ease;
}
</style>
