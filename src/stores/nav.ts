import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export type NavItem = {
  type: 'item'
  id: string
  label: string
  glyph?: string
  icon?: string
  hasUpdate?: boolean
  active?: boolean
}

export type NavGroup = {
  type: 'group'
  id: string
  label: string
  icon?: string
  expanded: boolean
  children: NavItem[]
}

export type NavNode = NavItem | NavGroup

export const NAV_DATA: NavNode[] = [
  { type: 'group', id: 'g-system', icon: 'server', label: '系统工具', expanded: true, children: [
    { type: 'item', id: 'port', glyph: '⚓', label: '端口管理' },
  ]},
  { type: 'group', id: 'g-codec', icon: 'swap', label: '编解码器', expanded: true, children: [
    { type: 'item', id: 'qrcode',       glyph: 'QR', label: '二维码', hasUpdate: true, active: true },
    { type: 'item', id: 'url',          icon: 'link', label: 'URL' },
    { type: 'item', id: 'base64-image', glyph: 'B图', label: 'Base64图片' },
    { type: 'item', id: 'base64-text',  glyph: 'B文', label: 'Base64文本' },
  ]},
  { type: 'group', id: 'g-format', icon: 'align', label: '格式化工具', expanded: true, children: [
    { type: 'item', id: 'json',    glyph: '{;}', label: 'JSON' },
    { type: 'item', id: 'sql',     glyph: 'SQ',  label: 'SQL' },
    { type: 'item', id: 'xml-fmt', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', id: 'g-net',  icon: 'activity', label: '测试工具', expanded: true, children: [
    { type: 'item', id: 'connectivity', icon: 'signal', label: '连通性测试' },
    { type: 'item', id: 'websocket',    icon: 'radio',  label: 'WebSocket' },
  ]},
  { type: 'group', id: 'g-gen',   icon: 'zap',   label: '生成器',   expanded: true, children: [
    { type: 'item', id: 'hash', icon: 'hash', label: '哈希 / 校验' },
    { type: 'item', id: 'password', icon: 'key', label: '密码' },
    { type: 'item', id: 'uuid', icon: 'fingerprint', label: 'UUID' },
  ]},
  { type: 'group', id: 'g-img', icon: 'image', label: '图像处理', expanded: true, children: [
    { type: 'item', id: 'image-format',   glyph: 'Fmt', label: '格式转换' },
    { type: 'item', id: 'image-compress', glyph: 'Cmp', label: '图片压缩' },
  ]},
  { type: 'group', id: 'g-text',  icon: 'type',  label: '文本处理', expanded: true, children: [
    { type: 'item', id: 'escape',   glyph: 'TX', label: '转义 / 反转义' },
    { type: 'item', id: 'list-cmp', glyph: '≡',  label: '列表比对' },
    { type: 'item', id: 'md',       glyph: 'MD', label: 'Markdown 预览' },
  ]},
]

export const FOOT_DATA: NavItem[] = [
  { type: 'item', id: 'settings', glyph: '☰', label: '设置' },
]

export const useNavStore = defineStore('nav', () => {
  const items = NAV_DATA
  const foot = FOOT_DATA
  const activeId = ref<string>('port')
  const query = ref('')

  function matchItem(item: NavItem, q: string) {
    return item.label.toLowerCase().includes(q) || item.id.toLowerCase().includes(q)
  }

  // 空 query 直接返回原数据；非空时按 item 的 label/id 过滤，
  // group 标题命中则整组保留，否则仅保留匹配的子项，并强制展开。
  const filteredItems = computed<NavNode[]>(() => {
    const q = query.value.trim().toLowerCase()
    if (!q) return items
    const out: NavNode[] = []
    for (const node of items) {
      if (node.type === 'item') {
        if (matchItem(node, q)) out.push(node)
        continue
      }
      const groupMatch = node.label.toLowerCase().includes(q)
      const children = groupMatch ? node.children : node.children.filter(c => matchItem(c, q))
      if (children.length) out.push({ ...node, children, expanded: true })
    }
    return out
  })

  function select(id: string) {
    activeId.value = id
  }

  function findLabel(id: string): string | null {
    for (const node of items) {
      if (node.type === 'item' && node.id === id) return node.label
      if (node.type === 'group') {
        const hit = node.children.find(c => c.id === id)
        if (hit) return hit.label
      }
    }
    for (const f of foot) {
      if (f.id === id) return f.label
    }
    return null
  }

  return { items, foot, activeId, query, filteredItems, select, findLabel }
})
