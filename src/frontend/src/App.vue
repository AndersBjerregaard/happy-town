<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HomeView from './Views/HomeView.vue'
</script>

<template>
    <header>
      <div class="header">
        <div>
          <h1>Logo</h1>
        </div>
        <div id="title">
          <h1 class="main-title">Happy Town</h1>
        </div>
      </div>
    </header>
    <div class="home">
      <HomeView/>
    </div>
    <div class="footer">
    <footer>
      <p>Footer</p>
    </footer>
    </div>
</template>

<style scoped>
</style>
