<template>
  <div class="img-view">
    <header class="page-head"><h1>图片压缩</h1></header>

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
          <div class="row-desc">选择要压缩的图片</div>
        </div>
        <button class="btn" @click="selectSource">选择文件</button>
      </div>
      <div v-if="sourcePath" class="row file-row">
        <span></span>
        <span class="file-path">{{ sourceName }}</span>
        <span v-if="sourceInfo" class="file-meta">{{ sourceInfo.width }}×{{ sourceInfo.height }} · {{ fmtSize(sourceInfo.size_bytes) }}</span>
      </div>

      <!-- 输出格式 -->
      <div class="row">
        <span class="row-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M7 7h11l-3-3" /><path d="M17 17H6l3 3" />
          </svg>
        </span>
        <div>
          <div class="row-title">输出格式</div>
          <div class="row-desc">选择压缩后的图片格式</div>
        </div>
        <div class="fmt-group">
          <button
            v-for="fmt in formats"
            :key="fmt"
            :class="['btn', outFmt === fmt ? 'btn-active' : '']"
            @click="outFmt = fmt"
          >{{ fmt.toUpperCase() }}</button>
        </div>
      </div>

      <!-- 质量滑块 -->
      <div class="row">
        <span class="row-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3" />
            <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" />
          </svg>
        </span>
        <div>
          <div class="row-title">压缩质量</div>
          <div class="row-desc">值越低压缩率越高，但画质会下降</div>
        </div>
        <div class="quality">
          <input
            v-model.number="quality"
            type="range"
            min="1"
            max="100"
            class="slider"
          />
          <span class="quality-val">{{ quality }}</span>
        </div>
      </div>
    </div>

    <!-- 原始与结果对比 -->
    <div class="compare-wrap" v-if="sourceInfo">
      <div class="section-title"><span>原始图片</span></div>
      <div class="preview-box half">
        <img :src="previewSrc" class="preview-img" />
        <div class="preview-info">
          {{ sourceInfo.width }}×{{ sourceInfo.height }} · {{ fmtSize(sourceInfo.size_bytes) }} · {{ sourceInfo.format.toUpperCase() }}
        </div>
      </div>
    </div>

    <div v-if="resultInfo" class="section-title">
      <span>压缩结果</span>
      <div class="section-actions">
        <button class="btn" @click="saveResult">另存为…</button>
      </div>
    </div>
    <div v-if="resultInfo" class="preview-wrap">
      <div class="preview-box">
        <img :src="resultSrc" class="preview-img" />
        <div class="preview-info">
          {{ resultInfo.width }}×{{ resultInfo.height }} · {{ fmtSize(resultInfo.size_bytes) }} · {{ resultInfo.format.toUpperCase() }}
          <span v-if="sourceInfo" class="size-diff" :class="resultInfo.size_bytes > sourceInfo.size_bytes ? 'diff-up' : 'diff-down'">
            压缩率：{{ compressRatio }}%
          </span>
        </div>
      </div>
    </div>

    <!-- 操作栏 -->
    <div class="bar bar-sticky">
      <span v-if="busy" class="bar-msg">处理中…</span>
      <span v-if="errMsg" class="bar-msg bar-err">{{ errMsg }}</span>
      <button class="btn btn-primary" :disabled="busy || !sourcePath" @click="compress">开始压缩</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import { imageApi, type ImageInfo } from '@/api/image'

const message = useMessage()

const formats = ['jpeg', 'webp', 'png']
const outFmt = ref('jpeg')
const quality = ref(70)

const sourcePath = ref('')
const sourceInfo = ref<ImageInfo | null>(null)
const resultInfo = ref<ImageInfo | null>(null)
const busy = ref(false)
const errMsg = ref('')

const sourceName = computed(() => sourcePath.value ? sourcePath.value.split('/').pop() ?? sourcePath.value : '')
const previewSrc = computed(() => sourceInfo.value ? `data:image/${sourceInfo.value.format};base64,${sourceInfo.value.data_base64}` : '')
const resultSrc = computed(() => resultInfo.value ? `data:image/${resultInfo.value.format};base64,${resultInfo.value.data_base64}` : '')

const compressRatio = computed(() => {
  if (!sourceInfo.value || !resultInfo.value) return 0
  return Math.round((1 - resultInfo.value.size_bytes / sourceInfo.value.size_bytes) * 100)
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

async function compress() {
  if (!sourcePath.value) return
  busy.value = true
  errMsg.value = ''
  resultInfo.value = null

  const ext = outFmt.value === 'jpeg' ? 'jpg' : outFmt.value
  const base = sourceName.value.replace(/\.[^.]+$/, '')
  const defaultName = `${base}_compressed.${ext}`
  const outPath = await saveDialog({
    defaultPath: defaultName,
    filters: [{ name: '图片', extensions: [ext] }],
  })
  if (typeof outPath !== 'string') {
    busy.value = false
    return
  }

  try {
    const r = await imageApi.compress(sourcePath.value, quality.value, outPath)
    resultInfo.value = r
    message.success('压缩成功')
  } catch (e) {
    errMsg.value = String(e)
    message.error(`压缩失败: ${e}`)
  } finally {
    busy.value = false
  }
}

function saveResult() {
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

.quality { display: flex; align-items: center; gap: 10px; }
.slider {
  width: 160px; height: 4px; -webkit-appearance: none; appearance: none;
  background: var(--border-accent); border-radius: 2px; outline: none;
}
.slider::-webkit-slider-thumb {
  -webkit-appearance: none; width: 16px; height: 16px; border-radius: 50%;
  background: var(--ink); cursor: pointer; border: none;
}
.quality-val {
  min-width: 28px; text-align: center; font-size: 14px; font-weight: 600;
  font-family: var(--mono); color: var(--ink);
}

.compare-wrap { display: flex; flex-direction: column; gap: 8px; }

.preview-wrap { flex: 1; min-height: 0; }
.preview-box {
  height: 100%; display: flex; flex-direction: column; gap: 8px;
  border: 1px solid var(--border-accent); border-radius: var(--r-md); padding: 12px;
  background: transparent;
}
.preview-box.half { max-height: 240px; }
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