<template>
  <aside class="aside grid grid-rows-[auto_1fr_auto] bg-aside border-r border-rule min-w-0 min-h-0 pt-2.5">
    <div class="search">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="7" />
        <path d="M21 21l-4.3-4.3" />
      </svg>
      <input v-model="search" type="text" placeholder="输入以搜索工具…" />
    </div>

    <nav class="px-2 pb-2 pt-1 overflow-y-auto">
      <template v-for="(node, i) in nav.items" :key="node.id">
        <hr
          v-if="i > 0 && nav.items[i - 1].type !== node.type"
          class="my-1.5 mx-1.5 border-0 border-t border-rule"
        />
        <NavGroup v-if="node.type === 'group'" :group="node" />
        <NavItem v-else :item="node" />
      </template>
    </nav>

    <div class="border-t border-rule p-2">
      <NavItem v-for="f in nav.foot" :key="f.id" :item="f" />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NavGroup from './NavGroup.vue'
import NavItem from './NavItem.vue'
import { useNavStore } from '@/stores/nav'

const nav = useNavStore()
const search = ref('')
</script>

<style scoped>
.search {
  margin: 0 12px 8px; height: 34px;
  background: var(--card-2);
  border: 1px solid var(--rule);
  border-radius: 8px;
  display: flex; align-items: center; gap: 8px;
  padding: 0 10px;
  transition: border-color .15s, box-shadow .15s;
}
.search:focus-within {
  border-color: #c5bfb4;
  box-shadow: 0 0 0 3px rgba(0,0,0,0.04);
}
.search svg { width: 14px; height: 14px; color: var(--ink-4); flex-shrink: 0; }
.search input { flex: 1; font-size: 13.5px; }
.search input::placeholder { color: var(--ink-4); }

nav::-webkit-scrollbar { width: 8px; }
nav::-webkit-scrollbar-thumb { background: var(--ink-5); border-radius: 4px; }
</style>
