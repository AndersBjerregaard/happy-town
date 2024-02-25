<script setup lang="ts">
import { RouterView } from 'vue-router'
import apiUrl from './config';
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
      <span class="span-label">Socials: </span>
      <span>https://github.com/AndersBjerregaard</span>
    </footer>
  </div>
</template>

<style scoped>
</style>
