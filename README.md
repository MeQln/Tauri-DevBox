# DevBox · 开发工具箱

本地开发者工具集桌面应用（DevToys 风格），左侧导航 + 右侧工具页。基于 **Tauri 2 + Vue 3 + Pinia + vue-router + Tailwind CSS** 构建。系统与网络类工具的真实逻辑下沉到 Rust（经 Tauri IPC 调用）；文本 / 格式化类工具使用前端原生能力或成熟库，避免不必要的跨端。

> 产品名已全面统一为 **DevBox**：仓库目录名、远端地址（`gitee.com/MeQln/DevBox.git`）、`package.json#name`、`tauri.conf.json`（`productName` / `identifier`）、窗口标题 `DevBox · 开发工具箱`、Cargo `[[bin]] name = "DevBox"` 全部一致，无 fs-tauri 残留。

## 预览

![DevBox 端口管理页](screenshot.png)

## 已实现工具

| 工具 | 路由 | 后端实现 | 说明 |
|------|------|----------|------|
| 端口管理 | `/tools/port` | `tools/port.rs` (`list_ports` / `kill_port`) | 列出 TCP 监听端口与占用进程，支持结束进程 |
| 二维码 | `/tools/qrcode` | `tools/qrcode.rs` (`qr_encode` / `qr_decode`) | 实时生成与图片解析，支持拖拽、读文件、剪贴板 |
| URL 编 / 解码 | `/tools/url` | `tools/url.rs` (`url_encode` / `url_decode`) | 字符集与 JS `encodeURIComponent` 对齐；解码失败返回原文 |
| Base64 文本 | `/tools/base64-text` | `tools/base64.rs` (`base64_encode` / `base64_decode`) | 标准 / URL-safe 两种字母表；解码自动剥离空白，失败返回原文 |
| Base64 图片 | `/tools/base64-image` | `tools/base64.rs` | 图片 ↔ Base64 互转预览 |
| 连通性测试 | `/tools/connectivity` | `tools/net.rs` (`ping_host` / `check_port`) | ICMP ping（系统命令跨平台适配）+ TCP 端口探测，过程流式实时输出 |
| WebSocket 测试 | `/tools/websocket` | — 前端 `WebSocket` API | 连接 / 断开、收发消息日志、状态指示 |
| JSON 格式化 | `/tools/json` | — 前端 `JSON` 原生 | 格式化 / 压缩，2 / 4 空格缩进，解析错误内联提示 |
| SQL 格式化 | `/tools/sql` | — 前端 `sql-formatter` | 格式化（关键字大写）/ 压缩，2 / 4 空格缩进 |
| XML 格式化 | `/tools/xml-fmt` | — 前端 `DOMParser` | 格式化 / 压缩，支持属性 / 注释 / CDATA / 处理指令 |
| 哈希 / 校验 | `/tools/hash` | `tools/hash.rs` (`hash_text` / `hash_bytes` / `hash_file`) | 文本 / 文件计算 MD5 / SHA1 / SHA256 / SHA384 / SHA512，文件 64KB 分块流式 |
| 密码生成 | `/tools/password` | `tools/password.rs` (`generate_passwords`) | OsRng 密码学安全随机，可选字符类别并保证覆盖，可排除易混淆字符 |
| UUID 生成 | `/tools/uuid` | `tools/uuid.rs` (`generate_uuids`) | v4（随机）/ v7（时间排序）批量生成，大写与连字符可选 |
| 图像格式转换 | `/tools/image-format` | `tools/image.rs` (`image_read` / `image_convert`) | 支持 PNG / JPEG / WebP / BMP / GIF / TIFF / ICO 互转，显示元信息 |
| 图片压缩 | `/tools/image-compress` | `tools/image.rs` (`image_compress`) | JPEG / WebP 有损压缩，可调质量滑块 |
| 转义 / 反转义 | `/tools/escape` | — 前端 | HTML / URL / JSON / Unicode 转义与反转义 |
| 文本比对 | `/tools/list-cmp` | — 前端 | 按行 / 字符比对，差异高亮 |
| Markdown 预览 | `/tools/md` | — 前端 | 实时渲染 Markdown，支持 GFM 扩展 |
| 设置 | `/tools/settings` | — 前端 | 应用配置 |

暂未实现的导航项统一落到 `PlaceholderView` 占位页。导航按分组组织：系统工具、编解码器、格式化工具、测试工具、生成器、图像处理、文本处理。

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
views (UrlView / QrCodeView / PortView / Base64ImageView / Base64TextView / JsonView / SqlView / XmlView / ConnectivityView / WebSocketView / HashView / PasswordView / UuidView / FormatConversionView / ImageCompressionView / EscapeView / ListCompareView / MarkdownView / SettingsView / PlaceholderView)
  ├─▶ components/nav (AsideNav / NavGroup / NavItem)
  ├─▶ components/ui  (Switch / PillBtn / CodeArea)
  ├─▶ stores/nav     (useNavStore — 单一全局 store，NAV_DATA 为导航真源)
  └─▶ api/*          (Tauri invoke 封装)
```

- `components/ui/*` 不感知路由 / store
- `stores/nav.ts` 不引用任何具体工具页

### 路由

vue-router 嵌套在 `AppShell` 之下：`/tools/:id` 命中 `PlaceholderView`，通过 `useNavStore.findLabel(id)` 反查工具名显示标题；已实现工具页（`port`、`qrcode`、`url`、`base64-text`、`base64-image`、`json`、`sql`、`xml-fmt`、`connectivity`、`websocket`、`hash`、`password`、`uuid`、`image-format`、`image-compress`、`escape`、`list-cmp`、`md`、`settings`）注册独立路由。根路径 `/` 默认重定向到 `/tools/port`。

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

> 需 Rust 能力（系统 / 网络 / 编解码对齐）的工具走完整 4 步；纯前端工具（如 JSON / SQL / XML 格式化、WebSocket）跳过第 1、2 步，视图直接用 JS 原生 API 或库。

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
DevBox/
├── DESIGN.md                 # UI 设计约束（单一真源）
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
│   │   └── tools/            #   #[tauri::command] 实现（base64 / encoding / hash / image / net / password / port / qrcode / url / uuid）
│   ├── capabilities/         #   权限配置
│   └── tauri.conf.json
└── docs/superpowers/         #   设计 spec 与实现计划
```

## 文档

| 文档 | 说明 |
|------|------|
| [`CLAUDE.md`](CLAUDE.md) | 项目专属指令（常用命令、架构概览、约束） |
| [`DESIGN.md`](DESIGN.md) | UI 视觉与交互约束（单一真源） |
| [`docs/superpowers/specs/2026-06-19-tauri-vue3-skeleton-design.md`](docs/superpowers/specs/2026-06-19-tauri-vue3-skeleton-design.md) | 架构设计 spec |
| [`docs/superpowers/plans/2026-06-19-tauri-vue3-skeleton.md`](docs/superpowers/plans/2026-06-19-tauri-vue3-skeleton.md) | 实现计划 |

## License

[MIT](./LICENSE) © 2026 MeQln
