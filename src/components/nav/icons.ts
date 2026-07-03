// 集中存放侧栏小图标的 SVG inner markup（24x24 viewBox，stroke 风格）。
// 数据来源：lucide 风格的简化 path，组件用 v-html 注入到 <svg> 内。

export const ICONS: Record<string, string> = {
  link:
    '<path d="M10 14a5 5 0 007 0l3-3a5 5 0 00-7-7l-1 1"/>' +
    '<path d="M14 10a5 5 0 00-7 0l-3 3a5 5 0 007 7l1-1"/>',
  activity:
    '<path d="M22 12h-4l-3 9L9 3l-3 9H2"/>',
  signal:
    '<path d="M2 20h.01"/>' +
    '<path d="M7 20v-4"/>' +
    '<path d="M12 20v-8"/>' +
    '<path d="M17 20V8"/>' +
    '<path d="M22 4v16"/>',
  radio:
    '<path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9"/>' +
    '<path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5"/>' +
    '<path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5"/>' +
    '<path d="M19.1 4.9C23 8.8 23 15.2 19.1 19.1"/>',
  align:
    '<path d="M3 6h18"/>' +
    '<path d="M3 12h13"/>' +
    '<path d="M3 18h10"/>',
  zap:
    '<path d="M13 2 3 14h9l-1 8 10-12h-9l1-8Z"/>',
  image:
    '<rect x="3" y="3" width="18" height="18" rx="2"/>' +
    '<circle cx="9" cy="9" r="2"/>' +
    '<path d="m21 15-3-3a2 2 0 0 0-3 0L6 21"/>',
  type:
    '<path d="M4 7V4h16v3"/>' +
    '<path d="M9 20h6"/>' +
    '<path d="M12 4v16"/>',
  server:
    '<rect x="2" y="3" width="20" height="8" rx="2"/>' +
    '<rect x="2" y="13" width="20" height="8" rx="2"/>' +
    '<path d="M6 7h.01"/>' +
    '<path d="M6 17h.01"/>',
  swap:
    '<path d="M8 3 4 7l4 4"/>' +
    '<path d="M4 7h16"/>' +
    '<path d="m16 21 4-4-4-4"/>' +
    '<path d="M20 17H4"/>',
}
