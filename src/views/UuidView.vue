<template>
  <header class="page-head"><h1>UUID 生成器</h1></header>

  <div class="section-title"><span>配置</span></div>
  <div class="config">
    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 7h18" /><path d="M3 12h18" /><path d="M3 17h18" />
        </svg>
      </span>
      <div>
        <div class="row-title">版本</div>
        <div class="row-desc">{{ isV7 ? 'v7 · 基于时间戳，可排序' : 'v4 · 完全随机' }}</div>
      </div>
      <Switch v-model="isV7" on-label="v7" off-label="v4" />
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 4v16" /><path d="M18 4v16" /><path d="M4 6h16" /><path d="M4 18h16" />
        </svg>
      </span>
      <div>
        <div class="row-title">大写输出</div>
        <div class="row-desc">字母显示为 A–F 大写</div>
      </div>
      <Switch v-model="uppercase" on-label="开" off-label="关" />
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M8 4v16" /><path d="M16 4v16" />
        </svg>
      </span>
      <div>
        <div class="row-title">连字符</div>
        <div class="row-desc">保留 8-4-4-4-12 的 - 分隔符</div>
      </div>
      <Switch v-model="hyphen" on-label="开" off-label="关" />
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" /><path d="M9 9h6v6H9z" />
        </svg>
      </span>
      <div>
        <div class="row-title">数量</div>
        <div class="row-desc">一次生成 1–100 个</div>
      </div>
      <div class="count-ctl">
        <button class="step" title="减少" @click="stepCount(-1)">−</button>
        <input type="number" min="1" max="100" v-model.number="count" class="num-input" />
        <button class="step" title="增加" @click="stepCount(1)">+</button>
      </div>
    </div>
  </div>

  <div class="section-title">
    <span>结果</span>
    <div class="section-actions">
      <PillBtn @click="generate" title="重新生成">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12a9 9 0 1 1-3-6.7L21 8" /><path d="M21 3v5h-5" />
        </svg>
        生成
      </PillBtn>
      <PillBtn icon-only title="全部复制" @click="copyAll" :disabled="!list.length">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" /><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
        </svg>
      </PillBtn>
    </div>
  </div>

  <div class="result-card">
    <div v-if="list.length" class="uuid-list">
      <div v-for="(u, i) in list" :key="i" class="uuid-row">
        <span class="idx">{{ String(i + 1).padStart(2, '0') }}</span>
        <span class="val">{{ u }}</span>
        <PillBtn icon-only title="复制" @click="copyOne(u)">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" /><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
          </svg>
        </PillBtn>
      </div>
    </div>
    <div v-else class="empty">点击「生成」创建 UUID</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import Switch from '@/components/ui/Switch.vue'
import { uuidApi } from '@/api/uuid'
import { clipboardApi } from '@/api/clipboard'

const isV7 = ref(false)
const uppercase = ref(false)
const hyphen = ref(true)
const count = ref(1)
const list = ref<string[]>([])
const message = useMessage()

const version = computed(() => (isV7.value ? 7 : 4))

function stepCount(delta: number) {
  const n = Math.max(1, Math.min(100, Math.floor((Number(count.value) || 1) + delta)))
  count.value = n
}

async function generate() {
  const n = Math.max(1, Math.min(100, Math.floor(Number(count.value) || 1)))
  count.value = n
  try {
    list.value = await uuidApi.generate(version.value, n, uppercase.value, hyphen.value)
  } catch (e) {
    message.error(typeof e === 'string' ? e : '生成失败')
  }
}

async function copyOne(text: string) {
  try {
    await clipboardApi.write(text)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

async function copyAll() {
  if (!list.value.length) return
  try {
    await clipboardApi.write(list.value.join('\n'))
    message.success('已复制全部')
  } catch {
    message.error('复制失败')
  }
}

// 任一配置变化即自动重新生成；首次进入也生成一次（与 PasswordView 一致）。
watch([isV7, uppercase, hyphen, count], generate, { immediate: true })
</script>

<style scoped>
.page-head h1 {
  font-family: var(--serif);
  font-size: 28px; font-weight: 500;
  letter-spacing: -0.015em;
  margin-bottom: 18px;
}

.section-title {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13.5px; font-weight: 500;
  color: var(--ink-2);
  margin: 12px 0 8px;
}
.section-actions { display: flex; gap: 4px; }

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

.count-ctl {
  display: flex; align-items: center; gap: 4px;
  background: transparent; border-radius: 8px; padding: 2px;
}
.step {
  width: 26px; height: 26px; border-radius: 6px;
  font-size: 16px; color: var(--ink-2);
  display: inline-flex; align-items: center; justify-content: center;
}
.step:hover { background: var(--aside-3); color: var(--ink); }
.num-input {
  width: 48px; text-align: center;
  font-family: var(--mono); font-size: 14px; font-weight: 600;
  color: var(--ink);
}
.num-input::-webkit-inner-spin-button,
.num-input::-webkit-outer-spin-button { -webkit-appearance: none; margin: 0; }

.result-card {
  background: transparent;
  border: 1px solid var(--border-accent);
  border-radius: var(--r-md);
  padding: 8px;
  flex: 1; min-height: 0;
  overflow: auto;
}
.uuid-list { display: flex; flex-direction: column; gap: 2px; }
.uuid-row {
  display: flex; align-items: center; gap: 10px;
  padding: 8px 10px; border-radius: 8px;
}
.uuid-row:hover { background: color-mix(in srgb, var(--aside-2) 10%, transparent); }
.uuid-row .idx {
  flex-shrink: 0; width: 28px;
  font-family: var(--mono); font-size: 12px; color: var(--ink-4);
}
.uuid-row .val {
  flex: 1;
  font-family: var(--mono); font-size: 13.5px; color: var(--ink);
  word-break: break-all;
}
.empty { color: var(--ink-3); font-size: 13px; text-align: center; padding: 32px 0; }
</style>
