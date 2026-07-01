import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { router } from './router'
import './styles/tokens.css'
import './styles/tailwind.css'

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')

// 全局拦截拖拽默认行为：避免把图片 / 文本文件拖入窗口时浏览器
// 默认当作 URL 在当前页打开。仅在显式 @drop 处理的区域（如二维码 dropzone）内放行，
// 其余位置一律 preventDefault，杜绝"拖到任意位置就自动打开文件"。
window.addEventListener('dragover', (e) => {
  if (!e.dataTransfer?.types.includes('Files')) return
  e.preventDefault()
})
window.addEventListener('drop', (e) => {
  if (!e.dataTransfer?.types.includes('Files')) return
  // dropzone 自身的 @drop handler 会先在目标阶段执行；此处兜底阻止默认打开行为
  e.preventDefault()
})
