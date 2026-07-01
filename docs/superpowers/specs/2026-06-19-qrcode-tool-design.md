# 二维码编 / 解码工具 — 设计

## 背景与定位

DevBox 第二个真实工具页（继 URL 编 / 解码之后）。导航 `qrcode` 项已存在于 `src/stores/nav.ts`，目前命中 `PlaceholderView`；本次实现将其替换为真实页 `QrCodeView`。

工具方向：**双向**。文本 → 二维码（编码）+ 图片 → 文本（解码）。

## 架构契约

延续 URL 工具的「核心逻辑下沉 Rust + 前端 invoke 封装」契约，前端不做 fallback。

```
UI 操作                              前端                          Rust
────────────────────────────────────────────────────────────────────────────
[点「生成二维码」]    ──▶  qrApi.encode(text)         ──▶  qr_encode(text)  → SVG 字符串
[拖入/浏览/粘贴图片]  ──▶  qrApi.decode(bytes)        ──▶  qr_decode(bytes) → text(写回左侧)
```

### Rust commands

```rust
#[tauri::command]
pub fn qr_encode(text: String) -> Result<String, String>;   // 返回 SVG markup

#[tauri::command]
pub fn qr_decode(image: Vec<u8>) -> Result<String, String>; // 失败返回 Err("未识别到二维码")
```

- `qr_encode` 返回 SVG 字符串（直接 `v-html` 注入预览面板，避免 PNG base64 体积膨胀与跨语言数据搬运）。
- `qr_decode` 入参图片二进制，前端用 `File.arrayBuffer()` → `Array.from(new Uint8Array(buf))` 传入。

### Rust crate 选型

| crate | 版本 | 用途 |
|---|---|---|
| `qrcode` | `0.14` | 编码：`QrCode::new(text)?.render::<svg::Color>().build()` |
| `rqrr` | `0.7` | 解码：纯 Rust，无 OpenCV 依赖 |
| `image` | `0.25` | 图片解码（BMP / GIF / JPEG / PNG / TIFF / WEBP），统一转灰度图喂给 rqrr |

### 错误处理策略

与 URL 工具的「失败返回原文」**不同** —— 二维码失败必须区分（输入空、图中无 QR、不是图片），故采用 `Result<String, String>` 显式上抛。

| 场景 | 反馈 |
|---|---|
| `qr_encode` 文本过长 | `message.error('文本过长，无法生成')` |
| `qr_decode` 无 QR / 非图片 | `message.error('未识别到二维码')` |
| 拖入非图片文件（前端预检） | `message.warning('请拖入图片文件')` |
| 剪贴板无图片 | `message.warning('剪贴板中没有图片')` |
| 输入文本为空 | 「生成二维码」按钮 disabled，不发请求 |

## 文件清单

### 新增

| 路径 | 用途 |
|---|---|
| `src/views/QrCodeView.vue` | 工具页主体 |
| `src/api/qrcode.ts` | `qrApi.encode / decode` 封装 |
| `src-tauri/src/tools/qrcode.rs` | 两个 `#[tauri::command]` + 单测 |

### 修改

| 路径 | 改动 |
|---|---|
| `src/router/index.ts` | 在 `tools/url` 后插入 `tools/qrcode` 路由 |
| `src-tauri/src/lib.rs` | `invoke_handler!` 追加 `tools::qrcode::qr_encode, tools::qrcode::qr_decode` |
| `src-tauri/Cargo.toml` | 增加 `qrcode`、`rqrr`、`image` 三个依赖 + `tauri-plugin-dialog`、`tauri-plugin-fs` |
| `package.json` | 增加 `@tauri-apps/plugin-dialog`、`@tauri-apps/plugin-fs` |
| `src-tauri/capabilities/default.json` | 放开 dialog / fs 权限（仅本工具用） |

**不改：** `src/stores/nav.ts`（导航项 `qrcode` 已存在）、`src-tauri/src/tools/url.rs`（精准变更）。

## 页面布局

```
┌─ 二维码编 / 解码工具 ──────────────────────────────────────────┐
│                                                                │
│  ┌─ 左：文本区 ──────────────┐  ┌─ 右上：图片输入区 ─────────┐ │
│  │ 工具栏  [粘贴][读文件][清空]│  │ 虚线框 dashed              │ │
│  │         ─────  [保存][复制] │  │ "拖放 BMP/JPG/PNG... 到此处"│ │
│  │ ┌──────────────────────────┐│  │ 或者                       │ │
│  │ │                          ││  │ [浏览文件] / [粘贴]        │ │
│  │ │  CodeArea (flex-1)       ││  └────────────────────────────┘ │
│  │ │                          ││  ┌─ 右下：二维码预览 ─────────┐ │
│  │ └──────────────────────────┘│  │ 标题：二维码               │ │
│  │ ┌─ 操作行 ──────────┐       │  │ SVG 居中 / 空态文案        │ │
│  │ │ [生成二维码] btn  │       │  │                            │ │
│  └──┴────────────────────┴─────┘  └────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

### 栅格

- 外层 `display: grid; grid-template-columns: 1fr 1fr; gap: 16px`
- 左列内部 `display: flex; flex-direction: column`，`CodeArea.flex-1` 撑满
- 右列内部 `display: flex; flex-direction: column; gap: 16px`；上区固定高度（约 180px），下区 `flex-1`

### 关键样式约定

- 顶部沿用 `<header class="page-head"><h1>二维码编 / 解码工具</h1></header>` 与 URL 页一致
- 左侧工具栏 5 个按钮全部作用于**文本区**；前 3（粘贴/读文件/清空）与后 2（保存/复制）之间用 `border-left` 做分隔线
- 「生成二维码」按钮放在左侧文本区**下方**作为操作行（语义上"对左侧文本生成"更直观，而非右下角）
- 右上图片区：`border: 1.5px dashed var(--border)`、`border-radius: var(--r-md)`，内部三段（说明文字 / "或者" / 两个 link-style 按钮）
- 右下预览：`background: var(--card)`、`border-radius: var(--r-md)`；标题"二维码"在左上、`color: var(--ink-2)`；空态居中、`color: var(--ink-3)`

样式仍走「`tokens.css` → Tailwind 子集 → scoped CSS」三层；新颜色/间距走 token 而非 magic number。

## 交互细节

| 操作 | 行为 |
|---|---|
| 输入文本 | 实时调用 `qrApi.encode(text)`，SVG 字符串 `v-html` 注入右下区；空文本时清空预览（详见下方「2026-06-21 修正」） |
| 文本变化 | **触发实时生成**；编码失败保留上次预览（沿用 URL 工具的 watcher 静默策略） |
| 拖入图片 | 右上虚线框 `@dragover.prevent` + `@drop`，取 `dataTransfer.files[0]`，`arrayBuffer()` → bytes → `qrApi.decode`，结果写入左侧 `input.value`；解析失败 toast |
| 「浏览文件」 | `tauri-plugin-dialog` 的 `open({ filters: [{ name: 'Image', extensions: ['bmp','gif','jpeg','jpg','pbm','png','tga','tiff','webp'] }] })` → 路径 → `@tauri-apps/plugin-fs` 的 `readFile()` → bytes → decode |
| 右上「粘贴」 | `navigator.clipboard.read()` → 找出 `image/*` item → `blob.arrayBuffer()` → bytes → decode |
| 左侧「粘贴」「复制」 | 沿用 URL 工具的 `navigator.clipboard` + `useMessage` 模式 |
| 左侧「读文件」 | dialog `open` filters `.txt` → `readTextFile` → 写入 input |
| 左侧「保存」 | dialog `save()` + `writeTextFile()` |
| 解码结果回填左侧 | 直接 `input.value = decodedText`；由实时 watcher 自动重新生成（与文本输入一致） |

### Race 处理

encode 与 decode 两条路径均异步，复用 URL 工具的 `reqId` 模式（`encodeReqId` / `decodeReqId`）：连续输入只取最后一次编码结果；连续拖入多张图只取最后一张写回。

## 验证点

| 步骤 | 验证方式 |
|---|---|
| Rust 编码 | `cd src-tauri && cargo test --lib qr_encode` ≥ 2 条：纯 ASCII / 中文混合都能产出非空 SVG（含 `<svg`） |
| Rust 解码 | `cargo test --lib qr_decode`：单测内用 `qrcode` crate 直接渲染成 `image::GrayImage`（不经 SVG 中转）→ PNG bytes → 喂给 `qr_decode`，能拿回原文；纯白 image bytes 返回 Err |
| 类型检查 | `pnpm exec vue-tsc -b` exit 0 |
| Cargo 编译 | `cd src-tauri && cargo build` 通过 |
| 路由 | 点导航"二维码"，地址栏跳 `/tools/qrcode`，渲染 `QrCodeView`（不再命中 `PlaceholderView`） |
| 端到端编码 | `pnpm tauri:dev` 启动 → 左侧输入"hello" → 点生成 → 右下出现 SVG |
| 端到端解码 | 用手机/截屏工具保存上一步的二维码图 → 拖入右上 → 左侧出现 "hello" |
| 浏览文件 | 点「浏览文件」选同一张图，与拖入结果一致 |
| 错误反馈 | 拖入纯白图片 → 看到 `未识别到二维码` toast；拖入 `.txt` → 看到 `请拖入图片文件` warning |

## 不做项（明确排除）

- ❌ ECC（错误纠正等级）、尺寸、边距等编码参数 UI
- ❌ 导出 PNG（预览即 SVG，YAGNI；浏览器原生右键即可保存 SVG，本期不做按钮）
- ❌ 批量生成 / 批量解码
- ❌ 扫描历史 / 收藏
- ❌ 暗色模式适配（仓库本就不做暗色）
- ❌ 摄像头实时扫码（与桌面工具定位无关）

## 修正记录

### 2026-06-21：编码改为实时生成

原 spec 规定「点『生成二维码』按钮才触发编码、空文本时按钮 disabled」。实现阶段改为 **watcher 实时生成**（与左侧文本同步，按 race token 模式去抖），删除了「生成二维码」按钮与操作行。

理由：
1. 与 URL 工具体感一致，左列只剩 textarea + 顶部工具栏，UI 更干净。
2. encode 是纯计算、无副作用，实时 + race token 不存在风险。

随之联动的变更：左侧「生成二维码」按钮删除；交互表中相关行已同步修正（见「交互细节」表）。
