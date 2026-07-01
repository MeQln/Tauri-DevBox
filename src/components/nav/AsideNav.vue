<template>
  <aside class="aside grid grid-rows-[1fr_auto] bg-aside border-r border-rule min-w-0 min-h-0 pt-2.5">
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
import NavGroup from './NavGroup.vue'
import NavItem from './NavItem.vue'
import { useNavStore } from '@/stores/nav'

const nav = useNavStore()
</script>

<style scoped>
nav::-webkit-scrollbar { width: 8px; }
nav::-webkit-scrollbar-thumb { background: var(--ink-5); border-radius: 4px; }
</style>
