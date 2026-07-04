<template>
  <button :class="['pill-btn', { 'icon-only': iconOnly }]">
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps<{ iconOnly?: boolean }>()
</script>

<style scoped>
.pill-btn {
  height: 30px; padding: 0 10px;
  border-radius: 8px;
  background: var(--card);
  display: inline-flex; align-items: center; gap: 6px;
  font-size: 12.5px; color: var(--ink-2);
  transition: background .15s;
}
.pill-btn:hover { background: var(--aside-2); }
.pill-btn:active { background: var(--aside-3); }
.pill-btn:disabled { opacity: .45; cursor: not-allowed; pointer-events: none; }
.pill-btn :deep(svg) { width: 14px; height: 14px; }
.pill-btn.icon-only { width: 32px; padding: 0; justify-content: center; }
</style>
