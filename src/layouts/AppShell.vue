<template>
  <div class="window grid grid-cols-[280px_1fr] w-screen h-screen min-h-0">
    <AsideNav />
    <main class="bg-surface min-w-0 min-h-0 flex flex-col overflow-auto px-8 pt-[22px] pb-8">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import AsideNav from '@/components/nav/AsideNav.vue'
</script>
