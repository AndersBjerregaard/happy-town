<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HomeView from './Views/HomeView.vue'
</script>

<template>
    <header>
      <div class="header">
        <div id="logo">
          <img src="../src/assets/monkey-192x192.png"/>
        </div>
        <div id="title">
          <h1 class="main-title">Happy Town</h1>
        </div>
      </div>
    </header>
    <nav>
      <p>Nav</p>
    </nav>
      <HomeView/>
    <footer>
      <p>Footer</p>
    </footer>
    <RouterView />
</template>

<style>
p {
  color: white;
}
.header {
  height: auto;
  width: 100%;
  padding: 5em;
  border: 5em;
  margin: 5em;
  display: flex;
}
.main-title {
  color: white;
  font-weight: bold;
}
#logo {
  max-width: 192px;
}
#title {
  width: 100%;
}
</style>
