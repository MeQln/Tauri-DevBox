# URL 工具页 HTML 原型 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 `fs-tauri/prototype/index.html` 产出一份单文件 HTML 原型，1:1 还原用户提供截图中的「URL 编码 / 解码工具」屏幕，并使核心编/解码交互真实可用。

**Architecture:** 单文件原型——HTML / CSS / JS 全部内联在一份 `index.html`。CSS 用 CSS variables 定义视觉 token，JS 包在 IIFE 中。导航数据（NAV / FOOT）用 `Array.map()` 拼字符串渲染，无任何模板/框架依赖。URL 编/解码用浏览器原生 `encodeURIComponent` / `decodeURIComponent`。

**Tech Stack:** 纯 HTML5 + CSS3 + 原生 JS（ES2020+），无依赖、无构建。目标浏览器：现代 Chromium / Safari。

## Global Constraints

> 以下值与 spec `docs/superpowers/specs/2026-06-19-html-prototype-design.md` 完全一致；每个任务隐式包含这些约束。

- **唯一新增文件**：`fs-tauri/prototype/index.html`
- **不动文件**：`README.md`、`.gitignore`、`.claude/`
- **总行数 ≤ 450 行**（含所有 HTML/CSS/JS）
- **不外链**：不引入 CSS 文件、JS 文件、字体文件、图标库
- **JS 必须 IIFE 包裹**，不污染 `window` 全局
- **目标浏览器**：现代 Chromium / Safari，不做兼容兜底
- **不做的事**（spec §三）：响应式断点、暗色模式、键盘快捷键、ARIA、单元测试、macOS 标题栏、其他工具页功能、读文件/保存/收藏/弹窗按钮、搜索过滤
- **视觉 token 来源**：复用 `fs-vue/fs-desktop/DESIGN.md` §2 全部值
- **侧栏宽度 280px**；**主区 padding 22px 32px 32px**；**输入/输出区 min-height 240px**；**gutter 56px**

---

## 文件结构

```
fs-tauri/
└── prototype/
    └── index.html        ← 唯一新增文件
```

单文件，分 5 个内聚段落（按出现顺序）：

| 段落 | 行数估算 | 内容 |
|---|---|---|
| `<head>` 内 `<style>`：tokens + base + window | ~30 | CSS variables、reset、`.window` 双栏 |
| `<style>`：aside 相关 | ~80 | `.aside` / `.search` / `.nav` / `.group` / `.item` / `.glyph` / `.bulb` / `.aside-foot` |
| `<style>`：main 相关 | ~110 | `.page-head` / `.section-title` / `.config` / `.row` / `.switch` / `.io` / `.gutter` / `.toast` / 各种按钮 |
| `<body>` HTML | ~70 | window 骨架 + 静态 main 内容 |
| `<script>` IIFE | ~90 | NAV/FOOT 数据 + 渲染 + 事件绑定 + convert + gutter + copy/clear/toast |

合计目标 ~380 行，留 70 行余量。

---

## 任务划分

整个原型按"骨架 → 左侧 → 右侧静态 → 右侧交互"的顺序拆 **6 个任务**。每个任务结束后浏览器打开 `prototype/index.html` 都能预览（页面有意义、无报错），可独立提交。

| # | 任务 | 产出 |
|---|---|---|
| 1 | HTML 骨架 + tokens + 两栏布局 | 打开页面看到灰白两栏，左 280px 右铺满 |
| 2 | 左侧导航壳（搜索 + nav 数据驱动 + 折叠 + foot） | 左侧导航完整渲染，分组可折叠，URL 项 active |
| 3 | 主区静态：page-head + section-title + 配置卡 | 顶部标题、收藏/弹窗按钮、配置区两个开关行（仅视觉） |
| 4 | 输入/输出区 DOM + gutter（无功能） | 输入/输出两个等高区域，行号 gutter 显示 `1` |
| 5 | URL 核心交互：convert + gutter 同步 + 开关联动 | 输入即输出，开关切换重算，Multiline 按行处理 |
| 6 | 复制 + 清空 + toast + 最终验收 | 复制写剪贴板 + toast 1.1s；清空重置；10 项验收全过 |

---

### Task 1: HTML 骨架 + tokens + 两栏布局

**Files:**
- Create: `fs-tauri/prototype/index.html`

**Interfaces:**
- Consumes: 无
- Produces:
  - DOM 顶层：`<div class="window"> <aside class="aside"></aside> <main class="main"></main> </div>`
  - CSS variables（`:root`）：`--bg --surface --aside --aside-2 --aside-3 --card --card-2 --rule --rule-soft --ink --ink-2 --ink-3 --ink-4 --ink-5 --amber --amber-d --link --ok --warn --r-sm --r-md --r-lg --serif --sans --mono`

- [ ] **Step 1：创建文件骨架**

创建 `fs-tauri/prototype/index.html`，写入完整内容：

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>URL 编码 / 解码工具</title>
  <style>
    /* === TOKENS === */
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

    /* === BASE === */
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body { height: 100%; }
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

    /* === WINDOW === */
    .window {
      width: 100vw;
      height: 100vh;
      display: grid;
      grid-template-columns: 280px 1fr;
      min-height: 0;
    }
    .aside {
      background: var(--aside);
      border-right: 1px solid var(--rule);
      min-width: 0; min-height: 0;
    }
    .main {
      background: var(--surface);
      min-width: 0; min-height: 0;
      display: flex;
      flex-direction: column;
      overflow: auto;
      padding: 22px 32px 32px;
    }
  </style>
</head>
<body>
  <div class="window">
    <aside class="aside"></aside>
    <main class="main"></main>
  </div>
  <script>
  (() => {
    'use strict';
    // 后续任务在此填充
  })();
  </script>
</body>
</html>
```

- [ ] **Step 2：浏览器目测验证**

在浏览器打开 `prototype/index.html`，预期：
- 页面整体是浅米色 + 白色两栏
- 左侧 280px 浅米色（`--aside #f1efeb`），右侧白色铺满剩余宽度
- 控制台无报错（F12 → Console）

- [ ] **Step 3：提交**

```bash
git add fs-tauri/prototype/index.html
git commit -m "feat: 添加原型骨架与视觉 tokens"
```

---

### Task 2: 左侧导航（搜索 + 数据驱动 nav + 分组折叠 + foot）

**Files:**
- Modify: `fs-tauri/prototype/index.html`（在 `<style>` 末尾追加 aside 相关样式；在 `<aside>` 内添加结构；在 IIFE 内添加 NAV/FOOT 数据 + 渲染）

**Interfaces:**
- Consumes: Task 1 的 CSS variables 与 `.aside` 容器
- Produces:
  - 全局（IIFE 内部）变量：`NAV` 数组、`FOOT` 数组、`renderNav()` 函数
  - DOM：`.aside-head` / `.search` / `.nav` / `.aside-foot`
  - CSS 类：`.icon-btn` / `.search` / `.nav` / `.group` / `.group-head` / `.chev` / `.group-body` / `.item` / `.it-icon` / `.glyph` / `.bulb` / `.aside-foot`

- [ ] **Step 1：在 `<style>` 末尾追加 aside 样式**

紧接在 `.main {...}` 后插入：

```css
    /* === ASIDE === */
    .aside { display: grid; grid-template-rows: auto auto 1fr auto; }
    .aside-head { display: flex; gap: 4px; padding: 10px 12px 6px; }
    .icon-btn {
      width: 30px; height: 30px;
      border-radius: 8px;
      display: inline-flex; align-items: center; justify-content: center;
      color: var(--ink-2); transition: background .15s;
    }
    .icon-btn:hover { background: var(--aside-2); }
    .icon-btn svg { width: 16px; height: 16px; }
    .search {
      margin: 0 12px 8px; height: 34px;
      background: var(--card-2);
      border: 1px solid var(--rule); border-radius: 8px;
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
    .nav { padding: 4px 8px 8px; overflow-y: auto; scrollbar-width: thin; }
    .nav::-webkit-scrollbar { width: 8px; }
    .nav::-webkit-scrollbar-thumb { background: var(--ink-5); border-radius: 4px; }
    .group-head {
      display: grid; grid-template-columns: 22px 1fr 16px; align-items: center;
      height: 36px; padding: 0 6px; border-radius: 8px;
      color: var(--ink-3); font-size: 13.5px;
      cursor: pointer; transition: background .15s;
    }
    .group-head:hover { background: var(--aside-2); }
    .group-head svg.gp-icon { width: 16px; height: 16px; }
    .group-head .chev { transition: transform .2s; color: var(--ink-4); }
    .group.collapsed .chev { transform: rotate(-90deg); }
    .group.collapsed .group-body { display: none; }
    .group-body { padding-left: 22px; }
    .item {
      display: grid; grid-template-columns: 22px 1fr auto;
      align-items: center; gap: 4px;
      height: 32px; padding: 0 6px; margin: 1px 0;
      border-radius: 8px;
      color: var(--ink-2); font-size: 13.5px;
      cursor: pointer; transition: background .15s;
    }
    .item:hover { background: var(--aside-2); }
    .item.active {
      background: linear-gradient(180deg, #e1ddd4, #d5d0c5);
      color: var(--ink); font-weight: 500;
      box-shadow: inset 0 0 0 1px rgba(0,0,0,0.04);
    }
    .item .it-icon { width: 16px; height: 16px; }
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
    .aside-foot {
      border-top: 1px solid var(--rule);
      padding: 8px;
    }
    .aside-foot .item { color: var(--ink-2); }
```

- [ ] **Step 2：替换 `<aside class="aside"></aside>` 为完整结构**

```html
    <aside class="aside">
      <div class="aside-head">
        <button class="icon-btn" title="返回" aria-label="返回">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 18l-6-6 6-6"/></svg>
        </button>
        <button class="icon-btn" title="折叠" aria-label="折叠">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18M3 12h18M3 18h18"/></svg>
        </button>
      </div>
      <div class="search">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="7"/><path d="M21 21l-4.3-4.3"/></svg>
        <input type="text" placeholder="输入以搜索工具…">
      </div>
      <nav class="nav" id="nav"></nav>
      <div class="aside-foot" id="footNav"></div>
    </aside>
```

- [ ] **Step 3：在 IIFE 内填充导航数据 + 渲染逻辑**

把 `// 后续任务在此填充` 那行替换为：

```js
    // === NAV DATA ===
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
      { glyph: '⚙', label: '管理扩展' },
      { glyph: '☰', label: '设置' },
    ];

    const ICON_LINK = '<svg class="it-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10 14a5 5 0 007 0l3-3a5 5 0 00-7-7l-1 1"/><path d="M14 10a5 5 0 00-7 0l-3 3a5 5 0 007 7l1-1"/></svg>';
    const ICON_CHEV = '<svg class="chev" viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>';

    function itemHTML(it) {
      const left = it.icon === 'link' ? ICON_LINK
                 : it.glyph ? `<span class="glyph">${it.glyph}</span>`
                 : '<span></span>';
      const right = it.hasUpdate ? '<span class="bulb"></span>' : '';
      const cls = it.active ? 'item active' : 'item';
      return `<div class="${cls}">${left}<span>${it.label}</span>${right}</div>`;
    }

    function groupHTML(g) {
      const cls = g.expanded ? 'group' : 'group collapsed';
      const body = g.children.map(itemHTML).join('');
      return `<div class="${cls}">
        <div class="group-head">
          <span></span>
          <span>${g.label}</span>
          ${ICON_CHEV}
        </div>
        <div class="group-body">${body}</div>
      </div>`;
    }

    function renderNav() {
      const navEl = document.getElementById('nav');
      navEl.innerHTML = NAV.map(n => n.type === 'group' ? groupHTML(n) : itemHTML(n)).join('');
      // 分组折叠
      navEl.querySelectorAll('.group-head').forEach(h => {
        h.addEventListener('click', () => h.parentElement.classList.toggle('collapsed'));
      });
      // foot
      document.getElementById('footNav').innerHTML = FOOT.map(f =>
        `<div class="item"><span class="glyph">${f.glyph}</span><span>${f.label}</span><span></span></div>`
      ).join('');
    }
    renderNav();
```

- [ ] **Step 4：浏览器目测验证**

刷新 `prototype/index.html`，预期：
- 左侧最上方两个图标按钮（返回、折叠）
- 搜索框 placeholder「输入以搜索工具…」
- 导航从上到下依次：二维码（带琥珀色圆点）、URL（active 高亮）、测试工具（展开 3 项）、格式化工具（展开 3 项）、生成器（折叠）、图像处理（折叠）、文本处理（展开 3 项）
- 点「测试工具」分组头 → 折叠/展开切换，箭头旋转
- 「正则表达式」项右侧有琥珀色圆点
- 底部「管理扩展 / 设置」两项
- 控制台无报错

- [ ] **Step 5：提交**

```bash
git add fs-tauri/prototype/index.html
git commit -m "feat: 实现左侧导航（搜索 + 分组折叠 + foot）"
```

---

### Task 3: 主区静态 — page-head + section-title + 配置卡

**Files:**
- Modify: `fs-tauri/prototype/index.html`（追加 main 静态部分样式 + DOM）

**Interfaces:**
- Consumes: Task 1 的 `.main` 容器
- Produces:
  - DOM：`.page-head` / `.page-actions` / `.section-title` / `.config` / `.row` / `.row-icon` / `.row-title` / `.row-desc` / `.switch`
  - 关键 id：`#switchTransform`（input checkbox，默认 checked）、`#switchMultiline`（默认未 checked）

- [ ] **Step 1：在 `<style>` 末尾追加 main 静态样式**

```css
    /* === MAIN STATIC === */
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
    .ghost-btn {
      height: 32px; padding: 0 12px;
      display: inline-flex; align-items: center; gap: 6px;
      border-radius: 8px; color: var(--ink-2); font-size: 13px;
      transition: background .15s;
    }
    .ghost-btn:hover { background: var(--card); }
    .ghost-btn svg { width: 14px; height: 14px; }

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
    .row-icon svg { width: 18px; height: 18px; }
    .row-title { font-size: 14px; font-weight: 500; }
    .row-desc { font-size: 12.5px; color: var(--ink-3); margin-top: 2px; }
    .row-ctl { display: flex; align-items: center; gap: 8px; font-size: 12.5px; color: var(--ink-3); }

    /* switch */
    .switch {
      position: relative;
      width: 44px; height: 24px;
      border-radius: 999px;
      background: #d8d4cc;
      transition: background .15s;
      flex-shrink: 0;
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
```

- [ ] **Step 2：替换 `<main class="main"></main>` 为含静态内容的版本**

```html
    <main class="main">
      <header class="page-head">
        <h1>URL 编码 / 解码工具</h1>
        <div class="page-actions">
          <button class="ghost-btn" title="添加到收藏夹">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2l3 7h7l-5.5 4 2 7-6.5-5-6.5 5 2-7L2 9h7z"/></svg>
            <span>添加到收藏夹</span>
          </button>
          <button class="ghost-btn" title="弹出窗口" aria-label="弹出窗口">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 4h6v6"/><path d="M20 4l-9 9"/><path d="M9 4H5a1 1 0 00-1 1v14a1 1 0 001 1h14a1 1 0 001-1v-4"/></svg>
          </button>
        </div>
      </header>

      <div class="section-title"><span>配置</span></div>
      <div class="config">
        <div class="row">
          <span class="row-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M7 7h11l-3-3"/><path d="M17 17H6l3 3"/></svg>
          </span>
          <div>
            <div class="row-title">转换</div>
            <div class="row-desc">选择你要使用的转换模式</div>
          </div>
          <div class="row-ctl">
            <span id="labelTransform">编码</span>
            <label class="switch on" id="switchTransformWrap">
              <input type="checkbox" id="switchTransform" checked>
            </label>
          </div>
        </div>

        <div class="row">
          <span class="row-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M7 7h11l-3-3"/><path d="M17 17H6l3 3"/></svg>
          </span>
          <div>
            <div class="row-title">Encoding / Decoding Multiline</div>
            <div class="row-desc">Encode / Decode each line separately</div>
          </div>
          <div class="row-ctl">
            <span id="labelMultiline">关闭</span>
            <label class="switch" id="switchMultilineWrap">
              <input type="checkbox" id="switchMultiline">
            </label>
          </div>
        </div>
      </div>
    </main>
```

- [ ] **Step 3：在 IIFE 末尾添加开关视觉联动（仅切换 .on 类与文字标签，convert 留给 Task 5）**

紧接在 `renderNav();` 后：

```js
    // === SWITCH VISUAL ===
    function bindSwitch(inputId, wrapId, labelId, onText, offText) {
      const inp = document.getElementById(inputId);
      const wrap = document.getElementById(wrapId);
      const lbl = document.getElementById(labelId);
      const sync = () => {
        wrap.classList.toggle('on', inp.checked);
        lbl.textContent = inp.checked ? onText : offText;
      };
      wrap.addEventListener('click', e => {
        if (e.target.tagName !== 'INPUT') {
          inp.checked = !inp.checked;
          inp.dispatchEvent(new Event('change'));
        }
      });
      inp.addEventListener('change', sync);
      sync();
    }
    bindSwitch('switchTransform', 'switchTransformWrap', 'labelTransform', '编码', '解码');
    bindSwitch('switchMultiline', 'switchMultilineWrap', 'labelMultiline', '开启', '关闭');
```

- [ ] **Step 4：浏览器目测验证**

刷新页面，预期：
- 右上角 H1「URL 编码 / 解码工具」+ 右侧「☆ 添加到收藏夹」「↗ 弹窗」按钮
- 「配置」小标题下白色卡片含两行：
  - 第一行：转换 / 选择你要使用的转换模式 / 右侧「编码」+ 黑色开启的开关
  - 第二行：Encoding / Decoding Multiline / Encode / Decode each line separately / 右侧「关闭」+ 灰色关闭的开关
- 点击第一行开关 → 切换为「解码」+ 灰色 / 黑色循环
- 点击第二行开关 → 「关闭」↔「开启」切换
- 控制台无报错

- [ ] **Step 5：提交**

```bash
git add fs-tauri/prototype/index.html
git commit -m "feat: 实现页头 + 配置卡（含两个开关）"
```

---

### Task 4: 输入 / 输出区 DOM + gutter（无 convert 逻辑）

**Files:**
- Modify: `fs-tauri/prototype/index.html`

**Interfaces:**
- Consumes: Task 1 的 `.main` / token；Task 3 的 `.section-title` 样式
- Produces:
  - DOM：`.io.io-input` / `.io.io-output` / `.gutter` / `<textarea id="input">` / `<pre id="output">`
  - 工具按钮：粘贴 / 读文件 / 清空（`#btnClear`）/ 保存 / 复制（`#btnCopy`）/ 展开 / 灯泡
  - 函数：`updateGutter(el, lines)` —— 给定 gutter 元素与行数，把 `1\n2\n…\nN` 写入

- [ ] **Step 1：在 `<style>` 末尾追加 IO 样式**

```css
    /* === IO === */
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
    .io textarea, .io pre {
      padding: 12px 14px;
      font-family: var(--mono); font-size: 13px; line-height: 1.85;
      white-space: pre-wrap; word-break: break-all;
      width: 100%; height: 100%;
      min-height: 216px;
      color: var(--ink);
    }
    .io textarea { display: block; }
    /* pill buttons */
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
    .pill-btn svg { width: 14px; height: 14px; }
    .pill-btn.icon-only { width: 32px; padding: 0; justify-content: center; }

    /* toast */
    .toast {
      position: fixed;
      top: 24px; right: 24px;
      padding: 8px 14px;
      background: rgba(20,20,22,0.92);
      color: #fff;
      border-radius: 8px;
      font-size: 13px;
      opacity: 0; transform: translateY(-6px);
      transition: opacity .15s, transform .15s;
      pointer-events: none;
      z-index: 100;
    }
    .toast.show { opacity: 1; transform: translateY(0); }
```

- [ ] **Step 2：在 `</main>` 前追加输入 / 输出区**

紧接在配置卡 `</div>` 之后、`</main>` 之前：

```html
      <div class="section-title">
        <span>输入</span>
        <div class="section-actions">
          <button class="pill-btn" title="粘贴">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="3" width="6" height="4" rx="1"/><path d="M9 5H6a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-3"/></svg>
            <span>粘贴</span>
          </button>
          <button class="pill-btn icon-only" title="读取文件" aria-label="读取文件">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 3v5h5"/><path d="M14 3H6a2 2 0 00-2 2v14a2 2 0 002 2h12a2 2 0 002-2V8z"/></svg>
          </button>
          <button class="pill-btn icon-only" id="btnClear" title="清空" aria-label="清空">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 6l12 12M18 6L6 18"/></svg>
          </button>
        </div>
      </div>
      <div class="io io-input">
        <div class="gutter" id="gutterInput">1</div>
        <textarea id="input" spellcheck="false"></textarea>
      </div>

      <div class="section-title">
        <span>输出</span>
        <div class="section-actions">
          <button class="pill-btn icon-only" title="保存" aria-label="保存">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z"/><path d="M17 21v-8H7v8M7 3v5h8"/></svg>
          </button>
          <button class="pill-btn" id="btnCopy" title="复制">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>
            <span>复制</span>
          </button>
          <button class="pill-btn icon-only" title="展开" aria-label="展开">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7"/></svg>
          </button>
          <button class="pill-btn icon-only" title="预览模式" aria-label="预览模式">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 21h6M10 17a5 5 0 114 0v3h-4z"/></svg>
          </button>
        </div>
      </div>
      <div class="io io-output">
        <div class="gutter" id="gutterOutput">1</div>
        <pre id="output"></pre>
      </div>
    </main>
    <div class="toast" id="toast">已复制</div>
```

注意：`</main>` 前最后一个 IO 区结束后保留 `</main>`；`<div class="toast">` 移到 `</div></main>` 之后、`</body>` 前（即在 `<div class="window">` 闭合之外）。最终 body 结构：

```html
<body>
  <div class="window">
    <aside class="aside">…</aside>
    <main class="main">… …<div class="io io-output">…</div></main>
  </div>
  <div class="toast" id="toast">已复制</div>
  <script>…</script>
</body>
```

- [ ] **Step 3：在 IIFE 末尾添加 gutter 工具函数（暂不接 convert）**

```js
    // === GUTTER ===
    function updateGutter(el, lines) {
      const n = Math.max(1, lines | 0);
      const buf = new Array(n);
      for (let i = 0; i < n; i++) buf[i] = i + 1;
      el.textContent = buf.join('\n');
    }
```

- [ ] **Step 4：浏览器目测验证**

刷新页面，预期：
- 配置卡下方两个等高灰白边框的区域
- 输入区左侧 56px 浅灰 gutter 显示 `1`，右侧 textarea 可点击聚焦输入
- 输出区结构相同，gutter 显示 `1`，右侧 pre 为空
- 输入区右上角三个按钮：「📋 粘贴」+ 文件图标 + ✕
- 输出区右上角四个按钮：保存图标 + 「📋 复制」+ 展开图标 + 灯泡
- 控制台无报错（即使在 textarea 里打字，输出框还不会更新——这是正确的，convert 留给 Task 5）

- [ ] **Step 5：提交**

```bash
git add fs-tauri/prototype/index.html
git commit -m "feat: 添加输入 / 输出区 DOM 与 gutter 行号"
```

---

### Task 5: URL 核心交互 — convert + gutter 同步 + 开关联动

**Files:**
- Modify: `fs-tauri/prototype/index.html`（仅 IIFE 内添加 convert + 事件绑定）

**Interfaces:**
- Consumes: Task 3 的 `#switchTransform` `#switchMultiline`；Task 4 的 `#input` `#output` `#gutterInput` `#gutterOutput` 与 `updateGutter`
- Produces: 函数 `convert()`，挂在 `input` / `change` 事件上

- [ ] **Step 1：在 IIFE 末尾添加 convert + 事件绑定**

紧接在 `updateGutter` 函数之后：

```js
    // === URL CONVERT ===
    const inputEl = document.getElementById('input');
    const outputEl = document.getElementById('output');
    const gutterIn = document.getElementById('gutterInput');
    const gutterOut = document.getElementById('gutterOutput');
    const swTransform = document.getElementById('switchTransform');
    const swMultiline = document.getElementById('switchMultiline');

    function safeDecode(s) {
      try { return decodeURIComponent(s); } catch { return s; }
    }
    function convert() {
      const text = inputEl.value;
      const isEncode = swTransform.checked;
      const isMultiline = swMultiline.checked;
      const fn = isEncode ? encodeURIComponent : safeDecode;
      const result = isMultiline
        ? text.split('\n').map(fn).join('\n')
        : fn(text);
      outputEl.textContent = result;
      updateGutter(gutterIn,  text.split('\n').length);
      updateGutter(gutterOut, result.split('\n').length);
    }

    inputEl.addEventListener('input', convert);
    swTransform.addEventListener('change', convert);
    swMultiline.addEventListener('change', convert);
    convert();   // 初始化（空输入空输出，行号 = 1）
```

- [ ] **Step 2：浏览器目测验证（功能验收 #3 ~ #5）**

刷新页面：

| 操作 | 预期 |
|---|---|
| 输入 `hello world` | 输出立即显示 `hello%20world`，输入 gutter 显示 `1`，输出 gutter `1` |
| 把转换开关切到「解码」，清空输入后输入 `hello%20world` | 输出 `hello world` |
| 切回「编码」，开启 Multiline，输入第一行 `a b`，回车再输入 `c d` | 输出两行：`a%20b` 与 `c%20d`，输入/输出 gutter 都显示 `1\n2` |
| 关闭 Multiline，保持上述输入 | 输出整段被一次性编码（含换行的 `%0A`） |
| 控制台 | 无报错 |

- [ ] **Step 3：提交**

```bash
git add fs-tauri/prototype/index.html
git commit -m "feat: 接入 URL 编 / 解码核心逻辑与 gutter 同步"
```

---

### Task 6: 复制 + 清空 + toast + 最终验收

**Files:**
- Modify: `fs-tauri/prototype/index.html`（仅 IIFE 内添加按钮事件）

**Interfaces:**
- Consumes: Task 4 的 `#btnClear` `#btnCopy` `#toast`；Task 5 的 `convert()` `inputEl` `outputEl`
- Produces: 函数 `showToast(msg)`、`#btnClear` / `#btnCopy` 的 click 处理

- [ ] **Step 1：在 IIFE 末尾添加 toast + 清空 + 复制**

```js
    // === TOAST + ACTIONS ===
    const toastEl = document.getElementById('toast');
    let toastTimer = null;
    function showToast(msg) {
      toastEl.textContent = msg;
      toastEl.classList.add('show');
      if (toastTimer) clearTimeout(toastTimer);
      toastTimer = setTimeout(() => toastEl.classList.remove('show'), 1100);
    }

    document.getElementById('btnClear').addEventListener('click', () => {
      inputEl.value = '';
      convert();
      inputEl.focus();
    });

    document.getElementById('btnCopy').addEventListener('click', async () => {
      const text = outputEl.textContent;
      try {
        await navigator.clipboard.writeText(text);
        showToast('已复制');
      } catch {
        showToast('复制失败');
      }
    });
```

- [ ] **Step 2：浏览器目测验证（功能验收 #6 ~ #7）**

刷新页面：

| 操作 | 预期 |
|---|---|
| 输入 `hello`，点「清空」按钮 | 输入框、输出框都清空，gutter 重置为 `1`，光标聚焦回输入框 |
| 重新输入 `hello`，点输出区「复制」 | 右上角浮出「已复制」黑底白字 toast，1.1s 后淡出 |
| 切到其他应用 / 别处粘贴 | 系统剪贴板内容是 `hello%20world`（注意：当前若仍为编码模式则是 `hello`，按当前开关结果验） |

- [ ] **Step 3：完整验收（spec §十一 全部 10 条）**

按 spec 验收表逐条手动验证：

```bash
# 1. 文件行数检查
wc -l /Users/mengql/workspace/ClaudeCode/Front-Skeleton/fs-desktop/fs-tauri/prototype/index.html
# 预期：≤ 450
```

| # | 验证 |
|---|---|
| 1 | 浏览器打开 → DevTools Console 无报错 |
| 2 | 视觉与用户截图对照：两栏布局、URL active、配置卡、输入/输出等高 |
| 3 | 输入 `hello world` → 输出 `hello%20world` |
| 4 | 切「解码」+ 输入 `hello%20world` → 输出 `hello world` |
| 5 | Multiline + 编码 + 多行 → 各行独立编码 |
| 6 | 点「清空」→ 全部重置 |
| 7 | 点「复制」→ 剪贴板更新 + toast 1.1s |
| 8 | 点「JSONPath」等其他导航项 → 无响应、无报错 |
| 9 | 点「测试工具」分组头 → 折叠/展开 + 箭头旋转 |
| 10 | `wc -l` 输出 ≤ 450 |

如有任何一项不通过，回到对应任务修复后再走一次。

- [ ] **Step 4：最终提交**

```bash
git add fs-tauri/prototype/index.html
git commit -m "feat: 完成原型 — 复制 / 清空 / toast 与最终验收"
```

---

## Self-Review

**Spec coverage：**
- §四 文件结构：Task 1（建文件）+ §全局约束（不动其他文件）✓
- §五 布局：Task 1（window）+ Task 2（aside）+ Task 3-4（main 各区） ✓
- §六 视觉 token：Task 1（CSS variables 全量复制） ✓
- §七 导航数据：Task 2（NAV/FOOT 完整复制 + 渲染 + 折叠） ✓
- §八 URL 编/解码 + 默认状态：Task 3（开关默认值）+ Task 5（convert） ✓
- §九 行号 gutter：Task 4（updateGutter）+ Task 5（接入） ✓
- §十 实现约束：每任务 IIFE 内、内联、无外链 ✓
- §十一 验收 10 条：Task 6 Step 3 逐条对应 ✓

**Placeholder scan：** 无 TBD/TODO；每个 step 含具体代码或具体命令；无 "similar to" 引用。

**Type consistency：** 关键标识符（`#input` `#output` `#gutterInput` `#gutterOutput` `#switchTransform` `#switchMultiline` `#btnClear` `#btnCopy` `#toast` 与 `convert()` `safeDecode()` `updateGutter()` `showToast()` `bindSwitch()` `renderNav()` `itemHTML()` `groupHTML()`）跨任务一致。

**右尺寸检查：** 6 个任务，每个任务结束都能浏览器预览，可独立提交、独立 review；最大的 Task 2 含约 80 行 CSS + 50 行 JS，仍在合理范围。

---

## 执行方式选择

Plan complete and saved to `docs/superpowers/plans/2026-06-19-html-prototype.md`. Two execution options:

**1. Subagent-Driven (recommended)** - 每个任务派发一个全新 subagent，任务间审查，迭代快

**2. Inline Execution** - 在当前会话直接逐任务执行，到检查点暂停

哪种方式？
