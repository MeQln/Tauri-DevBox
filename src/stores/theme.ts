import { defineStore } from 'pinia'
import { computed, ref, watch } from 'vue'

export type ThemeMode = 'light' | 'dark'
export type ThemeColor = 'blue' | 'purple' | 'green' | 'rose' | 'teal'

const MODE_KEY = 'devbox-theme'
const COLOR_KEY = 'devbox-color'

function readStoredMode(): ThemeMode {
  const v = localStorage.getItem(MODE_KEY)
  return v === 'dark' ? 'dark' : 'light'
}

function readStoredColor(): ThemeColor {
  const v = localStorage.getItem(COLOR_KEY)
  const valid: ThemeColor[] = ['blue', 'purple', 'green', 'rose', 'teal']
  return valid.includes(v as ThemeColor) ? (v as ThemeColor) : 'blue'
}

function applyDom(mode: ThemeMode, color: ThemeColor) {
  document.documentElement.dataset.theme = mode
  document.documentElement.dataset.color = color
}

/**
 * 启动前同步应用主题与配色，避免闪烁。
 * 在 main.ts 的 app.mount() 之前调用 —— 不依赖 pinia 实例。
 */
export function initTheme() {
  applyDom(readStoredMode(), readStoredColor())
}

export const useThemeStore = defineStore('theme', () => {
  const mode = ref<ThemeMode>(readStoredMode())
  const color = ref<ThemeColor>(readStoredColor())
  const isDark = computed(() => mode.value === 'dark')

  function setMode(next: ThemeMode) {
    mode.value = next
  }

  function setColor(next: ThemeColor) {
    color.value = next
  }

  function toggle() {
    mode.value = mode.value === 'dark' ? 'light' : 'dark'
  }

  // mode 变化时同步 DOM 与 localStorage
  watch(mode, (m) => {
    applyDom(m, color.value)
    localStorage.setItem(MODE_KEY, m)
  }, { immediate: true })

  // color 变化时同步 DOM 与 localStorage
  watch(color, (c) => {
    applyDom(mode.value, c)
    localStorage.setItem(COLOR_KEY, c)
  }, { immediate: true })

  return { mode, color, isDark, setMode, setColor, toggle }
})