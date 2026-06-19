import { defineStore } from 'pinia'
import { ref } from 'vue'

export type NavItem = {
  type: 'item'
  id: string
  label: string
  glyph?: string
  icon?: 'link'
  hasUpdate?: boolean
  active?: boolean
}

export type NavGroup = {
  type: 'group'
  id: string
  label: string
  expanded: boolean
  children: NavItem[]
}

export type NavNode = NavItem | NavGroup

export const NAV_DATA: NavNode[] = [
  { type: 'item', id: 'qrcode',   glyph: 'QR', label: '二维码', hasUpdate: true },
  { type: 'item', id: 'url',      icon: 'link', label: 'URL', active: true },
  { type: 'group', id: 'g-test',  label: '测试工具', expanded: true, children: [
    { type: 'item', id: 'jsonpath', glyph: '{;}', label: 'JSONPath' },
    { type: 'item', id: 'regex',    glyph: '.*',  label: '正则表达式', hasUpdate: true },
    { type: 'item', id: 'xml-test', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', id: 'g-format', label: '格式化工具', expanded: true, children: [
    { type: 'item', id: 'json',    glyph: '{;}', label: 'JSON' },
    { type: 'item', id: 'sql',     glyph: 'SQ',  label: 'SQL' },
    { type: 'item', id: 'xml-fmt', glyph: 'XM',  label: 'XML' },
  ]},
  { type: 'group', id: 'g-gen',   label: '生成器',   expanded: false, children: [] },
  { type: 'group', id: 'g-img',   label: '图像处理', expanded: false, children: [] },
  { type: 'group', id: 'g-text',  label: '文本处理', expanded: true, children: [
    { type: 'item', id: 'escape',   glyph: 'TX', label: '转义 / 反转义' },
    { type: 'item', id: 'list-cmp', glyph: '≡',  label: '列表比对' },
    { type: 'item', id: 'md',       glyph: 'MD', label: 'Markdown 预览' },
  ]},
]

export const FOOT_DATA: NavItem[] = [
  { type: 'item', id: 'extensions', glyph: '⚙', label: '管理扩展' },
  { type: 'item', id: 'settings',   glyph: '☰', label: '设置' },
]

export const useNavStore = defineStore('nav', () => {
  const items = NAV_DATA
  const foot = FOOT_DATA
  const activeId = ref<string>('url')

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

  return { items, foot, activeId, select, findLabel }
})
