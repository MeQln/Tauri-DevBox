# DevBox · 开发工具箱

本地开发者工具集桌面应用（DevToys 风格），左侧导航 + 右侧工具页。基于 **Tauri 2 + Vue 3 + Pinia + vue-router + Tailwind CSS** 构建，真实计算逻辑下沉到 Rust，前端通过 Tauri IPC 调用。

> 仓库目录名仍为 `fs-tauri`，但面向用户的产品名已统一为 **DevBox**（窗口标题 `DevBox · 开发工具箱`，Cargo `[[bin]] name = "DevBox"`）。

## 已实现工具

| 工具 | 路由 | 后端实现 | 说明 |
|------|------|----------|------|
| 二维码 | `/tools/qrcode` | `tools/qrcode.rs` (`qr_encode` / `qr_decode`) | 实时生成与图片解析，支持拖拽、读文件、剪贴板 |
| URL 编 / 解码 | `/tools/url` | `tools/url.rs` (`url_encode` / `url_decode`) | 字符集与 JS `encodeURIComponent` 对齐；解码失败返回原文 |
| 端口管理 | `/tools/port` | — | 规划中（Coming Soon 占位） |

其余导航项（JSONPath、正则、JSON/SQL/XML 格式化、转义、列表比对、Markdown 预览等）统一落到 `PlaceholderView` 占位页。

## 快速开始

### 环境要求

- Node.js + [pnpm](https://pnpm.io) 11（strict 策略）
- Rust toolchain（`cargo`）
- [Tauri 2 前置依赖](https://tauri.app/start/prerequisites/)（macOS / Windows / Linux 各有系统库要求）

### 常用命令

```bash
# 安装依赖
pnpm install

# 启动桌面应用（首次编译 1–3 分钟，含 Vite + Cargo）
pnpm tauri:dev

# 产出安装包到 src-tauri/target/release/bundle/
pnpm tauri:build

# 仅前端调试（Vite dev server @ 127.0.0.1:5173，但 invoke 调用会失败 — 桌面 IPC 需要 Tauri 进程）
pnpm dev

# 类型检查（提交前必须 exit 0）
pnpm exec vue-tsc -b

# 后端单测
cd src-tauri && cargo test --lib
```

## 架构概览

```
前端 src/ ──▶ src/api/* ──▶ invoke('xxx_command') ──▶ src-tauri/src/tools/*.rs (#[tauri::command])
```

**核心契约**：工具的真实逻辑下沉到 Rust，前端通过 `src/api/*` 单一入口调用，绝不在前端 fallback 浏览器 API，以免两端行为漂移。

### 单向依赖

```
views (UrlView / QrCodeView / PlaceholderView)
  ├─▶ components/nav (AsideNav / NavGroup / NavItem)
  ├─▶ components/ui  (Switch / PillBtn / CodeArea)
  ├─▶ stores/nav     (useNavStore — 单一全局 store，NAV_DATA 为导航真源)
  └─▶ api/*          (Tauri invoke 封装)
```

- `components/ui/*` 不感知路由 / store
- `stores/nav.ts` 不引用任何具体工具页

### 路由

vue-router 嵌套在 `AppShell` 之下：`/tools/:id` 命中 `PlaceholderView`，通过 `useNavStore.findLabel(id)` 反查工具名显示标题；具体工具页（`qrcode`、`url`）注册独立路由。

### 样式三层

1. **真源**：`src/styles/tokens.css`（CSS 变量：颜色 / 圆角 / 字体栈）
2. **Tailwind**：`tailwind.config.ts` 暴露常用子集，通过 `var(--xxx)` 引用 token
3. **scoped CSS**：原型自定义元件（Switch、CodeArea gutter、PillBtn）

新颜色 / 圆角 / 字体先加到 `tokens.css`，再决定是否暴露到 Tailwind。Naive UI 仅作 `<n-config-provider>` + `<n-message-provider>` 容器。

### Tauri 2 模板拆分

- `src-tauri/src/lib.rs` 是 crate root，含 `mod tools;` 与 `tauri::Builder` 链
- `src-tauri/src/main.rs` 仅 `fn main() { devbox_lib::run(); }`
- 注册新 `#[tauri::command]` 加在 `lib.rs` 的 `invoke_handler!` 里

## 新增工具指引

1. **后端**：在 `src-tauri/src/tools/` 新增模块，编写 `#[tauri::command]` 函数，并在 `lib.rs` 的 `invoke_handler!` 注册。
2. **前端 API**：在 `src/api/` 新增封装，统一调用 `invoke`。
3. **视图**：在 `src/views/` 新增 `XxxView.vue`，并在 `src/router/index.ts` 注册 `/tools/xxx` 路由。
4. **导航**：在 `src/stores/nav.ts` 的 `NAV_DATA` 对应分组下新增 `{ type: 'item', id, label, glyph? }`；如需新图标，在 `src/components/nav/icons.ts` 添加 key 后引用。

## 技术栈

- **桌面框架**：Tauri 2
- **前端**：Vue 3 + TypeScript + Pinia + vue-router + Tailwind CSS
- **UI**：Naive UI（仅容器与 message）+ 原型自定义元件
- **构建**：Vite 5
- **后端**：Rust

## 项目结构

```
fs-tauri/
├── src/                      # 前端
│   ├── api/                  #   Tauri invoke 封装
│   ├── components/
│   │   ├── nav/              #   侧栏导航（AsideNav / NavGroup / NavItem / icons）
│   │   └── ui/               #   基础元件（Switch / PillBtn / CodeArea）
│   ├── layouts/AppShell.vue
│   ├── router/index.ts
│   ├── stores/nav.ts         #   导航真源 NAV_DATA
│   ├── styles/tokens.css     #   设计 token
│   └── views/                #   工具页
├── src-tauri/                # 后端
│   ├── src/
│   │   ├── lib.rs            #   crate root + Builder 链
│   │   ├── main.rs           #   入口
│   │   └── tools/            #   #[tauri::command] 实现（url / qrcode）
│   ├── capabilities/         #   权限配置
│   └── tauri.conf.json
└── docs/superpowers/         #   设计 spec 与实现计划
```

## License

[MIT](./LICENSE) © 2026 MeQln
