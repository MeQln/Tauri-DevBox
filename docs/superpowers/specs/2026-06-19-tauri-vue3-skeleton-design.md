# fs-tauri Tauri + Vue3 桌面骨架设计稿

> 日期：2026-06-19
> 范围：基于 `prototype/index.html`，搭建 Tauri 2 + Vue 3 + TypeScript + Vite + pnpm 桌面应用骨架
> 上一份 spec：`2026-06-19-html-prototype-design.md`（HTML 原型，已实现）

---

## 一、背景与目标

### 背景

`fs-tauri/prototype/index.html` 已经把 URL 编 / 解码工具的视觉与交互完整落地为一份单文件 HTML 原型（米色 Apple/DevToys 风、左侧 280px 导航 + 右侧主区、自定义滑动开关、行号 gutter、复制 toast）。

本次任务：以原型为视觉与交互基准，在 `fs-tauri/` 根目录初始化一份 Tauri 桌面工程，让原型那一屏在桌面窗口里跑起来；同时为后续多工具页扩展预留路由 / 状态 / API 的最小骨架。

### 目标

- `pnpm tauri dev` 可启动桌面窗口，URL 工具页与原型视觉一致、功能可用
- URL 编 / 解码逻辑下沉到 Rust 后端 `#[command]`，前端通过 `invoke` 调用，体现真实的 Tauri 前后端分工
- 引入 vue-router，URL 页为真实页面；其他导航项跳转到一个统一的 Coming Soon 占位页
- 引入 Pinia 与 Tailwind 作为后续扩展底座，但不为不存在的需求过度配置

### 非目标（明确不做）

| 项 | 理由 |
|---|---|
| 暗色模式、国际化、自动更新、系统托盘、全局快捷键、多窗口 | 原型无、YAGNI |
| 「粘贴 / 读文件 / 保存 / 展开 / 灯泡 / 收藏夹 / 弹出窗口」按钮的功能 | 仅视觉，与原型 spec 保持一致 |
| 搜索框过滤导航 | 仅可输入，与原型 spec 保持一致 |
| ESLint / Prettier / Husky / CI | 骨架阶段不加，避免被规则配置拖住 |
| 前端单测、E2E | 已选「手工检查点表」验收策略 |
| 为每个导航项生成独立页面组件 | 已选「统一 Coming Soon 页」策略 |
| 全量替换为 Naive UI 组件 | 已选「原型优先，Naive UI 补充」策略 |

---

## 二、关键决策汇总

| 维度 | 选择 | 备注 |
|---|---|---|
| UI 库 | Naive UI | 仅作"补充"角色：当前只用到 `n-message-provider` |
| 还原策略 | 原型优先，Naive UI 补充 | 开关 / pill / ghost / gutter / toast 沿用原型自定义 CSS |
| 架构 | 多页 + vue-router + 占位页 | URL 页为真实页面，其他跳统一占位页 |
| Tauri 后端 | URL 编解码下沉到 Rust `#[command]` | 前端经 `src/api/url.ts` 单一入口调用 |
| 构建栈 | Tauri 2 + Vue 3 + TS + Vite + pnpm | 由 `create-tauri-app` 模板初始化 |
| 状态管理 | Pinia | 仅 `useNavStore` 管导航数据 + 当前选中项；URL 页内部状态局部 ref |
| 样式 | Tailwind + CSS 变量 + Scoped CSS 三层 | 详见 §6 |
| 工程位置 | `fs-tauri/` 根目录平铺 | `prototype/`、`docs/` 与工程文件同级 |
| 占位页 | 统一 `PlaceholderView.vue` | 通过路由参数读工具名 |
| 验收 | 手工检查点表 | 不写前端单测、不写 E2E |
| Rust 单测 | 保留 2-3 条作开发自查 | 不进验收清单 |

---

## 三、目录结构

```
fs-tauri/
├── prototype/index.html             ← 已有，保留作参考
├── docs/superpowers/                ← 已有 spec / plan
├── DESIGN.md                        ← 已有
├── README.md                        ← 已有
│
├── package.json                     ← 新建（pnpm 管理）
├── pnpm-lock.yaml                   ← 自动生成
├── vite.config.ts                   ← 新建
├── tsconfig.json / tsconfig.node.json ← 新建
├── tailwind.config.ts               ← 新建
├── postcss.config.js                ← 新建
├── index.html                       ← 新建（Vite 入口；与 prototype/index.html 是两份不同的文件）
│
├── src/                             ← 前端源
│   ├── main.ts                      入口：挂载 App、安装 router、Pinia、Naive UI providers
│   ├── App.vue                      根组件：仅含 <router-view> + Naive providers
│   ├── router/index.ts              路由表
│   ├── styles/
│   │   ├── tokens.css               CSS 变量（从 prototype 摘出来，单一真源）
│   │   └── tailwind.css             @tailwind 三件套 + 全局 base
│   ├── stores/
│   │   └── nav.ts                   Pinia store：nav 数据 + 当前选中项
│   ├── api/
│   │   └── url.ts                   封装 invoke('url_encode' / 'url_decode')
│   ├── layouts/
│   │   └── AppShell.vue             两栏壳：<aside> + <main><router-view/></main>
│   ├── components/
│   │   ├── nav/AsideNav.vue         整个左侧栏（head + search + nav + foot）
│   │   ├── nav/NavGroup.vue         可折叠分组
│   │   ├── nav/NavItem.vue          单项（含 active / glyph / bulb）
│   │   ├── ui/Switch.vue            原型样式滑动开关（受控）
│   │   ├── ui/PillBtn.vue           pill 按钮（含 icon-only 变体）
│   │   ├── ui/GhostBtn.vue          ghost 按钮（页头用）
│   │   └── ui/CodeArea.vue          gutter + textarea/pre 复合组件（输入 / 输出共用）
│   └── views/
│       ├── UrlView.vue              URL 工具页
│       └── PlaceholderView.vue      统一 Coming Soon 页
│
└── src-tauri/                       ← Rust 后端（create-tauri-app 生成）
    ├── Cargo.toml
    ├── tauri.conf.json              窗口尺寸 / 标题 / 图标
    ├── build.rs
    └── src/
        ├── main.rs                  注册 commands、启动 builder
        └── tools/
            ├── mod.rs               pub mod url;
            └── url.rs               url_encode / url_decode 实现 + 单测
```

**依赖方向（单向、无环）：**

```
views ──▶ components ──▶ ui (atoms)
  │           │
  ├──▶ stores (Pinia)
  └──▶ api ──▶ Tauri invoke ──▶ Rust commands
```

---

## 四、组件契约

| 组件 | Props | Emits | 内部状态 |
|---|---|---|---|
| `AppShell.vue` | — | — | 无 |
| `AsideNav.vue` | — | — | 读 `useNavStore` |
| `NavGroup.vue` | `group: NavGroup` | — | `expanded` 本地 ref（初始值取自 `group.expanded`） |
| `NavItem.vue` | `item: NavItem` | `click` | 无 |
| `Switch.vue` | `modelValue: boolean`, `onLabel?: string`, `offLabel?: string` | `update:modelValue` | 无（受控） |
| `PillBtn.vue` | `iconOnly?: boolean` | 透传 click | 无 |
| `GhostBtn.vue` | — | 透传 click | 无 |
| `CodeArea.vue` | `modelValue: string`, `readonly?: boolean` | `update:modelValue` | 计算 `lineCount`，渲染 gutter；`readonly === true` 渲染 `<pre>`，否则渲染 `<textarea>` |
| `UrlView.vue` | — | — | `input / transformMode / multiline / output` |
| `PlaceholderView.vue` | — | — | 从 `route.params.id` 经 `useNavStore` 反查工具 `label`；查不到时显示「未知工具」 |

**`CodeArea` 双形态原则：** 输入区为 `<textarea>`、输出区为 `<pre>`，两者共用同一个组件，通过 `readonly` 切换。形态贴合原型 HTML 结构（不强行统一为 `<textarea readonly>`）。

---

## 五、数据流

### URL 工具页

```
                 ┌────────────────────── UrlView.vue ──────────────────────┐
                 │                                                          │
  user input ─▶  │  ref: input, transformMode, multiline                    │
                 │                                                          │
                 │  watch([input, transformMode, multiline]) ──▶  api.url   │
                 │                                                  │       │
                 │                                                  ▼       │
                 │                                            invoke('url_encode'/'url_decode')
                 │                                                  │       │
                 │                       ref: output  ◀─────────────┘       │
                 │                                                          │
  user click ──▶ │  「复制」→ navigator.clipboard.writeText(output)          │
                 │            + n-message.success('已复制')                  │
                 │  「清空」→ input = ''；watcher 自动驱动 output / 行号    │
                 └──────────────────────────────────────────────────────────┘
```

### 关键设计点

1. **单向流：** UI 状态 → API 调用 → 渲染输出。`output` 是派生值，用户不可直接编辑（`readonly`）。
2. **不加防抖：** Rust `invoke` 本地往返 < 1ms。出现实测卡顿再加。
3. **Pinia store 边界：** `useNavStore` 只管「导航数据 + 当前选中项 id」。URL 页内部状态（`input/output/transformMode/multiline`）不进 store。
4. **race token：** watcher 内用一个递增 `reqId` 防止旧 invoke 结果覆盖新结果：

   ```ts
   let reqId = 0
   watch([input, mode, multi], async ([t, m, ml]) => {
     const my = ++reqId
     const fn = m === 'encode' ? urlApi.encode : urlApi.decode
     const r = await fn(t, ml)
     if (my === reqId) output.value = r
   }, { immediate: true })
   ```

### 路由

```ts
const routes = [
  { path: '/',                redirect: '/tools/url' },
  { path: '/tools/url',       component: () => import('@/views/UrlView.vue') },
  { path: '/tools/:id',       component: () => import('@/views/PlaceholderView.vue') },
]
```

`PlaceholderView` 通过 `route.params.id` 在 `useNavStore` 反查 `label`；查不到时显示「未知工具」。

### 导航数据

`NAV_DATA` 沿用原型 `prototype/index.html` 中 `const NAV` 的结构，对每项 item 增补一个 `id` 字段（`'qrcode'`、`'url'`、`'jsonpath'`、`'regex'`、`'json'`、`'sql'` 等），路由 `to` 由 `id` 派生。

---

## 六、Tauri 后端契约

### Rust 命令

```rust
// src-tauri/src/tools/url.rs
#[tauri::command]
pub fn url_encode(text: String, multiline: bool) -> String { ... }

#[tauri::command]
pub fn url_decode(text: String, multiline: bool) -> String { ... }
```

实现要点：

- 新增依赖：`percent-encoding = "2"`（Rust 生态标准库）
- `url_encode` 字符集：使用自定义 ASCII set，等同于 JS `encodeURIComponent` 的"不编码集"——**排除 `-_.!~*'()` 不编码**，否则两端会出现 `(` ↔ `%28` 不一致
- `url_decode` 失败时返回**原文**（不 `Result::Err`、不 panic），与原型 `safeDecode` 一致
- `multiline === true` 时按 `\n` 拆分，对每行独立编/解码后用 `\n` 拼回；为 `false` 时整体处理

### 注册

```rust
// src-tauri/src/main.rs
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

### 前端封装

```ts
// src/api/url.ts
import { invoke } from '@tauri-apps/api/core'

export const urlApi = {
  encode: (text: string, multiline: boolean) =>
    invoke<string>('url_encode', { text, multiline }),
  decode: (text: string, multiline: boolean) =>
    invoke<string>('url_decode', { text, multiline }),
}
```

组件层只调 `urlApi`，不直接 import `invoke`。

### Rust 单测（开发自查、不进验收）

```rust
#[test] fn encodes_space_as_pct20() { ... }
#[test] fn multiline_each_line_independently() { ... }
#[test] fn decode_invalid_returns_original() { ... }
```

### `tauri.conf.json` 关键字段

| 字段 | 值 | 理由 |
|---|---|---|
| `productName` | `fs-tauri` | 沿用包名 |
| `app.windows[0].width` | `1100` | 容纳 280 + 主区合理留白 |
| `app.windows[0].height` | `760` | 配置卡 + 双 IO 区可见 |
| `app.windows[0].title` | `URL 编码 / 解码工具` | 与原型 H1 一致 |
| `app.windows[0].resizable` | `true` | — |
| `app.windows[0].minWidth / minHeight` | `880 / 600` | 防止侧栏被压扁 |
| `app.security.csp` | 默认 | 无外链资源 |

---

## 七、样式三层与 Token 治理

### 总体结构

```
┌─────────────────────────────────────────────────────────┐
│ Layer 1  src/styles/tokens.css                          │
│   :root { --bg, --surface, --aside, --ink, --amber, ... }│
│   原型 26 个 CSS 变量原样搬入，作为唯一真源              │
└─────────────────────────────────────────────────────────┘
                         │
        ┌────────────────┼─────────────────┐
        ▼                ▼                 ▼
┌──────────────┐ ┌──────────────┐ ┌────────────────────┐
│ Layer 2a     │ │ Layer 2b     │ │ Layer 2c           │
│ Tailwind     │ │ Naive UI     │ │ Scoped CSS         │
│              │ │ themeOverrides│ │                    │
│ 间距/字号/圆角│ │ 仅在引入 n-* │ │ 高度自定义元件：   │
│ 通过 config  │ │ 时按需对齐   │ │ Switch、IO gutter、│
│ 引用 var()   │ │              │ │ pill-btn 等        │
└──────────────┘ └──────────────┘ └────────────────────┘
```

### Layer 1：`tokens.css`

完整搬运原型 `:root` 块（颜色 / 圆角 / 字体栈），不增不减。后续如要新增 token，先加在这里。

### Layer 2a：`tailwind.config.ts`

```ts
export default {
  content: ['./index.html', './src/**/*.{vue,ts}'],
  theme: {
    extend: {
      colors: {
        bg:      'var(--bg)',
        surface: 'var(--surface)',
        aside:   'var(--aside)',
        'aside-2': 'var(--aside-2)',
        card:    'var(--card)',
        rule:    'var(--rule)',
        ink:     'var(--ink)',
        'ink-2': 'var(--ink-2)',
        'ink-3': 'var(--ink-3)',
        amber:   'var(--amber)',
        link:    'var(--link)',
      },
      borderRadius: { sm: 'var(--r-sm)', md: 'var(--r-md)', lg: 'var(--r-lg)' },
      fontFamily: {
        sans: ['var(--sans)'],
        mono: ['var(--mono)'],
      },
    },
  },
  plugins: [],
}
```

**用法约定：** 布局、间距、文字尺寸、颜色、圆角 → 用 Tailwind 类。**只在 Tailwind 表达不到位时**才落到 scoped CSS。

### Layer 2b：Naive UI 主题对齐

`src/main.ts` 挂载 `n-config-provider` 时传 `themeOverrides`，**仅覆盖当前实际用到的组件**——目前只用到 `n-message-provider` 和 `n-message`，几乎不需要覆盖。后续真用到 `n-button` / `n-input` 再增量补充。

### Layer 2c：Scoped CSS

以下三类**保留**原型的精细化 CSS，不强行 Tailwind 化：

| 元件 | 理由 |
|---|---|
| `Switch.vue` | 滑动动画 cubic-bezier、24px 圆 + 20px 滑块、`.on` 状态——CSS 比 Tailwind 类组合短得多 |
| `CodeArea.vue` 的 gutter | `line-height 1.85`、`white-space: pre`、行号字色 `--link`——细节多 |
| `PillBtn.vue` / `GhostBtn.vue` | hover/active 渐变、SVG 14×14 尺寸——一次性写清更易维护 |

### 字体

`tokens.css` 仅声明字体栈，**不引入** woff2 文件。Inter Tight、JetBrains Mono 在 macOS 上自动回退到 PingFang SC / Menlo。未来如需真实 Inter Tight，再加 `@fontsource/inter-tight` 包。

### 不做项

- 不用 Tailwind 暗色（YAGNI）
- 不用 PostCSS nested / `@apply`（保持 CSS 简单）
- 不配置 Tailwind `prefix`（无冲突）

---

## 八、错误处理（最小化）

| 场景 | 行为 |
|---|---|
| Rust `url_decode` 输入含非法 percent 序列 | 返回原文，与原型 `safeDecode` 一致 |
| Rust `url_encode` | 不会失败，无需处理 |
| `invoke` 通信失败 | watcher 内 `try { await ... } catch { /* 静默 */ }`，保持上次 `output` |
| `navigator.clipboard.writeText` 失败 | `n-message.error('复制失败')` |
| 路由 `/tools/:id` 解析到未知 id | `PlaceholderView` 显示「未知工具」 |
| 启动期 Rust panic | 由 Tauri 默认行为处理，骨架阶段不加自定义 panic hook |

**反原则：** 不预先发明错误。不为还未发生的网络 / 权限 / 序列化问题写 try/catch。

---

## 九、验收检查点表（手工）

| # | 类别 | 验证点 |
|---|---|---|
| 1 | 启动 | `pnpm install` 无 error / 无需手动解决 peer dep 警告 |
| 2 | 启动 | `pnpm tauri dev` 弹出桌面窗口，标题"URL 编码 / 解码工具" |
| 3 | 视觉 | 整屏与 `prototype/index.html` 视觉一致（侧栏宽度、配色、字体、行号 gutter） |
| 4 | 导航 | 点 URL 项 → 路由切到 `/tools/url` |
| 5 | 导航 | 点 JSONPath → 路由切到 `/tools/jsonpath`，主区显示 Coming Soon 占位页带工具名 |
| 6 | 导航 | 点分组头 → 折叠 / 展开 + 箭头旋转 |
| 7 | 功能 | 输入 `hello world` → 输出 `hello%20world`（默认编码模式） |
| 8 | 功能 | 切换"解码"+ 输入 `hello%20world` → 输出 `hello world` |
| 9 | 功能 | Multiline + 编码 + 多行输入 → 多行各自独立编码 |
| 10 | 功能 | 输入 `%zz`（非法解码）→ 输出 `%zz`（保留原文，不报错） |
| 11 | 功能 | "清空"→ 输入 / 输出 / 行号都重置为空 + `1` |
| 12 | 功能 | "复制"→ 系统剪贴板更新 + `n-message` toast 显示「已复制」 |
| 13 | 功能 | 切换"编码 / 解码"开关 → 输出立即重算 |
| 14 | 视觉 | DevTools / Tauri 控制台无 error / warning（除已知 Vite HMR 噪声） |
| 15 | 工程 | `pnpm tauri build` 能产出 release 安装包（macOS .app / .dmg） |

---

## 十、工具链 / 脚本

`package.json` 至少提供：

```json
{
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

**关键依赖（精确版本由 plan 阶段定）：**

- runtime：`vue`、`vue-router`、`pinia`、`naive-ui`、`@tauri-apps/api`
- dev：`@tauri-apps/cli`、`vite`、`@vitejs/plugin-vue`、`typescript`、`vue-tsc`、`tailwindcss`、`postcss`、`autoprefixer`

**不引入：** ESLint / Prettier / Husky / Vitest / Playwright（骨架阶段）。

---

## 十一、参考

- 原型：`fs-tauri/prototype/index.html`
- 上一份 spec：`docs/superpowers/specs/2026-06-19-html-prototype-design.md`
- 上一份 plan：`docs/superpowers/plans/2026-06-19-html-prototype.md`
- 视觉 token 来源：`fs-vue/fs-desktop/DESIGN.md` §2（仅参考，token 已搬入 `prototype/index.html`）
