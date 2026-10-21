<script setup lang="ts">
import confetti from 'canvas-confetti'
import { onMounted } from 'vue'

defineProps<{ message: string }>()

// Who comes out to dance: a pretzel, a chubby baby, or a surprise guest.
const dancers = ['🥨', '👶', '🥨', '👶', '🦄', '🐙', '🦖', '🧁']
const dancer = dancers[Math.floor(Math.random() * dancers.length)]
const moves = ['wiggle', 'bounce', 'spin']
const move = moves[Math.floor(Math.random() * moves.length)]

onMounted(() => {
  if (window.matchMedia?.('(prefers-reduced-motion: reduce)').matches) return
  confetti({
    particleCount: 140,
    spread: 90,
    origin: { y: 0.6 },
    colors: ['#8e24aa', '#ec407a', '#ffb300', '#ce93d8', '#ffffff'],
    disableForReducedMotion: true,
  })
})
</script>

<template>
  <div class="text-center py-6">
    <div class="dancer" :class="move" aria-hidden="true">{{ dancer }}</div>
    <div class="text-headline-small mt-6">{{ message }}</div>
  </div>
</template>

<style scoped>
.dancer {
  font-size: 140px;
  line-height: 1;
  display: inline-block;
}
.wiggle {
  animation: wiggle 0.6s ease-in-out infinite alternate;
}
.bounce {
  animation: bounce 0.5s cubic-bezier(0.3, 0, 0.5, 1) infinite alternate;
}
.spin {
  animation:
    bounce 0.5s ease-in-out infinite alternate,
    spin 2s linear infinite;
}
@keyframes wiggle {
  from {
    transform: rotate(-18deg) scale(1);
  }
  to {
    transform: rotate(18deg) scale(1.1);
  }
}
@keyframes bounce {
  from {
    transform: translateY(0) scaleY(0.92);
  }
  to {
    transform: translateY(-40px) scaleY(1.05);
  }
}
@keyframes spin {
  to {
    rotate: 360deg;
  }
}
@media (prefers-reduced-motion: reduce) {
  .dancer {
    animation: none;
  }
}
</style>
