import { defineStore } from 'pinia'
import { computed, ref, watch } from 'vue'

export type ThemeMode = 'light' | 'dark'

const STORAGE_KEY = 'devbox-theme'

function readStored(): ThemeMode {
  const v = localStorage.getItem(STORAGE_KEY)
  return v === 'dark' ? 'dark' : 'light'
}

function applyDom(mode: ThemeMode) {
  document.documentElement.dataset.theme = mode
}

/**
 * 启动前同步应用主题，避免浅色 → 深色闪烁。
 * 在 main.ts 的 app.mount() 之前调用 —— 不依赖 pinia 实例。
 */
export function initTheme() {
  applyDom(readStored())
}

export const useThemeStore = defineStore('theme', () => {
  const mode = ref<ThemeMode>(readStored())
  const isDark = computed(() => mode.value === 'dark')

  function set(next: ThemeMode) {
    mode.value = next
  }

  function toggle() {
    mode.value = mode.value === 'dark' ? 'light' : 'dark'
  }

  // mode 变化时同步 DOM 与 localStorage；immediate 确保首屏与 store 初始化一致
  watch(mode, (m) => {
    applyDom(m)
    localStorage.setItem(STORAGE_KEY, m)
  }, { immediate: true })

  return { mode, isDark, set, toggle }
})
