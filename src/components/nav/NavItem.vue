<template>
  <div
    :class="[
      'grid grid-cols-[22px_1fr] items-center gap-1 h-8 px-1.5 my-px',
      'rounded-lg text-[13.5px] cursor-pointer transition-colors',
      isActive ? 'item-active' : 'text-ink-2 hover:bg-aside-2',
    ]"
    @click="onClick"
  >
    <span v-if="iconSvg" class="inline-flex items-center justify-center">
      <svg
        viewBox="0 0 24 24" width="16" height="16"
        fill="none" stroke="currentColor" stroke-width="2"
        stroke-linecap="round" stroke-linejoin="round"
        v-html="iconSvg"
      />
    </span>
    <span v-else-if="item.glyph" class="glyph">{{ item.glyph }}</span>
    <span v-else></span>

    <span class="truncate">{{ item.label }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useNavStore, type NavItem } from '@/stores/nav'
import { ICONS } from './icons'

const props = defineProps<{ item: NavItem }>()
const router = useRouter()
const nav = useNavStore()

const isActive = computed(() => nav.activeId === props.item.id)
const iconSvg = computed(() => (props.item.icon ? ICONS[props.item.icon] : ''))

function onClick() {
  nav.select(props.item.id)
  router.push(`/tools/${props.item.id}`)
}
</script>

<style scoped>
.item-active {
  background: linear-gradient(180deg, var(--aside-2), var(--aside-3));
  color: var(--ink);
  font-weight: 500;
  box-shadow: inset 0 0 0 1px rgba(0,0,0,0.04);
}
.glyph {
  width: 18px; height: 18px;
  background: color-mix(in srgb, var(--ink) 5%, transparent);
  border-radius: 4px;
  display: inline-flex; align-items: center; justify-content: center;
  font-family: var(--mono); font-size: 9px; text-transform: uppercase;
  color: var(--ink-3);
}
</style>
