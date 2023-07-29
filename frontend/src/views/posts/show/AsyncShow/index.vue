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

		const onLikePostRequesting = ref(false)
		const onLikePost = async () => {
			onLikePostRequesting.value = true
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
			}).finally(() => {
				onLikePostRequesting.value = false
			})
			Object.assign(post.value, rData.data.postLike)
		}

		await fetchPost()
		return { dayjs, route, router, userInfo, userStore, post, onLikePost, onLikePostRequesting }
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
						<div class="title font-bold text-lg">
							<span>{{ post?.title }}</span>
						</div>
						<div class="description mb-5">
							<t-space size="small">
								<span class="opacity-50">{{ post.user.name || post.user.username }}</span>
								<span class="opacity-70">发表于：{{ dayjs(post.createdAt).format('YYYY-MM-DD HH:mm:ss') }}</span>
							</t-space>
						</div>
						<t-divider></t-divider>
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
							<div class="auther-info flex gap-3">
								<div class="left-panel">
									<IdentAvatar class="h-[50px] rounded" :ident="post?.user.username"></IdentAvatar>
								</div>
								<div class="right-panel">
									<div class="auther-name text-base font-bold">{{ post?.user.username }}</div>
									<div class="text-xs my-1">{{ dayjs(post?.user.created_at).format('YYYY-MM-DD HH:mm:ss') }}</div>
								</div>
							</div>
						</t-card>
					</div>
					<div class="mb-4">
						<t-card class="card">
							<t-button variant="text" :theme="post.isLike ? 'primary' : 'default'" @click="onLikePost" :disabled="onLikePostRequesting">
								<span class="flex items-center">
									<t-icon name="thumb-up" style="font-size: 28px;" />
									<span class="font-bold">{{ post.likeCount }}</span>
								</span>
							</t-button>
						</t-card>
					</div>
				</div>
			</div>
		</div>
	</main>
</template>