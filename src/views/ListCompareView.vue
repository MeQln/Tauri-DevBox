<template>
  <div class="flex flex-col flex-1 min-h-0">
    <header class="page-head">
      <h1>文本比对工具</h1>
    </header>

    <div class="section-title">
      <span>原文 A</span>
      <div class="section-actions">
        <PillBtn icon-only title="粘贴" @click="() => pasteInto('a')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="3" width="6" height="4" rx="1" />
            <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
          </svg>
        </PillBtn>
        <PillBtn icon-only title="清空" @click="clearInput('a')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 6l12 12M18 6L6 18" />
          </svg>
        </PillBtn>
      </div>
    </div>
    <textarea v-model="inputA" class="text-area" placeholder="在此输入文本 A"></textarea>

    <div class="section-title">
      <span>原文 B</span>
      <div class="section-actions">
        <PillBtn icon-only title="粘贴" @click="() => pasteInto('b')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="3" width="6" height="4" rx="1" />
            <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
          </svg>
        </PillBtn>
        <PillBtn icon-only title="清空" @click="clearInput('b')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 6l12 12M18 6L6 18" />
          </svg>
        </PillBtn>
      </div>
    </div>
    <textarea v-model="inputB" class="text-area" placeholder="在此输入文本 B"></textarea>

    <div class="section-title">
      <span>比对结果</span>
      <div class="section-actions">
        <button v-if="diff.length" class="swap-btn" @click="swapInputs" title="交换 A / B">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M7 16l-4-4 4-4" /><path d="M3 12h18" /><path d="M17 8l4 4-4 4" />
          </svg>
        </button>
        <PillBtn icon-only title="复制结果" @click="copyDiff">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" />
            <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
          </svg>
        </PillBtn>
      </div>
    </div>
    <div v-if="!diff.length && !inputA && !inputB" class="diff-hint">
      在两侧输入文本后自动比对
    </div>
    <div v-else-if="sameHint" class="diff-hint diff-same">
      两段文本完全一致
    </div>
    <div v-else class="diff-wrap">
      <div v-for="(d, i) in diff" :key="i" :class="['diff-line', `diff-${d.type}`]">
        <span class="diff-ln">{{ d.ln }}</span>
        <span class="diff-marker">{{ d.type === 'same' ? ' ' : d.type === 'add' ? '+' : '-' }}</span>
        <span v-if="d.segments" class="diff-text">
          <template v-for="(seg, si) in d.segments" :key="si">
            <span v-if="seg.type === 'same'" class="seg-same">{{ seg.text }}</span>
            <span v-else-if="seg.type === 'remove'" class="seg-rm">{{ seg.text }}</span>
            <span v-else class="seg-add">{{ seg.text }}</span>
          </template>
        </span>
        <span v-else class="diff-text">{{ d.text }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import { clipboardApi } from '@/api/clipboard'

type DiffType = 'same' | 'add' | 'remove'
type SegType = 'same' | 'add' | 'remove'
type DiffSegment = { type: SegType; text: string }
type DiffLine = {
  type: DiffType
  text: string
  ln: string
  segments?: DiffSegment[]
}

const inputA     = ref('')
const inputB     = ref('')

const message = useMessage()

/** LCS 核心 */

function lcs<T>(a: T[], b: T[], equal: (x: T, y: T) => boolean): { type: 'same' | 'add' | 'remove'; val: T }[] {
  const m = a.length, n = b.length
  const dp: number[][] = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0))
  for (let i = 1; i <= m; i++)
    for (let j = 1; j <= n; j++)
      dp[i][j] = equal(a[i - 1], b[j - 1]) ? dp[i - 1][j - 1] + 1 : Math.max(dp[i - 1][j], dp[i][j - 1])

  const result: { type: 'same' | 'add' | 'remove'; val: T }[] = []
  let i = m, j = n
  while (i > 0 || j > 0) {
    if (i > 0 && j > 0 && equal(a[i - 1], b[j - 1])) {
      result.push({ type: 'same', val: a[i - 1] })
      i--; j--
    } else if (j > 0 && (i === 0 || dp[i][j - 1] >= dp[i - 1][j])) {
      result.push({ type: 'add', val: b[j - 1] })
      j--
    } else {
      result.push({ type: 'remove', val: a[i - 1] })
      i--
    }
  }
  return result.reverse()
}

/** 字符级 diff 分段（行内高亮用） */

function charSegments(textA: string, textB: string): DiffSegment[] {
  const charsA = [...textA]
  const charsB = [...textB]
  const diffs = lcs(charsA, charsB, (x, y) => x === y)
  const merged: DiffSegment[] = []
  for (const d of diffs) {
    const prev = merged[merged.length - 1]
    if (prev && prev.type === d.type) {
      prev.text += d.val
    } else {
      merged.push({ type: d.type as SegType, text: d.val as string })
    }
  }
  return merged
}

function injectIntraLineSegments(lines: DiffLine[]): void {
  for (let i = 0; i < lines.length - 1; i++) {
    const cur = lines[i]
    const next = lines[i + 1]
    if (cur.type === 'remove' && next.type === 'add') {
      const segs = charSegments(cur.text, next.text)
      cur.segments = segs.filter(s => s.type === 'same' || s.type === 'remove')
      next.segments = segs.filter(s => s.type === 'same' || s.type === 'add')
      i++
    } else if (cur.type === 'add' && next.type === 'remove') {
      const segs = charSegments(next.text, cur.text)
      cur.segments = segs.filter(s => s.type === 'same' || s.type === 'add')
      next.segments = segs.filter(s => s.type === 'same' || s.type === 'remove')
      i++
    }
  }
}

/** 行级 diff */

function lineDiff(a: string[], b: string[]): DiffLine[] {
  const raw = lcs(a, b, (x, y) => x === y)
  const result: DiffLine[] = []
  let la = 1, lb = 1
  for (const d of raw) {
    if (d.type === 'same') {
      result.push({ type: 'same', text: d.val as string, ln: `${la++}:${lb++}` })
    } else if (d.type === 'add') {
      result.push({ type: 'add', text: d.val as string, ln: `:${lb++}` })
    } else {
      result.push({ type: 'remove', text: d.val as string, ln: `${la++}:` })
    }
  }
  injectIntraLineSegments(result)
  return result
}

/** 计算属性 */

const diff = computed<DiffLine[]>(() => {
  const a = inputA.value
  const b = inputB.value
  if (!a && !b) return []
  return lineDiff(a.split('\n'), b.split('\n'))
})

const sameHint = computed(() => {
  if (!inputA.value && !inputB.value) return false
  if (!diff.value.length) return true
  return diff.value.every(d => d.type === 'same')
})

/** 交互函数 */

async function pasteInto(side: 'a' | 'b') {
  try {
    const text = await clipboardApi.read()
    if (!text) { message.info('剪贴板为空'); return }
    if (side === 'a') inputA.value = text
    else inputB.value = text
    message.success('已粘贴')
  } catch {
    message.error('粘贴失败')
  }
}

function clearInput(side: 'a' | 'b') {
  if (side === 'a') inputA.value = ''
  else inputB.value = ''
}

function swapInputs() {
  const t = inputA.value
  inputA.value = inputB.value
  inputB.value = t
}

async function copyDiff() {
  if (!diff.value.length) return
  const text = diff.value.map(d => `${d.type === 'add' ? '+' : d.type === 'remove' ? '-' : ' '} ${d.text}`).join('\n')
  try {
    await clipboardApi.write(text)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}
</script>

<style scoped>
.text-area {
  height: 130px; min-height: 130px; max-height: 130px;
  flex-shrink: 0;
}

.swap-btn {
  background: transparent; border: 1px solid var(--border-accent);
  border-radius: var(--r-sm); padding: 4px 6px;
  color: var(--ink-3); cursor: pointer; line-height: 0;
}
.swap-btn:hover { color: var(--ink); border-color: var(--aside-3); }
.swap-btn svg { width: 16px; height: 16px; }

/* 比对结果区域 — flex-1 填充剩余空间，随窗口调整 */
.diff-hint {
  flex: 1; min-height: 0; overflow: auto;
  padding: 24px; text-align: center;
  color: var(--ink-3); font-size: 13px;
  background: transparent; border: 1px solid var(--border-accent);
  border-radius: var(--r-md);
  display: flex; align-items: center; justify-content: center;
}
.diff-same { color: var(--ok); }

.diff-wrap {
  flex: 1; min-height: 0; overflow: auto;
  background: transparent; border: 1px solid var(--border-accent);
  border-radius: var(--r-md);
  font-family: var(--mono); font-size: 12.5px; line-height: 1.6;
}
.diff-line {
  display: flex; gap: 8px; padding: 2px 12px;
  align-items: baseline;
}
.diff-same  { background: transparent; }
.diff-add   { background: color-mix(in srgb, var(--ok) 10%, transparent); }
.diff-remove { background: color-mix(in srgb, var(--warn) 10%, transparent); }
.diff-ln {
  color: var(--ink-4); font-size: 11px;
  min-width: 60px; text-align: right; flex-shrink: 0;
  user-select: none;
}
.diff-marker {
  color: var(--ink-4); width: 12px; flex-shrink: 0; user-select: none;
}
.diff-add .diff-marker  { color: var(--ok); }
.diff-remove .diff-marker { color: var(--warn); }
.diff-text {
  white-space: pre-wrap; word-break: break-all;
  color: var(--ink);
}
.diff-same .diff-text { color: var(--ink); }

/* 行内字符级高亮 */
.seg-same { color: var(--ink); }
.seg-rm {
  color: var(--warn);
  background: color-mix(in srgb, var(--warn) 25%, transparent);
  border-radius: 2px;
}
.seg-add {
  color: var(--ok);
  background: color-mix(in srgb, var(--ok) 20%, transparent);
  border-radius: 2px;
}
</style>