# 二维码编 / 解码工具 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 DevBox 中新增「二维码编 / 解码工具」页面 `/tools/qrcode`，支持文本 → 二维码 SVG（点按钮生成）和图片（拖拽 / 浏览 / 粘贴）→ 文本（写回左侧）双向能力。

**Architecture:** 沿用 URL 工具的「核心逻辑下沉 Rust」契约：Rust 端两个 `#[tauri::command]`（`qr_encode` / `qr_decode`），前端经 `src/api/qrcode.ts` 单一入口 invoke。新增 `QrCodeView.vue` 与 `qrcode.rs`，**不**改 `nav.ts`、不改 `tools/url.rs`。引入 Tauri 官方 dialog / fs 插件以支持选文件 / 读文件 / 保存。

**Tech Stack:** Vue 3、TypeScript、Tauri 2、Pinia（已有，不动）、Naive UI（仅 `useMessage`）、Rust crate `qrcode 0.14` + `rqrr 0.7` + `image 0.25`、Tauri 插件 `tauri-plugin-dialog` + `tauri-plugin-fs`。

## Global Constraints

- **架构契约：** 编 / 解码核心逻辑下沉 Rust，前端不做 fallback；前端只做 UI 与 invoke 封装
- **错误返回策略：** `qr_encode` / `qr_decode` 用 `Result<String, String>`（与 URL 工具的「失败返回原文」**不同**）；前端 catch 后用 `n-message` 反馈
- **导出形态：** Rust 端返回 SVG 字符串（不是 PNG / base64）；前端用 `v-html` 注入预览面板
- **生成时机：** 编码 = 用户**点按钮**触发，**不**做实时生成；空文本时按钮 disabled
- **解码回填：** 解码结果写回左侧文本区，但**不**自动触发生成（避免连锁）
- **不改 nav.ts：** 导航项 `qrcode` 已存在于 `src/stores/nav.ts:26`
- **不改 tools/url.rs：** 精准变更原则
- **commit message：** 中文、`类型: 简短描述` 格式（参见 CLAUDE.md）
- **不引入：** ESLint / Prettier / Vitest / Playwright（CLAUDE.md 明确禁止）
- **不暴露配置项：** ECC 等级、二维码尺寸、边距等使用 crate 默认值
- **不做导出 PNG：** 预览即 SVG，YAGNI；浏览器右键即可保存

---

## File Structure

```
fs-tauri/
├── src-tauri/
│   ├── Cargo.toml                       [Task 1, 4 修改：加依赖]
│   ├── capabilities/default.json        [Task 4 修改：加 dialog/fs 权限]
│   └── src/
│       ├── lib.rs                       [Task 3, 4 修改：注册 command + 插件]
│       └── tools/
│           ├── url.rs                   ← 不动
│           └── qrcode.rs                [Task 1, 2 创建]
│
├── package.json                         [Task 4 修改：加 npm 插件]
└── src/
    ├── router/index.ts                  [Task 5 修改：加路由]
    ├── api/
    │   └── qrcode.ts                    [Task 5 创建]
    └── views/
        └── QrCodeView.vue               [Task 5 创建占位 → Task 6 布局 → Task 7 编码 → Task 8 解码 → Task 9 工具栏]
```

**职责划分：**
- `tools/qrcode.rs` —— 仅编 / 解码与 fixture 单测；不知道前端
- `api/qrcode.ts` —— 极薄 invoke 封装；不知道 UI
- `QrCodeView.vue` —— 页面 + 交互 + 调插件 dialog/fs；调 `qrApi`

---

## Task 1: Rust 编码命令 `qr_encode`

**Files:**
- Modify: `src-tauri/Cargo.toml`（追加 `qrcode`、`image` deps）
- Create: `src-tauri/src/tools/qrcode.rs`

**Interfaces:**
- Consumes: 无
- Produces: `pub fn qr_encode(text: String) -> Result<String, String>` —— 返回 SVG 字符串（含 `<svg`），文本为空返回 `Err("文本为空")`，其余生成失败返回 `Err("文本过长，无法生成")`

- [ ] **Step 1: 追加 Cargo 依赖**

修改 `src-tauri/Cargo.toml` 的 `[dependencies]` 段，在末尾追加：

```toml
qrcode = { version = "0.14", default-features = false, features = ["svg"] }
image = { version = "0.25", default-features = false, features = ["png", "jpeg", "bmp", "gif", "tiff", "webp", "pnm", "tga"] }
rqrr = "0.7"
```

`rqrr` 此 task 暂未用到，但 Task 2 会用，一次性加完免得 Cargo.lock 反复变。

- [ ] **Step 2: 创建 qrcode.rs 并写失败的单测**

创建 `src-tauri/src/tools/qrcode.rs`，写入：

```rust
use qrcode::QrCode;
use qrcode::render::svg;

#[tauri::command]
pub fn qr_encode(text: String) -> Result<String, String> {
    Err("not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ascii_returns_svg() {
        let r = qr_encode("hello".into()).expect("应成功");
        assert!(r.contains("<svg"), "返回值应是 SVG markup");
    }

    #[test]
    fn encode_chinese_returns_svg() {
        let r = qr_encode("你好世界".into()).expect("应成功");
        assert!(r.contains("<svg"));
    }

    #[test]
    fn encode_empty_returns_err() {
        assert!(qr_encode("".into()).is_err());
    }
}
```

- [ ] **Step 3: 把 qrcode 模块挂到 tools/mod.rs**

查看 `src-tauri/src/tools/mod.rs` 当前内容（应只有 `pub mod url;`）；追加一行：

```rust
pub mod qrcode;
```

- [ ] **Step 4: 跑测试确认失败**

```bash
cd src-tauri && cargo test --lib tools::qrcode
```

预期：3 个测试中至少 `encode_ascii_returns_svg`、`encode_chinese_returns_svg` FAIL（因为返回 Err），`encode_empty_returns_err` 巧合 PASS。

- [ ] **Step 5: 实现最小逻辑**

把 `qr_encode` 函数体替换为：

```rust
#[tauri::command]
pub fn qr_encode(text: String) -> Result<String, String> {
    if text.is_empty() {
        return Err("文本为空".into());
    }
    let code = QrCode::new(text.as_bytes())
        .map_err(|_| "文本过长，无法生成".to_string())?;
    Ok(code.render::<svg::Color>().build())
}
```

- [ ] **Step 6: 跑测试确认通过**

```bash
cd src-tauri && cargo test --lib tools::qrcode
```

预期：3 个 test 全部 PASS。

- [ ] **Step 7: 提交**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/tools/qrcode.rs src-tauri/src/tools/mod.rs
git commit -m "feat: 二维码编码 Rust command qr_encode"
```

---

## Task 2: Rust 解码命令 `qr_decode`

**Files:**
- Modify: `src-tauri/src/tools/qrcode.rs`

**Interfaces:**
- Consumes: Task 1 的 `qr_encode`（仅在单测中用，确保自洽）
- Produces: `pub fn qr_decode(image: Vec<u8>) -> Result<String, String>` —— 入参图片二进制，能识别返回文本，无法识别返回 `Err("未识别到二维码")`

- [ ] **Step 1: 写失败的单测**

在 `src-tauri/src/tools/qrcode.rs` 的 `mod tests` 内追加：

```rust
fn make_qr_png_bytes(text: &str) -> Vec<u8> {
    use image::{ImageBuffer, Luma};
    use std::io::Cursor;
    let code = QrCode::new(text.as_bytes()).unwrap();
    let img: ImageBuffer<Luma<u8>, Vec<u8>> = code
        .render::<Luma<u8>>()
        .module_dimensions(8, 8)
        .build();
    let mut buf = Cursor::new(Vec::new());
    image::DynamicImage::ImageLuma8(img)
        .write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();
    buf.into_inner()
}

#[test]
fn decode_round_trip_ascii() {
    let png = make_qr_png_bytes("hello");
    assert_eq!(qr_decode(png).expect("应解码成功"), "hello");
}

#[test]
fn decode_round_trip_chinese() {
    let png = make_qr_png_bytes("你好世界");
    assert_eq!(qr_decode(png).expect("应解码成功"), "你好世界");
}

#[test]
fn decode_blank_image_returns_err() {
    use image::{ImageBuffer, Luma};
    use std::io::Cursor;
    let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_pixel(64, 64, Luma([255]));
    let mut buf = Cursor::new(Vec::new());
    image::DynamicImage::ImageLuma8(img)
        .write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();
    assert!(qr_decode(buf.into_inner()).is_err());
}

#[test]
fn decode_garbage_bytes_returns_err() {
    assert!(qr_decode(vec![0u8, 1, 2, 3]).is_err());
}
```

- [ ] **Step 2: 添加 `qr_decode` 占位实现**

在 `qr_encode` 函数下方追加：

```rust
#[tauri::command]
pub fn qr_decode(image: Vec<u8>) -> Result<String, String> {
    Err("not implemented".into())
}
```

- [ ] **Step 3: 跑测试确认失败**

```bash
cd src-tauri && cargo test --lib tools::qrcode
```

预期：`decode_round_trip_*` 两条 FAIL，`decode_blank_image_returns_err` / `decode_garbage_bytes_returns_err` 巧合 PASS。

- [ ] **Step 4: 实现解码**

把 `qr_decode` 函数体替换为：

```rust
#[tauri::command]
pub fn qr_decode(image: Vec<u8>) -> Result<String, String> {
    let img = image::load_from_memory(&image).map_err(|_| "图片解析失败".to_string())?;
    let luma = img.to_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(luma);
    let grids = prepared.detect_grids();
    if grids.is_empty() {
        return Err("未识别到二维码".into());
    }
    let (_meta, content) = grids[0].decode().map_err(|_| "未识别到二维码".to_string())?;
    Ok(content)
}
```

- [ ] **Step 5: 跑测试确认通过**

```bash
cd src-tauri && cargo test --lib tools::qrcode
```

预期：7 条全部 PASS（Task 1 的 3 条 + 本任务 4 条）。

- [ ] **Step 6: 提交**

```bash
git add src-tauri/src/tools/qrcode.rs
git commit -m "feat: 二维码解码 Rust command qr_decode"
```

---

## Task 3: 注册 commands 到 invoke_handler

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: Task 1 / 2 的两个 `#[tauri::command]`
- Produces: 让前端 `invoke('qr_encode' | 'qr_decode')` 能成功路由到 Rust

- [ ] **Step 1: 修改 lib.rs**

打开 `src-tauri/src/lib.rs`，把 `invoke_handler!` 块改为：

```rust
.invoke_handler(tauri::generate_handler![
  tools::url::url_encode,
  tools::url::url_decode,
  tools::qrcode::qr_encode,
  tools::qrcode::qr_decode,
])
```

- [ ] **Step 2: 编译验证**

```bash
cd src-tauri && cargo build
```

预期：`Finished` 退出 0；不应出现 `unresolved import` 或 `not found in module tools::qrcode`。

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: 注册 qr_encode / qr_decode 到 invoke_handler"
```

---

## Task 4: 引入 Tauri dialog / fs 插件

**Files:**
- Modify: `src-tauri/Cargo.toml`、`src-tauri/src/lib.rs`、`src-tauri/capabilities/default.json`、`package.json`

**Interfaces:**
- Consumes: 无
- Produces: 前端可 `import { open, save } from '@tauri-apps/plugin-dialog'`、`import { readFile, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'` 并正常调用

> **说明：** Tauri 2 提供 `pnpm tauri add <plugin>` 一键脚本，会同时改 `Cargo.toml`、`lib.rs`、`capabilities/default.json`、`package.json`。本任务**优先**用此 CLI；下面的手工对照仅作 review 参考。

- [ ] **Step 1: 一键加 dialog 插件**

```bash
cd /Users/mengql/workspace/ClaudeCode/Front-Skeleton/fs-desktop/fs-tauri
pnpm tauri add dialog
```

预期 CLI 输出 `Adding dialog Tauri Plugin`，并自动修改：
- `src-tauri/Cargo.toml` 增 `tauri-plugin-dialog = "2"`
- `src-tauri/src/lib.rs` 增 `.plugin(tauri_plugin_dialog::init())`
- `src-tauri/capabilities/default.json` 增 `"dialog:default"`
- `package.json` 增 `@tauri-apps/plugin-dialog`

- [ ] **Step 2: 一键加 fs 插件**

```bash
pnpm tauri add fs
```

同上，期望增加：
- `tauri-plugin-fs = "2"`
- `.plugin(tauri_plugin_fs::init())`
- `"fs:default"` 到 capabilities
- `@tauri-apps/plugin-fs` 到 package.json

- [ ] **Step 3: 放开 fs 读写图片 / 文本权限**

打开 `src-tauri/capabilities/default.json`，确认 `permissions` 数组包含以下条目（CLI 通常只加 `fs:default`，需手动补允许具体操作的细粒度权限）：

```json
{
  "permissions": [
    "core:default",
    "dialog:default",
    "fs:default",
    "fs:allow-read-file",
    "fs:allow-write-text-file"
  ]
}
```

> 若 `fs:allow-read-file` / `fs:allow-write-text-file` 这两个条目运行时报 `permission not found`，改为 `"fs:allow-read"` / `"fs:allow-write"`（Tauri 2 不同 minor 版本权限标识有差异——以 `pnpm tauri:dev` 启动时控制台报错为准调整；这是已知**待运行时验证**的边界）。

- [ ] **Step 4: 编译验证**

```bash
cd src-tauri && cargo build
```

预期：`Finished` 退出 0；首次会下载两个新 crate。

- [ ] **Step 5: 启动一次确认无 capability 报错**

```bash
cd /Users/mengql/workspace/ClaudeCode/Front-Skeleton/fs-desktop/fs-tauri
pnpm tauri:dev
```

窗口启动后立刻 Ctrl+C 关闭。预期：终端不出现 `Permission denied` 或 `capability not allowed`。若有，按 Step 3 的 fallback 调权限标识、再启一次。

- [ ] **Step 6: 提交**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/lib.rs src-tauri/capabilities/default.json package.json pnpm-lock.yaml
git commit -m "chore: 接入 tauri-plugin-dialog / tauri-plugin-fs"
```

---

## Task 5: 前端 API 封装 + 路由 + 空白页跳通

**Files:**
- Create: `src/api/qrcode.ts`
- Create: `src/views/QrCodeView.vue`（占位版，下一 task 才铺布局）
- Modify: `src/router/index.ts`

**Interfaces:**
- Consumes: Task 3 注册的两个 invoke 命令
- Produces:
  - `qrApi.encode(text: string) => Promise<string>`（SVG markup）
  - `qrApi.decode(bytes: number[]) => Promise<string>`（解码文本）
  - 路由 `/tools/qrcode` 渲染 `QrCodeView`，导航点击「二维码」不再命中 `PlaceholderView`

- [ ] **Step 1: 创建 qrApi**

创建 `src/api/qrcode.ts`：

```ts
import { invoke } from '@tauri-apps/api/core'

export const qrApi = {
  encode: (text: string) => invoke<string>('qr_encode', { text }),
  decode: (image: number[]) => invoke<string>('qr_decode', { image }),
}
```

> 注意：Tauri 命令参数名与 Rust 函数参数名严格对应（Rust 的 `text: String` → JS 的 `{ text }`，Rust 的 `image: Vec<u8>` → JS 的 `{ image: number[] }`）。

- [ ] **Step 2: 创建占位 QrCodeView**

创建 `src/views/QrCodeView.vue`：

```vue
<template>
  <header class="page-head">
    <h1>二维码编 / 解码工具</h1>
  </header>
  <p style="color: var(--ink-3); font-size: 14px;">页面骨架待 Task 6 铺设</p>
</template>

<script setup lang="ts">
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
</style>
```

- [ ] **Step 3: 加路由**

打开 `src/router/index.ts`，在 `tools/url` 那一行**之后**插入新路由：

```ts
{ path: 'tools/url', component: () => import('@/views/UrlView.vue') },
{ path: 'tools/qrcode', component: () => import('@/views/QrCodeView.vue') },
{ path: 'tools/:id', component: () => import('@/views/PlaceholderView.vue') },
```

> `tools/qrcode` 必须放在 `tools/:id` **之前**，否则会被通配路由先匹配。

- [ ] **Step 4: 类型检查**

```bash
pnpm exec vue-tsc -b
```

预期：exit 0，无 error。

- [ ] **Step 5: 启动验证导航**

```bash
pnpm tauri:dev
```

启动后点左侧「二维码」导航项；预期地址栏（如可见）跳到 `/tools/qrcode`，页面顶部显示「二维码编 / 解码工具」标题与占位文字「页面骨架待 Task 6 铺设」，**不**显示 PlaceholderView 的 Coming Soon。

确认后 Ctrl+C 关闭。

- [ ] **Step 6: 提交**

```bash
git add src/api/qrcode.ts src/views/QrCodeView.vue src/router/index.ts
git commit -m "feat: 二维码工具路由 + qrApi 封装 + 空白页跳通"
```

---

## Task 6: QrCodeView 布局骨架

**Files:**
- Modify: `src/views/QrCodeView.vue`

**Interfaces:**
- Consumes: 现有 `PillBtn` 组件（`src/components/ui/PillBtn.vue`）、CSS 变量 `--card`、`--card-2`、`--border`、`--ink-2`、`--ink-3`、`--r-md`
- Produces: 完整双列布局（左文本区 + 右上图片输入 + 右下二维码预览），所有 UI 元素到位但**无逻辑**

- [ ] **Step 1: 替换 QrCodeView.vue 全部内容**

```vue
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
        <button class="primary-btn" :disabled="!input.trim()">生成二维码</button>
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
          <span class="empty-hint">输入文本后点「生成二维码」</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PillBtn from '@/components/ui/PillBtn.vue'

const input = ref('')
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
</style>
```

- [ ] **Step 2: 类型检查**

```bash
pnpm exec vue-tsc -b
```

预期：exit 0。

- [ ] **Step 3: 启动并视觉验证**

```bash
pnpm tauri:dev
```

进入「二维码」页，确认：
- 标题「二维码编 / 解码工具」位于顶部
- 左右双列等宽
- 左列从上到下：工具栏 5 按钮（中间分隔线）+ 文本区（占满）+「生成二维码」按钮（disabled 灰色）
- 右上虚线框含说明文字 + "或者" + 「浏览文件」/「粘贴」两个蓝色链接
- 右下灰色卡片中央显示「输入文本后点「生成二维码」」灰字提示
- 输入文字后「生成二维码」按钮变蓝可点（视觉上）

确认后 Ctrl+C 关闭。

- [ ] **Step 4: 提交**

```bash
git add src/views/QrCodeView.vue
git commit -m "feat: 二维码页布局骨架"
```

---

## Task 7: 编码联通（生成按钮 → SVG 显示）

**Files:**
- Modify: `src/views/QrCodeView.vue`

**Interfaces:**
- Consumes: `qrApi.encode`（Task 5）、Naive UI `useMessage`
- Produces: 点「生成二维码」后右下区显示 SVG；失败时 toast 错误

- [ ] **Step 1: 在 `<script setup>` 中加入生成逻辑**

把现有 `<script setup>` 整段替换为：

```ts
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
```

> 这里的生成函数只在按钮点击时调用，不监听 `input` 变化（保持「点按钮才生成」的契约）。

- [ ] **Step 2: 在按钮上挂 click**

把 `<button class="primary-btn" :disabled="!input.trim()">生成二维码</button>` 替换为：

```html
<button class="primary-btn" :disabled="!input.trim()" @click="generate">生成二维码</button>
```

- [ ] **Step 3: 在预览区注入 SVG**

把 `.preview-body` 整个 div 替换为：

```html
<div class="preview-body">
  <div v-if="svgMarkup" class="svg-wrap" v-html="svgMarkup" />
  <span v-else class="empty-hint">输入文本后点「生成二维码」</span>
</div>
```

并在 `<style scoped>` 末尾追加：

```css
.svg-wrap {
  width: 100%; height: 100%;
  display: flex; align-items: center; justify-content: center;
}
.svg-wrap :deep(svg) { width: min(100%, 320px); height: auto; }
```

- [ ] **Step 4: 类型检查**

```bash
pnpm exec vue-tsc -b
```

预期：exit 0。

- [ ] **Step 5: 启动端到端验证**

```bash
pnpm tauri:dev
```

进入「二维码」页：
1. 左侧文本框输入 `hello`，按钮变蓝；点击「生成二维码」
2. 预期右下区出现一个黑白二维码 SVG
3. 文本改为 `你好世界` 再点；预期 SVG 刷新（与上次不同）
4. 清空文本框；按钮变灰禁用；右下保留上一次 SVG（不清空，与策略一致）

> 用手机扫一扫此二维码，应能识别出对应文本，验证 SVG 内容正确。

Ctrl+C 关闭。

- [ ] **Step 6: 提交**

```bash
git add src/views/QrCodeView.vue
git commit -m "feat: 二维码生成（点按钮 → SVG 预览）"
```

---

## Task 8: 解码联通（拖拽 / 浏览 / 粘贴 → 写回文本）

**Files:**
- Modify: `src/views/QrCodeView.vue`

**Interfaces:**
- Consumes: `qrApi.decode`、`@tauri-apps/plugin-dialog` 的 `open`、`@tauri-apps/plugin-fs` 的 `readFile`、浏览器原生 `navigator.clipboard.read`
- Produces: 三种图片输入方式都能把识别文本写回 `input.value`；失败 toast；连续操作只取最后一次结果

- [ ] **Step 1: 在 `<script setup>` 增加 decode 流程**

在现有 `<script setup>` 顶部 `import` 段追加：

```ts
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
```

在 `generate` 函数下方追加：

```ts
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
  const buf = await file.arrayBuffer()
  await decodeBytes(Array.from(new Uint8Array(buf)))
}

async function onBrowseImage() {
  const path = await openDialog({
    multiple: false,
    filters: [{ name: '图片', extensions: IMAGE_EXTS }],
  })
  if (typeof path !== 'string') return
  const data = await readFile(path)
  await decodeBytes(Array.from(data))
}

async function onPasteImage() {
  try {
    const items = await navigator.clipboard.read()
    for (const item of items) {
      const imgType = item.types.find(t => t.startsWith('image/'))
      if (!imgType) continue
      const blob = await item.getType(imgType)
      const buf = await blob.arrayBuffer()
      await decodeBytes(Array.from(new Uint8Array(buf)))
      return
    }
    message.warning('剪贴板中没有图片')
  } catch {
    message.warning('剪贴板中没有图片')
  }
}
```

- [ ] **Step 2: 在 dropzone 上挂事件**

把 `<div class="dropzone">` 整段替换为：

```html
<div class="dropzone" @dragover.prevent @drop="onDrop">
  <p>将任意一个 BMP, GIF, JPEG, JPG, PBM, PNG, TGA, TIFF, WEBP 文件拖放到此处</p>
  <p class="muted">或者</p>
  <p>
    <a class="link" @click="onBrowseImage">浏览文件</a>
    <span class="sep">/</span>
    <a class="link" @click="onPasteImage">粘贴</a>
  </p>
</div>
```

- [ ] **Step 3: 类型检查**

```bash
pnpm exec vue-tsc -b
```

预期：exit 0。如果报 `Cannot find module '@tauri-apps/plugin-dialog'`，回到 Task 4 检查 `pnpm install` 是否完成。

- [ ] **Step 4: 启动端到端验证**

```bash
pnpm tauri:dev
```

**前置：** 用 Task 7 验证时生成的二维码截屏一张存到桌面（如 `~/Desktop/qr.png`）；或用任意网络二维码图片。

1. **拖拽：** 把 `qr.png` 拖到右上虚线框 → 左侧文本框出现原文（如 "hello"）
2. **浏览文件：** 点「浏览文件」→ 系统对话框弹出 → 选 `qr.png` → 左侧出现原文
3. **粘贴：** 截屏一张二维码到剪贴板（macOS：Cmd+Ctrl+Shift+4 框选） → 点「粘贴」→ 左侧出现原文
4. **错误反馈：** 拖入一张纯白 PNG / 一张普通照片 → toast 显示「未识别到二维码」
5. **错误反馈：** 拖入一个 `.txt` 文件 → toast 显示「请拖入图片文件」

Ctrl+C 关闭。

- [ ] **Step 5: 提交**

```bash
git add src/views/QrCodeView.vue
git commit -m "feat: 二维码解码（拖拽 / 浏览 / 粘贴 → 写回文本）"
```

---

## Task 9: 左侧文本工具栏 5 按钮联通

**Files:**
- Modify: `src/views/QrCodeView.vue`

**Interfaces:**
- Consumes: `@tauri-apps/plugin-dialog` `open` / `save`、`@tauri-apps/plugin-fs` `readTextFile` / `writeTextFile`、浏览器 `navigator.clipboard.readText` / `writeText`
- Produces: 工具栏五个按钮全部可用

- [ ] **Step 1: 在 `<script setup>` 顶部 import 段补充**

```ts
import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'
import { readFile, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
```

> `open` 已在 Task 8 imported；这里把 `save` 加上、`readTextFile` / `writeTextFile` 加上，避免重复 import 行。如果 IDE / vue-tsc 报 duplicate import，把 Task 8 那行合并为：
> `import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog'`
> `import { readFile, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'`

- [ ] **Step 2: 在 `<script setup>` 末尾追加五个 handler**

```ts
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

function clearInput() {
  input.value = ''
}

async function saveText() {
  const path = await saveDialog({
    filters: [{ name: '文本', extensions: ['txt'] }],
    defaultPath: 'qrcode-text.txt',
  })
  if (typeof path !== 'string') return
  try {
    await writeTextFile(path, input.value)
    message.success('已保存')
  } catch {
    message.error('保存失败')
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
```

- [ ] **Step 3: 把 5 个 PillBtn 接上 click**

把 `.section-actions` 那块替换为：

```html
<div class="section-actions">
  <PillBtn title="粘贴" @click="pasteText">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <rect x="9" y="3" width="6" height="4" rx="1" />
      <path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3" />
    </svg>
    <span>粘贴</span>
  </PillBtn>
  <PillBtn icon-only title="读取文件" @click="readTextFromFile">
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
  <span class="divider" />
  <PillBtn icon-only title="保存" @click="saveText">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z" />
      <path d="M17 21v-8H7v8M7 3v5h8" />
    </svg>
  </PillBtn>
  <PillBtn title="复制" @click="copyText">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <rect x="9" y="9" width="13" height="13" rx="2" />
      <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
    </svg>
    <span>复制</span>
  </PillBtn>
</div>
```

> `PillBtn` 当前模板是直接 `<button>`，原生支持 `@click` 冒泡到 button——无需修改 `PillBtn.vue`。

- [ ] **Step 4: 类型检查**

```bash
pnpm exec vue-tsc -b
```

预期：exit 0。

- [ ] **Step 5: 端到端验证**

```bash
pnpm tauri:dev
```

依次确认：
1. **粘贴：** 复制一段文本（Cmd+C 任意处） → 点「粘贴」 → 左侧出现该文本
2. **读取文件：** 桌面建一个 `test.txt` 写入 "ABC123" → 点 📄 图标 → 选 `test.txt` → 左侧出现 ABC123
3. **清空：** 点 ✕ 图标 → 文本清空 + 按钮变灰
4. **保存：** 文本框写入 "save me" → 点保存图标 → 选位置确认 → 终端窗口显示 "已保存" toast → 验证文件已生成
5. **复制：** 文本框留有内容 → 点「复制」 → 系统其他地方 Cmd+V → 内容一致 → toast 显示 "已复制"

Ctrl+C 关闭。

- [ ] **Step 6: 提交**

```bash
git add src/views/QrCodeView.vue
git commit -m "feat: 二维码页文本工具栏（粘贴 / 读文件 / 清空 / 保存 / 复制）"
```

---

## 收尾验证

完成 Task 9 后，做一次完整的 spec 验证清单（无新代码）：

- [ ] **类型检查全工程：** `pnpm exec vue-tsc -b` exit 0
- [ ] **Rust 全部单测：** `cd src-tauri && cargo test --lib` ≥ 12 PASS（URL 5 + QR 7）
- [ ] **打包通过：** `pnpm tauri:build` 产出 `src-tauri/target/release/bundle/` 安装包（可选；首次需较长时间）
- [ ] **README/CLAUDE.md 检查：** 二维码工具不需要新增 README 或 CLAUDE.md 段落（spec/plan 已自带文档；`spec.md` 与 `plan.md` 已 commit）
