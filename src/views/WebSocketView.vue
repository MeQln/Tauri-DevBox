<template>
  <div class="ws-view">
    <header class="page-head"><h1>WebSocket 测试</h1></header>

    <div class="bar">
      <input v-model="url" class="input" placeholder="ws:// 或 wss:// 地址" @keyup.enter="toggle" />
      <button class="btn" :class="{ 'btn-danger': connected }" :disabled="connecting" @click="toggle">
        {{ connected ? '断开' : (connecting ? '连接中…' : '连接') }}
      </button>
      <span :class="['dot', connected ? 'dot-on' : 'dot-off']" />
      <span class="state">{{ stateText }}</span>
    </div>

    <div class="log-wrap">
      <div class="log" ref="logRef">
        <div v-for="(m, i) in logs" :key="i" :class="['line', m.dir]">
          <span class="ts">{{ m.ts }}</span>
          <span class="dir-tag">{{ m.dir === 'in' ? '←' : m.dir === 'out' ? '→' : '·' }}</span>
          <span class="content">{{ m.text }}</span>
        </div>
        <div v-if="!logs.length" class="empty">暂无消息</div>
      </div>
    </div>

    <div class="send">
      <input v-model="draft" class="input" placeholder="输入要发送的消息" :disabled="!connected" @keyup.enter="send" />
      <button class="btn" :disabled="!connected" @click="send">发送</button>
      <button class="btn" @click="clearLog">清空</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onBeforeUnmount } from 'vue'
import { useMessage } from 'naive-ui'

type Dir = 'in' | 'out' | 'sys'
type LogEntry = { ts: string; dir: Dir; text: string }

const message = useMessage()

const url = ref('wss://echo.websocket.events')
const draft = ref('')
const connected = ref(false)
const connecting = ref(false)
const stateText = ref('未连接')
const logs = ref<LogEntry[]>([])
const logRef = ref<HTMLElement | null>(null)

let ws: WebSocket | null = null

function ts() {
  const d = new Date()
  const p = (n: number) => String(n).padStart(2, '0')
  return `${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`
}

function push(dir: Dir, text: string) {
  logs.value.push({ ts: ts(), dir, text })
  nextTick(() => {
    if (logRef.value) logRef.value.scrollTop = logRef.value.scrollHeight
  })
}

function toggle() {
  if (connected.value || connecting.value) {
    ws?.close()
    return
  }
  const u = url.value.trim()
  if (!u) { message.warning('请输入地址'); return }
  if (!/^wss?:\/\//.test(u)) { message.warning('地址需以 ws:// 或 wss:// 开头'); return }
  connecting.value = true
  stateText.value = '连接中…'
  try {
    ws = new WebSocket(u)
  } catch (e) {
    connecting.value = false
    stateText.value = '未连接'
    message.error(`创建连接失败: ${e}`)
    return
  }
  ws.onopen = () => {
    connecting.value = false
    connected.value = true
    stateText.value = '已连接'
    push('sys', `已连接 ${u}`)
  }
  ws.onmessage = (ev) => {
    push('in', typeof ev.data === 'string' ? ev.data : '[非文本消息]')
  }
  ws.onerror = () => {
    push('sys', '连接发生错误')
  }
  ws.onclose = () => {
    connected.value = false
    connecting.value = false
    stateText.value = '未连接'
    push('sys', '连接已关闭')
    ws = null
  }
}

function send() {
  if (!ws || ws.readyState !== WebSocket.OPEN) return
  const text = draft.value
  if (!text) return
  ws.send(text)
  push('out', text)
  draft.value = ''
}

function clearLog() {
  logs.value = []
}

onBeforeUnmount(() => {
  ws?.close()
  ws = null
})
</script>

<style scoped>
.ws-view { display: flex; flex-direction: column; gap: 12px; height: 100%; }
.page-head h1 { font-family: var(--serif); font-size: 28px; font-weight: 500; letter-spacing: -0.015em; }
.bar { display: flex; align-items: center; gap: 10px; }
.bar .btn { min-width: 88px; }
.send { display: flex; align-items: center; gap: 10px; }
.send .btn + .btn { margin-left: 8px; }
.input {
  flex: 1; padding: 7px 12px; border: 1px solid var(--rule); border-radius: var(--r-md);
  background: var(--card-2); color: var(--ink); font-size: 13px;
}
.input:focus { outline: none; border-color: var(--link); }
.input:disabled { opacity: 0.5; }
.btn {
  padding: 7px 16px; border: 1px solid var(--rule); border-radius: var(--r-md);
  background: var(--card-2); color: var(--ink); cursor: pointer; font-size: 13px; white-space: nowrap;
}
.btn:disabled { opacity: 0.5; cursor: not-allowed; }
.btn:not(:disabled):hover { background: var(--card); }
.btn-danger { color: #d33; border-color: rgba(211, 51, 51, 0.4); }
.dot { width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0; }
.dot-on { background: var(--ok); box-shadow: 0 0 0 3px rgba(79, 154, 89, 0.15); }
.dot-off { background: var(--ink-5); }
.state { font-size: 12.5px; color: var(--ink-3); min-width: 56px; }
.log-wrap {
  flex: 1; min-height: 0; border: 1px solid var(--rule); border-radius: var(--r-md);
  background: var(--card-2); overflow: hidden;
}
.log { height: 100%; overflow: auto; padding: 8px 0; font-family: var(--mono); font-size: 12.5px; }
.line { display: flex; gap: 8px; padding: 4px 14px; align-items: baseline; }
.line:hover { background: var(--card); }
.ts { color: var(--ink-4); font-size: 11px; flex-shrink: 0; }
.dir-tag { flex-shrink: 0; color: var(--ink-3); width: 12px; }
.line.in .dir-tag { color: var(--ok); }
.line.out .dir-tag { color: var(--link); }
.line.sys .content { color: var(--ink-3); font-style: italic; }
.content { word-break: break-all; white-space: pre-wrap; color: var(--ink); }
.empty { padding: 24px; text-align: center; color: var(--ink-3); font-size: 13px; font-family: var(--sans); }
</style>
