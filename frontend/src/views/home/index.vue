<script setup lang="ts">
import { useUserStore } from '@/stores/user'
const userStore = useUserStore()
</script>

<template>
  <main>
    home {{ userStore.userInfo }}
  </main>
</template>
