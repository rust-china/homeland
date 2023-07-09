<script lang="tsx" setup>
import { useForm } from '@/utils/hooks/useForm';
import type { TabValue } from 'tdesign-vue-next'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const userInfo = ref(null)
const defaultActiveTab = ref(route.name as any as string)

const onTabChange = (tabValue: TabValue) => {
	router.push({ name: tabValue as string })
}

const formState = useForm({
	model: {},
	async onSubmit() { }
})

const dropdownOptions = ref([
	{ content: '操作一', value: 1 },
	{ content: '操作二', value: 2 },
	{ content: '操作三', value: 3, divider: true },
	{ content: '退出', value: 'logout' },
])

onMounted(() => {
  watchEffect(() => {
		userInfo.value = userStore.userInfo
	})
})
</script>

<template>
	<div class="layout-header z-50">
		<div
			class="main-container h-full relative flex flex-wrap items-center justify-between px-3 mx-auto lg:px-0 md:flex-nowrap">
			<div class="left-panel flex flex-1">
				<RouterLink :to="{ path: '/' }" class="title">
					<b>Rust</b> China
				</RouterLink>
				<div>
					<t-tabs :default-value="defaultActiveTab" @change="onTabChange">
						<t-tab-panel value="posts" label="社区"></t-tab-panel>
						<t-tab-panel value="jobs" label="招聘"></t-tab-panel>
						<t-tab-panel value="wiki" label="Wiki"></t-tab-panel>
						<t-tab-panel value="sites" label="酷站"></t-tab-panel>
					</t-tabs>
				</div>
			</div>
			<t-space class="right-panel">
				<t-form :ref="formState.setFormRef" :data="formState.model" :rules="formState.rules" :requiredMark="false"
					:labelWidth="0" @reset="formState.onReset" @submit="formState.onSubmit">
					<t-form-item>
						<t-auto-complete v-model="formState.model.searchText" placeholder="搜索"></t-auto-complete>
					</t-form-item>
				</t-form>
				<template v-if="userInfo">
					<t-button shape="circle" variant="text">
						<template #icon>
							<t-icon name="notification"></t-icon>
						</template>
					</t-button>
					<t-dropdown :options="dropdownOptions" :min-column-width="88">
						<t-button theme="default" variant="text"> 
							<template #suffix>
								<t-icon name="caret-down-small"></t-icon>
							</template>
							{{ userInfo['name'] || userInfo['username'] }} 
						</t-button>
					</t-dropdown>
				</template>
				<template v-else>
					<RouterLink to="/auth/login">
						<t-button>登录</t-button>
					</RouterLink>
				</template>
			</t-space>
		</div>
	</div>
</template>

<style lang="scss" scoped>
.layout-header {
	@apply bg-white shadow-sm border-none text-gray-800 fixed left-0 right-0 top-0 h-[48px];
	@apply dark:bg-gray-900 dark:text-gray-200 dark:border-b dark:border-solid dark:border-gray-800;

	.title {
		font-family: PingFang SC, Noto Sans, Roboto, Microsoft Yahei, sans-serif;
		@apply text-lg py-2 mr-6 text-gray-500 border-none;
		@apply dark:text-gray-100;

		b {
			@apply text-blue-600;
			@apply dark:text-gray-300;
		}
	}
}
</style>