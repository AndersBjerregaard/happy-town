<script setup lang="ts">
  const handleSubmit = () => console.log('Submit');
  let username: string = '';
  const users = [ 'Foo', 'Bar' ];
</script>

<template>
  <h2>Create User Component</h2>
  <div id="create-user-section">
    <h3>Create User</h3>
    <form @submit.prevent="handleSubmit">
      <label for="username-input">Username</label>
      <input id="username-input" v-model="username" required type="text">
      <button type="submit">Submit</button>
    </form>
  </div>
  <div id="users-section">
    <h3>Users</h3>
    <li v-for="(item, index) in users" :key="index">
      {{ item }}
    </li>
  </div>
</template>

<style scoped></style>
