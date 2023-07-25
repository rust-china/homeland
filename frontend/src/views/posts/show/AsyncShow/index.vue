<script lang="tsx">
import { graphqlApi } from '@/api'
import { useUserStore } from '@/stores/user'
import { useSSRContext } from '@/utils/hooks/useSSRContext'
import MarkdownRender from '@/components/shared/MarkdownRender.vue'
import dayjs from 'dayjs'
import Comments from './Comments.vue'
import NewComment from './components/NewComment.vue'
import IdentAvatar from '@/components/shared/IdentAvatar.vue'

export default defineComponent({
	components: {
		Comments,
		NewComment,
		IdentAvatar,
		MarkdownRender,
	},
	async setup(_ctx) {
		const ssrContext = useSSRContext()
		const userStore = useUserStore()
		const route = useRoute();
		const router = useRouter();

		const userInfo = ref<any>(null)
		const post = ref<any>(null)
		const fetchPost = async () => {
			const { data: rData } = await graphqlApi({
				query: `
					query PostQuery($uuid: String!) {
						post(uuid: $uuid) {
							uuid,
							title,
							user,
							isLike,
							likeCount,
							commentCount,
							category,
							bodyHtml,
							updatedAt,
							createdAt,
						}
					}
				`,
				variables: {
					uuid: route.params.uuid
				}
			}, {
				headers: {
					Cookie: ssrContext?.ssrCookie
				}
			})

			const postData = rData.data.post
			post.value = {
				...postData,
				bodyHtml: postData.bodyHtml
			}
		}

		const onLikePost = async () => {
			const { data: rData } = await graphqlApi({
				query: `
					mutation GPostLike($uuid: String!) {
						postLike(uuid: $uuid)
					}
				`,
				variables: {
					uuid: route.params.uuid
				}
			}, {
				headers: {
					Cookie: ssrContext?.ssrCookie
				}
			})
			Object.assign(post.value, rData.data.postLike)
		}

		await fetchPost()
		return { dayjs, route, router, userInfo, userStore, post, onLikePost }
	},
	mounted() {
		this.userInfo = this.userStore.userInfo
	}
})
</script>

<template>
	<main class="post mb-5">
		<div class="post-body main-container px-0 mt-3 md:px-4 lg:px-0">
			<div class="flex flex-col gap-4 lg:flex-row">
				<div class="grow lg:w-3/4">
					<t-card bordered class="card">
						<template #title>
							<span class="font-bold text-lg">{{ post?.title }}</span>
						</template>
						<template #description>
							<t-space size="small">
								<span class="opacity-50">{{ post.user.name || post.user.username }}</span>
								<span>发表于：{{ dayjs(post.createdAt).format('YYYY-MM-DD HH:mm:ss') }}</span>
							</t-space>
						</template>
						<div class="post-body-render">
							<MarkdownRender :html="post?.bodyHtml"></MarkdownRender>
						</div>
					</t-card>
					<t-card bordered class="card mt-5">
						<Comments></Comments>
					</t-card>
					<t-card bordered class="card mt-5">
						<NewComment></NewComment>
					</t-card>
				</div>
				<div class="w-full lg:w-1/4">
					<template v-if="userInfo?.id === post.user?.id">
						<div class="mb-4">
							<t-card class="card">
								<t-button block theme="primary" variant="base"
									@click="router.push({ name: 'posts/edit', params: { uuid: post.uuid } })">编辑帖子</t-button>
							</t-card>
						</div>
					</template>
					<div class="mb-4">
						<t-card class="card">
							<t-list :split="false">
								<t-list-item>
									<t-list-item-meta :title="post?.user.username">
										<template #image>
											<IdentAvatar :ident="post?.user.username"></IdentAvatar>
										</template>
										<template #description>
											<p class="text-sm">{{ dayjs(post.user.createdAt).format('YYYY-MM-DD HH:mm:ss') }}</p>
										</template>
									</t-list-item-meta>
								</t-list-item>
							</t-list>
						</t-card>
					</div>
					<div class="mb-4">
						<t-card class="card">
							<t-button variant="text" :theme="post.isLike ? 'primary' : 'default'" ghost @click="onLikePost">
								<span class="flex items-center">
									<t-icon name="thumb-up" style="font-size: 32px;" />
									<span>{{ post.likeCount }}</span>
								</span>
							</t-button>
						</t-card>
					</div>
				</div>
			</div>
		</div>
	</main>
</template>