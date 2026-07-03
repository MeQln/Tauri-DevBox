import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import AppShell from '@/layouts/AppShell.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: AppShell,
    children: [
      { path: '', redirect: '/tools/port' },
      { path: 'tools/qrcode', component: () => import('@/views/QrCodeView.vue') },
      { path: 'tools/url', component: () => import('@/views/UrlView.vue') },
      { path: 'tools/port', component: () => import('@/views/PortView.vue') },
      { path: 'tools/base64-image', component: () => import('@/views/Base64ImageView.vue') },
      { path: 'tools/base64-text', component: () => import('@/views/Base64TextView.vue') },
      { path: 'tools/json', component: () => import('@/views/JsonView.vue') },
      { path: 'tools/sql', component: () => import('@/views/SqlView.vue') },
      { path: 'tools/xml-fmt', component: () => import('@/views/XmlView.vue') },
      { path: 'tools/connectivity', component: () => import('@/views/ConnectivityView.vue') },
      { path: 'tools/websocket', component: () => import('@/views/WebSocketView.vue') },
      { path: 'tools/:id', component: () => import('@/views/PlaceholderView.vue') },
    ],
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
