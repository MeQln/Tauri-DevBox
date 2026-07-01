<template>
  <header class="page-head">
    <h1>URL 编码 / 解码工具</h1>
  </header>

  <div class="section-title"><span>配置</span></div>
  <div class="config">
    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M7 7h11l-3-3" /><path d="M17 17H6l3 3" />
        </svg>
      </span>
      <div>
        <div class="row-title">转换</div>
        <div class="row-desc">选择你要使用的转换模式</div>
      </div>
      <Switch v-model="isEncode" on-label="编码" off-label="解码" />
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M7 7h11l-3-3" /><path d="M17 17H6l3 3" />
        </svg>
      </span>
      <div>
        <div class="row-title">Encoding / Decoding Multiline</div>
        <div class="row-desc">Encode / Decode each line separately</div>
      </div>
      <Switch v-model="multiline" on-label="开启" off-label="关闭" />
    </div>
  </div>

  <div class="section-title">
    <span>输入</span>
    <div class="section-actions">
      <PillBtn icon-only title="粘贴" @click="pasteInput">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="3" width="6" height="4" rx="1" />
          <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
        </svg>
      </PillBtn>
      <PillBtn icon-only title="读取文件" @click="readInput">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 3v5h5" />
          <path d="M14 3H6a2 2 0 00-2 2v14a2 2 0 002 2h12a2 2 0 002-2V8z" />
        </svg>
      </PillBtn>
      <PillBtn icon-only title="清空" @click="clearInput">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 6l12 12M18 6L6 18" />
        </svg>
      </PillBtn>
    </div>
  </div>
  <CodeArea v-model="input" class="flex-1" />

  <div class="section-title">
    <span>输出</span>
    <div class="section-actions">
      <PillBtn icon-only title="复制" @click="copyOutput">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" />
          <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
        </svg>
      </PillBtn>
    </div>
  </div>
  <CodeArea v-model="output" readonly class="flex-1" />
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import Switch from '@/components/ui/Switch.vue'
import CodeArea from '@/components/ui/CodeArea.vue'
import { urlApi } from '@/api/url'
import { clipboardApi } from '@/api/clipboard'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'

const isEncode  = ref(true)
const multiline = ref(false)
const input  = ref('')
const output = ref('')

const message = useMessage()

let reqId = 0
watch([input, isEncode, multiline], async ([t, enc, ml]) => {
  const my = ++reqId
  try {
    const fn = enc ? urlApi.encode : urlApi.decode
    const r = await fn(t, ml)
    if (my === reqId) output.value = r
  } catch {
    // invoke 失败：保持上次 output，不打扰用户
  }
}, { immediate: true })

function clearInput() {
  input.value = ''
}

async function readInput() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '文本文件', extensions: ['txt'] }],
  })
  if (typeof path !== 'string') return
  try {
    input.value = await readTextFile(path)
    message.success('已读取')
  } catch {
    message.error('读取文件失败')
  }
}

async function pasteInput() {
  try {
    const text = await clipboardApi.read()
    if (!text) {
      message.info('剪贴板为空')
      return
    }
    input.value = text
    message.success('已粘贴')
  } catch {
    message.error('粘贴失败')
  }
}

async function copyOutput() {
  try {
    await clipboardApi.write(output.value)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}
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
</style>
