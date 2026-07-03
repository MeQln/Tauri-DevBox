<template>
  <header class="page-head"><h1>哈希 / 校验</h1></header>

  <div class="grid">
    <!-- 左列：文本输入 -->
    <div class="left-col">
      <div class="section-title">
        <span>文本</span>
        <div class="section-actions">
          <PillBtn icon-only title="粘贴" @click="pasteText">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="3" width="6" height="4" rx="1" />
              <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
            </svg>
          </PillBtn>
          <PillBtn icon-only title="读取文本文件" @click="readTextFromFile">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 3v5h5" />
              <path d="M14 3H6a2 2 0 00-2 2v14a2 2 0 002 2h12a2 2 0 002-2V8z" />
            </svg>
          </PillBtn>
          <PillBtn icon-only title="清空" @click="clearAll">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 6l12 12M18 6L6 18" />
            </svg>
          </PillBtn>
        </div>
      </div>
      <textarea v-model="input" class="text-area" placeholder="输入要计算哈希的文本"></textarea>
    </div>

    <!-- 右列：文件输入 + 哈希结果 -->
    <div class="right-col">
      <div class="dropzone" @dragover.prevent @drop="onDrop">
        <p>拖放文件到此处，或 <a class="link" @click="onBrowse">浏览文件</a></p>
        <p class="muted">任意类型均可；大文件建议使用浏览（流式计算）</p>
      </div>

      <div class="result">
        <div class="result-head">
          <span class="result-title">哈希结果</span>
          <span v-if="source" class="source">{{ source.label }} · {{ formatSize(source.size) }}</span>
        </div>
        <div v-if="rows.length" class="hash-list">
          <div v-for="r in rows" :key="r.algo" class="hash-row">
            <span class="algo">{{ r.algo }}</span>
            <span class="value">{{ r.value }}</span>
            <PillBtn icon-only title="复制" @click="copy(r.value)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" />
                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
              </svg>
            </PillBtn>
          </div>
        </div>
        <div v-else class="empty">输入文本或选择文件后显示哈希值</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import { hashApi, type HashResult } from '@/api/hash'
import { clipboardApi } from '@/api/clipboard'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'

const input = ref('')
const result = ref<HashResult | null>(null)
const source = ref<{ label: string; size: number } | null>(null)
const message = useMessage()

const rows = computed(() => {
  const r = result.value
  if (!r) return []
  return [
    { algo: 'MD5', value: r.md5 },
    { algo: 'SHA-1', value: r.sha1 },
    { algo: 'SHA-256', value: r.sha256 },
    { algo: 'SHA-384', value: r.sha384 },
    { algo: 'SHA-512', value: r.sha512 },
  ]
})

// 文本实时哈希；reqId 防止连打时旧响应覆盖新结果（沿用 UrlView watcher race 模式）。
// 失败静默，不打扰用户（见项目 CLAUDE.md "错误处理反原则"）。
let reqId = 0
watch(input, async (text) => {
  const my = ++reqId
  if (!text) {
    if (my === reqId) {
      result.value = null
      source.value = null
    }
    return
  }
  try {
    const r = await hashApi.text(text)
    if (my === reqId) {
      result.value = r
      source.value = { label: '文本', size: r.size }
    }
  } catch {
    // 静默
  }
})

async function hashFilePath(path: string, name: string) {
  const my = ++reqId
  try {
    const r = await hashApi.file(path)
    if (my === reqId) {
      result.value = r
      source.value = { label: name, size: r.size }
    }
  } catch (e) {
    message.error(typeof e === 'string' ? e : '计算失败')
  }
}

async function hashFileBytes(file: File) {
  const my = ++reqId
  let buf: ArrayBuffer
  try {
    buf = await file.arrayBuffer()
  } catch {
    message.error('读取文件失败')
    return
  }
  try {
    const r = await hashApi.bytes(Array.from(new Uint8Array(buf)))
    if (my === reqId) {
      result.value = r
      source.value = { label: file.name, size: r.size }
    }
  } catch {
    message.error('计算失败')
  }
}

async function onDrop(e: DragEvent) {
  e.preventDefault()
  const file = e.dataTransfer?.files?.[0]
  if (!file) return
  await hashFileBytes(file)
}

async function onBrowse() {
  const path = await openDialog({ multiple: false })
  if (typeof path !== 'string') return
  const name = path.split(/[/\\]/).pop() || path
  await hashFilePath(path, name)
}

async function readTextFromFile() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '文本', extensions: ['txt', 'md', 'log', 'json', 'csv', 'xml', 'html', 'js', 'ts', 'rs', 'py', 'java', 'c', 'cpp', 'go'] }],
  })
  if (typeof path !== 'string') return
  try {
    input.value = await readTextFile(path)
  } catch {
    message.error('读取文件失败')
  }
}

async function pasteText() {
  try {
    input.value = await clipboardApi.read()
  } catch {
    message.error('粘贴失败')
  }
}

async function copy(text: string) {
  try {
    await clipboardApi.write(text)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

function clearAll() {
  input.value = ''
  result.value = null
  source.value = null
}

function formatSize(n: number): string {
  if (n < 1024) return `${n} B`
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`
  if (n < 1024 * 1024 * 1024) return `${(n / 1024 / 1024).toFixed(2)} MB`
  return `${(n / 1024 / 1024 / 1024).toFixed(2)} GB`
}
</script>

<style scoped>
.page-head h1 {
  font-family: var(--serif);
  font-size: 28px; font-weight: 500;
  letter-spacing: -0.015em;
  margin-bottom: 18px;
}

.grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  flex: 1; min-height: 0;
}
.left-col, .right-col {
  display: flex; flex-direction: column;
  min-height: 0; gap: 12px;
}

.section-title {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13.5px; font-weight: 500; color: var(--ink-2);
}
.section-actions { display: flex; gap: 4px; align-items: center; }

.text-area {
  flex: 1;
  min-height: 200px;
  padding: 12px 14px;
  font-family: var(--mono);
  font-size: 13.5px;
  background: var(--card);
  border: 1px solid var(--rule);
  border-radius: var(--r-md);
  resize: none; outline: none;
  color: var(--ink);
  word-break: break-all;
}
.text-area:focus { border-color: var(--link); }

.dropzone {
  border: 2px dashed var(--rule);
  border-radius: var(--r-md);
  padding: 18px;
  text-align: center;
  font-size: 13.5px; color: var(--ink-2);
  display: flex; flex-direction: column; gap: 6px;
}
.dropzone .muted { color: var(--ink-3); font-size: 12px; }
.dropzone .link { color: var(--link); cursor: pointer; font-weight: 500; }

.result {
  flex: 1; min-height: 0;
  background: var(--card);
  border: 1px solid var(--rule);
  border-radius: var(--r-md);
  padding: 12px 14px;
  display: flex; flex-direction: column; gap: 10px;
  overflow: auto;
}
.result-head {
  display: flex; align-items: center; justify-content: space-between;
}
.result-title { font-size: 13.5px; font-weight: 500; color: var(--ink-2); }
.source { font-size: 12px; color: var(--ink-3); font-family: var(--mono); }

.hash-list { display: flex; flex-direction: column; gap: 8px; }
.hash-row { display: flex; align-items: center; gap: 8px; }
.hash-row .algo {
  flex-shrink: 0; width: 64px;
  font-family: var(--mono); font-size: 12px; font-weight: 600; color: var(--ink-3);
}
.hash-row .value {
  flex: 1;
  font-family: var(--mono); font-size: 12.5px; color: var(--ink);
  word-break: break-all;
}
.empty { color: var(--ink-3); font-size: 13px; text-align: center; padding: 24px 0; }
</style>
