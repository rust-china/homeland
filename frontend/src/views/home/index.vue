<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { useQuery, gql } from '@/utils/graphql'

const { result: postsResult } = useQuery(gql`
	query {
		posts
	}
`)

const userStore = useUserStore()
const userInfo = ref(null)
const posts = computed(() => {
  return postsResult.value?.posts.data ?? []
})

onMounted(() => {
  userInfo.value = userStore.userInfo
})


</script>

<template>
  <main>
    home {{ userInfo }}
    <t-divider></t-divider>
    <ul>
      <template v-for="post in posts" :key="post.uuid">
        <li>{{ post }}</li>
      </template>
    </ul>

  </main>
</template>
