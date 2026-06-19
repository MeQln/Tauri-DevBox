# DevBox UI 设计约束

> 本文是 UI 视觉与交互的单一真源。技术架构、构建、命名层级见 [`CLAUDE.md`](CLAUDE.md)。

视觉与交互的最终参照是 [`prototype/index.html`](prototype/index.html)（DevToys 风原型）。当原型与本文冲突时，**以原型为准**，并修订本文使其重新对齐。

---

## 一、视觉

颜色 / 圆角 / 字体栈集中在 `src/styles/tokens.css`（26 个 CSS 变量），是唯一真源。Tailwind 仅暴露常用子集（`bg-aside`、`text-ink-2`、`rounded-md` 等），剩余在 scoped CSS 里直接用 `var(--xxx)`。

新增 / 调整 token 时**先改 `tokens.css`**，再决定是否暴露到 Tailwind。

### 颜色

| 类别 | 变量 | 用途 |
|---|---|---|
| Surface | `--bg #f6f5f3` / `--surface #ffffff` | 页面 / 主区底 |
| Aside | `--aside #f1efeb` / `--aside-2 #e9e6e0` / `--aside-3 #ddd9d2` | 侧栏底 / hover / active 渐变下色 |
| Card | `--card #f3f2ef` / `--card-2 #ffffff` | 配置卡 / 行内卡 |
| Rule | `--rule #e3e0d9` / `--rule-soft #ecebe6` | 主分割线 / 次分割线 |
| Ink | `--ink #1d1d1f` → `--ink-5 #c2c0bb` | 文字色阶（5 级，越小越深） |
| Accent | `--amber #e8a534` / `--amber-d #b8821e` | 警告 / 强调（如「有更新」bulb） |
| Semantic | `--link #2769d0` / `--ok #4f9a59` / `--warn #d97a3b` | 行号 / 成功 / 警告 |

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
│ 280px      │   ├─ page-head        H1                        │
│ bg-aside   │   ├─ section-title    配置                      │
│            │   ├─ config           (2 row)                   │
│            │   ├─ section-title    输入 + actions            │
│            │   ├─ CodeArea  (flex-1, 输入)                   │
│            │   ├─ section-title    输出 + actions            │
│            │   └─ CodeArea  (flex-1, 输出 readonly)          │
└────────────┴─────────────────────────────────────────────────┘

最小窗口  880 × 600     默认窗口  1100 × 760
```

**自适应**：宽度由 `[280px 1fr]` 网格驱动；高度由两个 `<CodeArea class="flex-1">` 平分主区剩余空间——**不要给 CodeArea 加 `min-height`**，会破坏平分。

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

### CodeArea（输入 / 输出区）

- 容器 `.io`：`1px solid var(--rule)` 描边、圆角 `var(--r-md)`、背景 `var(--card-2)`、`min-h-0`（允许 flex shrink）
- 网格 `[56px | 1fr]`：左 gutter 行号、右 content
- gutter：右对齐、字体 `var(--mono) 13px`、行高 `1.85`、字色 `var(--link)`、`white-space: pre`、`user-select: none`
- content：`padding 12px 14px`、`white-space: pre-wrap`、`word-break: break-all`、`overflow: auto`（自滚）
- `readonly === true` 渲染 `<pre>`、否则 `<textarea>`（**不**统一为 `textarea readonly`，贴合原型 HTML 结构）

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

- 单输入 → 单输出，**输出区只读**（`<CodeArea readonly>`），用户不可编辑
- 输入 / 开关变化触发 watcher，结果立即写回输出区（无防抖；本地 invoke 往返 < 1ms 不需要）
- 点击行为对外可见性：当前 URL 工具页之外的导航项**点击都跳转**到 `/tools/:id` 占位页；`AsideNav` 的搜索框、PillBtn 的「粘贴 / 读取文件 / 保存 / 展开 / 预览模式」**仅视觉**，无交互

### 错误的 UI 表现

| 错误源 | 表现 |
|---|---|
| Rust decode 非法 percent 序列 | 输出原文（用户视角下相当于"无变化"） |
| 前端 watcher invoke 失败 | 静默，保留上次输出（不弹 toast，不打扰） |
| 剪贴板权限 / 失败 | `n-message.error('复制失败')` 右上角 toast |

详细策略与代码位置见 `CLAUDE.md`。

---

## 六、不做项（UI 范畴）

| 项 | 理由 |
|---|---|
| 暗色模式 | YAGNI，原型未设计 |
| 国际化 i18n | 仅中文 |
| 响应式 / 移动端断点 | 桌面应用，最小窗口 880 已限定 |
| 自定义主题色 / 用户配置 | 当前阶段无 |
| 动画曲线之外的过渡（弹簧、惯性等） | 已用 `cubic-bezier(.2, .7, .2, 1)` 与 `.15s` 标准 |
| ARIA 完整无障碍 | 仅基础 `aria-label` / `title`；不为屏幕阅读器做完整支持 |

---

## 七、修订规则

UI 改动时：

1. 改原型 `prototype/index.html` 或本文，二者**不能**同时落后于代码
2. 颜色 / 圆角 / 字体先进 `tokens.css`，再决定 Tailwind 暴露
3. 自定义元件（Switch / Pill / CodeArea / NavItem 等）保留 scoped CSS；不强求 Tailwind 化
4. 新增 Naive UI 组件需评估是否破坏「原型优先」基线——默认**不**用 `n-switch` / `n-button` / `n-input`
