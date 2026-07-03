<template>
  <aside class="aside grid grid-rows-[auto_1fr_auto] bg-aside border-r border-rule min-w-0 min-h-0">
    <div class="search p-2">
      <div class="search-box">
        <svg class="search-ico" viewBox="0 0 24 24" width="14" height="14"
          fill="none" stroke="currentColor" stroke-width="2"
          stroke-linecap="round" stroke-linejoin="round"
          v-html="ICONS.search" />
        <input
          ref="searchInput"
          v-model="nav.query"
          type="text"
          placeholder="搜索工具"
          @keydown.esc="nav.query = ''"
        />
        <button v-if="nav.query" class="clear" title="清除" @click="nav.query = ''">
          <svg viewBox="0 0 24 24" width="12" height="12"
            fill="none" stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 6 6 18" /><path d="m6 6 12 12" />
          </svg>
        </button>
      </div>
    </div>

    <nav class="px-2 pb-2 overflow-y-auto">
      <template v-if="nav.filteredItems.length">
        <template v-for="(node, i) in nav.filteredItems" :key="node.id">
          <hr
            v-if="i > 0 && nav.filteredItems[i - 1].type !== node.type"
            class="my-1.5 mx-1.5 border-0 border-t border-rule"
          />
          <NavGroup v-if="node.type === 'group'" :group="node" />
          <NavItem v-else :item="node" />
        </template>
      </template>
      <div v-else class="empty">未找到匹配的工具</div>
    </nav>

    <div class="border-t border-rule p-2">
      <NavItem v-for="f in nav.foot" :key="f.id" :item="f" />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import NavGroup from './NavGroup.vue'
import NavItem from './NavItem.vue'
import { useNavStore } from '@/stores/nav'
import { ICONS } from './icons'

const nav = useNavStore()
const searchInput = ref<HTMLInputElement | null>(null)

function onGlobalKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'f') {
    e.preventDefault()
    searchInput.value?.focus()
    searchInput.value?.select()
  }
}

onMounted(() => window.addEventListener('keydown', onGlobalKeydown))
onUnmounted(() => window.removeEventListener('keydown', onGlobalKeydown))
</script>

<style scoped>
.search-box {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 32px;
  padding: 0 8px;
  background: var(--surface);
  border: 1px solid var(--rule);
  border-radius: var(--r-md);
  transition: border-color 0.15s, box-shadow 0.15s;
}
.search-box:focus-within {
  border-color: var(--ink-5);
  box-shadow: 0 0 0 3px rgba(0, 0, 0, 0.04);
}
.search-ico { color: var(--ink-4); flex-shrink: 0; }
.search-box input {
  flex: 1;
  min-width: 0;
  font-size: 13px;
  color: var(--ink);
}
.search-box input::placeholder { color: var(--ink-4); }
.clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  border-radius: 4px;
  color: var(--ink-4);
  flex-shrink: 0;
}
.clear:hover { color: var(--ink-2); background: var(--aside-2); }
.empty {
  padding: 24px 8px;
  text-align: center;
  font-size: 12.5px;
  color: var(--ink-4);
}
nav::-webkit-scrollbar { width: 6px; }
nav::-webkit-scrollbar-track { background: transparent; }
nav::-webkit-scrollbar-thumb {
  background: var(--ink-5);
  border-radius: 3px;
}
nav::-webkit-scrollbar-thumb:hover { background: var(--ink-4); }
</style>
