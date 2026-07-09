<template>
  <header class="page-head">
    <h1>Markdown 预览工具</h1>
  </header>

  <div class="section-title"><span>配置</span></div>
  <div class="config">
    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </span>
      <div>
        <div class="row-title">视图模式</div>
        <div class="row-desc">编辑与预览的分栏方式</div>
      </div>
      <div class="view-group">
        <button v-for="v in viewModes" :key="v.key"
          :class="['view-btn', { active: viewMode === v.key }]"
          @click="viewMode = v.key">{{ v.label }}</button>
      </div>
    </div>
  </div>

  <div class="md-body" :class="`md-${viewMode}`">
    <div v-show="viewMode !== 'preview'" class="md-pane">
      <div class="pane-head">
        <span class="pane-title">Markdown</span>
        <div class="section-actions">
          <PillBtn icon-only title="粘贴" @click="pasteInput">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="3" width="6" height="4" rx="1" />
              <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
            </svg>
          </PillBtn>
          <PillBtn icon-only title="清空" @click="clearInput">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 6l12 12M18 6L6 18" />
            </svg>
          </PillBtn>
        </div>
      </div>
      <CodeArea v-model="input" class="md-editor" />
    </div>

    <div v-show="viewMode !== 'edit'" class="md-pane md-preview-pane">
      <div class="pane-head">
        <span class="pane-title">预览</span>
        <div class="section-actions">
          <PillBtn icon-only title="复制 HTML" @click="copyHtml">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="9" y="9" width="13" height="13" rx="2" />
              <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
            </svg>
          </PillBtn>
        </div>
      </div>
      <div class="md-preview markdown-body" v-html="rendered" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useMessage } from 'naive-ui'
import { marked } from 'marked'
import PillBtn from '@/components/ui/PillBtn.vue'
import CodeArea from '@/components/ui/CodeArea.vue'
import { clipboardApi } from '@/api/clipboard'

type ViewMode = 'split' | 'edit' | 'preview'

const viewModes: { key: ViewMode; label: string }[] = [
  { key: 'split',   label: '分栏' },
  { key: 'edit',    label: '编辑' },
  { key: 'preview', label: '预览' },
]

const viewMode = ref<ViewMode>('split')
const input    = ref(`# Markdown 预览

## 标题

### 三级标题

**粗体** · *斜体* · ~~删除线~~ · \`行内代码\`

\`\`\`python
def hello():
    print("Hello, DevBox!")
\`\`\`

- 列表项 1
- 列表项 2
  - 嵌套列表

1. 有序列表
2. 第二项

> 引用文本

[链接](https://example.com)

---

| 列1 | 列2 |
|-----|-----|
| A   | B   |

![图片占位](https://placehold.co/400x200)
`)
const message = useMessage()

const rendered = computed(() => {
  const text = input.value
  if (!text) return '<p style="color:var(--ink-3)">输入 Markdown 后在此预览</p>'
  try {
    return marked.parse(text, { breaks: true }) as string
  } catch {
    return '<p style="color:var(--warn)">渲染失败</p>'
  }
})

function clearInput() {
  input.value = ''
}

async function pasteInput() {
  try {
    const text = await clipboardApi.read()
    if (!text) { message.info('剪贴板为空'); return }
    input.value = text
    message.success('已粘贴')
  } catch {
    message.error('粘贴失败')
  }
}

async function copyHtml() {
  try {
    await clipboardApi.write(rendered.value)
    message.success('HTML 已复制')
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
.section-actions { display: flex; gap: 4px; align-items: center; }

.config {
  background: color-mix(in srgb, var(--aside-2) 6%, var(--card-2));
  border: 1px solid var(--border-accent);
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

.view-group {
  display: flex; gap: 2px;
  background: color-mix(in srgb, var(--aside-2) 12%, transparent);
  border-radius: 6px; padding: 2px;
}
.view-btn {
  padding: 4px 12px; border: none; border-radius: 4px;
  background: transparent; color: var(--ink-2);
  font-size: 12.5px; cursor: pointer;
  transition: all .15s;
}
.view-btn.active {
  background: var(--card-2); color: var(--ink);
  box-shadow: 0 1px 2px rgba(0,0,0,0.06);
}
.view-btn:hover:not(.active) { color: var(--ink); }

/* 主体区域 */
.md-body {
  flex: 1; min-height: 0;
  display: flex; gap: 12px;
}
.md-body.md-preview { gap: 0; }   /* 预览模式全宽 */

.md-pane {
  display: flex; flex-direction: column;
  min-height: 0; min-width: 0;
  flex: 1; /* 等分 */
}
.md-preview-pane {
  border-left: 1px solid var(--border-accent);
  padding-left: 12px;
}
.md-body.md-edit .md-preview-pane,
.md-body.md-preview .md-pane { display: none; }
.md-body.md-preview .md-preview-pane {
  display: flex; border-left: none; padding-left: 0;
}

.pane-head {
  display: flex; align-items: center; justify-content: space-between;
  margin: 12px 0 8px;
}
.pane-title {
  font-size: 13.5px; font-weight: 500; color: var(--ink-2);
}

.md-editor { flex: 1; min-height: 0; }

/* 预览区域 — 可滚动 */
.md-preview {
  flex: 1; min-height: 0; overflow-y: auto;
  padding: 0 4px;
  font-size: 14px; line-height: 1.7;
  color: var(--ink);
}

/* GitHub 风格 Markdown 样式 */
.md-preview :deep(h1) {
  font-size: 1.85em; font-weight: 600;
  margin: 0.6em 0 0.3em; padding-bottom: 0.2em;
  border-bottom: 1px solid var(--border-accent);
}
.md-preview :deep(h2) {
  font-size: 1.5em; font-weight: 600;
  margin: 0.6em 0 0.25em; padding-bottom: 0.15em;
  border-bottom: 1px solid var(--border-accent);
}
.md-preview :deep(h3) {
  font-size: 1.25em; font-weight: 600;
  margin: 0.5em 0 0.2em;
}
.md-preview :deep(h4),
.md-preview :deep(h5),
.md-preview :deep(h6) {
  font-size: 1.1em; font-weight: 600;
  margin: 0.4em 0 0.2em;
}
.md-preview :deep(p) { margin: 0.5em 0; }
.md-preview :deep(a) { color: var(--link); text-decoration: none; }
.md-preview :deep(a):hover { text-decoration: underline; }
.md-preview :deep(strong) { font-weight: 600; }
.md-preview :deep(code) {
  background: color-mix(in srgb, var(--aside-2) 15%, transparent);
  padding: 2px 5px; border-radius: 3px;
  font-family: var(--mono); font-size: 0.9em;
}
.md-preview :deep(pre) {
  background: color-mix(in srgb, var(--aside-2) 8%, transparent);
  border: 1px solid var(--border-accent);
  border-radius: var(--r-sm);
  padding: 12px; overflow-x: auto;
}
.md-preview :deep(pre code) {
  background: none; padding: 0;
  font-size: 13px; line-height: 1.5;
}
.md-preview :deep(blockquote) {
  border-left: 3px solid var(--aside-3);
  margin: 0.5em 0; padding: 4px 12px;
  color: var(--ink-2);
  background: color-mix(in srgb, var(--aside-2) 6%, transparent);
  border-radius: 0 var(--r-sm) var(--r-sm) 0;
}
.md-preview :deep(ul),
.md-preview :deep(ol) {
  padding-left: 1.8em; margin: 0.4em 0;
}
.md-preview :deep(li) { margin: 0.15em 0; }
.md-preview :deep(hr) {
  border: none; border-top: 1px solid var(--border-accent);
  margin: 1.2em 0;
}
.md-preview :deep(table) {
  border-collapse: collapse; width: 100%;
  margin: 0.6em 0;
}
.md-preview :deep(th),
.md-preview :deep(td) {
  border: 1px solid var(--border-accent);
  padding: 6px 10px; text-align: left;
}
.md-preview :deep(th) {
  background: color-mix(in srgb, var(--aside-2) 8%, transparent);
  font-weight: 500;
}
.md-preview :deep(tr:nth-child(even)) {
  background: color-mix(in srgb, var(--aside-2) 4%, transparent);
}
.md-preview :deep(img) {
  max-width: 100%; height: auto;
  border-radius: var(--r-sm);
}
.md-preview :deep(del) { color: var(--ink-3); }
</style>