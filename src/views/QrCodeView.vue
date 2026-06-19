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
          <PillBtn title="粘贴">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="3" width="6" height="4" rx="1" />
              <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
            </svg>
            <span>粘贴</span>
          </PillBtn>
          <PillBtn icon-only title="读取文件">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 3v5h5" />
              <path d="M14 3H6a2 2 0 00-2 2v14a2 2 0 002 2h12a2 2 0 002-2V8z" />
            </svg>
          </PillBtn>
          <PillBtn icon-only title="清空">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 6l12 12M18 6L6 18" />
            </svg>
          </PillBtn>
          <span class="divider" />
          <PillBtn icon-only title="保存">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z" />
              <path d="M17 21v-8H7v8M7 3v5h8" />
            </svg>
          </PillBtn>
          <PillBtn title="复制">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="9" width="13" height="13" rx="2" />
              <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
            </svg>
            <span>复制</span>
          </PillBtn>
        </div>
      </div>

      <textarea v-model="input" class="text-area" placeholder="在此输入要生成二维码的文本"></textarea>

      <div class="action-row">
        <button class="primary-btn" :disabled="!input.trim()" @click="generate">生成二维码</button>
      </div>
    </div>

    <!-- 右列 -->
    <div class="right-col">
      <!-- 右上：图片输入 -->
      <div class="dropzone">
        <p>将任意一个 BMP, GIF, JPEG, JPG, PBM, PNG, TGA, TIFF, WEBP 文件拖放到此处</p>
        <p class="muted">或者</p>
        <p>
          <a class="link">浏览文件</a>
          <span class="sep">/</span>
          <a class="link">粘贴</a>
        </p>
      </div>

      <!-- 右下：二维码预览 -->
      <div class="preview">
        <div class="preview-title">二维码</div>
        <div class="preview-body">
          <div v-if="svgMarkup" class="svg-wrap" v-html="svgMarkup" />
          <span v-else class="empty-hint">输入文本后点「生成二维码」</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useMessage } from 'naive-ui'
import PillBtn from '@/components/ui/PillBtn.vue'
import { qrApi } from '@/api/qrcode'

const input = ref('')
const svgMarkup = ref('')

const message = useMessage()

async function generate() {
  const text = input.value.trim()
  if (!text) return
  try {
    svgMarkup.value = await qrApi.encode(text)
  } catch (e) {
    const msg = typeof e === 'string' ? e : '生成失败'
    message.error(msg)
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
.divider {
  width: 1px; height: 18px; background: var(--border);
  margin: 0 6px;
}

.text-area {
  flex: 1;
  min-height: 200px;
  padding: 12px 14px;
  font-family: var(--mono, 'SF Mono', Menlo, Consolas, monospace);
  font-size: 13.5px;
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: var(--r-md);
  resize: none; outline: none;
  color: var(--ink-1);
}
.text-area:focus { border-color: var(--accent, #5b8cff); }

.action-row { display: flex; }
.primary-btn {
  height: 32px; padding: 0 16px;
  border-radius: 8px;
  background: var(--accent, #5b8cff);
  color: white; font-size: 13px; font-weight: 500;
  border: none; cursor: pointer;
}
.primary-btn:disabled {
  background: var(--card-2); color: var(--ink-3); cursor: not-allowed;
}

.dropzone {
  border: 1.5px dashed var(--border);
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
.dropzone .sep { margin: 0 12px; color: var(--ink-3); }

.preview {
  flex: 1;
  background: var(--card);
  border-radius: var(--r-md);
  padding: 14px 16px;
  display: flex; flex-direction: column; min-height: 0;
}
.preview-title { font-size: 13.5px; color: var(--ink-2); margin-bottom: 10px; }
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
