<template>
  <header class="page-head">
    <h1>二维码编 / 解码工具</h1>
  </header>

  <div class="grid">
    <!-- 左列：文本区 -->
    <div class="left-col">
      <div class="section-title">
        <span>文本</span>
        <div class="section-actions">
          <PillBtn title="复制" @click="copyText">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="9" width="13" height="13" rx="2" />
              <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
            </svg>
            复制
          </PillBtn>
          <PillBtn title="粘贴" @click="pasteText">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="3" width="6" height="4" rx="1" />
              <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
            </svg>
            粘贴
          </PillBtn>
          <PillBtn title="文件" @click="readTextFromFile">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 3v5h5" />
              <path d="M14 3H6a2 2 0 00-2 2v14a2 2 0 002 2h12a2 2 0 002-2V8z" />
            </svg>
            文件
          </PillBtn>
                    <PillBtn title="清空" @click="clearInput">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 6l12 12M18 6L6 18" />
            </svg>
            清空
          </PillBtn>
        </div>
      </div>

      <textarea v-model="input" class="text-area" placeholder="在此输入要生成二维码的文本" autocorrect="off" spellcheck="false" autocapitalize="off"></textarea>
    </div>

    <!-- 右列 -->
    <div class="right-col">
      <!-- 右上：图片输入 -->
      <div class="dropzone" @dragover.prevent @drop="onDrop">
        <p>将任意一个 BMP, GIF, JPEG, JPG, PBM, PNG, TGA, TIF, TIFF, WEBP 文件拖放到此处</p>
        <p class="muted">或者</p>
        <p>
          <a class="link" @click="onBrowseImage">浏览文件</a>
        </p>
      </div>

      <!-- 右下：二维码预览 -->
      <div class="preview">
        <div class="preview-title">
          <span>二维码</span>
          <PillBtn icon-only title="保存为图片" @click="saveImage" :disabled="!svgMarkup">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4" />
              <path d="M7 10l5 5 5-5" />
              <path d="M12 15V3" />
            </svg>
          </PillBtn>
        </div>
        <div class="preview-body">
          <div v-if="svgMarkup" class="svg-wrap" v-html="svgMarkup" />
          <span v-else class="empty-hint">输入文本后自动生成二维码</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import { qrApi } from '@/api/qrcode'
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import { readFile, readTextFile, writeFile } from '@tauri-apps/plugin-fs'

const input = ref('')
const svgMarkup = ref('')

const message = useMessage()

// 实时生成：输入文本变化即调 Rust 编码；空文本清空预览。
// reqId 防止旧响应覆盖新结果；编码失败保留上次预览，不打扰用户（沿用 URL 工具的 watcher 静默策略，见项目 CLAUDE.md "错误处理反原则"）。
let encodeReqId = 0
watch(input, async (text) => {
  const my = ++encodeReqId
  if (!text) {
    svgMarkup.value = ''
    return
  }
  try {
    const svg = await qrApi.encode(text)
    if (my === encodeReqId) svgMarkup.value = svg
  } catch {
    // 静默：保留上次 svgMarkup
  }
}, { immediate: true })

const IMAGE_EXTS = ['bmp', 'gif', 'jpeg', 'jpg', 'pbm', 'png', 'tga', 'tif', 'tiff', 'webp']

let decodeReqId = 0

async function decodeBytes(bytes: number[]) {
  const my = ++decodeReqId
  try {
    const text = await qrApi.decode(bytes)
    if (my === decodeReqId) input.value = text
  } catch (e) {
    if (my !== decodeReqId) return
    const msg = typeof e === 'string' ? e : '未识别到二维码'
    message.error(msg)
  }
}

async function onDrop(e: DragEvent) {
  e.preventDefault()
  const file = e.dataTransfer?.files?.[0]
  if (!file) return
  if (!file.type.startsWith('image/')) {
    message.warning('请拖入图片文件')
    return
  }
  let buf
  try {
    buf = await file.arrayBuffer()
  } catch {
    message.error('读取文件失败')
    return
  }
  await decodeBytes(Array.from(new Uint8Array(buf)))
}

async function onBrowseImage() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '图片', extensions: IMAGE_EXTS }],
  })
  if (typeof path !== 'string') return
  let data
  try {
    data = await readFile(path)
  } catch {
    message.error('读取文件失败')
    return
  }
  await decodeBytes(Array.from(data))
}

async function pasteText() {
  try {
    input.value = await navigator.clipboard.readText()
  } catch {
    message.error('粘贴失败')
  }
}

async function readTextFromFile() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '文本', extensions: ['txt', 'md', 'log', 'json', 'csv'] }],
  })
  if (typeof path !== 'string') return
  try {
    input.value = await readTextFile(path)
  } catch {
    message.error('读取文件失败')
  }
}

async function copyText() {
  try {
    await navigator.clipboard.writeText(input.value)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

// 把当前 SVG 二维码光栅化为 PNG 字节：Blob URL → Image → canvas（白底）→ toBlob。
async function saveImage() {
  if (!svgMarkup.value) return
  const path = await saveDialog({
    filters: [{ name: 'PNG 图片', extensions: ['png'] }],
    defaultPath: 'qrcode.png',
  })
  if (typeof path !== 'string') return
  try {
    const bytes = await svgToPngBytes(svgMarkup.value)
    await writeFile(path, bytes)
    message.success('已保存')
  } catch (e) {
    console.error('保存二维码图片失败:', e)
    message.error('保存失败')
  }
}

function svgToPngBytes(svg: string): Promise<Uint8Array> {
  const size = 1024
  return new Promise((resolve, reject) => {
    const blob = new Blob([svg], { type: 'image/svg+xml;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const img = new Image()
    img.onload = () => {
      const canvas = document.createElement('canvas')
      canvas.width = size
      canvas.height = size
      const ctx = canvas.getContext('2d')
      if (!ctx) { URL.revokeObjectURL(url); reject(new Error('no 2d context')); return }
      ctx.fillStyle = '#ffffff'
      ctx.fillRect(0, 0, size, size)
      ctx.drawImage(img, 0, 0, size, size)
      URL.revokeObjectURL(url)
      canvas.toBlob(async (b) => {
        if (!b) { reject(new Error('toBlob failed')); return }
        resolve(new Uint8Array(await b.arrayBuffer()))
      }, 'image/png')
    }
    img.onerror = () => { URL.revokeObjectURL(url); reject(new Error('svg load failed')) }
    img.src = url
  })
}

function clearInput() {
  input.value = ''
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
  font-family: var(--mono, 'SF Mono', Menlo, Consolas, monospace);
  font-size: 13.5px;
  background: transparent;
  border: 1px solid var(--border-accent);
  border-radius: var(--r-md);
  resize: none; outline: none;
  color: var(--ink);
}
.text-area:focus { border-color: var(--aside-3); }

.dropzone {
  border: 2px dashed var(--border-accent);
  border-radius: var(--r-md);
  padding: 24px 18px;
  text-align: center;
  font-size: 13.5px; color: var(--ink-2);
  display: flex; flex-direction: column; gap: 8px;
}
.dropzone .muted { color: var(--ink-3); }
.dropzone .link {
  color: var(--accent, #5b8cff); cursor: pointer; font-weight: 500;
}

.preview {
  flex: 1;
  background: transparent;
  border: 1px solid var(--border-accent);
  border-radius: var(--r-md);
  padding: 14px 16px;
  display: flex; flex-direction: column; min-height: 0;
}
.preview-title {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13.5px; color: var(--ink-2); margin-bottom: 10px;
}
.preview-body {
  flex: 1;
  display: flex; align-items: center; justify-content: center;
}
.preview-body :deep(svg) { max-width: 100%; max-height: 100%; }
.empty-hint { color: var(--ink-3); font-size: 13px; }
.svg-wrap {
  width: 100%; height: 100%;
  display: flex; align-items: center; justify-content: center;
}
.svg-wrap :deep(svg) { width: min(100%, 320px); height: auto; }
</style>
