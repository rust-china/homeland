<script lang="tsx" setup>
import { useList } from '@/utils/hooks/useList'
import { useQuery, gql } from '@/utils/graphql'

const route = useRoute()
const listState = useList()

const { result: gResult, loading: gLoading } = useQuery(gql`
	query QueryPosts($pageNo: Int!, $pageSize: Int!) {
		posts(pageNo: $pageNo, pageSize: $pageSize)
	}
`, listState.pagination)

watchEffect(() => {
	listState.isLoading = gLoading.value
	let postsData = gResult.value?.posts
	if (postsData) {
		listState.records = postsData.data
		Object.assign(listState.pagination, {
			total: postsData.total_count
		})
	}
})

</script>

<template>
	<main class="posts">
		<div class="posts-header flex items-stretch header navbar navbar-expand-md mb-3">
			<div class="main-container w-full flex justify-start px-3 py-2 mx-auto md:px-4">
				<t-space class="filter">
					<router-link :to="{ path: '/posts' }" :class="{active: !route.params.sort}">默认</router-link>
					<router-link :to="{ path: '/posts/last' }" :class="{active: route.params.sort === 'last'}">新帖</router-link>
					<router-link :to="{ path: '/posts/excellent' }" :class="{active: route.params.sort === 'excellent'}">精华帖</router-link>
					<router-link :to="{ path: '/posts/popular' }" :class="{active: route.params.sort === 'popular'}">优质讨论</router-link>
					<router-link :to="{ path: '/posts/lastReply' }" :class="{active: route.params.sort === 'lastReply'}">最新回复</router-link>
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
									<t-list-item-meta :title="post.title" :description="JSON.stringify(post)" />
								</t-list-item>
							</template>
						</t-list>
						<t-pagination class="mt-3" show-jumper v-bind="listState.pagination" />
					</t-card>
				</div>
				<div class="w-full lg:w-1/4">
					<div class="mb-4">111</div>
					<div class="mb-4">2222</div>
					<div class="mb-4">333</div>
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

.card {
	@apply border-none bg-white shadow-sm sm:rounded-lg;
	@apply bg-white;
	@apply dark:bg-gray-900 dark:border dark:border-solid dark:border-gray-800;
}
</style>