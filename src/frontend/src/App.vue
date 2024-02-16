<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="navbar">
    <div class="navlink-container">
      <router-link class="navlink" to="/">Home</router-link>
    </div>
    <div class="navlink-container">
      <router-link class="navlink" to="/user">User</router-link>
    </div>
  </div>
  <div class="home">
    <main>
      <RouterView />
    </main>
  </div>
  <div class="footer">
    <footer>
      <p>Footer</p>
    </footer>
  </div>
</template>

<style scoped>
</style>
