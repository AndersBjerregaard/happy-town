<script setup lang="ts">
  import UserComponent from '@/components/UserComponent.vue'
  const items: string[] = ["Andy", "Joe", "Foo"];
</script>

<template>
  <header>
    <div class="header home-header">
      <div>
        <h1>Logo</h1>
      </div>
      <div>
        <h1>Users</h1>
      </div>
    </div>
  </header>
  <div class="main">
    <div class="main-border">
    </div>
    <div class="main-center">
      <div class="create-user">
        <h2>Create User</h2>
      </div>
      <div class="users-main">
        <h2>Users</h2>
        <div class="users-list">
          <li v-for="(item, index) in items" :key="index">
            <UserComponent :username="item" />
          </li>
        </div>
      </div>
    </div>
    <div class="main-border">
    </div>
  </div>
</template>

<style scoped>

</style>
