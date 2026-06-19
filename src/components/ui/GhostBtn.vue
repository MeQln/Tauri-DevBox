<template>
  <button class="ghost-btn">
    <slot />
  </button>
</template>

<script setup lang="ts"></script>

<style scoped>
.ghost-btn {
  height: 32px; padding: 0 12px;
  display: inline-flex; align-items: center; gap: 6px;
  border-radius: 8px; color: var(--ink-2); font-size: 13px;
  transition: background .15s;
}
.ghost-btn:hover { background: var(--card); }
.ghost-btn :deep(svg) { width: 14px; height: 14px; }
</style>
