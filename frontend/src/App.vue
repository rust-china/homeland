<script setup lang="ts">
import Layout from '@/components/Layout/index.vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore();
if (!import.meta.env.SSR) {
  userStore.fetchUserInfo()
}
</script>

<template>
  <Layout>
    <RouterView />
  </Layout>
</template>

<style lang="scss" scoped></style>
