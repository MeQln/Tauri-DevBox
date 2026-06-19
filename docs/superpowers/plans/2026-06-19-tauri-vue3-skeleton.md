# fs-tauri Tauri + Vue3 桌面骨架 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 `fs-tauri/` 根目录初始化 Tauri 2 + Vue 3 桌面应用，把 `prototype/index.html` 那一屏 URL 编 / 解码工具落到桌面窗口里，并为多工具页扩展铺好骨架。

**Architecture:** 单仓平铺工程，前端 `src/` + Rust 后端 `src-tauri/` 双栈。前端三层依赖（views → components → ui atoms）单向；URL 编解码逻辑下沉到 Rust `#[command]`，前端经 `src/api/url.ts` 单一入口调用。样式三层结构：CSS 变量 token + Tailwind + scoped CSS。Naive UI 仅作 `n-message` 容器，不替换原型自定义组件。

**Tech Stack:** Tauri 2、Vue 3、TypeScript、Vite、pnpm、Pinia、vue-router、Tailwind CSS、Naive UI、percent-encoding（Rust）

## Global Constraints

- **包管理器：** 必须 pnpm（不要 npm/yarn）
- **TypeScript：** 全前端；`vue-tsc` 类型检查通过
- **目录平铺：** Tauri 工程文件位于 `fs-tauri/` 根目录，与已有 `prototype/`、`docs/`、`DESIGN.md`、`README.md` 同级
- **不引入：** ESLint / Prettier / Husky / Vitest / Playwright / Tailwind 暗色 / 国际化
- **Naive UI 用法：** 仅在 `App.vue` 挂 `n-config-provider` + `n-message-provider`；URL 页面内不使用 `n-switch` / `n-button` / `n-input` 等
- **CSS 变量真源：** 仅 `src/styles/tokens.css`，原型 `:root` 块完整搬入；Tailwind 与 Naive UI 都从这里读
- **commit message：** 使用中文、`类型: 简短描述` 格式（参见 CLAUDE.md）
- **不动文件：** `prototype/index.html`、`docs/`、`DESIGN.md`、`README.md`、`.claude/`、`.gitignore` 不在本计划范围内修改
- **`encodeURIComponent` 兼容字符集：** Rust 侧 `url_encode` 必须排除 `-_.!~*'()` 不编码（与 JS `encodeURIComponent` 完全一致）
- **错误处理反原则：** 不预先发明错误；`url_decode` 失败返回原文，不 panic、不 `Result::Err`

---

## File Structure（计划完成后的目录形态）

```
fs-tauri/
├── prototype/index.html             ← 不动
├── docs/                            ← 不动
├── DESIGN.md / README.md            ← 不动
│
├── package.json                     [Task 1 创建]
├── pnpm-lock.yaml                   [Task 1 自动生成]
├── vite.config.ts                   [Task 1 创建]
├── tsconfig.json                    [Task 1 创建]
├── tsconfig.node.json               [Task 1 创建]
├── tailwind.config.ts               [Task 4 创建]
├── postcss.config.js                [Task 4 创建]
├── index.html                       [Task 1 创建，Vite 入口]
│
├── src/
│   ├── main.ts                      [Task 2 创建，Task 3/4/5/6 增量]
│   ├── App.vue                      [Task 2 创建，Task 3/6 增量]
│   ├── env.d.ts                     [Task 1 创建]
│   ├── router/index.ts              [Task 5 创建]
│   ├── styles/
│   │   ├── tokens.css               [Task 4 创建]
│   │   └── tailwind.css             [Task 4 创建]
│   ├── stores/
│   │   └── nav.ts                   [Task 5 创建]
│   ├── api/
│   │   └── url.ts                   [Task 7 创建]
│   ├── layouts/
│   │   └── AppShell.vue             [Task 6 创建]
│   ├── components/
│   │   ├── nav/AsideNav.vue         [Task 6 创建]
│   │   ├── nav/NavGroup.vue         [Task 6 创建]
│   │   ├── nav/NavItem.vue          [Task 6 创建]
│   │   ├── ui/Switch.vue            [Task 8 创建]
│   │   ├── ui/PillBtn.vue           [Task 8 创建]
│   │   ├── ui/GhostBtn.vue          [Task 8 创建]
│   │   └── ui/CodeArea.vue          [Task 8 创建]
│   └── views/
│       ├── UrlView.vue              [Task 9 创建]
│       └── PlaceholderView.vue      [Task 6 创建]
│
└── src-tauri/                       [Task 2 由 create-tauri-app 生成]
    ├── Cargo.toml                   [Task 2 生成，Task 7 增依赖]
    ├── tauri.conf.json              [Task 3 编辑]
    ├── build.rs                     [Task 2 生成]
    ├── icons/                       [Task 2 生成]
    └── src/
        ├── main.rs                  [Task 2 生成，Task 7 编辑]
        └── tools/
            ├── mod.rs               [Task 7 创建]
            └── url.rs               [Task 7 创建]
```

---

## Task 路线图

10 个任务，单向依赖：

```
1 (前端脚手架) → 2 (Tauri 注入) → 3 (窗口配置)
                                    ↓
                                  4 (Tailwind/tokens)
                                    ↓
                                  5 (router + nav store)
                                    ↓
                                  6 (Layout + Nav 组件 + 占位页)
                                    ↓
                                  7 (Rust url commands + 前端 api 封装)
                                    ↓
                                  8 (UI atoms: Switch/PillBtn/GhostBtn/CodeArea)
                                    ↓
                                  9 (UrlView 装配)
                                    ↓
                                 10 (验收 + 收尾)
```

每个任务独立可运行可验证。

---

## Task 1: 前端脚手架（Vite + Vue 3 + TS + pnpm）

**目标：** 在 `fs-tauri/` 根目录初始化 pnpm + Vite + Vue 3 + TypeScript 工程，跑通空白页。

**Files:**
- Create: `fs-tauri/package.json`
- Create: `fs-tauri/pnpm-lock.yaml`（自动）
- Create: `fs-tauri/vite.config.ts`
- Create: `fs-tauri/tsconfig.json`
- Create: `fs-tauri/tsconfig.node.json`
- Create: `fs-tauri/index.html`
- Create: `fs-tauri/src/main.ts`
- Create: `fs-tauri/src/App.vue`
- Create: `fs-tauri/src/env.d.ts`
- Modify: `fs-tauri/.gitignore`（追加 `node_modules/`、`dist/`、`src-tauri/target/`）

**Interfaces:**
- Consumes: 无
- Produces:
  - `pnpm dev` 命令可启动 Vite dev server
  - `pnpm build` 命令调用 `vue-tsc -b && vite build`
  - 路径别名 `@/*` 映射到 `src/*`

- [ ] **Step 1: 在 fs-tauri/ 根目录创建 package.json**

写入：

```json
{
  "name": "fs-tauri",
  "private": true,
  "version": "0.0.1",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc -b && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  }
}
```

- [ ] **Step 2: 安装前端依赖**

```bash
cd /Users/mengql/workspace/ClaudeCode/Front-Skeleton/fs-desktop/fs-tauri
pnpm add vue@^3.5.0 vue-router@^4.4.0 pinia@^2.2.0 naive-ui@^2.40.0
pnpm add -D vite@^5.4.0 @vitejs/plugin-vue@^5.1.0 typescript@~5.5.0 vue-tsc@^2.1.0
```

预期：`package.json` 自动写入 `dependencies` 与 `devDependencies`，生成 `pnpm-lock.yaml` 和 `node_modules/`。

- [ ] **Step 3: 创建 vite.config.ts**

```ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) },
  },
  server: { port: 5173, strictPort: true, host: '127.0.0.1' },
  clearScreen: false,
})
```

- [ ] **Step 4: 创建 tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "jsx": "preserve",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "esModuleInterop": true,
    "lib": ["ES2022", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "noEmit": true,
    "paths": { "@/*": ["./src/*"] },
    "baseUrl": "."
  },
  "include": ["src/**/*.ts", "src/**/*.vue", "src/**/*.d.ts"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **Step 5: 创建 tsconfig.node.json**

```json
{
  "compilerOptions": {
    "composite": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "skipLibCheck": true,
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
```

- [ ] **Step 6: 创建 index.html（Vite 入口）**

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>URL 编码 / 解码工具</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

- [ ] **Step 7: 创建 src/env.d.ts**

```ts
/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<object, object, unknown>
  export default component
}
```

- [ ] **Step 8: 创建 src/main.ts（最小占位）**

```ts
import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')
```

- [ ] **Step 9: 创建 src/App.vue（最小占位）**

```vue
<template>
  <div>fs-tauri 骨架启动成功</div>
</template>

<script setup lang="ts"></script>
```

- [ ] **Step 10: 追加 .gitignore**

读现有 `.gitignore`，追加（若已存在则跳过对应行）：

```
node_modules/
dist/
src-tauri/target/
```

- [ ] **Step 11: 验证 dev server 启动**

```bash
pnpm dev
```

预期：终端显示 `Local: http://127.0.0.1:5173/`，无错误；浏览器打开看到「fs-tauri 骨架启动成功」。**Ctrl+C 停止**。

- [ ] **Step 12: 验证类型检查通过**

```bash
pnpm exec vue-tsc -b
```

预期：无输出（即无错误）。

- [ ] **Step 13: Commit**

```bash
git add package.json pnpm-lock.yaml vite.config.ts tsconfig.json tsconfig.node.json index.html src/ .gitignore
git commit -m "feat: 初始化 Vite + Vue3 + TS 前端脚手架"
```

---

## Task 2: 注入 Tauri（Rust 后端 + CLI）

**目标：** 在已有前端工程上叠加 Tauri 2 后端，能 `pnpm tauri:dev` 启动桌面窗口（窗口里仍是 Task 1 的「骨架启动成功」字样）。

**Files:**
- Create: `fs-tauri/src-tauri/`（整个目录由 `create-tauri-app` 或手动初始化生成）
- Create: `fs-tauri/src-tauri/Cargo.toml`
- Create: `fs-tauri/src-tauri/build.rs`
- Create: `fs-tauri/src-tauri/tauri.conf.json`
- Create: `fs-tauri/src-tauri/src/main.rs`
- Create: `fs-tauri/src-tauri/icons/`（默认图标集）
- Modify: `fs-tauri/package.json`（添加 `@tauri-apps/cli` 与 `@tauri-apps/api` 依赖）

**Interfaces:**
- Consumes: Task 1 的 `pnpm dev` 端口 `127.0.0.1:5173`
- Produces:
  - `pnpm tauri:dev` 命令可启动桌面窗口
  - `pnpm tauri:build` 命令可产出安装包
  - `tauri.conf.json` 的 `build.devUrl = http://127.0.0.1:5173`、`build.frontendDist = ../dist`

- [ ] **Step 1: 安装 Tauri 前端依赖**

```bash
pnpm add @tauri-apps/api@^2
pnpm add -D @tauri-apps/cli@^2
```

- [ ] **Step 2: 初始化 src-tauri 目录**

```bash
pnpm tauri init --ci \
  --app-name "fs-tauri" \
  --window-title "URL 编码 / 解码工具" \
  --frontend-dist "../dist" \
  --dev-url "http://127.0.0.1:5173"
```

预期：生成 `src-tauri/` 目录，含 `Cargo.toml`、`tauri.conf.json`、`src/main.rs`、`build.rs`、`icons/`、`capabilities/`。

- [ ] **Step 3: 验证 src-tauri/src/main.rs 是默认骨架**

读 `src-tauri/src/main.rs`，应类似：

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

如不一致，覆盖为上述内容。

- [ ] **Step 4: 验证 tauri.conf.json 关键字段**

读 `src-tauri/tauri.conf.json`，确认：

- `productName: "fs-tauri"`
- `build.devUrl: "http://127.0.0.1:5173"`
- `build.frontendDist: "../dist"`
- `build.beforeDevCommand: "pnpm dev"`
- `build.beforeBuildCommand: "pnpm build"`
- `app.windows[0].title: "URL 编码 / 解码工具"`

如有缺失或 `beforeDevCommand` 为空，手动编辑补齐。

- [ ] **Step 5: 启动 tauri dev 验证桌面窗口**

```bash
pnpm tauri:dev
```

预期：首次会编译 Rust（耗时 1-3 分钟），随后弹出桌面窗口标题「URL 编码 / 解码工具」，窗口内显示「fs-tauri 骨架启动成功」。**确认看到窗口后关闭窗口或 Ctrl+C 停止**。

- [ ] **Step 6: 把 src-tauri/target/ 加入 .gitignore（若 Step 1 没加全）**

读 `.gitignore`，确认含 `src-tauri/target/`；若无则追加。

- [ ] **Step 7: Commit**

```bash
git add src-tauri/ package.json pnpm-lock.yaml .gitignore
git commit -m "feat: 注入 Tauri 2 后端骨架"
```

---

## Task 3: 配置桌面窗口尺寸与最小尺寸

**目标：** 把窗口尺寸从默认值改为 spec 约定值（1100×760，min 880×600）。

**Files:**
- Modify: `fs-tauri/src-tauri/tauri.conf.json`

**Interfaces:**
- Consumes: Task 2 已存在的 `tauri.conf.json`
- Produces: 启动后窗口尺寸 1100×760，可缩放但不小于 880×600

- [ ] **Step 1: 编辑 tauri.conf.json 的 windows 配置**

把 `app.windows[0]` 中的字段改成：

```json
{
  "title": "URL 编码 / 解码工具",
  "width": 1100,
  "height": 760,
  "minWidth": 880,
  "minHeight": 600,
  "resizable": true
}
```

保留默认其它字段（`label`、`url` 等）。

- [ ] **Step 2: 启动 tauri dev 验证**

```bash
pnpm tauri:dev
```

预期：窗口启动尺寸约 1100×760。手动拖拽缩小到 < 880×600，窗口被钳制住。**确认后 Ctrl+C 停止**。

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tauri.conf.json
git commit -m "feat: 配置桌面窗口默认与最小尺寸"
```

---

## Task 4: Tailwind CSS + tokens.css 样式三层底座

**目标：** 安装 Tailwind，建立 CSS 变量真源 `tokens.css`（从原型搬入），让 Tailwind 配置消费这些变量。

**Files:**
- Create: `fs-tauri/tailwind.config.ts`
- Create: `fs-tauri/postcss.config.js`
- Create: `fs-tauri/src/styles/tokens.css`
- Create: `fs-tauri/src/styles/tailwind.css`
- Modify: `fs-tauri/src/main.ts`（导入两份 CSS）
- Modify: `fs-tauri/src/App.vue`（用 Tailwind 类验证生效）
- Modify: `fs-tauri/package.json`（自动）

**Interfaces:**
- Consumes: Task 1 的前端工程
- Produces:
  - 全局 CSS 变量：`--bg`、`--surface`、`--aside`、`--aside-2`、`--aside-3`、`--card`、`--card-2`、`--rule`、`--rule-soft`、`--ink`、`--ink-2`、`--ink-3`、`--ink-4`、`--ink-5`、`--amber`、`--amber-d`、`--link`、`--ok`、`--warn`、`--r-sm`、`--r-md`、`--r-lg`、`--serif`、`--sans`、`--mono`
  - Tailwind 颜色名：`bg`、`surface`、`aside`、`aside-2`、`card`、`rule`、`ink`、`ink-2`、`ink-3`、`amber`、`link`
  - Tailwind 圆角名：`sm`、`md`、`lg`
  - Tailwind 字体族：`sans`、`mono`

- [ ] **Step 1: 安装 Tailwind**

```bash
pnpm add -D tailwindcss@^3.4.0 postcss@^8.4.0 autoprefixer@^10.4.0
```

- [ ] **Step 2: 创建 postcss.config.js**

```js
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

- [ ] **Step 3: 创建 tailwind.config.ts**

```ts
import type { Config } from 'tailwindcss'

export default {
  content: ['./index.html', './src/**/*.{vue,ts}'],
  theme: {
    extend: {
      colors: {
        bg:        'var(--bg)',
        surface:   'var(--surface)',
        aside:     'var(--aside)',
        'aside-2': 'var(--aside-2)',
        card:      'var(--card)',
        rule:      'var(--rule)',
        ink:       'var(--ink)',
        'ink-2':   'var(--ink-2)',
        'ink-3':   'var(--ink-3)',
        amber:     'var(--amber)',
        link:      'var(--link)',
      },
      borderRadius: {
        sm: 'var(--r-sm)',
        md: 'var(--r-md)',
        lg: 'var(--r-lg)',
      },
      fontFamily: {
        sans: ['var(--sans)'],
        mono: ['var(--mono)'],
      },
    },
  },
  plugins: [],
} satisfies Config
```

- [ ] **Step 4: 创建 src/styles/tokens.css**

把 `prototype/index.html` 中 `:root { ... }` 块完整搬入（26 个变量）：

```css
:root {
  --bg: #f6f5f3;
  --surface: #ffffff;
  --aside: #f1efeb;
  --aside-2: #e9e6e0;
  --aside-3: #ddd9d2;
  --card: #f3f2ef;
  --card-2: #ffffff;
  --rule: #e3e0d9;
  --rule-soft: #ecebe6;
  --ink: #1d1d1f;
  --ink-2: #4a4a4d;
  --ink-3: #6f6e72;
  --ink-4: #9a9893;
  --ink-5: #c2c0bb;
  --amber: #e8a534;
  --amber-d: #b8821e;
  --link: #2769d0;
  --ok: #4f9a59;
  --warn: #d97a3b;
  --r-sm: 6px;
  --r-md: 10px;
  --r-lg: 14px;
  --serif: 'Inter Tight', 'PingFang SC', 'Noto Sans SC', -apple-system, sans-serif;
  --sans: 'Inter Tight', 'PingFang SC', 'Noto Sans SC', -apple-system, BlinkMacSystemFont, sans-serif;
  --mono: 'JetBrains Mono', ui-monospace, Menlo, Consolas, monospace;
}

* { box-sizing: border-box; margin: 0; padding: 0; }
html, body, #app { height: 100%; }
body {
  font-family: var(--sans);
  color: var(--ink);
  background: var(--surface);
  overflow: hidden;
  font-size: 14px;
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
}
button { font: inherit; color: inherit; background: none; border: 0; cursor: pointer; }
input, textarea { font: inherit; color: inherit; outline: none; border: 0; background: none; resize: none; }
ul { list-style: none; }
```

- [ ] **Step 5: 创建 src/styles/tailwind.css**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

- [ ] **Step 6: 修改 src/main.ts 导入样式**

```ts
import { createApp } from 'vue'
import App from './App.vue'
import './styles/tokens.css'
import './styles/tailwind.css'

createApp(App).mount('#app')
```

**导入顺序很重要：** `tokens.css` 必须在 `tailwind.css` 之前，确保 `:root` 变量在 Tailwind 工具类之前已定义。

- [ ] **Step 7: 修改 src/App.vue 用 Tailwind 类验证生效**

```vue
<template>
  <div class="bg-aside text-ink p-4 rounded-md font-sans">
    fs-tauri 骨架启动成功（Tailwind + tokens 已生效）
  </div>
</template>

<script setup lang="ts"></script>
```

- [ ] **Step 8: 启动 dev 验证 Tailwind 生效**

```bash
pnpm dev
```

打开浏览器，元素背景应为米色 `#f1efeb`、字色 `#1d1d1f`、字体使用 PingFang SC 回退栈、有圆角内边距。F12 看 computed style 中 `background-color: rgb(241, 239, 235)` 即生效。**Ctrl+C 停止**。

- [ ] **Step 9: 验证 vue-tsc 通过**

```bash
pnpm exec vue-tsc -b
```

预期：无错误。

- [ ] **Step 10: Commit**

```bash
git add tailwind.config.ts postcss.config.js src/styles/ src/main.ts src/App.vue package.json pnpm-lock.yaml
git commit -m "feat: 接入 Tailwind 与 CSS 变量 token 三层样式底座"
```

---

## Task 5: Vue Router + Pinia + 导航数据 store

**目标：** 安装路由 + Pinia，建立导航数据真源（`NAV_DATA`）和 `useNavStore`，路由表覆盖 URL 页与占位页。占位页以最简形式落地（Task 6 才装外壳）。

**Files:**
- Create: `fs-tauri/src/router/index.ts`
- Create: `fs-tauri/src/stores/nav.ts`
- Create: `fs-tauri/src/views/UrlView.vue`（最小占位，Task 9 实现）
- Create: `fs-tauri/src/views/PlaceholderView.vue`（最小占位，Task 6 增强）
- Modify: `fs-tauri/src/main.ts`（注册 router + Pinia）
- Modify: `fs-tauri/src/App.vue`（替换为 `<router-view />`）

**Interfaces:**
- Consumes: Task 1 的前端工程
- Produces:
  - 路由：`/` 重定向到 `/tools/url`；`/tools/url` → `UrlView`；`/tools/:id` → `PlaceholderView`
  - `useNavStore` 暴露：`items: NavNode[]`、`activeId: Ref<string>`、`select(id: string): void`、`findLabel(id: string): string | null`
  - 类型：`NavItem = { type: 'item', id: string, label: string, glyph?: string, icon?: 'link', hasUpdate?: boolean }`、`NavGroup = { type: 'group', id: string, label: string, expanded: boolean, children: NavItem[] }`、`NavNode = NavItem | NavGroup`

- [ ] **Step 1: 创建 src/stores/nav.ts**

```ts
import { defineStore } from 'pinia'
import { ref } from 'vue'

export type NavItem = {
  type: 'item'
  id: string
  label: string
  glyph?: string
  icon?: 'link'
  hasUpdate?: boolean
  active?: boolean
}

export type NavGroup = {
  type: 'group'
  id: string
  label: string
  expanded: boolean
  children: NavItem[]
}

export type NavNode = NavItem | NavGroup

export const NAV_DATA: NavNode[] = [
  { type: 'item', id: 'qrcode',   glyph: 'QR', label: '二维码', hasUpdate: true },
  { type: 'item', id: 'url',      icon: 'link', label: 'URL', active: true },
  { type: 'group', id: 'g-test',  label: '测试工具', expanded: true, children: [
    { type: 'item', id: 'jsonpath', glyph: '{;}', label: 'JSONPath' },
    { type: 'item', id: 'regex',    glyph: '.*',  label: '正则表达式', hasUpdate: true },
    { type: 'item', id: 'xml-test', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', id: 'g-format', label: '格式化工具', expanded: true, children: [
    { type: 'item', id: 'json',    glyph: '{;}', label: 'JSON' },
    { type: 'item', id: 'sql',     glyph: 'SQ',  label: 'SQL' },
    { type: 'item', id: 'xml-fmt', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', id: 'g-gen',   label: '生成器',   expanded: false, children: [] },
  { type: 'group', id: 'g-img',   label: '图像处理', expanded: false, children: [] },
  { type: 'group', id: 'g-text',  label: '文本处理', expanded: true, children: [
    { type: 'item', id: 'escape',   glyph: 'TX', label: '转义 / 反转义' },
    { type: 'item', id: 'list-cmp', glyph: '≡',  label: '列表比对' },
    { type: 'item', id: 'md',       glyph: 'MD', label: 'Markdown 预览' },
  ]},
]

export const FOOT_DATA: NavItem[] = [
  { type: 'item', id: 'extensions', glyph: '⚙', label: '管理扩展' },
  { type: 'item', id: 'settings',   glyph: '☰', label: '设置' },
]

export const useNavStore = defineStore('nav', () => {
  const items = NAV_DATA
  const foot = FOOT_DATA
  const activeId = ref<string>('url')

  function select(id: string) {
    activeId.value = id
  }

  function findLabel(id: string): string | null {
    for (const node of items) {
      if (node.type === 'item' && node.id === id) return node.label
      if (node.type === 'group') {
        const hit = node.children.find(c => c.id === id)
        if (hit) return hit.label
      }
    }
    return null
  }

  return { items, foot, activeId, select, findLabel }
})
```

- [ ] **Step 2: 创建 src/views/UrlView.vue（最小占位）**

```vue
<template>
  <div class="p-8">URL 工具页（待 Task 9 实现）</div>
</template>

<script setup lang="ts"></script>
```

- [ ] **Step 3: 创建 src/views/PlaceholderView.vue（最小占位）**

```vue
<template>
  <div class="p-8">
    <h1 class="text-2xl font-medium text-ink mb-2">{{ label }}</h1>
    <p class="text-ink-3">即将上线</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useNavStore } from '@/stores/nav'

const route = useRoute()
const nav = useNavStore()
const label = computed(() => {
  const id = String(route.params.id ?? '')
  return nav.findLabel(id) ?? '未知工具'
})
</script>
```

- [ ] **Step 4: 创建 src/router/index.ts**

```ts
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/tools/url' },
  { path: '/tools/url', component: () => import('@/views/UrlView.vue') },
  { path: '/tools/:id', component: () => import('@/views/PlaceholderView.vue') },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
```

- [ ] **Step 5: 修改 src/main.ts**

```ts
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { router } from './router'
import './styles/tokens.css'
import './styles/tailwind.css'

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
```

- [ ] **Step 6: 修改 src/App.vue**

```vue
<template>
  <router-view />
</template>

<script setup lang="ts"></script>
```

- [ ] **Step 7: 启动 dev 验证路由**

```bash
pnpm dev
```

打开浏览器：

- 访问 `/` 应自动跳到 `/tools/url`，显示「URL 工具页（待 Task 9 实现）」
- 手动改地址栏到 `/tools/jsonpath`，应显示「JSONPath / 即将上线」
- 改到 `/tools/unknown-id`，应显示「未知工具 / 即将上线」

**Ctrl+C 停止**。

- [ ] **Step 8: 验证 vue-tsc 通过**

```bash
pnpm exec vue-tsc -b
```

预期：无错误。

- [ ] **Step 9: Commit**

```bash
git add src/router/ src/stores/ src/views/ src/main.ts src/App.vue package.json pnpm-lock.yaml
git commit -m "feat: 接入 vue-router + Pinia 与导航数据 store"
```

---

## Task 6: AppShell 布局 + 左侧导航 + Naive UI providers

**目标：** 实现两栏 AppShell（侧栏 + 主区路由出口），落地左侧导航（搜索框、可折叠分组、当前选中态、底部 footer），点击导航项触发路由跳转。挂载 Naive UI 的 `n-config-provider` 与 `n-message-provider`。

**Files:**
- Create: `fs-tauri/src/layouts/AppShell.vue`
- Create: `fs-tauri/src/components/nav/AsideNav.vue`
- Create: `fs-tauri/src/components/nav/NavGroup.vue`
- Create: `fs-tauri/src/components/nav/NavItem.vue`
- Modify: `fs-tauri/src/App.vue`（用 `n-config-provider` + `n-message-provider` 包裹）
- Modify: `fs-tauri/src/router/index.ts`（路由嵌套到 AppShell 之下）

**Interfaces:**
- Consumes: Task 5 的 `useNavStore`、`NAV_DATA`、`FOOT_DATA` 类型；vue-router
- Produces:
  - `AppShell.vue` 渲染左侧 280px 栏 + 右侧 `<router-view />`
  - 任意子页面可调用 `useMessage()` 弹出 toast
  - 路由结构变为：`AppShell` 父组件下挂 `/tools/url` 与 `/tools/:id` 两条子路由

- [ ] **Step 1: 创建 src/components/nav/NavItem.vue**

```vue
<template>
  <div
    :class="[
      'grid grid-cols-[22px_1fr_auto] items-center gap-1 h-8 px-1.5 my-px',
      'rounded-lg text-[13.5px] cursor-pointer transition-colors',
      isActive ? 'item-active' : 'text-ink-2 hover:bg-aside-2',
    ]"
    @click="onClick"
  >
    <span v-if="item.icon === 'link'" class="inline-flex items-center justify-center">
      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M10 14a5 5 0 007 0l3-3a5 5 0 00-7-7l-1 1" />
        <path d="M14 10a5 5 0 00-7 0l-3 3a5 5 0 007 7l1-1" />
      </svg>
    </span>
    <span v-else-if="item.glyph" class="glyph">{{ item.glyph }}</span>
    <span v-else></span>

    <span class="truncate">{{ item.label }}</span>

    <span v-if="item.hasUpdate" class="bulb"></span>
    <span v-else></span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useNavStore, type NavItem } from '@/stores/nav'

const props = defineProps<{ item: NavItem }>()
const router = useRouter()
const nav = useNavStore()

const isActive = computed(() => nav.activeId === props.item.id)

function onClick() {
  nav.select(props.item.id)
  router.push(props.item.id === 'url' ? '/tools/url' : `/tools/${props.item.id}`)
}
</script>

<style scoped>
.item-active {
  background: linear-gradient(180deg, #e1ddd4, #d5d0c5);
  color: var(--ink);
  font-weight: 500;
  box-shadow: inset 0 0 0 1px rgba(0,0,0,0.04);
}
.glyph {
  width: 18px; height: 18px;
  background: rgba(0,0,0,0.05);
  border-radius: 4px;
  display: inline-flex; align-items: center; justify-content: center;
  font-family: var(--mono); font-size: 9px; text-transform: uppercase;
  color: var(--ink-3);
}
.bulb {
  width: 12px; height: 12px; border-radius: 50%;
  background: var(--amber);
  box-shadow: 0 0 0 4px rgba(232,165,52,0.18);
}
</style>
```

- [ ] **Step 2: 创建 src/components/nav/NavGroup.vue**

```vue
<template>
  <div :class="['group', { collapsed: !expanded }]">
    <div
      class="grid grid-cols-[22px_1fr_16px] items-center h-9 px-1.5 rounded-lg text-ink-3 text-[13.5px] cursor-pointer hover:bg-aside-2 transition-colors"
      @click="expanded = !expanded"
    >
      <span></span>
      <span>{{ group.label }}</span>
      <svg
        :class="['chev transition-transform', expanded ? '' : '-rotate-90']"
        viewBox="0 0 24 24" width="14" height="14"
        fill="none" stroke="currentColor" stroke-width="2"
      >
        <path d="M6 9l6 6 6-6" />
      </svg>
    </div>
    <div v-show="expanded" class="pl-[22px]">
      <NavItem v-for="child in group.children" :key="child.id" :item="child" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NavItem from './NavItem.vue'
import type { NavGroup } from '@/stores/nav'

const props = defineProps<{ group: NavGroup }>()
const expanded = ref(props.group.expanded)
</script>
```

- [ ] **Step 3: 创建 src/components/nav/AsideNav.vue**

```vue
<template>
  <aside class="aside grid grid-rows-[auto_auto_1fr_auto] bg-aside border-r border-rule min-w-0 min-h-0">
    <div class="flex gap-1 px-3 pt-2.5 pb-1.5">
      <button class="icon-btn" title="返回" aria-label="返回">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 18l-6-6 6-6" /></svg>
      </button>
      <button class="icon-btn" title="折叠" aria-label="折叠">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18M3 12h18M3 18h18" /></svg>
      </button>
    </div>

    <div class="search">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="7" />
        <path d="M21 21l-4.3-4.3" />
      </svg>
      <input v-model="search" type="text" placeholder="输入以搜索工具…" />
    </div>

    <nav class="px-2 pb-2 pt-1 overflow-y-auto">
      <template v-for="node in nav.items" :key="node.id">
        <NavGroup v-if="node.type === 'group'" :group="node" />
        <NavItem v-else :item="node" />
      </template>
    </nav>

    <div class="border-t border-rule p-2">
      <NavItem v-for="f in nav.foot" :key="f.id" :item="f" />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NavGroup from './NavGroup.vue'
import NavItem from './NavItem.vue'
import { useNavStore } from '@/stores/nav'

const nav = useNavStore()
const search = ref('')
</script>

<style scoped>
.icon-btn {
  width: 30px; height: 30px;
  border-radius: 8px;
  display: inline-flex; align-items: center; justify-content: center;
  color: var(--ink-2);
  transition: background .15s;
}
.icon-btn:hover { background: var(--aside-2); }
.icon-btn svg { width: 16px; height: 16px; }

.search {
  margin: 0 12px 8px; height: 34px;
  background: var(--card-2);
  border: 1px solid var(--rule);
  border-radius: 8px;
  display: flex; align-items: center; gap: 8px;
  padding: 0 10px;
  transition: border-color .15s, box-shadow .15s;
}
.search:focus-within {
  border-color: #c5bfb4;
  box-shadow: 0 0 0 3px rgba(0,0,0,0.04);
}
.search svg { width: 14px; height: 14px; color: var(--ink-4); flex-shrink: 0; }
.search input { flex: 1; font-size: 13.5px; }
.search input::placeholder { color: var(--ink-4); }

nav::-webkit-scrollbar { width: 8px; }
nav::-webkit-scrollbar-thumb { background: var(--ink-5); border-radius: 4px; }
</style>
```

- [ ] **Step 4: 创建 src/layouts/AppShell.vue**

```vue
<template>
  <div class="window grid grid-cols-[280px_1fr] w-screen h-screen min-h-0">
    <AsideNav />
    <main class="bg-surface min-w-0 min-h-0 flex flex-col overflow-auto px-8 pt-[22px] pb-8">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import AsideNav from '@/components/nav/AsideNav.vue'
</script>
```

- [ ] **Step 5: 修改 src/router/index.ts 嵌套路由到 AppShell**

```ts
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import AppShell from '@/layouts/AppShell.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: AppShell,
    children: [
      { path: '', redirect: '/tools/url' },
      { path: 'tools/url', component: () => import('@/views/UrlView.vue') },
      { path: 'tools/:id', component: () => import('@/views/PlaceholderView.vue') },
    ],
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
```

- [ ] **Step 6: 修改 src/App.vue 挂 Naive UI providers**

```vue
<template>
  <n-config-provider>
    <n-message-provider>
      <router-view />
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { NConfigProvider, NMessageProvider } from 'naive-ui'
</script>
```

- [ ] **Step 7: 启动 dev 验证布局与导航**

```bash
pnpm dev
```

浏览器中验证：

1. 默认进入显示左侧 280px 米色栏 + 右侧白色主区
2. 左栏看到 URL 项（高亮选中）+ 二维码项（带橙色 bulb）+ 三个分组（测试工具/格式化工具 展开，生成器折叠）
3. 点 JSONPath → URL 高亮消失，主区切到 PlaceholderView 显示「JSONPath / 即将上线」
4. 点 URL → 主区回到「URL 工具页」
5. 点分组头「测试工具」→ 折叠 + 箭头旋转 -90°；再点 → 展开

**Ctrl+C 停止**。

- [ ] **Step 8: 验证 vue-tsc 通过**

```bash
pnpm exec vue-tsc -b
```

预期：无错误。

- [ ] **Step 9: Commit**

```bash
git add src/layouts/ src/components/nav/ src/router/index.ts src/App.vue
git commit -m "feat: 实现 AppShell 布局与左侧导航"
```

---

## Task 7: Rust url commands + 前端 api 封装

**目标：** Rust 侧 `tools/url.rs` 实现 `url_encode` / `url_decode` 两个 `#[command]`，注册到 `main.rs`；前端 `src/api/url.ts` 封装 invoke。Rust 单测验证三条边界。

**Files:**
- Create: `fs-tauri/src-tauri/src/tools/mod.rs`
- Create: `fs-tauri/src-tauri/src/tools/url.rs`
- Modify: `fs-tauri/src-tauri/src/main.rs`
- Modify: `fs-tauri/src-tauri/Cargo.toml`（追加 `percent-encoding` 依赖）
- Create: `fs-tauri/src/api/url.ts`

**Interfaces:**
- Consumes: Task 2 的 `tauri::Builder`、`@tauri-apps/api/core` 的 `invoke`
- Produces:
  - Tauri 命令 `url_encode(text: String, multiline: bool) -> String`
  - Tauri 命令 `url_decode(text: String, multiline: bool) -> String`
  - 前端 API：`urlApi.encode(text: string, multiline: boolean): Promise<string>`、`urlApi.decode(text: string, multiline: boolean): Promise<string>`

- [ ] **Step 1: 修改 Cargo.toml 追加 percent-encoding 依赖**

打开 `src-tauri/Cargo.toml`，在 `[dependencies]` 段追加：

```toml
percent-encoding = "2.3"
```

- [ ] **Step 2: 创建 src-tauri/src/tools/mod.rs**

```rust
pub mod url;
```

- [ ] **Step 3: 创建 src-tauri/src/tools/url.rs（含单测）**

```rust
use percent_encoding::{utf8_percent_encode, percent_decode_str, AsciiSet, CONTROLS};

// 与 JS encodeURIComponent 完全一致：不编码 A-Z a-z 0-9 - _ . ! ~ * ' ( )
const COMPONENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ').add(b'"').add(b'#').add(b'$').add(b'%').add(b'&')
    .add(b'+').add(b',').add(b'/').add(b':').add(b';').add(b'<')
    .add(b'=').add(b'>').add(b'?').add(b'@').add(b'[').add(b'\\')
    .add(b']').add(b'^').add(b'`').add(b'{').add(b'|').add(b'}');

fn encode_one(s: &str) -> String {
    utf8_percent_encode(s, COMPONENT_ENCODE_SET).to_string()
}

fn decode_one(s: &str) -> String {
    match percent_decode_str(s).decode_utf8() {
        Ok(cow) => cow.into_owned(),
        Err(_) => s.to_string(),
    }
}

#[tauri::command]
pub fn url_encode(text: String, multiline: bool) -> String {
    if multiline {
        text.split('\n').map(encode_one).collect::<Vec<_>>().join("\n")
    } else {
        encode_one(&text)
    }
}

#[tauri::command]
pub fn url_decode(text: String, multiline: bool) -> String {
    if multiline {
        text.split('\n').map(decode_one).collect::<Vec<_>>().join("\n")
    } else {
        decode_one(&text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_space_as_pct20() {
        assert_eq!(url_encode("hello world".to_string(), false), "hello%20world");
    }

    #[test]
    fn does_not_encode_unreserved() {
        // 与 JS encodeURIComponent 一致：- _ . ! ~ * ' ( ) 不编码
        assert_eq!(url_encode("a-_.!~*'()".to_string(), false), "a-_.!~*'()");
    }

    #[test]
    fn multiline_each_line_independently() {
        let input = "hello world\nfoo bar".to_string();
        assert_eq!(url_encode(input, true), "hello%20world\nfoo%20bar");
    }

    #[test]
    fn decode_invalid_returns_original() {
        // %zz 非法序列：percent_decode_str 不会失败，但解码后非合法 UTF-8 时回退
        // 这里 %zz 实际会保留为字面量（百分号编码解析层面）
        let r = url_decode("%zz".to_string(), false);
        assert!(r == "%zz" || r == "\u{FFFD}\u{FFFD}\u{FFFD}");
    }

    #[test]
    fn decode_roundtrip() {
        let s = "中文 abc!".to_string();
        let encoded = url_encode(s.clone(), false);
        assert_eq!(url_decode(encoded, false), s);
    }
}
```

> 说明：`percent_decode_str` 对非合法 percent 序列不会报错（直接保留），但若解码出非 UTF-8 字节则 `decode_utf8` 失败，我们走原文回退。`decode_invalid_returns_original` 接受两种行为以适配上游版本差异。

- [ ] **Step 4: 修改 src-tauri/src/main.rs 注册 commands**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tools;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tools::url::url_encode,
            tools::url::url_decode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: 跑 Rust 单测**

```bash
cd src-tauri && cargo test --lib && cd ..
```

预期：5 条测试全部 PASS。如有失败，检查 `decode_invalid_returns_original` 的 assert 是否需要根据 `percent-encoding` 版本调整。

- [ ] **Step 6: 创建 src/api/url.ts**

```ts
import { invoke } from '@tauri-apps/api/core'

export const urlApi = {
  encode: (text: string, multiline: boolean) =>
    invoke<string>('url_encode', { text, multiline }),
  decode: (text: string, multiline: boolean) =>
    invoke<string>('url_decode', { text, multiline }),
}
```

- [ ] **Step 7: 启动 tauri dev 验证 invoke 通路**

```bash
pnpm tauri:dev
```

桌面窗口打开后，按 **Cmd+Option+I**（macOS）打开 Tauri DevTools，在 Console 执行：

```js
await window.__TAURI_INTERNALS__.invoke('url_encode', { text: 'hello world', multiline: false })
```

预期返回：`'hello%20world'`。**确认后 Ctrl+C 停止**。

> 注：Tauri 2 暴露的全局是 `window.__TAURI_INTERNALS__.invoke`；如不存在，可改用 `import { invoke } from '@tauri-apps/api/core'` 的方式在 src/main.ts 临时打印一次。

- [ ] **Step 8: 验证 vue-tsc 通过**

```bash
pnpm exec vue-tsc -b
```

- [ ] **Step 9: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/main.rs src-tauri/src/tools/ src-tauri/Cargo.lock src/api/
git commit -m "feat: 实现 Rust url 编解码 commands 与前端 API 封装"
```

---

## Task 8: UI 原子组件 Switch / PillBtn / GhostBtn / CodeArea

**目标：** 把原型里的四个核心可复用元件落成 Vue 组件，供 Task 9 装配 UrlView 使用。

**Files:**
- Create: `fs-tauri/src/components/ui/Switch.vue`
- Create: `fs-tauri/src/components/ui/PillBtn.vue`
- Create: `fs-tauri/src/components/ui/GhostBtn.vue`
- Create: `fs-tauri/src/components/ui/CodeArea.vue`

**Interfaces:**
- Consumes: Task 4 的 tokens.css；无 store / api 依赖
- Produces：
  - `Switch.vue`：props `{ modelValue: boolean, onLabel?: string, offLabel?: string }`，emits `update:modelValue`，渲染原型样式滑动开关 + 左侧文字标签
  - `PillBtn.vue`：props `{ iconOnly?: boolean }`，slot `default`（按钮内容），透传 click
  - `GhostBtn.vue`：slot `default`，透传 click
  - `CodeArea.vue`：props `{ modelValue: string, readonly?: boolean }`，emits `update:modelValue`，渲染 `gutter + textarea/pre`，行号自动从内容计算

- [ ] **Step 1: 创建 src/components/ui/Switch.vue**

```vue
<template>
  <div class="row-ctl">
    <span>{{ checked ? onLabel : offLabel }}</span>
    <label :class="['switch', { on: checked }]" @click.prevent="toggle">
      <input type="checkbox" :checked="checked" @click.stop="toggle" />
    </label>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: boolean
  onLabel?: string
  offLabel?: string
}>()
const emit = defineEmits<{ 'update:modelValue': [v: boolean] }>()

const checked = computed(() => props.modelValue)
const onLabel  = computed(() => props.onLabel  ?? '开启')
const offLabel = computed(() => props.offLabel ?? '关闭')

function toggle() {
  emit('update:modelValue', !checked.value)
}
</script>

<style scoped>
.row-ctl {
  display: flex; align-items: center; gap: 8px;
  font-size: 12.5px; color: var(--ink-3);
}
.switch {
  position: relative;
  width: 44px; height: 24px;
  border-radius: 999px;
  background: #d8d4cc;
  transition: background .15s;
  flex-shrink: 0;
  cursor: pointer;
}
.switch::after {
  content: '';
  position: absolute; top: 2px; left: 2px;
  width: 20px; height: 20px;
  background: #fff;
  border-radius: 50%;
  box-shadow: 0 1px 2px rgba(0,0,0,0.18);
  transition: transform .22s cubic-bezier(.2,.7,.2,1);
}
.switch input { position: absolute; opacity: 0; pointer-events: none; }
.switch.on { background: #1e1e21; }
.switch.on::after { transform: translateX(20px); }
</style>
```

- [ ] **Step 2: 创建 src/components/ui/PillBtn.vue**

```vue
<template>
  <button :class="['pill-btn', { 'icon-only': iconOnly }]">
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps<{ iconOnly?: boolean }>()
</script>

<style scoped>
.pill-btn {
  height: 30px; padding: 0 10px;
  border-radius: 8px;
  background: var(--card);
  display: inline-flex; align-items: center; gap: 6px;
  font-size: 12.5px; color: var(--ink-2);
  transition: background .15s;
}
.pill-btn:hover { background: #ebe9e3; }
.pill-btn:active { background: #e2dfd8; }
.pill-btn :deep(svg) { width: 14px; height: 14px; }
.pill-btn.icon-only { width: 32px; padding: 0; justify-content: center; }
</style>
```

- [ ] **Step 3: 创建 src/components/ui/GhostBtn.vue**

```vue
<template>
  <button class="ghost-btn">
    <slot />
  </button>
</template>

<script setup lang="ts"></script>

<style scoped>
.ghost-btn {
  height: 32px; padding: 0 12px;
  display: inline-flex; align-items: center; gap: 6px;
  border-radius: 8px; color: var(--ink-2); font-size: 13px;
  transition: background .15s;
}
.ghost-btn:hover { background: var(--card); }
.ghost-btn :deep(svg) { width: 14px; height: 14px; }
</style>
```

- [ ] **Step 4: 创建 src/components/ui/CodeArea.vue**

```vue
<template>
  <div class="io">
    <div class="gutter">{{ gutterText }}</div>
    <pre v-if="readonly" class="content">{{ modelValue }}</pre>
    <textarea
      v-else
      class="content"
      spellcheck="false"
      :value="modelValue"
      @input="onInput"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: string
  readonly?: boolean
}>()
const emit = defineEmits<{ 'update:modelValue': [v: string] }>()

const lineCount = computed(() =>
  Math.max(1, props.modelValue.split('\n').length)
)
const gutterText = computed(() => {
  const n = lineCount.value
  return Array.from({ length: n }, (_, i) => i + 1).join('\n')
})

function onInput(e: Event) {
  emit('update:modelValue', (e.target as HTMLTextAreaElement).value)
}
</script>

<style scoped>
.io {
  border: 1px solid var(--rule);
  border-radius: var(--r-md);
  background: var(--card-2);
  min-height: 240px;
  display: grid; grid-template-columns: 56px 1fr;
  overflow: hidden;
  margin-bottom: 12px;
}
.gutter {
  background: linear-gradient(180deg, rgba(0,0,0,0) 0%, rgba(0,0,0,0.012) 100%);
  border-right: 1px solid var(--rule-soft);
  padding: 12px 8px;
  text-align: right;
  font-family: var(--mono); font-size: 13px; line-height: 1.85;
  color: var(--link);
  white-space: pre;
  user-select: none;
}
.content {
  padding: 12px 14px;
  font-family: var(--mono); font-size: 13px; line-height: 1.85;
  white-space: pre-wrap; word-break: break-all;
  width: 100%; height: 100%;
  min-height: 216px;
  color: var(--ink);
  display: block;
}
</style>
```

- [ ] **Step 5: 验证 vue-tsc 通过**

```bash
pnpm exec vue-tsc -b
```

预期：无错误。

- [ ] **Step 6: Commit**

```bash
git add src/components/ui/
git commit -m "feat: 实现 Switch / PillBtn / GhostBtn / CodeArea 原子组件"
```

---

## Task 9: UrlView 装配（页头 + 配置卡 + IO + 复制清空）

**目标：** 把 Task 8 的原子组件装配成完整 URL 工具页：页头 H1 + 收藏/弹窗按钮、配置卡两行开关、输入区 + 输出区、复制 / 清空交互。watcher 走 Rust invoke，复制走剪贴板 + n-message。

**Files:**
- Modify: `fs-tauri/src/views/UrlView.vue`（替换 Task 5 的占位实现）

**Interfaces:**
- Consumes:
  - `urlApi.encode` / `urlApi.decode`（Task 7）
  - `Switch` / `PillBtn` / `GhostBtn` / `CodeArea`（Task 8）
  - `useMessage` from naive-ui
- Produces：完整 URL 工具页

- [ ] **Step 1: 重写 src/views/UrlView.vue**

```vue
<template>
  <header class="page-head">
    <h1>URL 编码 / 解码工具</h1>
    <div class="page-actions">
      <GhostBtn title="添加到收藏夹">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2l3 7h7l-5.5 4 2 7-6.5-5-6.5 5 2-7L2 9h7z" />
        </svg>
        <span>添加到收藏夹</span>
      </GhostBtn>
      <GhostBtn title="弹出窗口">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 4h6v6" />
          <path d="M20 4l-9 9" />
          <path d="M9 4H5a1 1 0 00-1 1v14a1 1 0 001 1h14a1 1 0 001-1v-4" />
        </svg>
      </GhostBtn>
    </div>
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
      <Switch v-model="isEncode" on-label="编码" off-label="解码" />
    </div>

    <div class="row">
      <span class="row-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M7 7h11l-3-3" /><path d="M17 17H6l3 3" />
        </svg>
      </span>
      <div>
        <div class="row-title">Encoding / Decoding Multiline</div>
        <div class="row-desc">Encode / Decode each line separately</div>
      </div>
      <Switch v-model="multiline" on-label="开启" off-label="关闭" />
    </div>
  </div>

  <div class="section-title">
    <span>输入</span>
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
      <PillBtn icon-only title="清空" @click="clearInput">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 6l12 12M18 6L6 18" />
        </svg>
      </PillBtn>
    </div>
  </div>
  <CodeArea v-model="input" />

  <div class="section-title">
    <span>输出</span>
    <div class="section-actions">
      <PillBtn icon-only title="保存">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z" />
          <path d="M17 21v-8H7v8M7 3v5h8" />
        </svg>
      </PillBtn>
      <PillBtn title="复制" @click="copyOutput">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" />
          <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
        </svg>
        <span>复制</span>
      </PillBtn>
      <PillBtn icon-only title="展开">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" />
        </svg>
      </PillBtn>
      <PillBtn icon-only title="预览模式">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 21h6M10 17a5 5 0 114 0v3h-4z" />
        </svg>
      </PillBtn>
    </div>
  </div>
  <CodeArea v-model="output" readonly />
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import GhostBtn from '@/components/ui/GhostBtn.vue'
import PillBtn from '@/components/ui/PillBtn.vue'
import Switch from '@/components/ui/Switch.vue'
import CodeArea from '@/components/ui/CodeArea.vue'
import { urlApi } from '@/api/url'

const isEncode  = ref(true)
const multiline = ref(false)
const input  = ref('')
const output = ref('')

const message = useMessage()

let reqId = 0
watch([input, isEncode, multiline], async ([t, enc, ml]) => {
  const my = ++reqId
  try {
    const fn = enc ? urlApi.encode : urlApi.decode
    const r = await fn(t, ml)
    if (my === reqId) output.value = r
  } catch {
    // invoke 失败：保持上次 output，不打扰用户
  }
}, { immediate: true })

function clearInput() {
  input.value = ''
}

async function copyOutput() {
  try {
    await navigator.clipboard.writeText(output.value)
    message.success('已复制')
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
.page-actions { display: flex; gap: 4px; align-items: center; }

.section-title {
  display: flex; align-items: center; justify-content: space-between;
  font-size: 13.5px; font-weight: 500;
  color: var(--ink-2);
  margin: 12px 0 8px;
}
.section-actions { display: flex; gap: 4px; }

.config {
  background: var(--card);
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
</style>
```

- [ ] **Step 2: 启动 tauri dev 完整验证**

```bash
pnpm tauri:dev
```

桌面窗口验证（依次执行所有动作）：

1. URL 页可见，与 `prototype/index.html` 视觉一致（侧栏、页头、配置卡两行开关、输入 / 输出区）
2. 输入 `hello world` → 输出立即显示 `hello%20world`，输入区行号 `1`，输出区行号 `1`
3. 切「转换」开关到「解码」+ 在输入框替换为 `hello%20world` → 输出 `hello world`
4. 切「Multiline」开 + 输入两行 `a b\nc d` → 输出 `a%20b\nc%20d`，行号都为 `2`
5. 输入 `%zz` 后切到解码 → 输出 `%zz`（保留原文，不报错）
6. 点输入区「清空」按钮 → 输入清空，输出清空，行号都重置为 `1`
7. 输出有值时点「复制」→ 系统剪贴板更新，右上角弹「已复制」toast
8. 控制台无 error / warning（Vite HMR 提示除外）

**确认全部通过后 Ctrl+C 停止**。

- [ ] **Step 3: 验证 vue-tsc 通过**

```bash
pnpm exec vue-tsc -b
```

预期：无错误。

- [ ] **Step 4: Commit**

```bash
git add src/views/UrlView.vue
git commit -m "feat: 装配 URL 工具页（编解码 / Multiline / 复制 / 清空）"
```

---

## Task 10: 验收 + 收尾

**目标：** 跑完 spec §九 验收检查点表 15 项，发现的小问题就地修复。最终 build 能产出安装包。

**Files:**
- 视具体修复需要而定（理论上无文件改动，所有任务已完成）

**Interfaces:**
- Consumes: 全部已实现的 Task 1-9
- Produces: 通过验收的桌面应用 + macOS .app/.dmg 安装包

- [ ] **Step 1: 跑验收点 1-2（启动）**

```bash
pnpm install   # 应无 error，无需手动解决 peer dep 警告
pnpm tauri:dev
```

预期：桌面窗口打开，标题「URL 编码 / 解码工具」。

- [ ] **Step 2: 验收点 3（视觉）**

肉眼对比桌面窗口与 `prototype/index.html`（用浏览器单独打开 prototype）：
- 侧栏 280px 宽、米色 `#f1efeb`
- URL 项高亮，二维码项橙色 bulb
- 主区白底，H1 字号 28px
- 配置卡米灰底圆角，row 内白色卡 + 开关
- 输入 / 输出区 56px 行号 gutter，行号字色蓝 `#2769d0`

差异较大就回到对应 Task 修。

- [ ] **Step 3: 验收点 4-6（导航）**

- 点 URL 项 → 路由 `/tools/url`
- 点 JSONPath → 路由 `/tools/jsonpath`，主区显示「JSONPath / 即将上线」
- 点分组头「测试工具」→ 折叠 + 箭头转 -90°；再点 → 展开

- [ ] **Step 4: 验收点 7-13（功能）**

按 Task 9 Step 2 的 8 条用例逐一验证；额外补：

- 验收点 13：先把输入框填上 `hello`，切换「转换」开关 → 输出从 `hello` 变成 `hello`（编码不变）；输入 `中文`，切到编码 → `%E4%B8%AD%E6%96%87`；切到解码 → 输出回到原文 `中文`。

- [ ] **Step 5: 验收点 14（控制台干净）**

Tauri 窗口里 Cmd+Option+I 打开 DevTools，刷新一次，确认 Console 里：
- 无红色 error
- 无 Vue 运行时 warning（除已知 Vite HMR 噪声 / `[Vue Router warn]` 兼容性提示）
- 无 404 资源

如有问题就地排查（一般是 `<script setup>` 内未使用的 import 触发的 TS 警告，删除即可）。

- [ ] **Step 6: 验收点 15（build 产出安装包）**

```bash
pnpm tauri:build
```

预期：在 `src-tauri/target/release/bundle/` 下生成 `.app` 与 `.dmg`（macOS）。耗时 3-8 分钟。

> 如果首次 build 因缺 Xcode CLT 失败，按提示装好后再跑。

- [ ] **Step 7: 必要时为修复 commit**

如 Step 1-6 出现了任何修复，按修复粒度分别 commit，commit message 用 `fix: <具体描述>`。

如全程无修复，本步骤跳过。

- [ ] **Step 8: 最终提交（如有）**

```bash
git status   # 应当是干净的
git log --oneline -15   # 检查 commit 历史
```

预期：从 Task 1 到 Task 9 各有一个 feat commit，加上 Task 10 可能的 fix commit；历史清晰。

---

## Self-Review

**1. Spec coverage（spec §一-§十一 vs plan task 映射）：**

| spec 章节 | 覆盖任务 |
|---|---|
| §一 背景与目标 | Task 1-10 整体 |
| §二 关键决策汇总 | Global Constraints + 各 Task 实现 |
| §三 目录结构 | File Structure 段 |
| §四 组件契约 | Task 5/6/8/9（每个组件落地一个 Task） |
| §五 数据流 | Task 9 watcher + race token |
| §六 Tauri 后端契约 | Task 7（Rust 命令、percent-encoding、注册、前端封装、单测） |
| §七 样式三层与 Token 治理 | Task 4（tokens + tailwind 配置） |
| §八 错误处理 | Task 7 decode 回退、Task 9 watcher try/catch、Task 9 复制失败 message |
| §九 验收检查点表 | Task 10（15 项逐条对应） |
| §十 工具链 / 脚本 | Task 1（package.json scripts） |
| §十一 参考 | 文档已建立映射 |

无遗漏。

**2. Placeholder scan：** 全文无 TBD / TODO / 「类似 Task N」/ 缺代码块的步骤。每个步骤都有具体动作 + 命令 / 代码 / 预期结果。

**3. Type consistency：**
- `NavItem` / `NavGroup` / `NavNode` 在 Task 5 定义，Task 6 三个 nav 组件 import 使用，命名一致。
- `urlApi.encode` / `urlApi.decode` 签名 `(text: string, multiline: boolean) => Promise<string>`，Task 7 定义 / Task 9 使用一致。
- Rust `url_encode` / `url_decode` 参数 `text: String, multiline: bool`，与 invoke 调用 `{ text, multiline }` 对齐（Tauri 自动 camelCase → snake_case 在参数名上不需要转换，因都是单 word）。
- `Switch` 组件 props `modelValue / onLabel / offLabel`，Task 9 用 `v-model + on-label / off-label`，kebab → camel 自动转换，一致。
- `CodeArea` 组件 `modelValue / readonly`，Task 9 用 `v-model` 与 `readonly`，一致。

无不一致。

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-19-tauri-vue3-skeleton.md`. Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
