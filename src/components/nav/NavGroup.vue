<template>
  <div :class="['group', { collapsed: !expanded }]">
    <div
      class="grid grid-cols-[22px_1fr_16px] items-center h-9 px-1.5 rounded-lg text-ink-3 text-[13.5px] cursor-pointer hover:bg-aside-2 transition-colors"
      @click="toggle"
    >
      <span class="inline-flex items-center justify-center">
        <svg
          v-if="iconSvg"
          viewBox="0 0 24 24" width="16" height="16"
          fill="none" stroke="currentColor" stroke-width="2"
          stroke-linecap="round" stroke-linejoin="round"
          v-html="iconSvg"
        />
      </span>
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
import { computed, ref } from 'vue'
import NavItem from './NavItem.vue'
import { useNavStore, type NavGroup } from '@/stores/nav'
import { ICONS } from './icons'

const props = defineProps<{ group: NavGroup }>()
const nav = useNavStore()
const localExpanded = ref(props.group.expanded)
// 搜索时强制展开，避免折叠态挡住匹配到的子项
const expanded = computed(() => (nav.query ? true : localExpanded.value))
const iconSvg = computed(() => (props.group.icon ? ICONS[props.group.icon] : ''))

function toggle() {
  if (nav.query) return
  localExpanded.value = !localExpanded.value
}
</script>
