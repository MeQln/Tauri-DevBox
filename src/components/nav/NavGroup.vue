<template>
  <div :class="['group', { collapsed: !expanded }]">
    <div
      class="grid grid-cols-[22px_1fr_16px] items-center h-9 px-1.5 rounded-lg text-ink-3 text-[13.5px] cursor-pointer hover:bg-aside-2 transition-colors"
      @click="expanded = !expanded"
    >
      <span></span>
      <span>{{ group.label }}</span>
      <svg
        :class="['chev transition-transform', expanded ? '' : '-rotate-90']"
        viewBox="0 0 24 24" width="14" height="14"
        fill="none" stroke="currentColor" stroke-width="2"
      >
        <path d="M6 9l6 6 6-6" />
      </svg>
    </div>
    <div v-show="expanded" class="pl-[22px]">
      <NavItem v-for="child in group.children" :key="child.id" :item="child" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import NavItem from './NavItem.vue'
import type { NavGroup } from '@/stores/nav'

const props = defineProps<{ group: NavGroup }>()
const expanded = ref(props.group.expanded)
</script>
