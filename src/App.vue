<template>
  <n-config-provider :theme="naiveTheme">
    <n-message-provider>
      <router-view />
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NConfigProvider, NMessageProvider, darkTheme } from 'naive-ui'
import { useThemeStore } from '@/stores/theme'

const theme = useThemeStore()
// 深色时传 darkTheme，浅色传 null（naive-ui 默认浅色）
const naiveTheme = computed(() => (theme.isDark ? darkTheme : null))
</script>
