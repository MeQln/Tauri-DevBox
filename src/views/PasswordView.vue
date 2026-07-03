<template>
  <header class="page-head"><h1>密码生成器</h1></header>

  <div class="section-title"><span>配置</span></div>
  <div class="config">
    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 12h14" /><path d="M12 5v14" />
        </svg>
      </span>
      <div>
        <div class="row-title">长度</div>
        <div class="row-desc">{{ length }} 位字符（4–64）</div>
      </div>
      <div class="length-ctl">
        <input type="range" min="4" max="64" v-model.number="length" class="slider" />
        <span class="len-num">{{ length }}</span>
      </div>
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" /><path d="M9 9h6v6H9z" />
        </svg>
      </span>
      <div>
        <div class="row-title">数量</div>
        <div class="row-desc">一次生成 1–10 个</div>
      </div>
      <div class="count-ctl">
        <button class="step" title="减少" @click="stepCount(-1)">−</button>
        <input type="number" min="1" max="10" v-model.number="count" class="num-input" />
        <button class="step" title="增加" @click="stepCount(1)">+</button>
      </div>
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="11" width="18" height="11" rx="2" /><path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
      </span>
      <div class="cat-grid">
        <label class="cat"><span>大写 A–Z</span><Switch v-model="upper" on-label="开" off-label="关" /></label>
        <label class="cat"><span>小写 a–z</span><Switch v-model="lower" on-label="开" off-label="关" /></label>
        <label class="cat"><span>数字 0–9</span><Switch v-model="digit" on-label="开" off-label="关" /></label>
        <label class="cat"><span>符号 !@#</span><Switch v-model="symbol" on-label="开" off-label="关" /></label>
      </div>
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" /><circle cx="12" cy="12" r="3" />
        </svg>
      </span>
      <div>
        <div class="row-title">排除易混淆字符</div>
        <div class="row-desc">剔除 I l 1 O 0 o 等肉眼易混字符</div>
      </div>
      <Switch v-model="excludeAmbiguous" on-label="开" off-label="关" />
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
    <div class="strength">
      <div class="bars">
        <span v-for="i in 4" :key="i" class="bar" :class="{ on: i <= strength.score }" :style="i <= strength.score ? { background: strength.color } : null" />
      </div>
      <span class="strength-label" :style="{ color: strength.color }">{{ strength.label }} · 约 {{ strength.bits }} bit</span>
    </div>

    <div v-if="list.length" class="pwd-list">
      <div v-for="(p, i) in list" :key="i" class="pwd-row">
        <span class="idx">{{ String(i + 1).padStart(2, '0') }}</span>
        <span class="val">{{ p }}</span>
        <PillBtn icon-only title="复制" @click="copyOne(p)">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" /><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
          </svg>
        </PillBtn>
      </div>
    </div>
    <div v-else class="empty">点击「生成」创建密码</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import Switch from '@/components/ui/Switch.vue'
import { passwordApi } from '@/api/password'
import { clipboardApi } from '@/api/clipboard'

const length = ref(16)
const count = ref(1)
const upper = ref(true)
const lower = ref(true)
const digit = ref(true)
const symbol = ref(true)
const excludeAmbiguous = ref(false)

const list = ref<string[]>([])
const message = useMessage()

// 估算熵：长度 × log2(字符池大小)。基于配置预算，仅用于强度提示，非精确度量。
const strength = computed(() => {
  let pool = 0
  if (lower.value) pool += 26
  if (upper.value) pool += 26
  if (digit.value) pool += 10
  if (symbol.value) pool += 24
  if (excludeAmbiguous.value) {
    // AMBIGUOUS="Il1O0o"：upper 剔 I/O、lower 剔 l/o、digit 剔 0/1，各 2 个（与后端 strip_ambiguous 对齐）
    let drop = 0
    if (upper.value) drop += 2
    if (lower.value) drop += 2
    if (digit.value) drop += 2
    pool = Math.max(1, pool - drop)
  }
  if (pool === 0) return { score: 0, label: '—', bits: 0, color: 'var(--ink-3)' }
  const bits = Math.round(length.value * Math.log2(pool))
  if (bits < 40) return { score: 1, label: '弱', bits, color: 'var(--warn)' }
  if (bits < 80) return { score: 2, label: '中', bits, color: 'var(--amber)' }
  if (bits < 128) return { score: 3, label: '强', bits, color: 'var(--link)' }
  return { score: 4, label: '极强', bits, color: 'var(--ok)' }
})

function stepCount(delta: number) {
  count.value = Math.max(1, Math.min(10, Math.floor((Number(count.value) || 1) + delta)))
}

// reqId 防止连打时旧响应覆盖新结果（沿用 UrlView watcher race 模式）。
let reqId = 0
async function generate() {
  const my = ++reqId
  const n = Math.max(1, Math.min(10, Math.floor(Number(count.value) || 1)))
  count.value = n
  try {
    const r = await passwordApi.generate({
      length: length.value,
      upper: upper.value,
      lower: lower.value,
      digit: digit.value,
      symbol: symbol.value,
      excludeAmbiguous: excludeAmbiguous.value,
    }, n)
    if (my === reqId) list.value = r
  } catch (e) {
    if (my === reqId) {
      list.value = []
      message.error(typeof e === 'string' ? e : '生成失败')
    }
  }
}

// 任一配置变化即自动重新生成；首次进入也生成一次。
watch([length, count, upper, lower, digit, symbol, excludeAmbiguous], generate, { immediate: true })

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
  background: var(--card);
  border-radius: var(--r-md);
  padding: 6px;
  display: flex; flex-direction: column; gap: 4px;
}
.row {
  background: var(--card-2);
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

.length-ctl { display: flex; align-items: center; gap: 10px; }
.slider {
  width: 180px; height: 4px;
  background: var(--aside-3); border-radius: 999px;
  appearance: none; -webkit-appearance: none; cursor: pointer;
}
.slider::-webkit-slider-thumb {
  -webkit-appearance: none; appearance: none;
  width: 16px; height: 16px; border-radius: 50%;
  background: var(--ink); border: 2px solid var(--surface);
  box-shadow: 0 1px 3px rgba(0,0,0,0.2);
}
.len-num {
  font-family: var(--mono); font-size: 14px; font-weight: 600;
  color: var(--ink); min-width: 22px; text-align: center;
}

.count-ctl {
  display: flex; align-items: center; gap: 4px;
  background: var(--card); border-radius: 8px; padding: 2px;
}
.step {
  width: 26px; height: 26px; border-radius: 6px;
  font-size: 16px; color: var(--ink-2);
  display: inline-flex; align-items: center; justify-content: center;
}
.step:hover { background: var(--aside-3); color: var(--ink); }
.num-input {
  width: 44px; text-align: center;
  font-family: var(--mono); font-size: 14px; font-weight: 600;
  color: var(--ink);
}
.num-input::-webkit-inner-spin-button,
.num-input::-webkit-outer-spin-button { -webkit-appearance: none; margin: 0; }

.cat-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 6px 32px;
}
.cat {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13px; color: var(--ink-2);
}

.result-card {
  background: var(--card);
  border: 1px solid var(--rule);
  border-radius: var(--r-md);
  padding: 10px;
  flex: 1; min-height: 0;
  overflow: auto;
  display: flex; flex-direction: column; gap: 8px;
}
.strength { display: flex; align-items: center; gap: 10px; padding: 4px 6px; }
.bars { display: flex; gap: 4px; }
.bar {
  width: 40px; height: 5px; border-radius: 999px;
  background: var(--aside-3);
  transition: background .15s;
}
.strength-label { font-size: 12.5px; font-weight: 500; font-family: var(--mono); }

.pwd-list { display: flex; flex-direction: column; gap: 2px; }
.pwd-row {
  display: flex; align-items: center; gap: 10px;
  padding: 8px 10px; border-radius: 8px;
}
.pwd-row:hover { background: var(--card-2); }
.pwd-row .idx {
  flex-shrink: 0; width: 28px;
  font-family: var(--mono); font-size: 12px; color: var(--ink-4);
}
.pwd-row .val {
  flex: 1;
  font-family: var(--mono); font-size: 14px; font-weight: 500;
  color: var(--ink);
  word-break: break-all;
  letter-spacing: 0.02em;
}
.empty { color: var(--ink-3); font-size: 13px; text-align: center; padding: 32px 0; }
</style>
