<script setup lang="ts">
  const handleSubmit = () => console.log('Submit')
</script>

<template>
  <h2>Create User Component</h2>
  <div>
    <form @submit.prevent="handleSubmit">
      <label>Username</label>
      <input v-model="username" required type="text">
      <button type="submit">Submit</button>
    </form>
  </div>
</template>

<style scoped></style>
