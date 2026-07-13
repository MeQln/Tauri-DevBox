<template>
  <header class="page-head">
    <h1>Base64 图片编 / 解码工具</h1>
  </header>

  <div class="grid">
    <!-- 左列：Base64 文本区 -->
    <div class="left-col">
      <div class="section-title">
        <span>Base64</span>
        <div class="section-actions">
          <PillBtn title="复制" @click="copyText" :disabled="!input">
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

      <textarea v-model="input" class="text-area" placeholder="在此输入 Base64 字符串（可含 data: 前缀）" autocorrect="off" spellcheck="false" autocapitalize="off"></textarea>
    </div>

    <!-- 右列 -->
    <div class="right-col">
      <!-- 右上：图片输入 -->
      <div class="dropzone" @dragover.prevent @drop="onDrop">
        <p>将任意一个 BMP, GIF, JPEG, JPG, PNG, WEBP 文件拖放到此处</p>
        <p class="muted">或者</p>
        <p>
          <a class="link" @click="onBrowseImage">浏览文件</a>
        </p>
      </div>

      <!-- 右下：图片预览 -->
      <div class="preview">
        <div class="preview-title">
          <span>图片预览</span>
          <PillBtn icon-only title="保存为图片" @click="saveImage" :disabled="!dataUrl">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4" />
              <path d="M7 10l5 5 5-5" />
              <path d="M12 15V3" />
            </svg>
          </PillBtn>
        </div>
        <div class="preview-body">
          <img v-if="dataUrl" :src="dataUrl" class="preview-img" alt="预览" />
          <span v-else class="empty-hint">输入 Base64 后自动渲染图片</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import { clipboardApi } from '@/api/clipboard'
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import { readFile, readTextFile, writeFile } from '@tauri-apps/plugin-fs'

const input = ref('')
const dataUrl = ref('')
// 编码时记录的 mime；解码裸 Base64 时用于补全 data URL，嗅探失败回退 png。
const mime = ref('image/png')

const message = useMessage()

// 实时解码：文本变化即尝试渲染为图片。
// 解码失败清空预览，不打扰用户（沿用 URL 工具的 watcher 静默策略，见项目 CLAUDE.md "错误处理反原则"）。
watch(input, (raw) => {
  const trimmed = raw.trim()
  if (!trimmed) {
    dataUrl.value = ''
    return
  }
  // 已是 data URL 直接使用。
  if (/^data:image\/[a-zA-Z0-9.+-]+;base64,/.test(trimmed)) {
    dataUrl.value = trimmed
    return
  }
  // 裸 base64：剥离内部换行/空格（终端复制的常带 76 字符折行），否则会同时破坏魔数嗅探与 <img> 解码。
  const b64 = trimmed.replace(/\s+/g, '')
  // 嗅探 mime；失败回退 png（兑现注释，避免沿用上一次旧值造成类型错标）。
  mime.value = sniffMime(b64) ?? 'image/png'
  dataUrl.value = `data:${mime.value};base64,${b64}`
})

const IMAGE_EXTS = ['bmp', 'gif', 'jpeg', 'jpg', 'png', 'webp']

// 由 base64 头部魔数嗅探图片 mime；无法识别返回 null。
function sniffMime(b64: string): string | null {
  const head = b64.slice(0, 16)
  if (head.startsWith('iVBORw0KGgo')) return 'image/png'
  if (head.startsWith('/9j/')) return 'image/jpeg'
  if (head.startsWith('R0lGOD')) return 'image/gif'
  if (head.startsWith('UklGR')) return 'image/webp'
  if (head.startsWith('Qk')) return 'image/bmp'
  return null
}

// 图片 → Base64：FileReader 读取为 DataURL，剥掉前缀得到裸 Base64 写入文本区。
function encodeFile(file: File) {
  if (!file.type.startsWith('image/')) {
    message.warning('请拖入图片文件')
    return
  }
  mime.value = file.type || 'image/png'
  const reader = new FileReader()
  reader.onload = () => {
    const result = String(reader.result || '')
    const idx = result.indexOf(',')
    input.value = idx >= 0 ? result.slice(idx + 1) : result
  }
  reader.onerror = () => message.error('读取文件失败')
  reader.readAsDataURL(file)
}

async function onDrop(e: DragEvent) {
  e.preventDefault()
  const file = e.dataTransfer?.files?.[0]
  if (!file) return
  encodeFile(file)
}

async function onBrowseImage() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '图片', extensions: IMAGE_EXTS }],
  })
  if (typeof path !== 'string') return
  let bytes
  try {
    bytes = await readFile(path)
  } catch {
    message.error('读取文件失败')
    return
  }
  const detected = sniffMimeFromExt(path) || mime.value
  mime.value = detected
  const blob = new Blob([bytes], { type: detected })
  const reader = new FileReader()
  reader.onload = () => {
    const result = String(reader.result || '')
    const idx = result.indexOf(',')
    input.value = idx >= 0 ? result.slice(idx + 1) : result
  }
  reader.onerror = () => message.error('读取文件失败')
  reader.readAsDataURL(blob)
}

// 由扩展名推断 mime（readFile 仅返回字节，不带类型）。
function sniffMimeFromExt(path: string): string | null {
  const ext = path.split('.').pop()?.toLowerCase() || ''
  const map: Record<string, string> = {
    png: 'image/png', jpg: 'image/jpeg', jpeg: 'image/jpeg',
    gif: 'image/gif', webp: 'image/webp', bmp: 'image/bmp',
  }
  return map[ext] || null
}

async function pasteText() {
  try {
    input.value = await clipboardApi.read()
  } catch {
    message.error('粘贴失败')
  }
}

async function readTextFromFile() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '文本', extensions: ['txt'] }],
  })
  if (typeof path !== 'string') return
  try {
    input.value = await readTextFile(path)
  } catch {
    message.error('读取文件失败')
  }
}

async function copyText() {
  if (!input.value) return
  try {
    await clipboardApi.write(input.value)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

// 把当前预览图片落盘：dataUrl → Blob → 字节写文件。
async function saveImage() {
  if (!dataUrl.value) return
  const ext = mime.value.split('/')[1] || 'png'
  const path = await saveDialog({
    filters: [{ name: '图片', extensions: [ext] }],
    defaultPath: `base64-image.${ext}`,
  })
  if (typeof path !== 'string') return
  try {
    const res = await fetch(dataUrl.value)
    const bytes = new Uint8Array(await res.arrayBuffer())
    await writeFile(path, bytes)
    message.success('已保存')
  } catch (e) {
    console.error('保存图片失败:', e)
    message.error('保存失败')
  }
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
  word-break: break-all;
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
.empty-hint { color: var(--ink-3); font-size: 13px; }
.preview-img { max-width: 100%; max-height: 100%; object-fit: contain; }
</style>
