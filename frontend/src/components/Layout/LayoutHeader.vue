<script lang="tsx" setup>
import { useForm } from '@/utils/hooks/useForm';
import type { TabValue } from 'tdesign-vue-next'
import { useUserStore } from '@/stores/user'
import { logoutApi } from '@/api'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const userInfo = ref(null)
const activeTab = computed(() => `${route.name as any}`.split('/')[0])

const onTabChange = (tabValue: TabValue) => {
	router.push({ name: tabValue as string })
}

const formState = useForm({
	model: {},
	async onSubmit() { }
})

const menuOptions = ref([
	{ content: '社区', value: 'posts' },
	{ content: '招聘', value: 'jobs' },
	{ content: 'Wiki', value: 'wiki' },
	{ content: '酷站', value: 'sites' },
])
const userOptions = ref([
	{ content: '操作一', value: 1 },
	{ content: '操作二', value: 2 },
	{ content: '操作三', value: 3, divider: true },
	{ content: '退出', value: 'logout', onClick: async () => {
			await logoutApi()
			userStore.setUserInfo(null)
			router.push({ name: 'home' })
		} 
	},
])

onMounted(() => {
  watchEffect(() => {
		userInfo.value = userStore.userInfo
	})
})
</script>

<template>
	<div class="layout-header">
		<div class="layout-header-fixed z-50">
			<div
				class="main-container h-full relative flex flex-wrap items-center justify-between px-3 mx-auto lg:px-0 md:flex-nowrap">
				<div class="left-panel flex-1 h-full flex items-center">
					<RouterLink :to="{ path: '/' }" class="title whitespace-nowrap">
						<b>Rust</b> China
					</RouterLink>
					<t-dropdown class="block md:hidden" :options="menuOptions" trigger="click" @click="(option) => onTabChange(option.value as any)">
						<t-icon name="view-list" size="16" />
					</t-dropdown>
					<div class="hidden md:block">
						<div class="tabs">
							<t-space>
								<template v-for="option in menuOptions" :key="option.value">
									<RouterLink :to="{ name: option.value }">
										<span class="tab-item" :class="{active: activeTab === option.value}">{{ option.content }}</span>
									</RouterLink>
								</template>
							</t-space>
						</div>
					</div>
				</div>
				<t-space class="right-panel">
					<t-form class="hidden md:block" :ref="formState.setFormRef" :data="formState.model" :rules="formState.rules" :requiredMark="false" layout="inline"
						:labelWidth="0" @reset="formState.onReset" @submit="formState.onSubmit">
						<t-auto-complete v-model="formState.model.searchText" placeholder="搜索"></t-auto-complete>
					</t-form>
					<template v-if="userInfo">
						<t-button shape="circle" variant="text">
							<template #icon>
								<t-icon name="notification"></t-icon>
							</template>
						</t-button>
						<t-dropdown :options="userOptions" :min-column-width="88">
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
	</div>
</template>

<style lang="scss" scoped>
.layout-header {
	@apply h-5;
}
.layout-header-fixed {
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

	.tabs {
		.tab-item {
			&.active {
				@apply font-bold;
			}
		}
	}
}
</style>