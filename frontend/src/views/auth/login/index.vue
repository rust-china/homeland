<script lang="tsx" setup>
import { onMounted } from 'vue'
// import apiAxios from '@/utils/apiAxios'
import { useUserStore } from '@/stores/user'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore();

async function fetchUser() {
	// const { data: rData } = await apiAxios.get('/auth/user')
	// userStore.setUserInfo(rData)
	await userStore.fetchUserInfo()
	router.replace({ name: 'home' })
}

onMounted(() => {
	if (route.query.ret === 'success') {
		fetchUser()
	} else {
		window.location.href = `${import.meta.env.VITE_APP_API_BASE_URL}/github`
	}
})
</script>

<template>
	<main class="main-container">
		<t-card class="card my-5">
			正在登录...
		</t-card>
	</main>
</template>

<style lang="scss" scoped>
</style>