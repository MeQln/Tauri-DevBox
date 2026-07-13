# DevBox UI 设计约束

> 本文是 UI 视觉与交互的单一真源。技术架构、构建、命名层级见 [`CLAUDE.md`](CLAUDE.md)。

视觉与交互的最终参照是 [`prototype/index.html`](prototype/index.html)（DevToys 风原型）。当原型与本文冲突时，**以原型为准**，并修订本文使其重新对齐。

---

## 一、视觉

颜色 / 圆角 / 字体栈集中在 `src/styles/tokens.css`（114 个变量声明，含深色与配色覆盖），是唯一真源。Tailwind 仅暴露常用子集（`bg-aside`、`text-ink-2`、`rounded-md` 等），剩余在 scoped CSS 里直接用 `var(--xxx)`。

新增 / 调整 token 时 **先改 `tokens.css`**，再决定是否暴露到 Tailwind。

### 颜色

**基准（浅色 · 蓝色配色）**：

| 类别 | 变量 | 用途 |
|---|---|---|
| Surface | `--bg #f6f5f3` / `--surface #ffffff` | 页面 / 主区底 |
| Aside | `--aside #ffffff` / `--aside-2 #dce8f9` / `--aside-3 #c8d8f0` | 侧栏底 / hover / active 渐变下色 |
| Card | `--card #f3f2ef` / `--card-2 #ffffff` | 配置卡 / 行内卡 |
| Rule | `--rule #e3e0d9` / `--rule-soft #ecebe6` | 主分割线 / 次分割线 |
| Ink | `--ink #1d1d1f` → `--ink-5 #c2c0bb` | 文字色阶（5 级，越小越深） |
| Accent | `--amber #e8a534` / `--amber-d #b8821e` | 警告 / 强调（如「有更新」bulb） |
| Semantic | `--link #2769d0` / `--ok #4f9a59` / `--warn #d97a3b` | 行号 / 成功 / 警告 |
| Main | `--aside-top #f3f6fd` / `--border-accent #c4d4eb` / `--accent #5b8cff` / `--danger #d33` | 主区渐变底色、描边、强调色、危险色 |

**`--aside-2` / `--aside-3` / `--aside-top` / `--border-accent` / `--link` / `--accent` 的值随当前配色（[data-color]）变化**。支持 6 种配色：
- `blue`（默认）、`purple`、`green`、`rose`、`teal`、`warm`
- 各配色的精确值见 `tokens.css` 中 `[data-color="*"]` 规则

**深色模式**通过 `[data-theme="dark"]` 覆盖 Surface / Aside / Rule 等 token，圆角与字体栈沿用浅色值。深色模式下 `--aside: #ffffff`（侧栏不变），Surface 类与 Rule 类调暗。各色配在 `[data-theme="dark"][data-color="*"]` 中有对应的深色 aside 色值。

### 圆角

| 变量 | 值 | 用途 |
|---|---|---|
| `--r-sm` | 6px | （预留） |
| `--r-md` | 10px | 配置卡 / IO 区 |
| `--r-lg` | 14px | （预留） |

按钮 / 菜单项独立用 `8px`（不在 token 中）以贴合原型。

### 字体

```
sans  Inter Tight, PingFang SC, Noto Sans SC, -apple-system, BlinkMacSystemFont, sans-serif
mono  JetBrains Mono, ui-monospace, Menlo, Consolas, monospace
serif Inter Tight, PingFang SC, Noto Sans SC, -apple-system, sans-serif  （H1 用）
```

不引入 woff2 字体文件，依赖系统回退。

### 过渡

- 标准 `.15s`（hover / 颜色切换）
- 滑动 `.22s cubic-bezier(.2, .7, .2, 1)`（开关滑块）

---

## 二、整体布局

```
┌──────────────────────────────────────────────────────────────┐
│ window  100vw × 100vh    grid-cols-[280px 1fr]               │
├────────────┬─────────────────────────────────────────────────┤
│ AsideNav   │ main  bg-surface  flex-col  px-8 pt-[22px] pb-8 │
│ 280px      │  背景渐变: linear-gradient(90deg, #fff, var(--aside-top))
│ bg-aside   │   └─ n-dialog-provider (flex-1 min-h-0)         │
│            │       ├─ 标准工具页（JSON / URL / SQL / …）      │
│            │       │   page-head → config → text-area×2      │
│            │       ├─ 特殊工具页（WebSocket / Markdown / …） │
│            │       │   各自特有布局（scoped CSS 定义）        │
│            │       └─ PlaceholderView（未实现工具的统一占位）  │
└────────────┴─────────────────────────────────────────────────┘

最小窗口  880 × 600     默认窗口  1100 × 760
```

**标准工具页布局**（JSON / URL / SQL / XML / Base64 / 哈希 / 密码 / UUID / 转义 / 文本比对 等）：

```
page-head          <h1>         （18px mb）
section-title     配置          （12px mt 8px mb）
config → row × N               （配置项行，每行网格 [44px 1fr auto]）
section-title     输入 + actions
text-area         输入          （flex-1 min-h-200px，可编辑）
section-title     输出 + actions
text-area         输出          （flex-1 min-h-200px，readonly）
error-bar         错误栏        （v-if 条件显示，解析/格式化错误时出现）
```

**特殊页** 布局各异：WebSocket 为连接栏 + 日志区 + 发送栏；Markdown 为左右分栏；图像处理页含 Canvas / 预览区等——各页 scoped CSS 自行定义。

- 标准页高度由 `flex-1` 和整体 `flex-col` 约束，`text-area` 设 `min-height: 200px`，超出时 `overflow: auto` 自滚
- 不要给标准页的 `text-area` 加 `position: absolute` 或固定高度，它们靠 flex 上下文自适应

**自适应**：宽度由 `[280px 1fr]` 网格驱动；标准页高度由两个 `text-area` 通过 `flex-1 min-h-200px` 展开；特殊页各自定义布局。

---

## 三、组件视觉规范

### 侧栏 AsideNav

- 三段：搜索框 → 导航 → footer，分别由 grid `[auto 1fr auto]` 行划分
- 顶部留 `pt-2.5` 替代之前的 head 区
- 搜索框 `34px` 高、`1px solid var(--rule)` 描边、focus 时 `border #c5bfb4` + `box-shadow 0 0 0 3px rgba(0,0,0,0.04)`；**只接受输入，不做过滤**（仅占位）
- footer 与导航之间用 `border-t border-rule` 分割

### NavItem（菜单项）

- 高度 `32px`，圆角 `8px`，左右内边距 `6px`
- 网格 `[22px | 1fr]`：左槽放 SVG / glyph，右栏 label
- **active 状态**：`linear-gradient(180deg, #e1ddd4, #d5d0c5)` 背景 + `inset 0 0 0 1px rgba(0,0,0,0.04)` 内描 + `font-weight: 500`
- **hover**：`bg-aside-2`
- 不渲染右侧圆点（之前的 `hasUpdate` bulb 已移除，但字段保留以备未来复用）

### NavGroup（分组头）

- 高度 `36px`，网格 `[22px | 1fr | 16px]`：左槽 SVG 图标、中间 label、右侧箭头
- **折叠 / 展开**：点击切换 `expanded`；`v-show` 控制子项；箭头 `-rotate-90` 与 0 之间过渡

### 顶层节点之间的分割线

`AsideNav` 循环渲染时，**当前节点 `type` 与前一个不同**则插入 `<hr>` 分隔（`my-1.5 mx-1.5 border-0 border-t border-rule`）。当前数据下只在 `url`（item）↔ `g-test`（group）之间出现一条；group ↔ group 不画线。

### 图标

侧栏图标统一从 `src/components/nav/icons.ts` 的 `ICONS` 表取，lucide 风 SVG inner markup（24×24 viewBox，stroke 风，1.5/2 px 描边），通过 `v-html` 注入 `<svg width="16" height="16" stroke="currentColor">`。新增图标只需在 `icons.ts` 加 key，store 中相应节点写 `icon: 'newkey'`。

### Switch（滑动开关）

- `44 × 24px`、圆角 `999px`
- 滑块 `20 × 20px`，white + `box-shadow 0 1px 2px rgba(0,0,0,0.18)`
- on 态：背景 `#1e1e21`，滑块 `translateX(20px)`
- 标签文字在开关左侧（`row-ctl` 容器，`text-ink-3 text-12.5px`）
- **事件绑定只在 `<label>`**，`<input>` 仅作可访问性挂载（`tabindex="-1"`）；不要给 input 加 `@click`，会导致 label-input 自动 click 链路双触发

### PillBtn / GhostBtn

- **PillBtn**：`30 × auto`，圆角 `8px`，背景 `var(--card)`，hover `#ebe9e3`，active `#e2dfd8`；`icon-only` 变体宽度固定 `32px`；内嵌 SVG 统一 `14 × 14`
- **GhostBtn**：仅在页头使用（已移除）；如未来复用，参考 `prototype/index.html`

### 公共样式（common.css）

`src/styles/common.css` 定义了所有视图共享的公共样式，避免逐文件重复：

- **`.page-head`**：flex row，标题与操作按钮行，`mb-18px`
- **`.section-title`**：flex row，节标题（配置 / 输入 / 输出），`12px mt 8px mb`
- **`.config`**：flex column `gap-4px`，包配置项行
- **`.row`**：grid `[44px 1fr auto]`，配置项卡片行，`border: 1px solid var(--border-accent)`，圆角 8px，`padding 14px 16px`
- **`.text-area`**：flex-1 `min-height: 200px`，`padding 12px 14px`，mono 字体，圆角 `var(--r-md)`，`border: 1px solid var(--border-accent)`，`resize: none`；`readonly` 时不可编辑
- **`.error-bar`**：warn 色背景 + 左边框，v-if 条件显示解析/格式化错误

### CodeArea（已废弃）

`src/components/ui/CodeArea.vue` 组件存在于代码中但**不再被任何视图使用**。所有视图已迁移至 `common.css` 的 `.text-area` 类（`<textarea class="text-area">`）。保留 CodeArea 组件源文件仅作历史参考，新增视图不应引用它。

### 主题系统

`src/stores/theme.ts` 管理全局主题，支持：
- **深色模式**：通过 `[data-theme="dark"]` 覆盖 `tokens.css` 中的 Surface / Rule / Ink 等色值
- **6 种配色**：blue（默认）/ purple / green / rose / teal / warm，通过 `[data-color="*"]` 选择器控制 `--aside-*` / `--link` / `--accent` 等色值
- **持久化**：mode 存 `localStorage('devbox-theme')`，color 存 `localStorage('devbox-color')`
- **初始化**：`initTheme()` 在 `app.mount()` 前调用，同步 DOM 防闪烁
- **Naive UI 联动**：`App.vue` 中根据 `isDark` 传 `darkTheme / null` 给 `<n-config-provider>`

`AppShell.vue` 的 main 区使用 `background: linear-gradient(90deg, #ffffff, var(--aside-top))` 渐变背景，其中 `--aside-top` 随配色切换。

### Toast / Message

仅用 `useMessage()` 的 `success / error`，不引入 `useDialog` / `useNotification`。文案：「已复制」 / 「复制失败」。

---

## 四、文案规范

- 全中文（原型已确定的中英混排短语保留，如「Encoding / Decoding Multiline」）
- 标题用全角斜杠 `/` 与半角空格组合，如「URL 编码 / 解码工具」「转义 / 反转义」
- 「即将上线」用于占位页（PlaceholderView）
- **产品名固定 `DevBox`**：窗口标题 `DevBox · 开发工具箱`、页内不出现「fs-tauri」字样

---

## 五、交互约束

### 工具视图分类

工具视图分为两类：

- **标准输入/输出页**（JSON / URL / SQL / XML / Base64 / 哈希 / 密码 / UUID / 转义 / 文本比对）：单输入 → 单输出，**输出区只读**（`<textarea class="text-area" readonly>`），用户不可编辑
- **特殊交互页**（WebSocket / Markdown / 图像 / 连通性 / 端口管理）：布局与交互各不相同，各页 scoped CSS 自行定义

### 响应式

- 输入 / 开关变化触发 watcher，结果立即写回输出区（无防抖；本地 invoke 往返 < 1ms 不需要）
- 标准页使用 `text-area` 的 `flex-1` + `min-height: 200px` 自适应，超出时 `overflow: auto` 自滚
- `AsideNav` 的搜索框、PillBtn 的「粘贴 / 读取文件 / 保存 / 展开 / 预览模式」**仅视觉**，无交互（部分已实现，未来视需要开放）

### 错误的 UI 表现

| 错误源 | 表现 |
|---|---|
| Rust decode 非法 percent 序列 | 输出原文（用户视角下相当于"无变化"） |
| 前端 watcher invoke 失败 | 静默，保留上次输出（不弹 toast，不打扰） |
| 剪贴板权限 / 失败 | `n-message.error('复制失败')` 右上角 toast |

详细策略与代码位置见 `CLAUDE.md`。

---

## 六、已实现项（UI 范畴）

以下项目已从"不做项"实现：

| 项 | 实现情况 |
|---|---|
| 暗色模式 | `src/stores/theme.ts` + `[data-theme="dark"]` 覆盖 token |
| 自定义主题色 / 用户配置 | 6 种配色（blue / purple / green / rose / teal / warm），持久化 localStorage |

## 七、不做项（UI 范畴）

| 项 | 理由 |
|---|---|
| 国际化 i18n | 仅中文 |
| 响应式 / 移动端断点 | 桌面应用，最小窗口 880 已限定 |
| 动画曲线之外的过渡（弹簧、惯性等） | 已用 `cubic-bezier(.2, .7, .2, 1)` 与 `.15s` 标准 |
| ARIA 完整无障碍 | 仅基础 `aria-label` / `title`；不为屏幕阅读器做完整支持 |

---

## 八、修订规则

UI 改动时：

1. 改原型 `prototype/index.html` 或本文，二者**不能**同时落后于代码；原型尚未覆盖的功能（暗色模式 / 多配色 / 新工具页）以代码为准
2. 颜色 / 圆角 / 字体先进 `tokens.css`，再决定 Tailwind 暴露
3. 公共布局样式（`.page-head` / `.section-title` / `.config` / `.row` / `.text-area` / `.error-bar`）优先进 `common.css`；自定义元件（Switch / Pill / NavItem 等）保留 scoped CSS；不强求 Tailwind 化
4. 新增 Naive UI 组件需评估是否破坏「原型优先」基线——默认**不**用 `n-switch` / `n-button` / `n-input`
5. 深色模式 / 配色系统扩展时：先在 `tokens.css` 加 `[data-theme="dark"]` / `[data-color="*"]` 覆盖，再更新 `theme.ts` 对应的 mode / color 类型
