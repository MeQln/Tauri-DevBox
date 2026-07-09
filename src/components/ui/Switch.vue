<template>
  <div class="row-ctl">
    <span>{{ checked ? onLabel : offLabel }}</span>
    <label :class="['switch', { on: checked }]" @click.prevent="toggle">
      <input type="checkbox" :checked="checked" tabindex="-1" />
    </label>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: boolean
  onLabel?: string
  offLabel?: string
}>()
const emit = defineEmits<{ 'update:modelValue': [v: boolean] }>()

const checked = computed(() => props.modelValue)
const onLabel  = computed(() => props.onLabel  ?? '开启')
const offLabel = computed(() => props.offLabel ?? '关闭')

function toggle() {
  emit('update:modelValue', !checked.value)
}
</script>

<style scoped>
.row-ctl {
  display: flex; align-items: center; gap: 8px;
  font-size: 12.5px; color: var(--ink-3);
}
.switch {
  position: relative;
  width: 44px; height: 24px;
  border-radius: 999px;
  background: var(--aside-3);
  transition: background .15s;
  flex-shrink: 0;
  cursor: pointer;
}
.switch::after {
  content: '';
  position: absolute; top: 2px; left: 2px;
  width: 20px; height: 20px;
  background: var(--surface);
  border-radius: 50%;
  box-shadow: 0 1px 2px rgba(0,0,0,0.18);
  transition: transform .22s cubic-bezier(.2,.7,.2,1);
}
.switch input { position: absolute; opacity: 0; pointer-events: none; }
.switch.on { background: var(--accent); }
.switch.on::after { transform: translateX(20px); }
</style>
