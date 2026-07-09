<template>
  <div class="img-view">
    <header class="page-head"><h1>格式转换</h1></header>

    <div class="section-title"><span>配置</span></div>
    <div class="config">
      <!-- 源文件 -->
      <div class="row">
        <span class="row-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14 3v5h5" />
            <path d="M14 3H6a2 2 0 00-2 2v14a2 2 0 002 2h12a2 2 0 002-2V8z" />
          </svg>
        </span>
        <div>
          <div class="row-title">源文件</div>
          <div class="row-desc">选择要转换格式的图片</div>
        </div>
        <button class="btn" @click="selectSource">选择文件</button>
      </div>
      <div v-if="sourcePath" class="row file-row">
        <span></span>
        <span class="file-path">{{ sourceName }}</span>
        <span v-if="sourceInfo" class="file-meta">{{ sourceInfo.width }}×{{ sourceInfo.height }} · {{ fmtSize(sourceInfo.size_bytes) }}</span>
      </div>

      <!-- 目标格式 -->
      <div class="row">
        <span class="row-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M7 7h11l-3-3" /><path d="M17 17H6l3 3" />
          </svg>
        </span>
        <div>
          <div class="row-title">目标格式</div>
          <div class="row-desc">选择要转换成的图片格式</div>
        </div>
        <div class="fmt-group">
          <button
            v-for="fmt in formats"
            :key="fmt"
            :class="['btn', targetFmt === fmt ? 'btn-active' : '']"
            @click="targetFmt = fmt"
          >{{ fmt.toUpperCase() }}</button>
        </div>
      </div>
    </div>

    <!-- 预览 -->
    <div v-if="sourceInfo" class="section-title"><span>预览</span></div>
    <div v-if="sourceInfo" class="preview-wrap">
      <div class="preview-box">
        <img :src="previewSrc" class="preview-img" />
        <div class="preview-info">
          原始：{{ sourceInfo.width }}×{{ sourceInfo.height }} · {{ fmtSize(sourceInfo.size_bytes) }} · {{ sourceInfo.format.toUpperCase() }}
        </div>
      </div>
    </div>

    <!-- 结果 -->
    <div v-if="resultInfo" class="section-title">
      <span>转换结果</span>
      <div class="section-actions">
        <button class="btn" @click="saveResult">另存为…</button>
      </div>
    </div>
    <div v-if="resultInfo" class="preview-wrap">
      <div class="preview-box">
        <img :src="resultSrc" class="preview-img" />
        <div class="preview-info">
          转换后：{{ resultInfo.width }}×{{ resultInfo.height }} · {{ fmtSize(resultInfo.size_bytes) }} · {{ resultInfo.format.toUpperCase() }}
          <span v-if="sourceInfo && resultInfo" class="size-diff" :class="resultInfo.size_bytes > sourceInfo.size_bytes ? 'diff-up' : 'diff-down'">
            {{ resultInfo.size_bytes > sourceInfo.size_bytes ? '+' : '-' }}{{ fmtSize(diffSize) }}
          </span>
        </div>
      </div>
    </div>

    <!-- 操作栏 -->
    <div class="bar bar-sticky">
      <span v-if="busy" class="bar-msg">处理中…</span>
      <span v-if="errMsg" class="bar-msg bar-err">{{ errMsg }}</span>
      <button class="btn btn-primary" :disabled="busy || !sourcePath" @click="convert">开始转换</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import { imageApi, type ImageInfo } from '@/api/image'

const message = useMessage()

const formats = ['png', 'jpeg', 'webp', 'bmp', 'gif']
const targetFmt = ref('png')

const sourcePath = ref('')
const sourceInfo = ref<ImageInfo | null>(null)
const resultInfo = ref<ImageInfo | null>(null)
const busy = ref(false)
const errMsg = ref('')

const sourceName = computed(() => sourcePath.value ? sourcePath.value.split('/').pop() ?? sourcePath.value : '')
const previewSrc = computed(() => sourceInfo.value ? `data:image/${sourceInfo.value.format};base64,${sourceInfo.value.data_base64}` : '')
const resultSrc = computed(() => resultInfo.value ? `data:image/${resultInfo.value.format};base64,${resultInfo.value.data_base64}` : '')
const diffSize = computed(() => {
  if (!sourceInfo.value || !resultInfo.value) return 0
  return Math.abs(resultInfo.value.size_bytes - sourceInfo.value.size_bytes)
})

function fmtSize(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}KB`
  return `${(bytes / (1024 * 1024)).toFixed(2)}MB`
}

async function selectSource() {
  errMsg.value = ''
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'webp', 'bmp', 'gif', 'tiff', 'tif', 'ico'] }],
  })
  if (typeof path !== 'string') return
  sourcePath.value = path
  resultInfo.value = null
  try {
    sourceInfo.value = await imageApi.read(path)
  } catch (e) {
    message.error(`读取图片失败: ${e}`)
    sourceInfo.value = null
  }
}

async function convert() {
  if (!sourcePath.value) return
  busy.value = true
  errMsg.value = ''
  resultInfo.value = null

  // 自动生成输出文件名
  const base = sourceName.value.replace(/\.[^.]+$/, '')
  const ext = targetFmt.value === 'jpeg' ? 'jpg' : targetFmt.value
  const defaultName = `${base}.${ext}`
  const outPath = await saveDialog({
    defaultPath: defaultName,
    filters: [{ name: '图片', extensions: [ext] }],
  })
  if (typeof outPath !== 'string') {
    busy.value = false
    return
  }

  try {
    const r = await imageApi.convert(sourcePath.value, targetFmt.value, outPath)
    resultInfo.value = r
    message.success('转换成功')
  } catch (e) {
    errMsg.value = String(e)
    message.error(`转换失败: ${e}`)
  } finally {
    busy.value = false
  }
}

async function saveResult() {
  // resultInfo 已有输出路径（通过 saveDialog 已保存），无需另存
  message.info('文件已保存到指定位置')
}
</script>

<style scoped>
.img-view { display: flex; flex-direction: column; gap: 10px; height: 100%; position: relative; }
.page-head h1 {
  font-family: var(--serif); font-size: 28px; font-weight: 500; letter-spacing: -0.015em;
}

.section-title {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13.5px; font-weight: 500; color: var(--ink-2); margin: 6px 0 4px;
}
.section-actions { display: flex; gap: 4px; }

.config {
  background: color-mix(in srgb, var(--aside-2) 6%, var(--card-2));
  border: 1px solid var(--border-accent); border-radius: var(--r-md); padding: 6px;
  display: flex; flex-direction: column; gap: 4px;
}
.row {
  background: var(--card-2); border-radius: 8px; padding: 14px 16px;
  min-height: 64px; display: grid; grid-template-columns: 44px 1fr auto;
  align-items: center; gap: 12px; box-shadow: 0 1px 0 rgba(0,0,0,0.02);
}
.file-row {
  grid-template-columns: 44px 1fr auto;
  padding: 8px 16px; min-height: auto;
}
.row-icon {
  width: 22px; height: 22px; display: inline-flex;
  align-items: center; justify-content: center; color: var(--ink-2);
}
.row-icon :deep(svg) { width: 18px; height: 18px; }
.row-title { font-size: 14px; font-weight: 500; }
.row-desc { font-size: 12.5px; color: var(--ink-3); margin-top: 2px; }

.file-path { font-size: 13px; color: var(--ink); font-family: var(--mono); }
.file-meta { font-size: 12px; color: var(--ink-3); }

.fmt-group { display: flex; gap: 4px; }
.fmt-group .btn { min-width: 52px; padding: 5px 8px; }
.fmt-group .btn-active {
  background: var(--ink); color: var(--card-2); border-color: var(--ink);
}

.preview-wrap { flex: 1; min-height: 0; }
.preview-box {
  height: 100%; display: flex; flex-direction: column; gap: 8px;
  border: 1px solid var(--border-accent); border-radius: var(--r-md); padding: 12px;
  background: transparent;
}
.preview-img {
  flex: 1; min-height: 0; object-fit: contain;
  background: repeating-conic-gradient(#e0e0e0 0% 25%, transparent 0% 50%) 0 0 / 16px 16px;
  border-radius: var(--r-sm); image-rendering: auto;
}
.preview-info { font-size: 12.5px; color: var(--ink-3); display: flex; align-items: center; gap: 8px; }
.size-diff { font-family: var(--mono); font-size: 12px; }
.diff-down { color: var(--ok); }
.diff-up { color: var(--warn); }

.bar-sticky {
  position: sticky; bottom: 0; padding-top: 8px;
  background: var(--card); margin-top: auto;
}
.bar { display: flex; align-items: center; gap: 10px; justify-content: flex-end; }
.bar-msg { font-size: 13px; color: var(--ink-3); flex: 1; }
.bar-err { color: var(--warn); }

.btn {
  padding: 7px 16px; border: 1px solid var(--border-accent); border-radius: var(--r-md);
  background: transparent; color: var(--ink); cursor: pointer; font-size: 13px; white-space: nowrap;
}
.btn:disabled { opacity: 0.5; cursor: not-allowed; }
.btn:not(:disabled):hover { background: color-mix(in srgb, var(--aside-2) 10%, transparent); }
.btn-primary {
  background: var(--ink); color: var(--card-2); border-color: var(--ink);
}
.btn-primary:not(:disabled):hover { opacity: 0.85; }
</style>