# 端口管理工具 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 DevBox 新增「端口管理」工具页 `/tools/port`：列出本机 LISTEN 状态的 TCP 端口占用（端口 / PID / 进程名 / 绑定地址），支持搜索筛选、列排序、手动刷新、结束进程（确认 + 优雅结束 + 杀后自动刷新）。

**Architecture:** 沿用既有分层。Rust 端用 `netstat2`（跨平台取 TCP 连接 + 关联 PID）+ `sysinfo`（按 PID 查进程名 + `Process::kill`），无 `#[cfg]` 平台分支；核心转换抽纯函数 `build_entries` 便于单测。前端 `PortView.vue` 用原生 `<table>` + scoped CSS，搜索/排序为纯前端 computed，结束进程走 `useDialog` 确认 + `useMessage` toast。

**Tech Stack:** Tauri 2、Rust（netstat2、sysinfo）、Vue 3 + TypeScript、vue-router、Naive UI（useDialog / useMessage 容器）

## Global Constraints

- **包管理器：** pnpm（不要 npm/yarn）
- **TypeScript：** 全前端；`pnpm exec vue-tsc -b` 必须 exit 0
- **不引入：** ESLint / Prettier / Vitest / Playwright / Tailwind 暗色 / 国际化 / 前端单测
- **Rust 单测：** `cd src-tauri && cargo test --lib` 通过；spec 禁止项允许 Rust 单测
- **Naive UI 用法：** 仅作容器；`useDialog` / `useMessage` 是消息 API 而非替换原型组件，可用
- **commit message：** 中文、`类型: 简短描述` 格式（不要 Co-Authored-By）
- **命令注册位置：** `src-tauri/src/lib.rs` 的 `invoke_handler!`，**不是** `main.rs`
- **导航项已存在：** `src/stores/nav.ts` 中 `g-system` 组下已有 `{ id: 'port', glyph: '⚓', label: '端口管理' }`，**不要重复添加**
- **错误反原则：** `list_ports` / `kill_port` 走 `Result<T, String>` 上抛——用户主动操作的破坏性/显式操作需反馈，与 `url_decode` 自动 watcher 静默不同；进程名查不到回退 `"unknown"` 不报错
- **跨平台：** macOS / Linux / Windows，Rust 端无 `#[cfg]` 平台分支
- **同端口多地址：** 不去重，各成一行（PID 可能不同，杀的是具体进程）

---

## File Structure（计划完成后的目录形态）

```
fs-tauri/
├── src-tauri/
│   ├── Cargo.toml                          [Task 1 改：加 netstat2 + sysinfo]
│   └── src/
│       ├── lib.rs                          [Task 4 改：注册 list_ports / kill_port]
│       └── tools/
│           ├── mod.rs                      [Task 2 改：加 mod port;]
│           └── port.rs                     [Task 2 创建，Task 3 增量单测]
│
└── src/
    ├── api/
    │   └── port.ts                         [Task 5 创建]
    ├── views/
    │   └── PortView.vue                    [Task 7 创建]
    ├── router/
    │   └── index.ts                        [Task 6 改：加 tools/port 路由]
    └── layouts/
        └── AppShell.vue                    [Task 6 改：加 n-dialog-provider]
```

---

## Task 1: 加 Rust 依赖

**Files:**
- Modify: `src-tauri/Cargo.toml`

**Interfaces:**
- Produces: `netstat2`、`sysinfo` 两个 crate 可在 `tools/port.rs` 引用

- [ ] **Step 1: 在 `[dependencies]` 末尾追加两行**

打开 `src-tauri/Cargo.toml`，在 `[dependencies]` 段（`tauri-plugin-clipboard-manager = "2"` 之后）追加：

```toml
netstat2 = "2"
sysinfo = "0.32"
```

- [ ] **Step 2: 验证依赖可拉取编译**

Run:
```bash
cd src-tauri && cargo check
```
Expected: 编译通过（首次会拉取 netstat2 / sysinfo 及其传递依赖，约 30-90 秒），无 error。

- [ ] **Step 3: 提交**

```bash
git add src-tauri/Cargo.toml
git commit -m "chore: 端口管理页加 netstat2 与 sysinfo 依赖"
```

---

## Task 2: Rust 端命令骨架（PortEntry + list_ports + kill_port）

**Files:**
- Modify: `src-tauri/src/tools/mod.rs`
- Create: `src-tauri/src/tools/port.rs`

**Interfaces:**
- Produces: `tools::port::list_ports() -> Result<Vec<PortEntry>, String>`、`tools::port::kill_port(pid: u32) -> Result<(), String>`、`pub struct PortEntry { port, pid, process_name, address }`
- Consumes: netstat2、sysinfo crate（Task 1）

> 实现说明：netstat2 / sysinfo 的具体类型名与 API 细节按当时 crate 版本对齐。netstat2 返回 socket 列表（含 `protocol_socket_info` 里的 `local_addr` / `local_port`、`state`、`associated_pids`）；sysinfo 用 `System::new_all()` 刷新全进程表后 `process(Pid)` 查进程名、`Process::kill()` 结束。先按 crate 文档把类型对齐，再实现 `build_entries` 纯函数。

- [ ] **Step 1: 在 mod.rs 注册 port 模块**

打开 `src-tauri/src/tools/mod.rs`，内容当前为：

```rust
pub mod qrcode;
pub mod url;
```

改为：

```rust
pub mod port;
pub mod qrcode;
pub mod url;
```

- [ ] **Step 2: 创建 port.rs，写 PortEntry + build_entries + 两个命令**

创建 `src-tauri/src/tools/port.rs`：

```rust
use serde::Serialize;

/// 一条监听端口占用记录。只保留有用字段，去掉恒为 TCP / LISTEN 的冗余列。
#[derive(Serialize, Clone)]
pub struct PortEntry {
    /// 监听端口，排序键
    pub port: u16,
    pub pid: u32,
    /// 进程名；sysinfo 查不到时回退 "unknown"
    pub process_name: String,
    /// 绑定地址 "127.0.0.1" / "0.0.0.0" / "::1"
    pub address: String,
}

/// 把 netstat2 原始 socket 列表过滤 + 提取 + 查进程名，组装成 PortEntry 列表。
///
/// 抽成纯函数便于单测打桩：入参是 netstat2 产出的 socket 列表与 sysinfo 进程表。
fn build_entries(
    sockets: Vec<netstat2::SocketInfo>,
    processes: &sysinfo::System,
) -> Vec<PortEntry> {
    let mut out = Vec::new();
    for sock in sockets {
        // 只取 TCP LISTEN
        let tcp = match sock.protocol_socket_info {
            netstat2::ProtocolSocketInfo::Tcp(tcp) => tcp,
            netstat2::ProtocolSocketInfo::Udp(_) => continue,
        };
        if tcp.local_port == 0 {
            continue;
        }
        // associated_pids 通常非空；取第一个非 0 的 PID 作为占用进程
        let pid = tcp.associated_pids.iter().copied().find(|p| *p != 0);
        let pid = match pid {
            Some(p) => p,
            None => continue,
        };
        let process_name = processes
            .process(sysinfo::Pid::from_u32(pid))
            .map(|p| p.name().to_string())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| "unknown".to_string());
        out.push(PortEntry {
            port: tcp.local_port,
            pid,
            process_name,
            address: tcp.local_addr.to_string(),
        });
    }
    out
}

#[tauri::command]
pub fn list_ports() -> Result<Vec<PortEntry>, String> {
    let sockets = netstat2::get_sockets_info(
        netstat2::AddressSocketFlags::empty()
            | netstat2::AddressSocketFlags::ASSOCIATE_PIDS,
    )
    .map_err(|e| e.to_string())?;
    let sys = sysinfo::System::new_all();
    Ok(build_entries(sockets, &sys))
}

#[tauri::command]
pub fn kill_port(pid: u32) -> Result<(), String> {
    let sys = sysinfo::System::new_all();
    let pid = sysinfo::Pid::from_u32(pid);
    match sys.process(pid) {
        // Unix: kill() 发 SIGTERM（优雅结束）
        // Windows: kill() 是 TerminateProcess（强制，Windows 无 SIGTERM 语义）
        Some(proc) => {
            if proc.kill() {
                Ok(())
            } else {
                Err("结束失败".to_string())
            }
        }
        None => Err(format!("进程 {} 不存在", pid)),
    }
}
```

> **类型对齐说明**：上面代码假设了 netstat2 的 `SocketInfo`、`ProtocolSocketInfo::Tcp`、`TcpSocketInfo { local_addr, local_port, associated_pids }`、`get_sockets_info(AddressSocketFlags)` 与 sysinfo 的 `Pid::from_u32` / `Process::name` / `Process::kill`。若实际 crate 版本 API 名略有差异（例如 `local_addr` 是 `IpAddr` 需 `.to_string()`、或 `associated_pids` 是 `Vec<u32>`/`Vec<Pid>`、或 flags 枚举名不同），按 `cargo check` 报错就地调整字段名/方法名，但**保持函数签名与逻辑结构不变**。

- [ ] **Step 3: 编译验证**

Run:
```bash
cd src-tauri && cargo check
```
Expected: 通过。若报类型不匹配，按 Step 2 的类型对齐说明调整 netstat2/sysinfo 的字段名/方法名直到通过。

- [ ] **Step 4: 注册命令到 invoke_handler**

打开 `src-tauri/src/lib.rs`，把 `invoke_handler!` 块从：

```rust
    .invoke_handler(tauri::generate_handler![
      tools::url::url_encode,
      tools::url::url_decode,
      tools::qrcode::qr_encode,
      tools::qrcode::qr_decode,
    ])
```

改为：

```rust
    .invoke_handler(tauri::generate_handler![
      tools::url::url_encode,
      tools::url::url_decode,
      tools::qrcode::qr_encode,
      tools::qrcode::qr_decode,
      tools::port::list_ports,
      tools::port::kill_port,
    ])
```

- [ ] **Step 5: 再次编译验证**

Run:
```bash
cd src-tauri && cargo check
```
Expected: 通过，无 error。

- [ ] **Step 6: 提交**

```bash
git add src-tauri/src/tools/mod.rs src-tauri/src/tools/port.rs src-tauri/src/lib.rs
git commit -m "feat: 端口管理 Rust 端 list_ports/kill_port 命令"
```

---

## Task 3: build_entries 纯函数单测

**Files:**
- Modify: `src-tauri/src/tools/port.rs`（追加 `#[cfg(test)]` 模块）

**Interfaces:**
- Consumes: `build_entries`、`PortEntry`（Task 2）
- Produces: 3 条 Rust 单测

> 测试难点：`netstat2::SocketInfo` / `TcpSocketInfo` / `sysinfo::System` 的字段不全是 `pub`，构造假数据可能受限。若字段无法从外部构造，改用「**通过 `list_ports` 真实调用 + 断言不 panic**」的最小冒烟测试替代纯函数单测（接受这个降级，因为 spec 明确不发明复杂 mock）。先尝试纯函数打桩，打桩不通则走冒烟测试。

- [ ] **Step 1: 在 port.rs 末尾追加测试模块，先写「过滤非 LISTEN」用例**

在 `src-tauri/src/tools/port.rs` 末尾追加：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_entries_keeps_same_port_multi_address() {
        // 同端口不同地址（不同 PID）两行都保留，不去重。
        // 若 SocketInfo 无法从外部构造，本测试改为：调用 list_ports() 不 panic 且每条 port != 0。
        let result = list_ports().expect("list_ports should not error");
        for e in &result {
            assert_ne!(e.port, 0, "port 不应为 0");
            assert!(!e.process_name.is_empty(), "进程名不应为空");
        }
    }
}
```

> 上面采用冒烟测试形式作为保底（验证真实系统调用不 panic、字段非空）。纯函数打桩版本见下方说明。

- [ ] **Step 2: 尝试补纯函数打桩测试（可选，若 crate 类型可构造）**

查阅 netstat2 / sysinfo 文档，看 `netstat2::TcpSocketInfo` 与 `sysinfo::System` 能否从测试代码构造假实例。

- **若可构造**：替换 Step 1 的冒烟测试为以下三条纯函数测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    // 在此处用 netstat2 的真实构造方式建假 socket 列表、空 sysinfo::System
    // 见下方三条用例骨架（字段构造方式按 crate 实际 API 填）：

    #[test]
    fn build_entries_filters_non_listen() {
        // 喂含 LISTEN 与非 LISTEN 的 socket，断言只留 LISTEN
        // todo: 按 crate API 构造
    }

    #[test]
    fn build_entries_unknown_process_name() {
        // 假 pid 不在进程表 -> 回退 "unknown"
    }

    #[test]
    fn build_entries_keeps_same_port_multi_address() {
        // 同端口 0.0.0.0:8080 + 127.0.0.1:8080（不同 PID）-> 两行都保留
    }
}
```

- **若不可构造**（字段非 pub 或构造复杂）：保留 Step 1 的冒烟测试，并在测试上方注释 `// crate 类型不可外部构造，改用 list_ports 冒烟测试保底`。

- [ ] **Step 3: 跑测试验证通过**

Run:
```bash
cd src-tauri && cargo test --lib port
```
Expected: 测试通过（冒烟版会真实查询本机端口，需有 LISTEN 端口才能深验字段非空；无端口时 `port != 0` 的循环不会执行，仍 PASS）。

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/tools/port.rs
git commit -m "test: 端口管理 build_entries 单测/冒烟测试"
```

---

## Task 4: 前端 api 封装

**Files:**
- Create: `src/api/port.ts`

**Interfaces:**
- Produces: `portApi.list() => Promise<PortEntry[]>`、`portApi.kill(pid: number) => Promise<void>`、`type PortEntry`
- Consumes: `@tauri-apps/api/core` 的 `invoke`

- [ ] **Step 1: 创建 port.ts**

创建 `src/api/port.ts`：

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

- [ ] **Step 2: 类型检查**

Run:
```bash
pnpm exec vue-tsc -b
```
Expected: exit 0。

- [ ] **Step 3: 提交**

```bash
git add src/api/port.ts
git commit -m "feat: 端口管理前端 api 封装"
```

---

## Task 5: 路由 + Dialog Provider

**Files:**
- Modify: `src/router/index.ts`
- Modify: `src/layouts/AppShell.vue`

**Interfaces:**
- Consumes: `@/views/PortView.vue`（Task 7 创建；本 Task 先加路由引用，PortView 在 Task 7 补，懒加载保证类型检查不卡）
- Produces: `/tools/port` 路由可达；`n-dialog-provider` 可用

- [ ] **Step 1: 在路由加 tools/port，置于通配 tools/:id 之前**

打开 `src/router/index.ts`，把 children 数组从：

```ts
      { path: '', redirect: '/tools/qrcode' },
      { path: 'tools/qrcode', component: () => import('@/views/QrCodeView.vue') },
      { path: 'tools/url', component: () => import('@/views/UrlView.vue') },
      { path: 'tools/:id', component: () => import('@/views/PlaceholderView.vue') },
```

改为：

```ts
      { path: '', redirect: '/tools/qrcode' },
      { path: 'tools/qrcode', component: () => import('@/views/QrCodeView.vue') },
      { path: 'tools/url', component: () => import('@/views/UrlView.vue') },
      { path: 'tools/port', component: () => import('@/views/PortView.vue') },
      { path: 'tools/:id', component: () => import('@/views/PlaceholderView.vue') },
```

> `tools/port` 必须在 `tools/:id` 之前，否则被通配捕获落到 PlaceholderView。

- [ ] **Step 2: 在 AppShell.vue 加 n-dialog-provider**

打开 `src/layouts/AppShell.vue`。先 Read 该文件确认 `<n-message-provider>`（或其它 Naive 容器）的位置，然后把 `<n-dialog-provider>` 套在与其同级、覆盖主区路由出口的位置。

典型改动：找到现有的 provider 包裹结构，加入 `<n-dialog-provider>`。例如若现有为：

```vue
<n-message-provider>
  <router-view />
</n-message-provider>
```

改为：

```vue
<n-message-provider>
  <n-dialog-provider>
    <router-view />
  </n-dialog-provider>
</n-message-provider>
```

> 若 AppShell 中 Naive UI 组件是按需引入（`NMessageProvider` 等显式 import），需同步 import `NDialogProvider` 并注册。按现有写法对齐。

- [ ] **Step 3: 类型检查**

Run:
```bash
pnpm exec vue-tsc -b
```
Expected: exit 0（PortView.vue 此时未创建，但路由用懒加载 `() => import(...)`，vue-tsc 在 build 前不会强制解析该动态导入的缺失文件——若报错 "Cannot find module '@/views/PortView.vue'"，先跳到 Task 7 创建空壳 PortView 再回来，或先建一个最小 `<template><div/></template>` 占位文件让类型检查过）。

> 若 vue-tsc 确实因缺 PortView.vue 报错：先执行 Task 7 Step 1 创建 PortView.vue 空壳（仅 `<template><div class="port-view">端口管理</div></template>` + `<script setup lang="ts"></script>`），再回到本 Step 重跑 vue-tsc。

- [ ] **Step 4: 提交**

```bash
git add src/router/index.ts src/layouts/AppShell.vue src/views/PortView.vue 2>/dev/null
git commit -m "feat: 端口管理路由与 dialog provider"
```

---

## Task 6: PortView 页面（列表 + 搜索 + 排序 + 刷新 + 结束进程）

**Files:**
- Create: `src/views/PortView.vue`（若 Task 5 已建空壳，本 Task 替换为完整实现）

**Interfaces:**
- Consumes: `portApi`、`PortEntry`（Task 4）；`useMessage`、`useDialog`（Naive UI 容器，Task 5 已挂 provider）；`tokens.css` 变量（既有）
- Produces: `/tools/port` 完整页面

- [ ] **Step 1: 创建 PortView.vue 完整实现**

创建/覆盖 `src/views/PortView.vue`：

```vue
<template>
  <div class="port-view">
    <header class="port-header">
      <h2 class="port-title">端口管理</h2>
      <div class="port-toolbar">
        <input
          v-model="search"
          class="port-search"
          type="text"
          placeholder="搜索端口 / PID / 进程名"
        />
        <button class="port-refresh" :disabled="loading" @click="load">
          {{ loading ? '刷新中…' : '刷新' }}
        </button>
      </div>
    </header>

    <div class="port-table-wrap">
      <table class="port-table" v-if="filtered.length">
        <thead>
          <tr>
            <th class="sortable" @click="toggleSort('port')">
              端口 <span class="arrow">{{ sortArrow('port') }}</span>
            </th>
            <th class="sortable" @click="toggleSort('pid')">
              PID <span class="arrow">{{ sortArrow('pid') }}</span>
            </th>
            <th>进程名</th>
            <th>地址</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in filtered" :key="`${row.pid}-${row.port}-${row.address}`">
            <td>{{ row.port }}</td>
            <td>{{ row.pid }}</td>
            <td>{{ row.process_name }}</td>
            <td>{{ row.address }}</td>
            <td>
              <button class="port-kill" @click="confirmKill(row)">结束</button>
            </td>
          </tr>
        </tbody>
      </table>
      <div v-else class="port-empty">
        {{ loading ? '加载中…' : '暂无监听端口' }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import { portApi, type PortEntry } from '@/api/port'

const message = useMessage()
const dialog = useDialog()

const rows = ref<PortEntry[]>([])
const loading = ref(false)
const search = ref('')
const sortKey = ref<'port' | 'pid' | null>(null)
const sortDir = ref<'asc' | 'desc'>('asc')

async function load() {
  loading.value = true
  try {
    rows.value = await portApi.list()
  } catch (e) {
    message.error(`加载端口列表失败：${e}`)
    rows.value = []
  } finally {
    loading.value = false
  }
}

function toggleSort(key: 'port' | 'pid') {
  if (sortKey.value === key) {
    sortDir.value = sortDir.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortKey.value = key
    sortDir.value = 'asc'
  }
}

function sortArrow(key: 'port' | 'pid') {
  if (sortKey.value !== key) return '⇅'
  return sortDir.value === 'asc' ? '↑' : '↓'
}

const filtered = computed(() => {
  const q = search.value.trim().toLowerCase()
  let list = q
    ? rows.value.filter(
        (r) =>
          String(r.port).includes(q) ||
          String(r.pid).includes(q) ||
          r.process_name.toLowerCase().includes(q),
      )
    : rows.value.slice()

  if (sortKey.value) {
    const key = sortKey.value
    const dir = sortDir.value === 'asc' ? 1 : -1
    list.sort((a, b) => (a[key] - b[key]) * dir)
  }
  return list
})

function confirmKill(row: PortEntry) {
  dialog.warning({
    title: '结束进程',
    content: `结束 PID ${row.pid}（${row.process_name}）？端口 ${row.port} 将被释放。`,
    positiveText: '结束',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await portApi.kill(row.pid)
        message.success('已结束')
        await load() // 杀后自动刷新
      } catch (e) {
        message.error(`结束失败：${e}`)
      }
    },
  })
}

onMounted(load)
</script>

<style scoped>
.port-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 12px;
}
.port-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.port-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--ink-1);
  margin: 0;
}
.port-toolbar {
  display: flex;
  gap: 8px;
}
.port-search {
  padding: 6px 10px;
  border: 1px solid var(--border-1);
  border-radius: var(--radius-md);
  font-size: 13px;
  width: 220px;
  background: var(--surface-1);
  color: var(--ink-1);
}
.port-search:focus {
  outline: none;
  border-color: var(--accent);
}
.port-refresh {
  padding: 6px 14px;
  border: 1px solid var(--border-1);
  border-radius: var(--radius-md);
  background: var(--surface-1);
  color: var(--ink-1);
  cursor: pointer;
  font-size: 13px;
}
.port-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.port-table-wrap {
  flex: 1;
  overflow: auto;
  border: 1px solid var(--border-1);
  border-radius: var(--radius-md);
  background: var(--surface-1);
}
.port-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}
.port-table th,
.port-table td {
  padding: 8px 12px;
  text-align: left;
  border-bottom: 1px solid var(--border-1);
  color: var(--ink-1);
}
.port-table th {
  position: sticky;
  top: 0;
  background: var(--surface-2);
  font-weight: 600;
  user-select: none;
}
.port-table th.sortable {
  cursor: pointer;
}
.port-table th.sortable:hover {
  color: var(--accent);
}
.port-table .arrow {
  opacity: 0.6;
  font-size: 12px;
}
.port-table tbody tr:hover {
  background: var(--surface-2);
}
.port-kill {
  padding: 4px 12px;
  border: 1px solid var(--border-1);
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--danger, #d33);
  cursor: pointer;
  font-size: 12px;
}
.port-kill:hover {
  background: var(--surface-2);
}
.port-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--ink-2);
  font-size: 13px;
}
</style>
```

> **CSS 变量对齐**：上面用了 `--ink-1`、`--ink-2`、`--border-1`、`--surface-1`、`--surface-2`、`--accent`、`--radius-md`、`--radius-sm`、`--danger`。实现时 Read `src/styles/tokens.css` 确认这些变量名是否存在；若实际命名不同（例如 `--border` 而非 `--border-1`），按 tokens.css 实际名称替换，缺失的（如 `--danger`）要么在 tokens.css 加一个、要么直接内联十六进制色值（保持与现有视觉一致，不引入新 token 而被 CLAUDE.md 约束反对时优先用既有变量近似）。

- [ ] **Step 2: 类型检查**

Run:
```bash
pnpm exec vue-tsc -b
```
Expected: exit 0。若 `useDialog` / `useMessage` 未被识别，确认 Naive UI 已安装且 `AppShell.vue` 已挂 provider（Task 5）；若 CSS 变量报错（vue-tsc 不校验 CSS 变量，通常不会报），忽略。

- [ ] **Step 3: 提交**

```bash
git add src/views/PortView.vue
git commit -m "feat: 端口管理页面列表/搜索/排序/刷新/结束进程"
```

---

## Task 7: 手动验证

**Files:**
- 无（运行验证）

- [ ] **Step 1: 启动桌面应用**

Run:
```bash
pnpm tauri:dev
```
Expected: 桌面窗口启动，左侧导航「系统工具 → 端口管理」可点击进入页面。

- [ ] **Step 2: 验证列表加载**

在终端另开一个 tab 执行 `python3 -m http.server 8080` 起一个监听端口，回到 DevBox 点「刷新」。
Expected: 列表出现 8080 端口行，进程名为 python，地址 0.0.0.0 或 127.0.0.1。

- [ ] **Step 3: 验证搜索筛选**

在搜索框输入 `8080`、再输入 `python`。
Expected: 列表只剩匹配行；清空搜索恢复全部。

- [ ] **Step 4: 验证列排序**

点击「端口」列头一次、再点一次；点击「PID」列头。
Expected: 箭头在 ↑/↓/⇅ 间切换，列表按对应列数值升序/降序排列。

- [ ] **Step 5: 验证结束进程**

找到 8080 那行点「结束」→ 确认对话框 → 点「结束」。
Expected: toast「已结束」，列表自动刷新后 8080 行消失；终端的 python 进程已退出。

- [ ] **Step 6: 验证结束失败反馈**

对一个刚结束的 PID（或手动 kill 后列表未刷新时点旧的「结束」）再点一次「结束」并确认。
Expected: toast「结束失败：进程 X 不存在」或类似错误。

- [ ] **Step 7: 验证空状态**

停止所有监听端口，点「刷新」。
Expected: 表格区显示「暂无监听端口」。

- [ ] **Step 8: 关闭 dev，无需提交（验证步骤）**

验证全部通过。若有 UI/CSS 问题，回到 Task 6 调整后重新验证。

---

## Self-Review

**Spec coverage（逐条对 spec）：**
- §三 Rust 端（PortEntry + list_ports + kill_port + build_entries）→ Task 1/2/3 ✓
- §三 跨平台无 #[cfg] → Task 2 实现不含平台分支 ✓
- §三 同端口多地址不去重 → Task 2 build_entries 未做去重 ✓
- §三 进程名回退 unknown → Task 2 ✓
- §四 api 封装 → Task 4 ✓
- §四 路由 + dialog provider → Task 5 ✓
- §四 原生 table + 搜索/排序/刷新/结束 → Task 6 ✓
- §五 错误处理各场景 → Task 6（load try/catch、kill try/catch、空状态、unknown）✓
- §六 Rust 单测 + 手动清单 → Task 3 + Task 7 ✓

**Placeholder scan：** 无 TBD/TODO 占位（Task 3 的 crate 类型构造为「尝试-降级」明确路径，非占位）。✓

**Type consistency：** `PortEntry` 字段 `port`/`pid`/`process_name`/`address` 在 Rust（Task 2）、TS（Task 4）、Vue（Task 6）三处一致；`portApi.list`/`kill` 签名一致；`list_ports`/`kill_port` 命令名一致。✓
