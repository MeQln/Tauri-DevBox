<template>
  <div class="window grid grid-cols-[280px_1fr] w-screen h-screen min-h-0">
    <AsideNav />
    <main class="bg-surface min-w-0 min-h-0 flex flex-col px-8 pt-[22px] pb-8 overflow-hidden">
      <n-dialog-provider>
        <router-view />
      </n-dialog-provider>
    </main>
  </div>
</template>

<script setup lang="ts">
import { NDialogProvider } from 'naive-ui'
import AsideNav from '@/components/nav/AsideNav.vue'
</script>
