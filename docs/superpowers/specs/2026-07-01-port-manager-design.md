# DevBox 端口管理工具设计稿

> 日期：2026-07-01
> 范围：在 DevBox 桌面应用新增「端口管理」工具页——查看本机 LISTEN 状态的 TCP 端口占用并结束占用进程
> 上一份 spec：`2026-06-19-tauri-vue3-skeleton-design.md`（Tauri + Vue3 桌面骨架，已实现）

---

## 一、背景与目标

### 背景

DevBox 已是「左侧导航 + 右侧工具页」的桌面工具集，导航数据 `src/stores/nav.ts` 的 `NAV_DATA` 在 `g-system`（系统工具）组下已预留 `{ id: 'port', glyph: '⚓', label: '端口管理' }`，目前命中通配路由 `/tools/:id` 落到 `PlaceholderView`（Coming Soon）。

开发场景下最高频的痛点是「端口被占，找出占用进程并处理」——手动 `lsof`/`netstat` + `kill` 链路长且跨平台命令不同。本次将其做成 DevBox 的一个真实工具页。

### 目标

- 新增 `/tools/port` 真实页面，列出本机 LISTEN 状态的 TCP 端口及占用进程（PID + 进程名 + 绑定地址）
- 提供「结束进程」能力：单按钮 → 确认对话框 → 优雅结束
- 跨平台（macOS / Linux / Windows），Rust 端无 `#[cfg]` 平台分支
- 沿用既有分层：Rust `tools/port.rs` → `lib.rs` 注册 → `src/api/port.ts` → `PortView.vue`

### 非目标（明确不做）

| 项 | 理由 |
|---|---|
| 自动轮询 / 实时刷新 | 仅手动刷新 + 杀后自动刷新，避免复杂状态（YAGNI） |
| UDP 端口、非 LISTEN 状态连接（ESTABLISHED / TIME_WAIT 等） | 开发场景聚焦 LISTEN 的 TCP，列表保持干净 |
| 强制结束（SIGKILL -9）独立按钮 | 单按钮 + 优雅结束覆盖绝大多数场景；卡死进程少见，后置 |
| 进程详情面板（CPU / 内存 / 命令行 / 启动时间） | 超出「端口管理」范围，属进程管理工具 |
| 按端口范围 / 协议族筛选 | 搜索框已够，不堆筛选器 |
| 前端单测、E2E | 沿用 spec 禁止项；Rust 单测 + 手动验证清单 |
| 仅 macOS | 已选跨平台 |

---

## 二、关键决策汇总

| 维度 | 选择 | 备注 |
|---|---|---|
| 端口/PID 获取 | `netstat2` crate | 跨平台结构化输出，避免解析 `lsof`/`netstat` 文本 |
| 进程名 + kill | `sysinfo` crate | 跨平台按 PID 查进程名、`Process::kill` 发 SIGTERM(Unix)/TerminateProcess(Win) |
| 平台分支 | Rust 端无 `#[cfg]` | 方案 B 的额外收益：两 crate 已抹平平台差异 |
| 数据范围 | 仅 LISTEN 的 TCP | 开发场景聚焦 |
| 结束方式 | 单按钮 + 确认 + 优雅结束 | `useDialog.warning` 确认 → `kill_port` |
| 列表呈现 | 原生 `<table>` + scoped CSS | 行数小、贴合原型风格；Naive UI 仅作容器 |
| 排序 / 搜索 | 纯前端 computed | 端口、PID 数值排序；进程名字符串排序 |
| `list_ports` 失败 | `Result<Vec<_>, String>`，前端 toast | 列表为空是正常态，不报错 |
| `kill_port` 失败 | `Result<(), String>`，前端 toast | 主动破坏性操作必须反馈原因 |
| 验收 | 手动验证清单 + Rust 单测 | 延续项目既有策略 |

---

## 三、Rust 端设计

### 依赖（`src-tauri/Cargo.toml`）

```toml
netstat2 = "2"     # 跨平台 TCP 连接列表 + 关联 PID
sysinfo = "0.32"   # 按 PID 查进程名 + Process::kill
```

### 数据结构

只保留有用字段，去掉恒为 TCP / LISTEN 的冗余列：

```rust
#[derive(serde::Serialize, Clone)]
pub struct PortEntry {
    pub port: u16,            // 监听端口，排序键
    pub pid: u32,
    pub process_name: String, // sysinfo 查不到时回退 "unknown"
    pub address: String,      // 绑定地址 "127.0.0.1" / "0.0.0.0" / "::1"
}
```

### 命令

| 命令 | 签名 | 说明 |
|------|------|------|
| `list_ports` | `() -> Result<Vec<PortEntry>, String>` | netstat2 取 TCP 连接，过滤 `State::Listen`，sysinfo 批量查进程名 |
| `kill_port` | `(pid: u32) -> Result<(), String>` | sysinfo `Process::kill()`（Unix=SIGTERM，Win=TerminateProcess） |

注册位置：`src-tauri/src/lib.rs` 的 `invoke_handler!` 数组追加两行，**不是** `main.rs`。

### 核心纯函数（为可测而抽）

把"从 netstat2 原始结构过滤 + 提取 + 查进程名"这一步抽成纯函数 `build_entries`，接收 netstat2 产出的 socket 列表与 sysinfo 进程表两个入参，返回 `Vec<PortEntry>`。`list_ports` 命令只负责调用 netstat2 / sysinfo 取数据并喂给它，错误处理走 `map_err`。

> 注：netstat2 的具体类型（`SocketInfo` / `AddressSocketInfo` / `ProtocolSocketInfo` / `State`）与 sysinfo 的 `Pid` / `Process` / `System` API 细节留给实现阶段按当时 crate 版本对齐，spec 不绑定具体签名，避免版本漂移导致 spec 失准。

```rust
fn build_entries(
    sockets: Vec</* netstat2 socket 结构 */>,
    processes: &sysinfo::System,
) -> Vec<PortEntry> {
    // 过滤 Listen + 提取 port/pid + 查进程名(查不到回退 "unknown") + 保留同端口多地址行
}

#[tauri::command]
pub fn list_ports() -> Result<Vec<PortEntry>, String> {
    let sockets = /* netstat2::get_sockets_info(...).map_err(|e| e.to_string())? */;
    let sys = sysinfo::System::new_all();
    Ok(build_entries(sockets, &sys))
}

#[tauri::command]
pub fn kill_port(pid: u32) -> Result<(), String> {
    let pid = sysinfo::Pid::from_u32(pid);
    match sysinfo::System::new_all().process(pid) {
        Some(proc) => proc.kill().then_some(()).ok_or_else(|| "结束失败".into()),
        None => Err(format!("进程 {} 不存在", pid)),
    }
}
```

### 关键约束

- netstat2 与 sysinfo 都跨平台，**Rust 代码无 `#[cfg]` 平台分支**。
- `list_ports` 每次新建 `sysinfo::System::new_all()`（刷新全进程表），保证进程名实时。开销可接受——手动刷新触发，非轮询。
- `kill_port` 在 Unix 发 SIGTERM（优雅）；Windows 是 `TerminateProcess`（强制，Windows 无 SIGTERM 语义，属平台固有限制，代码注释注明）。
- 进程名查不到（pid 已退出 / 跨用户权限不足）→ 回退 `"unknown"`，不阻塞列表。
- `kill_port` 走 `Result` 而非项目"返回原文"反原则：url_decode 是 watcher 自动调用需静默；kill 是用户主动点击的破坏性操作，**失败必须反馈原因**，与"剪贴板失败才 toast"同档。

### 同端口多地址处理

LISTEN 同一端口可能同时绑定 `0.0.0.0:8080` 和 `127.0.0.1:8080`（不同进程或同进程多绑）。**都保留**，各成一行——PID 可能不同，杀的是具体进程，去重会丢信息。排序时同端口自然相邻。

---

## 四、前端设计

### 新增 / 改动文件

| 文件 | 动作 | 说明 |
|------|------|------|
| `src/api/port.ts` | 新增 | invoke 封装 + `PortEntry` TS 类型 |
| `src/views/PortView.vue` | 新增 | 端口管理页 |
| `src/router/index.ts` | 改 | 加 `tools/port` 路由，置于 `tools/:id` **之前**（否则被通配捕获） |
| `src/layouts/AppShell.vue` | 改 | 加 `<n-dialog-provider>`（杀进程确认框需要） |

### api 层

```ts
import { invoke } from '@tauri-apps/api/core'

export type PortEntry = {
  port: number
  pid: number
  process_name: string
  address: string
}

export const portApi = {
  list: () => invoke<PortEntry[]>('list_ports'),
  kill: (pid: number) => invoke<void>('kill_port', { pid }),
}
```

### 页面布局

原生 `<table>` + scoped CSS（与项目"原型自定义元件用 scoped CSS"一致；端口列表通常几十行，不需虚拟滚动）：

```
┌─────────────────────────────────────────────────┐
│ 端口管理        [搜索框........]  [刷新]         │
├──────────┬───────┬──────────────┬────────┬──────┤
│ 端口 ⇅   │ PID ⇅ │ 进程名       │ 地址   │ 操作 │
├──────────┼───────┼──────────────┼────────┼──────┤
│ 8080     │ 12345 │ node         │ 0.0.0.0│ [结束]│
│ 5173     │ 12346 │ node         │ 127.0.0.1│[结束]│
└──────────┴───────┴──────────────┴────────┴──────┘
```

### 交互

- **搜索框**：输入即筛，匹配 `port` / `pid` / `process_name` 三列（不区分大小写）。
- **列排序**：点击列头 `⇅` 切换 asc/desc；端口与 PID 按数值排，进程名按字符串排；同一 `sortKey + sortDir` ref 驱动 computed。
- **刷新按钮**：重新 `invoke list_ports`，期间按钮 disabled + 文案变「刷新中」。
- **结束按钮**：`useDialog().warning` 弹确认（`结束 PID 12345 (node)?`），确认后 `invoke kill_port`：
  - 成功 → `useMessage.success('已结束')` + 重新 `list_ports`（杀后自动刷新）
  - 失败 → `useMessage.error(返回的 String)`（权限不足 / 进程已退出）

### 选型理由

- **表格用原生 `<table>` 而非 `n-data-table`**：CLAUDE.md 限定 Naive UI 仅作容器；表格是新增原型元件，scoped CSS 更贴合现有视觉（`tokens.css`）；行数小，内置排序/筛选无必要。
- **确认框用 `useDialog`**：比手写 modal 省，且与已有 `useMessage` 风格统一；`n-dialog-provider` 是容器性质，不违反"不用 n-button 替换原型组件"。

---

## 五、错误处理

分场景，延续项目"三套策略各得其所"：

| 场景 | 策略 | 用户感知 |
|------|------|---------|
| `list_ports` 整体失败 | `Result<Vec,_>`，前端 catch | toast 错误 + 列表空状态 |
| 列表为空 | 正常返回 `vec![]` | 表格区显示「暂无监听端口」 |
| 进程名查不到（pid 已退出 / 跨用户） | 回退 `"unknown"`，不报错 | 该行进程名显示 unknown |
| `kill_port` 失败（权限不足 / 进程已没） | `Result<(), String>` | toast 返回的错误串 |
| kill 成功但随后刷新失败 | toast「已结束,刷新失败」 | 进程确实已杀，列表可能短暂滞后 |
| 排序 / 搜索 | 纯前端 computed，无错误路径 | — |

---

## 六、测试与验收

### Rust 单测（`port.rs` 的 `#[cfg(test)]`）

延续项目"Rust 单测有、前端无"策略，对纯函数 `build_entries` 打桩测试：

- `build_entries_filters_non_listen` — 喂含 ESTABLISHED + LISTEN 的假 `AddressSocketInfo`，断言只留 LISTEN。
- `build_entries_unknown_process_name` — 假 pid 不在进程表 → 回退 `"unknown"`。
- `build_entries_keeps_same_port_multi_address` — 同端口 `0.0.0.0:8080` + `127.0.0.1:8080`（不同 PID）→ 两行都保留。

`kill_port` / `list_ports` 整体依赖真实系统，不单测，靠手动验证。

### 手动验证清单（开发自查，写进 plan）

1. `python3 -m http.server 8080` 起监听 → 列表出现 8080。
2. 搜索 `8080` / `python` 筛选生效。
3. 点端口列头排序 asc/desc 正确（数值序）。
4. 点「结束」→ 确认 → 进程消失 + toast「已结束」。
5. 结束一个已退出的 pid → toast 错误。
6. 无任何监听端口时 → 空状态文案「暂无监听端口」。

---

## 七、与既有架构的契合

- **单向依赖**：`PortView` → `api/port` → invoke → `tools/port.rs`；`components/ui` 不感知此页，`stores/nav` 不变（导航项已存在）。
- **样式三层**：表格用 scoped CSS 引用 `tokens.css` 变量，不新增 Tailwind 暴露。
- **Tauri 2 模板**：命令注册在 `lib.rs` 的 `invoke_handler!`，非 `main.rs`。
- **错误反原则**：Rust `list_ports`/`kill_port` 失败用 `Result` 上抛（与 `url_decode` 返回原文不同——后者是自动 watcher 需静默，此处是用户主动操作需反馈）。
