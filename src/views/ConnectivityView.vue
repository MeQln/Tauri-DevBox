<template>
  <div class="net-view">
    <header class="page-head"><h1>连通性测试</h1></header>

    <section class="card">
      <div class="card-head">
        <div class="card-title">Ping 主机</div>
        <div class="card-actions">
          <input v-model="pingHost" class="input" placeholder="主机名 / IP，如 8.8.8.8" @keyup.enter="runPing" />
          <button class="btn" :disabled="pinging" @click="runPing">{{ pinging ? '测试中…' : '测试' }}</button>
        </div>
      </div>
      <div class="result">
        <div v-if="pingAlive !== null" class="badge-row">
          <span :class="['badge', pingAlive ? 'badge-ok' : 'badge-err']">
            {{ pingAlive ? '可达' : '不可达' }}
          </span>
        </div>
        <pre class="output">{{ pingLines.join('\n') }}</pre>
      </div>
    </section>

    <section class="card">
      <div class="card-head">
        <div class="card-title">端口测试</div>
        <div class="card-actions">
          <input v-model="portHost" class="input" placeholder="主机 / IP" @keyup.enter="runPort" />
          <input v-model.number="portNum" class="input port-input" type="number" placeholder="端口" @keyup.enter="runPort" />
          <button class="btn" :disabled="porting" @click="runPort">{{ porting ? '测试中…' : '测试' }}</button>
        </div>
      </div>
      <div class="result">
        <div v-if="portResult" class="badge-row">
          <span :class="['badge', portResult.open ? 'badge-ok' : 'badge-err']">
            {{ portResult.open ? '开放' : '关闭' }}
          </span>
          <span class="meta">{{ portResult.host }}:{{ portResult.port }} · {{ portResult.latency_ms }}ms</span>
          <span class="msg">{{ portResult.message }}</span>
        </div>
        <pre class="output">{{ portLines.join('\n') }}</pre>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onBeforeUnmount } from 'vue'
import { useMessage } from 'naive-ui'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { netApi, type PortCheckResult } from '@/api/net'

const message = useMessage()

const pingHost = ref('127.0.0.1')
const pinging = ref(false)
const pingLines = ref<string[]>([])
const pingAlive = ref<boolean | null>(null)
let pingUnlisten: UnlistenFn | null = null

const portHost = ref('127.0.0.1')
const portNum = ref<number | null>(80)
const porting = ref(false)
const portResult = ref<PortCheckResult | null>(null)
const portLines = ref<string[]>([])

async function runPing() {
  const host = pingHost.value.trim()
  if (!host) { message.warning('请输入主机'); return }
  pinging.value = true
  pingLines.value = []
  pingAlive.value = null
  pingUnlisten?.()
  pingUnlisten = await listen<{ host: string; line: string }>('ping:line', (e) => {
    if (e.payload.host === host) pingLines.value.push(e.payload.line)
  })
  try {
    pingAlive.value = await netApi.ping(host)
  } catch (e) {
    message.error(`ping 失败: ${e}`)
  } finally {
    pinging.value = false
  }
}

async function runPort() {
  const host = portHost.value.trim()
  const port = portNum.value
  if (!host) { message.warning('请输入主机'); return }
  if (port == null || !Number.isInteger(port) || port < 1 || port > 65535) {
    message.warning('端口须为 1-65535 的整数')
    return
  }
  porting.value = true
  portResult.value = null
  portLines.value = [`正在连接 ${host}:${port} …`]
  try {
    const r = await netApi.checkPort(host, port)
    portResult.value = r
    portLines.value.push(`${r.open ? '连接成功' : '连接失败'} · ${r.latency_ms}ms`)
  } catch (e) {
    message.error(`端口测试失败: ${e}`)
  } finally {
    porting.value = false
  }
}

onBeforeUnmount(() => {
  pingUnlisten?.()
})
</script>

<style scoped>
.net-view { display: flex; flex-direction: column; gap: 14px; height: 100%; }
.page-head { margin-bottom: 4px; }
.page-head h1 {
  font-family: var(--serif); font-size: 28px; font-weight: 500; letter-spacing: -0.015em;
}
.card {
  flex: 1; min-height: 0;
  background: var(--card); border-radius: var(--r-md); padding: 12px;
  display: flex; flex-direction: column; gap: 10px;
}
.card-head { display: flex; align-items: center; justify-content: space-between; gap: 12px; flex-wrap: wrap; }
.card-title { font-size: 14px; font-weight: 500; color: var(--ink); }
.card-actions { display: flex; gap: 8px; align-items: center; }
.input {
  padding: 7px 12px; border: 1px solid var(--rule); border-radius: var(--r-md);
  background: var(--card-2); color: var(--ink); font-size: 13px; min-width: 220px;
}
.input:focus { outline: none; border-color: var(--link); }
.port-input { min-width: 90px; }
.btn {
  padding: 7px 16px; border: 1px solid var(--rule); border-radius: var(--r-md);
  background: var(--card-2); color: var(--ink); cursor: pointer; font-size: 13px;
}
.btn:disabled { opacity: 0.5; cursor: not-allowed; }
.btn:not(:disabled):hover { background: var(--card); }
.result { display: flex; flex-direction: column; gap: 8px; flex: 1; min-height: 0; }
.badge-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.badge {
  padding: 3px 10px; border-radius: 999px; font-size: 12px; font-weight: 500;
}
.badge-ok { background: color-mix(in srgb, var(--ok) 12%, transparent); color: var(--ok); }
.badge-err { background: color-mix(in srgb, var(--warn) 12%, transparent); color: var(--warn); }
.meta { font-size: 12.5px; color: var(--ink-3); font-family: var(--mono); }
.msg { font-size: 12.5px; color: var(--ink-2); }
.output {
  background: var(--card-2); border: 1px solid var(--rule-soft); border-radius: var(--r-sm);
  padding: 10px 12px; font-family: var(--mono); font-size: 12px; line-height: 1.6;
  white-space: pre-wrap; word-break: break-all; color: var(--ink-2);
  flex: 1; min-height: 120px; overflow: auto;
}
</style>
