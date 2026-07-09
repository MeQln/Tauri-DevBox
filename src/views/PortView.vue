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
  --port-rule: var(--border-accent);
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
  color: var(--ink);
  margin: 0;
}
.port-toolbar {
  display: flex;
  gap: 8px;
}
.port-search {
  padding: 6px 10px;
  border: 1px solid var(--port-rule);
  border-radius: var(--r-md);
  font-size: 13px;
  width: 220px;
  background: transparent;
  color: var(--ink);
}
.port-search:focus {
  outline: none;
  border-color: var(--aside-3);
}
.port-refresh {
  padding: 6px 14px;
  border: 1px solid var(--port-rule);
  border-radius: var(--r-md);
  background: transparent;
  color: var(--ink);
  cursor: pointer;
  font-size: 13px;
  transition: background 0.15s;
}
.port-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.port-refresh:not(:disabled):hover {
  background: color-mix(in srgb, var(--aside-2) 12%, transparent);
}
.port-table-wrap {
  flex: 1;
  overflow: auto;
  border: 1px solid var(--port-rule);
  border-radius: var(--r-md);
  background: transparent;
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
  border-bottom: 1px solid var(--port-rule);
  color: var(--ink);
}
.port-table th {
  position: sticky;
  top: 0;
  background: color-mix(in srgb, var(--aside-2) 6%, transparent);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
  font-weight: 600;
  user-select: none;
}
.port-table th.sortable {
  cursor: pointer;
}
.port-table th.sortable:hover {
  color: var(--aside-3);
}
.port-table .arrow {
  opacity: 0.6;
  font-size: 12px;
}
.port-table tbody tr:hover {
  background: color-mix(in srgb, var(--aside-2) 10%, transparent);
}
.port-kill {
  padding: 4px 12px;
  border: 1px solid var(--port-rule);
  border-radius: var(--r-sm);
  background: transparent;
  color: var(--danger);
  cursor: pointer;
  font-size: 12px;
  transition: background 0.15s;
}
.port-kill:hover {
  background: color-mix(in srgb, var(--danger) 8%, transparent);
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
