import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import AppShell from '@/layouts/AppShell.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: AppShell,
    children: [
      { path: '', redirect: '/tools/url' },
      { path: 'tools/url', component: () => import('@/views/UrlView.vue') },
      { path: 'tools/qrcode', component: () => import('@/views/QrCodeView.vue') },
      { path: 'tools/:id', component: () => import('@/views/PlaceholderView.vue') },
    ],
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
