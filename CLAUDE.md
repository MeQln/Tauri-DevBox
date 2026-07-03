# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目定位

**DevBox** 是一个本地开发者工具集（DevToys 风格）的 Tauri 2 桌面应用：左侧导航 + 右侧工具页。当前已实现 URL 编 / 解码工具一页，其余导航项落到统一的 Coming Soon 占位页。

历史背景：仓库目录名仍是 `DevBox`、远端仓库地址也是 `gitee.com/MeQln/DevBox.git`，但**面向用户的产品名已统一为 DevBox**：`tauri.conf.json#productName`、桌面窗口标题 `DevBox · 开发工具箱`、Cargo `[[bin]] name = "DevBox"`。改文案时不要把 DevBox 又改回 fs-tauri。

## 常用命令

```bash
# 桌面应用
pnpm tauri:dev               # 启动 Tauri 桌面窗口（首次编译 1–3 分钟，含 Vite + Cargo）
pnpm tauri:build             # 产出安装包到 src-tauri/target/release/bundle/

# 仅前端（用于纯 UI 调试，但 invoke 调用会失败 — 桌面 IPC 需要 Tauri 进程）
pnpm dev                     # Vite dev server @ 127.0.0.1:5173
pnpm build                   # vue-tsc -b && vite build

# 类型检查 / 单测
pnpm exec vue-tsc -b                    # 全前端类型检查；提交前必须 exit 0
cd src-tauri && cargo test --lib        # 5 条 url 编解码单测（开发自查）
cd src-tauri && cargo test --lib decode_invalid_returns_original   # 跑单条
```

**没有** ESLint / Prettier / Vitest / Playwright — 这是 spec 明确禁止的（YAGNI），不要新增。

## 架构概览

### 前后端分工

```
前端 src/  ──▶  src/api/url.ts  ──▶  invoke('url_encode' / 'url_decode')  ──▶  src-tauri/src/tools/url.rs (#[tauri::command])
```

**核心契约**：URL 编 / 解码的真实逻辑下沉到 Rust，前端通过 `urlApi.encode / decode(text, multiline)` 单一入口调用。**修改编 / 解码行为时两侧都要改**：Rust 函数签名 + 字符集 + 单测；TS 端类型与 invoke 参数。绝不在前端 fallback `encodeURIComponent`，那会让两端行为漂移。

`COMPONENT_ENCODE_SET`（`src-tauri/src/tools/url.rs:6-9`）必须排除 `-_.!~*'()` 不编码 — 与 JS `encodeURIComponent` 字面对齐。

### 单向依赖

```
views (UrlView / PlaceholderView)
  ├─▶ components/nav (AsideNav / NavGroup / NavItem)
  ├─▶ components/ui  (Switch / PillBtn / CodeArea)
  ├─▶ stores/nav     (useNavStore — 单一全局 store)
  └─▶ api/url        (Tauri invoke 封装)
```

**不要反向引用**：`components/ui/*` 不知道路由 / store 存在；`stores/nav.ts` 不知道任何具体工具页。

### 路由结构

vue-router 嵌套在 `AppShell` 之下。`/tools/url` 是真实页 `UrlView`，`/tools/:id` 命中 `PlaceholderView`（统一 Coming Soon 页，通过 `useNavStore.findLabel(id)` 反查工具名，未命中显示「未知工具」）。

### 样式三层

- **真源**：`src/styles/tokens.css`（26 个 CSS 变量，从 `prototype/index.html` 搬入；颜色 / 圆角 / 字体栈都在这里）
- **Tailwind**：`tailwind.config.ts` 仅暴露**常用子集**（`bg-aside`、`text-ink-2`、`rounded-md` 等）通过 `var(--xxx)` 引用 token；剩余 token 在 scoped CSS 里直接 `var()` 用
- **scoped CSS**：仅用于原型自定义元件（Switch 滑动开关、CodeArea gutter、PillBtn）

新颜色 / 圆角 / 字体先加到 `tokens.css`，再决定是否暴露到 Tailwind。Naive UI 仅作 `<n-config-provider>` + `<n-message-provider>` 容器，**不**用 `n-switch` / `n-button` 替换原型自定义组件。

### 导航数据

`src/stores/nav.ts` 中的 `NAV_DATA` 是导航真源（顶层 item + 5 个 group）。每项有 `id`，路由 `to` 由 id 派生 (`/tools/${id}`)。`icon` 字段引用 `src/components/nav/icons.ts` 的 ICONS 表（lucide 风 SVG inner markup，通过 `v-html` 注入）；新增图标只需在 `icons.ts` 加 key + 在 store 里写 `icon: 'newkey'`。

## 关键约束

### Tauri 2 模板拆分（重要）

CLI 默认生成 `src-tauri/src/lib.rs` + `src-tauri/src/main.rs` 拆分，**不是**单文件 main.rs：
- `lib.rs` 是 crate root，含 `mod tools;` 与 `tauri::Builder` 链
- `main.rs` 仅 `fn main() { devbox_lib::run(); }`
- Cargo `[lib].name = "devbox_lib"`、`[[bin]].name = "DevBox"`

**注册新 `#[tauri::command]` 加在 `lib.rs` 的 `invoke_handler!` 里，不是 main.rs。**

### pnpm 11 严格策略

`pnpm-workspace.yaml` 同时含 `onlyBuiltDependencies` 与 `allowBuilds` 两份字段（pnpm 11.6 strict 策略要求）。**不要删任何一个** — 删了 `pnpm install` 会因 `ERR_PNPM_IGNORED_BUILDS` 失败。

### 错误处理反原则

不预先发明错误。三套策略各得其所：
- **Rust `url_decode` 失败**：返回原文，不 panic / 不 `Result::Err`（与原型 `safeDecode` 一致）
- **前端 watcher invoke 失败**：`try/catch` 静默，保留上次 output，**不打扰用户**
- **剪贴板失败**：才用 `n-message.error('复制失败')` toast

`src/views/UrlView.vue:122-132` 的 watcher race token (`reqId`) 模式不要简化 — 输入连打时防止旧响应覆盖新结果。

### Commit message 规范

中文 + `类型: 简短描述` 格式：`feat: …`、`fix: …`、`chore: …`、`refactor: …`、`docs: …`、`test: …`。**不要**生成英文 commit message 或加 `Co-Authored-By` 之类标记 — 仓库历史不带它们。

## 文档与设计

- **当前架构 spec**：`docs/superpowers/specs/2026-06-19-tauri-vue3-skeleton-design.md`（Tauri + Vue3 桌面骨架）
- **当前实现计划**：`docs/superpowers/plans/2026-06-19-tauri-vue3-skeleton.md`（10 个 Task 的 bite-sized 步骤）
- `docs/superpowers/specs/2026-06-19-html-prototype-design.md` + `docs/superpowers/plans/2026-06-19-html-prototype.md`：上一阶段 HTML 原型，已实现，仅作历史参考
- `prototype/index.html`：单文件 HTML 原型，是当前 Vue 工程的视觉与交互参考，**不要修改也不要从这里直接搬代码** —— 原型已经搬进 `src/` 与 `src/styles/tokens.css`

## 不做项（spec 明确禁止）

暗色模式、国际化、自动更新、系统托盘、ESLint / Prettier、前端单测 / E2E、Tailwind 暗色配置。除非用户明确要求引入，否则不要主动加。
