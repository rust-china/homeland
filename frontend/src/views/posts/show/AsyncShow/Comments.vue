<script lang="tsx" setup>
import { useList } from '@/utils/hooks/useList'
import { useUserStore } from '@/stores/user'
import { graphqlApi } from '@/api'
import { trimQuery } from '@/utils/libs'
import { emitter } from '@/utils/emitter'
import { useDialog } from '@/utils/hooks/useDialog'
import { MessagePlugin } from 'tdesign-vue-next';
import Comment from './components/Comment.vue'

const dialog = useDialog()
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
							commentCount,
							likeCount,
							ancestry,
							parentId,
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

const refetchComments = async () => {
	listState.onLoad()
}

const onDestroy = (record: any) => {
	const instance = dialog.create({
		header: '确认',
		async onConfirm() {
			await graphqlApi({
				query: `
					mutation CommentDelete($id: Int!) {
						commentDelete(id: $id)
					}
				`,
				variables: {
					id: record.id
				}
			})
			listState.records = listState.records.filter(item => item.id !== record.id)
			listState.pagination.total -= 1
			instance.close()
			MessagePlugin.success("删除成功")
		},
		slots: {
			default: () => (
				<span>确定要删除评论吗？</span>
			)
		}
	})
	// console.log(instance)
}

onMounted(() => {
	userInfo.value = userStore.userInfo
	emitter.on('newRootComment', refetchComments)
})

onUnmounted(() => {
	emitter.off('newRootComment', refetchComments)
})

await listState.onLoad()
</script>

<template>
	<main class="comments">
		<template v-if="listState.records.length">
			<t-list :split="true">
				<!-- <template #header> 通过 Slot 插入的 Header </template> -->
				<template v-for="comment in listState.records" :key="comment.id">
					<t-list-item>
						<Comment :comment="comment" :loginInfo="userInfo" @destroy="onDestroy"></Comment>
					</t-list-item>
				</template>
				<template #footer>
					<t-pagination class="mt-3" size="small" show-jumper v-bind="listState.pagination" />
				</template>
			</t-list>
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