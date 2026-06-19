<template>
  <div class="io min-h-0">
    <div class="gutter">{{ gutterText }}</div>
    <pre v-if="readonly" class="content">{{ modelValue }}</pre>
    <textarea
      v-else
      class="content"
      spellcheck="false"
      :value="modelValue"
      @input="onInput"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: string
  readonly?: boolean
}>()
const emit = defineEmits<{ 'update:modelValue': [v: string] }>()

const lineCount = computed(() =>
  Math.max(1, props.modelValue.split('\n').length)
)
const gutterText = computed(() => {
  const n = lineCount.value
  return Array.from({ length: n }, (_, i) => i + 1).join('\n')
})

function onInput(e: Event) {
  emit('update:modelValue', (e.target as HTMLTextAreaElement).value)
}
</script>

<style scoped>
.io {
  border: 1px solid var(--rule);
  border-radius: var(--r-md);
  background: var(--card-2);
  display: grid; grid-template-columns: 56px 1fr;
  overflow: hidden;
  margin-bottom: 12px;
}
.gutter {
  background: linear-gradient(180deg, rgba(0,0,0,0) 0%, rgba(0,0,0,0.012) 100%);
  border-right: 1px solid var(--rule-soft);
  padding: 12px 8px;
  text-align: right;
  font-family: var(--mono); font-size: 13px; line-height: 1.85;
  color: var(--link);
  white-space: pre;
  user-select: none;
  overflow: hidden;
}
.content {
  padding: 12px 14px;
  font-family: var(--mono); font-size: 13px; line-height: 1.85;
  white-space: pre-wrap; word-break: break-all;
  width: 100%; height: 100%;
  color: var(--ink);
  display: block;
  overflow: auto;
}
</style>
