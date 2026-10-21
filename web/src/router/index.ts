import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'

export const routes: RouteRecordRaw[] = [
  { path: '/', name: 'home', component: () => import('@/views/HomeView.vue') },
  { path: '/session', name: 'session', component: () => import('@/views/SessionView.vue') },
  { path: '/calendar', name: 'calendar', component: () => import('@/views/CalendarView.vue') },
  {
    path: '/check-in/weekly',
    name: 'weekly-check-in',
    component: () => import('@/views/WeeklyCheckInView.vue'),
  },
  {
    path: '/check-in/monthly',
    name: 'monthly-check-in',
    component: () => import('@/views/MonthlyCheckInView.vue'),
  },
  { path: '/backup', name: 'backup', component: () => import('@/views/BackupView.vue') },
]

// Hash history: GitHub Pages has no server-side rewrites for deep links.
const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
