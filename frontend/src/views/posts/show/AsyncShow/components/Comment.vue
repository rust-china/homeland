<script lang="tsx">
import { useList } from '@/utils/hooks/useList'
import { graphqlApi } from '@/api'
import { trimQuery } from '@/utils/libs'
import dayjs from 'dayjs'
import { useDialog } from '@/utils/hooks/useDialog'
import NewComment from './NewComment.vue'
import { MessagePlugin } from 'tdesign-vue-next';
import IdentAvatar from '@/components/shared/IdentAvatar.vue'
import MarkdownRender from '@/components/shared/MarkdownRender.vue'

const Comment = defineComponent({
	components: {
		IdentAvatar,
		MarkdownRender,
	},
	props: {
		comment: {
			type: Object,
			required: true
		},
		loginInfo: [Object],
	},
	emits: ['reply', 'destroy'],
	setup(props, _ctx) {
		const dialogState = inject('dialogState', null)
		const route = useRoute()
		const dialog = useDialog()
		const listState = useList({
			query: {
				ancestry: props.comment.ancestry ? `${props.comment.ancestry}/${props.comment.id}` : `/${props.comment.id}`
			},
			pagination: { pageNo: 0, pageSize: 10 },
			async onLoad(query) {
				const { data: rData } = await graphqlApi({
					query: `
					query CommentListQuery($query: GraCommentListQuery!) {
							commentList(query: $query) {
								records {
									id,
									bodyHtml,
									user,
									ancestry,
									parentId,
									isLike,
									likeCount,
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
						query: trimQuery({
							pageNo: listState.pagination.pageNo,
							pageSize: listState.pagination.pageSize,
							postUuid: route.params.uuid,
							...query
						})
					}
				})
				const beforeIds = listState.records.map(i => i.id)
				const records = [...listState.records]
				const newRecords = rData.data.commentList.records;

				for (const newRecord of newRecords) {
					if (beforeIds.indexOf(newRecord.id) === -1) {
						records.push(newRecord)
					}
				}
				listState.records = records
				listState.pagination.total = rData.data.commentList.pagination.totalCount
			}
		})
		const remainingQuantity = computed(() => {
			if (listState.records.length) {
				return listState.pagination.total - listState.records.length
			} else {
				return props.comment.commentCount
			}
		})

		const onSubDestroy = async (record: any) => {
			const instance = dialog.create({
				them: 'warning',
				header: '确定要删除该评论吗？',
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
					// eslint-disable-next-line vue/no-mutating-props
					props.comment.commentCount -= 1;
					listState.onLoad()
					instance.close()
					MessagePlugin.success("删除成功")
				},
				slots: {
					default: () => (
						<Comment comment={record}></Comment>
					)
				}
			})
			// console.log(instance)
		}

		const onReply = async (record: any) => {
			const instance = dialog.create({
				header: '回复',
				width: '80%',
				cancelBtn: null,
				confirmBtn: null,
				slots: {
					default: () => (
						<div>
							<Comment comment={record}></Comment>
							<t-divider></t-divider>
							<NewComment parentComment={record} onSubmited={() => {
								listState.onLoad({ pageNo: listState.pagination.pageNo || 1 })
								// eslint-disable-next-line vue/no-mutating-props
								props.comment.commentCount += 1
								instance.close()
							}}></NewComment>
						</div>
					)
				}
			})
		}

		const onLikeComment = async (comment: any) => {
			const { data: rData } = await graphqlApi({
				query: `
					mutation GCommentLike($id: Int!) {
						commentLike(id: $id)
					}
				`,
				variables: {
					id: comment.id
				}
			})
			Object.assign(comment, rData.data.commentLike)
		}

		return {
			dayjs,
			dialogState,
			remainingQuantity,
			listState,
			onReply,
			onSubDestroy,
			onLikeComment,
		}
	}
})
export default Comment
</script>

<template>
	<div class="comment w-full flex gap-5">
		<t-comment>
			<template #avatar>
				<IdentAvatar class="avatar" :ident="comment.user.username"></IdentAvatar>
			</template>
			<template #author>
				<div>{{ comment.user.name || comment.user.username }}</div>
			</template>
			<template #datetime>{{ dayjs(comment.createdAt).format('YYYY-MM-DD HH:mm:ss') }}</template>
			<template #content>
				<div class="comment-body-rende w-full p-1">
					<MarkdownRender :html="comment?.bodyHtml"></MarkdownRender>
				</div>
			</template>
			<template #actions>
				<div class="flex gap-3">
					<span class="action-item flex items-center" @click="onLikeComment(comment)">
						<t-icon name="thumb-up" />
						<span>{{ comment.likeCount }}</span>
					</span>
					<span class="action-item flex items-center" @click="onReply(comment)">
						<t-icon name="chat" />
						<span>回复</span>
					</span>
					<template v-if="comment.user.id === loginInfo?.id">
						<span class="action-item flex items-center" @click="$emit('destroy', comment)">
							<t-icon name="delete" />
							<span>删除</span>
						</span>
					</template>
				</div>
			</template>
			<template #reply>
				<template v-if="comment.commentCount">
					<div class="reply-container">
						<div class="reply list">
							<template v-for="(subComment, idx) in listState.records" :key="subComment.id">
								<div class="reply-item">
									<Comment :comment="subComment" :loginInfo="loginInfo" @destroy="onSubDestroy">
									</Comment>
								</div>
								<t-divider v-if="idx < listState.records.length - 1"></t-divider>
							</template>
						</div>
						<template v-if="remainingQuantity">
							<p class="text-center">
								<t-link theme="primary" underline @click="listState.onLoad({ pageNo: listState.pagination.pageNo + 1 })">
									显示{{ remainingQuantity }}条回复 </t-link>
							</p>
						</template>
					</div>
				</template>
			</template>
		</t-comment>
	</div>
</template>

<style lang="scss" scoped>
.comment {
	::v-deep(.t-comment__reply) {
		margin-left: 20px;

		&:empty {
			display: none;
		}
	}

	.action-item {
		&:hover {
			color: #000;;
		}

		@include when-inside('html[theme-mode=dark]') {
			&:hover {
				color: #fff;
			}

		}
	}
}

.avatar {
	width: 48px;
	height: 48px;
	border-radius: 50%;
}
</style>