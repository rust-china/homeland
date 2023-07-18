<script lang="tsx" setup>
import { useList } from '@/utils/hooks/useList'
import { useUserStore } from '@/stores/user'
import { graphqlApi } from '@/api'
import { trimQuery } from '@/utils/libs'
import dayjs from 'dayjs'
import { emitter } from '@/utils/emitter'

const route = useRoute()
const userStore = useUserStore()

const userInfo = ref<any>(null)
const listState = useList({
	async onLoad(query) {
		const { data: rData } = await graphqlApi({
			query: `
			query CommentListQuery($query: GraCommentListQuery!) {
					commentList(query: $query) {
						records {
							id,
							bodyHtml,
							user,
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
				query: trimQuery({
					pageNo: listState.pagination.pageNo,
					pageSize: listState.pagination.pageSize,
					postUuid: route.params.uuid,
					...query
				})
			}
		})
		listState.records = rData.data.commentList.records
		listState.pagination.total = rData.data.commentList.pagination.totalCount
	}
})

const fetchComment = async (commentId: any) => {
	const { data: rData } = await graphqlApi({
		query: `
			query CommentQuery($id: Int!) {
					comment(id: $id) {
						id,
						bodyHtml,
						user,
						updatedAt,
						createdAt,
					}
				}
			`,
		variables: {
			id: commentId
		}
	})
	listState.records.push(rData.data.comment)
	listState.pagination.total += 1
}

onMounted(() => {
	userInfo.value = userStore.userInfo
	emitter.on('newComment', fetchComment)
})

onUnmounted(() => {
	emitter.off('newComment', fetchComment)
})

await listState.onLoad()
</script>

<template>
	<main class="comments">
		<template v-if="listState.records.length">
			<t-list :split="true">
				<template v-for="(comment, idx) in listState.records" :key="comment.uuid">
					<t-list-item>
						<div class="w-full flex gap-5">
							<div class="left-panel flex-1">
								<div class="title">
									<t-space size="small">
										<span class="opacity-30 text-sm">{{ comment.user.name || comment.user.username }}</span>
										<span class="text-sm">#{{ (listState.pagination.pageNo - 1) * listState.pagination.pageSize + idx + 1
										}}</span>
										<span class="opacity-50 text-xs">更新于：{{ dayjs(comment.updatedAt).format('YYYY-MM-DD HH:mm:ss')
										}}</span>
									</t-space>
								</div>
								<div class="description">
									<div class="html-render markdown-body w-full" v-html="comment?.bodyHtml"></div>
								</div>
							</div>
							<div class="right-panel">
								<template v-if="comment.user.id === userInfo?.id">
									<t-space size="small">
										<t-button size="small">编辑</t-button>
										<t-button size="small" theme="danger">删除</t-button>
									</t-space>
								</template>
							</div>
						</div>
					</t-list-item>
				</template>
			</t-list>
			<t-pagination class="mt-3" show-jumper v-bind="listState.pagination" />
		</template>
		<template v-else>暂无评论</template>
	</main>
</template>

<style lang="scss" scoped>
.comments {
	::v-deep(.t-list-item__meta-content) {
		@apply w-full;
	}
}
</style>