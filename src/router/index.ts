import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/tools/url' },
  { path: '/tools/url', component: () => import('@/views/UrlView.vue') },
  { path: '/tools/:id', component: () => import('@/views/PlaceholderView.vue') },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
