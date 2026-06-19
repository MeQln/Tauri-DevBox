<template>
  <div
    :class="[
      'grid grid-cols-[22px_1fr_auto] items-center gap-1 h-8 px-1.5 my-px',
      'rounded-lg text-[13.5px] cursor-pointer transition-colors',
      isActive ? 'item-active' : 'text-ink-2 hover:bg-aside-2',
    ]"
    @click="onClick"
  >
    <span v-if="item.icon === 'link'" class="inline-flex items-center justify-center">
      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M10 14a5 5 0 007 0l3-3a5 5 0 00-7-7l-1 1" />
        <path d="M14 10a5 5 0 00-7 0l-3 3a5 5 0 007 7l1-1" />
      </svg>
    </span>
    <span v-else-if="item.glyph" class="glyph">{{ item.glyph }}</span>
    <span v-else></span>

    <span class="truncate">{{ item.label }}</span>

    <span v-if="item.hasUpdate" class="bulb"></span>
    <span v-else></span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useNavStore, type NavItem } from '@/stores/nav'

const props = defineProps<{ item: NavItem }>()
const router = useRouter()
const nav = useNavStore()

const isActive = computed(() => nav.activeId === props.item.id)

function onClick() {
  nav.select(props.item.id)
  router.push(props.item.id === 'url' ? '/tools/url' : `/tools/${props.item.id}`)
}
</script>

<style scoped>
.item-active {
  background: linear-gradient(180deg, #e1ddd4, #d5d0c5);
  color: var(--ink);
  font-weight: 500;
  box-shadow: inset 0 0 0 1px rgba(0,0,0,0.04);
}
.glyph {
  width: 18px; height: 18px;
  background: rgba(0,0,0,0.05);
  border-radius: 4px;
  display: inline-flex; align-items: center; justify-content: center;
  font-family: var(--mono); font-size: 9px; text-transform: uppercase;
  color: var(--ink-3);
}
.bulb {
  width: 12px; height: 12px; border-radius: 50%;
  background: var(--amber);
  box-shadow: 0 0 0 4px rgba(232,165,52,0.18);
}
</style>
