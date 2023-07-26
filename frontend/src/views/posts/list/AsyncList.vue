<script lang="tsx">
import { useList } from '@/utils/hooks/useList'
import { useSSRContext } from '@/utils/hooks/useSSRContext'
import { graphqlApi } from '@/api'
import dayjs from 'dayjs'

export default defineComponent({
	async setup(_ctx) {
		const ssrContext = useSSRContext()
		const route = useRoute()
		const router = useRouter()
		const sortMap: any = {
			'posts/default': 'DEFAULT',
			'posts/last': 'LAST',
			'posts/excellent': 'EXCELLENT',
			'posts/popular': 'POPULAR',
			'posts/lastComment': 'LAST_COMMENT'
		}
		const listState = useList({
			async onLoad() {
				const { data: rData } = await graphqlApi({
					query: `
						query PostListQuery($query: GraPostListQuery!) {
							postList(query: $query) {
								records {
									uuid,
									title,
									user,
									category,
									commentCount,
									updatedAt,
									createdAt,
								},
								pagination {
									pageNo,
									pageSize,
									totalCount,
								},
							}
						}
					`,
					variables: {
						query: {
							pageNo: listState.pagination.pageNo,
							pageSize: listState.pagination.pageSize,
							sort: sortMap[route.name as any] || sortMap['posts/default']
						}
					}
				}, {
					headers: {
						Cookie: ssrContext?.ssrCookie
					}
				})
				listState.records = rData.data.postList.records
				listState.pagination.total = rData.data.postList.pagination.totalCount
			}
		})

		await listState.onLoad()
		watch(() => route.name, async () => {
			await listState.onLoad()
		}, { immediate: false })
		return { dayjs, route, router, listState }
	}
})


</script>

<template>
	<main class="posts">
		<div class="posts-header flex items-stretch header navbar navbar-expand-md mb-3">
			<div class="main-container w-full flex justify-start px-3 py-2 mx-auto md:px-4">
				<t-space class="filter">
					<router-link :to="{ name: 'posts/default' }"
						:class="{ active: route.name === 'posts/default' }">默认</router-link>
					<router-link :to="{ name: 'posts/last' }" :class="{ active: route.name === 'posts/last' }">新帖</router-link>
					<router-link :to="{ name: 'posts/excellent' }"
						:class="{ active: route.name === 'posts/excellent' }">精华帖</router-link>
					<router-link :to="{ name: 'posts/popular' }"
						:class="{ active: route.name === 'posts/popular' }">优质讨论</router-link>
					<router-link :to="{ name: 'posts/lastComment' }"
						:class="{ active: route.name === 'posts/lastComment' }">最新回复</router-link>
				</t-space>
			</div>
		</div>
		<div class="posts-body main-container px-0 md:px-4 lg:px-0">
			<div class="flex flex-col gap-4 lg:flex-row">
				<div class="grow lg:w-3/4">
					<t-card bordered class="card">
						<t-list :split="true">
							<template v-for="post in listState.records" :key="post.uuid">
								<t-list-item>
									<t-list-item-meta>
										<template #title>
											<RouterLink :to="{ name: 'posts/show', params: { uuid: post.uuid } }">
												<t-space size="small">
													<span class="opacity-50">{{ post.category.name }}</span>
													{{ post.title }}
												</t-space>
											</RouterLink>
										</template>
										<template #description>
											<t-space size="small">
												<span class="opacity-30">{{ post.user.name || post.user.username }}</span>
												<span class="opacity-70">发表于：{{ dayjs(post.createdAt).format('YYYY-MM-DD HH:mm:ss') }}</span>
											</t-space>
										</template>
									</t-list-item-meta>
									<template #action>
										<t-tag>{{ post.commentCount }}</t-tag>
									</template>
								</t-list-item>
							</template>
						</t-list>
						<t-pagination class="mt-3" show-jumper v-bind="listState.pagination" />
					</t-card>
				</div>
				<div class="w-full lg:w-1/4">
					<div class="mb-4">
						<t-card class="card">
							<t-button block theme="primary" variant="base" @click="router.push({ name: 'posts/new' })">发布新帖子</t-button>
						</t-card>
					</div>
					<div class="mb-4">
						<t-card title="小贴士" header-bordered class="card">
							<p>管理员会定期检查帖子，发现有描述不清晰，或者不知道说什么的帖子移动到「NoPoint」节点，此节点永远不会上首页，如果你发现你的帖子进入了「NoPoint」里面，请检查调整你的标题和内容。</p>
						</t-card>
					</div>
				</div>
			</div>
		</div>
	</main>
</template>

<style lang="scss" scoped>
.posts-header {
	@apply text-sm;
	@apply bg-white shadow-sm;
	@apply dark:bg-gray-900 dark:border-t dark:bg-opacity-70 dark:border-solid dark:border-gray-900;

	a {
		&.active {
			@apply font-bold;
		}
	}
}
</style>