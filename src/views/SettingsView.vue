<template>
  <header class="page-head">
    <h1>设置</h1>
  </header>

  <div class="section-title"><span>外观</span></div>
  <div class="config">
    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z" />
        </svg>
      </span>
      <div>
        <div class="row-title">主题</div>
        <div class="row-desc">切换应用整体配色，深色模式适合夜间使用</div>
      </div>
      <div class="segmented" role="radiogroup" aria-label="主题">
        <button
          v-for="opt in modeOptions"
          :key="opt.value"
          :class="['seg-btn', { active: theme.mode === opt.value }]"
          role="radio"
          :aria-checked="theme.mode === opt.value"
          @click="theme.setMode(opt.value)"
        >
          <span class="seg-ico" v-html="opt.icon" />
          {{ opt.label }}
        </button>
      </div>
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="4"/>
          <path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/>
        </svg>
      </span>
      <div>
        <div class="row-title">配色</div>
        <div class="row-desc">选择应用的强调色</div>
      </div>
      <div class="color-picker" role="radiogroup" aria-label="配色">
        <button
          v-for="opt in colorOptions"
          :key="opt.value"
          :class="['color-btn', { active: theme.color === opt.value }]"
          role="radio"
          :aria-checked="theme.color === opt.value"
          :style="{ '--swatch': opt.swatch }"
          :title="opt.label"
          @click="theme.setColor(opt.value)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useThemeStore, type ThemeMode, type ThemeColor } from '@/stores/theme'

const theme = useThemeStore()

const modeOptions: { value: ThemeMode; label: string; icon: string }[] = [
  {
    value: 'light',
    label: '浅色',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg>',
  },
  {
    value: 'dark',
    label: '深色',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"/></svg>',
  },
]

const colorOptions: { value: ThemeColor; label: string; swatch: string }[] = [
  { value: 'blue', label: '蓝色', swatch: '#5b8cff' },
  { value: 'purple', label: '浅紫色', swatch: '#a78bfa' },
  { value: 'green', label: '绿色', swatch: '#6cb86c' },
  { value: 'rose', label: '玫瑰色', swatch: '#e88a8a' },
  { value: 'teal', label: '青色', swatch: '#5cb8b8' },
  { value: 'warm', label: '暖色', swatch: '#d4a040' },
]
</script>

<style scoped>
.page-head {
  display: flex; align-items: flex-start; justify-content: space-between;
  margin-bottom: 18px;
}
.page-head h1 {
  font-family: var(--serif);
  font-size: 28px; font-weight: 500;
  letter-spacing: -0.015em;
}

.section-title {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13.5px; font-weight: 500;
  color: var(--ink-2);
  margin: 12px 0 8px;
}

.config {
  background: transparent;
  border-radius: var(--r-md);
  padding: 6px;
  display: flex; flex-direction: column; gap: 4px;
}
.row {
  background: transparent;
  border: 1px solid var(--border-accent);
  border-radius: 8px;
  padding: 14px 16px;
  min-height: 64px;
  display: grid; grid-template-columns: 44px 1fr auto;
  align-items: center; gap: 12px;
  box-shadow: 0 1px 0 rgba(0,0,0,0.02);
}
.row-icon {
  width: 22px; height: 22px;
  display: inline-flex; align-items: center; justify-content: center;
  color: var(--ink-2);
}
.row-icon :deep(svg) { width: 18px; height: 18px; }
.row-title { font-size: 14px; font-weight: 500; }
.row-desc { font-size: 12.5px; color: var(--ink-3); margin-top: 2px; }

.segmented {
  display: inline-flex;
  background: var(--aside);
  border-radius: var(--r-md);
  padding: 3px;
  gap: 2px;
}
.seg-btn {
  display: inline-flex; align-items: center; gap: 6px;
  padding: 6px 14px;
  border-radius: 7px;
  font-size: 13px;
  color: var(--ink-3);
  transition: background .15s, color .15s, box-shadow .15s;
}
.seg-btn:hover { color: var(--ink); }
.seg-btn.active {
  background: var(--card);
  color: var(--ink);
  box-shadow: 0 1px 2px rgba(0,0,0,0.08);
}
.seg-ico { display: inline-flex; width: 14px; height: 14px; }
.seg-ico :deep(svg) { width: 14px; height: 14px; }

.color-picker {
  display: inline-flex;
  gap: 8px;
  align-items: center;
}
.color-btn {
  width: 28px; height: 28px;
  border-radius: 50%;
  background: var(--swatch);
  border: 2px solid transparent;
  cursor: pointer;
  transition: border-color .15s, box-shadow .15s, transform .15s;
}
.color-btn:hover {
  transform: scale(1.15);
}
.color-btn.active {
  border-color: var(--ink);
  box-shadow: 0 0 0 2px var(--surface), 0 0 0 4px var(--ink);
}
</style>