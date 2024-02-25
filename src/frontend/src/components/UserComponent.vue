<script setup lang="ts">
  defineProps<{
    username?: string
  }>()
</script>

<template>
  <div class="user-component">
    <span class="span-label">Username:</span>
    <span>{{ username }}</span>
  </div>
</template>

<style scoped>
  
</style>
