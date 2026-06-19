<template>
  <div class="p-8">
    <h1 class="text-2xl font-medium text-ink mb-2">{{ label }}</h1>
    <p class="text-ink-3">即将上线</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useNavStore } from '@/stores/nav'

const route = useRoute()
const nav = useNavStore()
const label = computed(() => {
  const id = String(route.params.id ?? '')
  return nav.findLabel(id) ?? '未知工具'
})
</script>
