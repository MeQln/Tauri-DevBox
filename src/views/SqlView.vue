<template>
  <header class="page-head">
    <h1>SQL 格式化工具</h1>
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
      <Switch v-model="isFormat" on-label="格式化" off-label="压缩" />
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M8 3H6a2 2 0 00-2 2v14a2 2 0 002 2h2" />
          <path d="M16 3h2a2 2 0 012 2v14a2 2 0 01-2 2h-2" />
        </svg>
      </span>
      <div>
        <div class="row-title">缩进</div>
        <div class="row-desc">格式化时使用的缩进宽度（压缩模式下无效）</div>
      </div>
      <Switch v-model="indent4" on-label="4 空格" off-label="2 空格" />
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
  <textarea v-model="input" class="text-area" placeholder="在此输入 SQL" autocorrect="off" spellcheck="false" autocapitalize="off"></textarea>

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
  <div v-if="error" class="error-bar">{{ error }}</div>
  <textarea v-model="output" class="text-area" readonly placeholder="格式化结果将在此显示"></textarea>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import { format } from 'sql-formatter'
import PillBtn from '@/components/ui/PillBtn.vue'
import Switch from '@/components/ui/Switch.vue'
import { clipboardApi } from '@/api/clipboard'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'

const isFormat = ref(true)
const indent4  = ref(false)
const input  = ref('')
const output = ref('')
const error  = ref('')

const message = useMessage()

watch([input, isFormat, indent4], ([t, fmt, i4]) => {
  const text = t.trim()
  if (!text) { output.value = ''; error.value = ''; return }
  try {
    output.value = fmt
      ? format(text, { tabWidth: i4 ? 4 : 2, keywordCase: 'upper' })
      : text.replace(/\s+/g, ' ').trim()
    error.value = ''
  } catch (e) {
    output.value = ''
    error.value = (e as Error).message
  }
}, { immediate: true })

function clearInput() {
  input.value = ''
}

async function readInput() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: 'SQL 文件', extensions: ['sql', 'txt'] }],
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
  if (!output.value) return
  try {
    await clipboardApi.write(output.value)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}
</script>

<style scoped>
/* 所有样式已移至 src/styles/common.css */
</style>
