# fs-tauri HTML 原型设计稿

> 日期：2026-06-19
> 范围：仅 `URL 编码 / 解码工具` 这一屏的纯 HTML 静态原型
> 产物：`prototype/index.html`（单文件）

---

## 一、背景

`fs-tauri` 是 `Front-Skeleton/fs-desktop/` 下规划中的 Tauri 桌面工具集项目，当前仓库仅有 README 与 `.git`，尚未初始化 Tauri 工程。

兄弟项目 `fs-vue/fs-desktop/` 已存在一份完整的 DevToys 风格 Web 原型（`prototype/index.html` 973 行）和详细设计规范（`DESIGN.md`）。本次任务**不复用**那份原型，而是基于用户提供的截图，从零写一份只覆盖 `URL` 工具页的精简版原型，作为后续 Tauri 落地的视觉与交互参考。

## 二、目标

- 视觉上与用户截图一致（左侧导航 + 右侧 URL 编/解码工具页）
- 双击 HTML 即可在浏览器预览，无构建、无依赖
- URL 编/解码核心交互真实可用（输入即输出 + 开关切换 + 复制 + 清空）
- 文件总行数 ≤ 450 行

## 三、非目标（明确不做）

- macOS 标题栏与 traffic lights（与图片一致）
- 其他工具页（JSONPath、JSON、SQL、正则等导航项**纯静态**，点击无响应）
- 「粘贴」「读文件」「保存」「展开」「灯泡预览模式」「添加到收藏夹」「弹出窗口」按钮（仅视觉，无功能）
- 搜索框过滤（仅可输入，不做导航过滤）
- 响应式（不实现 980px 断点）
- 暗色模式、键盘快捷键、ARIA 无障碍、单元测试

## 四、产物清单

```
fs-tauri/
└── prototype/
    └── index.html        ← 唯一新增文件
```

不动：`README.md`、`.gitignore`、`.claude/`。

预览方式：浏览器直接打开 `prototype/index.html`，无需服务器。

## 五、布局

整屏 `100vw × 100vh`，CSS Grid 两栏：`280px / 1fr`。

```
window
├── aside (280px)
│   ├── aside-head    返回按钮 / 折叠按钮
│   ├── search        搜索输入框（仅视觉，可输入但不过滤）
│   ├── nav           导航（NAV 数据驱动渲染）
│   └── aside-foot    管理扩展 / 设置
└── main (1fr，可滚动)
    ├── page-head     H1 标题 + 收藏 / 弹窗按钮
    ├── section-title 配置
    ├── config        2 行 row（转换开关 + Multiline 开关）
    ├── section-title 输入 + [粘贴/读文件/清空]
    ├── io-input      gutter 56px + textarea
    ├── section-title 输出 + [保存/复制/展开/灯泡]
    ├── io-output     gutter 56px + pre
    └── toast         右上角浮层（复制成功提示）
```

关键尺寸：

| 项 | 值 |
|---|---|
| 侧栏宽度 | 280px |
| 主区内边距 | 22px 32px 32px |
| 配置卡 row 高度 | min-height 64px |
| 输入 / 输出区 min-height | 240px / 240px（等高） |
| 行号 gutter 宽度 | 56px |

## 六、视觉 Token

直接复用 `fs-vue/fs-desktop/DESIGN.md` §2 全部 token：

- **颜色**：Surface（`--bg #f6f5f3` / `--surface #ffffff` / `--aside #f1efeb` 等）/ Ink（`--ink #1d1d1f` 等）/ Accent（`--amber #e8a534`）
- **圆角**：`--r-sm 6px` / `--r-md 10px` / `--r-lg 14px`
- **字体**：sans 用 `Inter Tight, PingFang SC, Noto Sans SC, -apple-system`；mono 用 `JetBrains Mono, ui-monospace, Menlo`
- **过渡**：标准 `.15s`、滑动 `.22s cubic-bezier(.2,.7,.2,1)`

不引入外部字体文件，仅在 `font-family` 声明字体栈。

## 七、导航数据（写死）

```js
const NAV = [
  { type: 'item', glyph: 'QR', label: '二维码', hasUpdate: true },
  { type: 'item', icon: 'link', label: 'URL', active: true },
  { type: 'group', label: '测试工具', expanded: true, children: [
    { type: 'item', glyph: '{;}', label: 'JSONPath' },
    { type: 'item', glyph: '.*',  label: '正则表达式', hasUpdate: true },
    { type: 'item', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', label: '格式化工具', expanded: true, children: [
    { type: 'item', glyph: '{;}', label: 'JSON' },
    { type: 'item', glyph: 'SQ',  label: 'SQL' },
    { type: 'item', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', label: '生成器',   expanded: false, children: [] },
  { type: 'group', label: '图像处理', expanded: false, children: [] },
  { type: 'group', label: '文本处理', expanded: true, children: [
    { type: 'item', glyph: 'TX', label: '转义 / 反转义' },
    { type: 'item', glyph: '≡',  label: '列表比对' },
    { type: 'item', glyph: 'MD', label: 'Markdown 预览' },
  ]},
];

const FOOT = [
  { icon: 'gear',     label: '管理扩展' },
  { icon: 'settings', label: '设置' },
];
```

**交互：**
- 仅 `URL` 项 active；其他项 hover 有底色但**点击无响应**
- 分组头点击切换 `.collapsed` 类（CSS 控制展开 / 折叠 + 箭头旋转）
- 搜索框可输入，不做过滤

## 八、URL 编 / 解码逻辑

**默认状态（与截图一致）：**
- 「转换」开关 = 编码（`checked = true`）
- 「Multiline」开关 = 关闭（`checked = false`）
- 输入框为空，输出框为空，两边行号均显示 `1`

**可工作的部分：**

| 触发 | 行为 |
|---|---|
| 输入框 `input` 事件 | 立即调用 `convert()` |
| 「转换」开关切换 | 立即重新 `convert()` |
| 「Multiline」开关切换 | 立即重新 `convert()` |
| 输入区「清空」按钮 | 清空输入 + 输出，行号重置为 1 |
| 输出区「复制」按钮 | `navigator.clipboard.writeText` + toast「已复制」1.1s |

**核心函数：**

```js
function convert() {
  const text = input.value;
  const isEncode = switchTransform.checked;
  const isMultiline = switchMultiline.checked;
  const fn = isEncode ? encodeURIComponent : safeDecode;
  const result = isMultiline
    ? text.split('\n').map(fn).join('\n')
    : fn(text);
  output.textContent = result;
  updateGutters();
}

function safeDecode(s) {
  try { return decodeURIComponent(s); } catch { return s; }
}
```

**错误处理（最小化）：**
- 解码失败保留原文，不抛错
- 剪贴板 API 失败 → toast「复制失败」

## 九、行号 gutter

- 输入：`Math.max(1, input.value.split('\n').length)`
- 输出：`Math.max(1, output.textContent.split('\n').length)`
- gutter 内容：`Array.from({length: n}, (_, i) => i + 1).join('\n')` 写到 `.gutter > pre`
- 空内容时显示 `1`

## 十、实现约束

- HTML / CSS / JS 全部内联在 `index.html`，不外链 CSS/JS/字体
- JS 包在 IIFE 里，不污染全局
- SVG 内联，按钮内统一 14×14，icon-btn 内 16×16
- 总行数 ≤ 450 行
- 仅目标现代 Chromium / Safari，不做兼容兜底

## 十一、验收标准

| # | 验证点 |
|---|---|
| 1 | 浏览器打开**无 console 报错** |
| 2 | 视觉与用户截图整体一致 |
| 3 | 输入 `hello world` → 输出 `hello%20world` |
| 4 | 切到「解码」+ 输入 `hello%20world` → 输出 `hello world` |
| 5 | Multiline + 编码 + 多行输入 → 多行各自独立编码 |
| 6 | 点「清空」→ 输入 / 输出 / 行号都重置 |
| 7 | 点「复制」→ 剪贴板更新 + toast 1.1s |
| 8 | 点其他导航项 → 无切换、无报错 |
| 9 | 点分组头 → 折叠 / 展开 + 箭头旋转 |
| 10 | `wc -l prototype/index.html` ≤ 450 |

## 十二、参考

- 视觉参考：用户提供的截图（DevToys 风 URL 编 / 解码工具页）
- Token 参考：`fs-vue/fs-desktop/DESIGN.md` §2
- 信息架构参考：`fs-vue/fs-desktop/prototype/index.html`（仅参考，不复用代码）
