<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HomeView from './Views/HomeView.vue'
</script>

<template>
  <div class="navbar">
    <div class="navlink-container">
      <router-link class="navlink" to="homeView">Home</router-link>
    </div>
    <div class="navlink-container">
      <router-link class="navlink" to="userView">User</router-link>
    </div>
  </div>
  <div class="home">
    <main>
      <HomeView/>
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
