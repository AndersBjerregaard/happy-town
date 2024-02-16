<script setup lang="ts">
  defineProps<{
    username?: string
  }>()
</script>

<template>
  <div>
    <span>Username:      {{ username }}</span>
  </div>
</template>

<style scoped>
  
</style>
