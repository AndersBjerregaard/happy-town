<script setup lang="ts">
  defineProps<{
    username?: string
  }>()
</script>

<template>
  <div class="user-component">
    <span class="user-username-label">Username:</span>
    <span>{{ username }}</span>
  </div>
</template>

<style scoped>
  
</style>
