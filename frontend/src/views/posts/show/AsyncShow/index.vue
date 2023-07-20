<script lang="tsx">
import { graphqlApi } from '@/api'
import { useUserStore } from '@/stores/user'
import DOMPurify from 'isomorphic-dompurify';
import 'github-markdown-css/github-markdown.css'
// import '@/assets/stylesheets/syntect-highlight-code/syntect-highlight-code.scss'
import hljs from 'highlight.js'
import dayjs from 'dayjs'
import Comments from './Comments.vue'
import NewComment from './components/NewComment.vue'
import IdentAvatar from '@/components/shared/IdentAvatar.vue'

export default defineComponent({
	components: {
		Comments,
		NewComment,
		IdentAvatar,
	},
	async setup(_ctx) {
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
			})

			const postData = rData.data.post
			post.value = {
				...postData,
				// eslint-disable-next-line no-misleading-character-class
				bodyHtml: DOMPurify.sanitize(postData.bodyHtml.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, ''))
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
			})
			Object.assign(post.value, rData.data.postLike)
		}

		await fetchPost()
		return { dayjs, route, router, userInfo, userStore, post, onLikePost }
	},
	mounted() {
		this.userInfo = this.userStore.userInfo
		hljs.highlightAll()
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
							{{ post?.title }}
						</template>
						<template #description>
							<t-space size="small">
								<span class="opacity-50">{{ post.user.name || post.user.username }}</span>
								<span>最后更新于：{{ dayjs(post.updatedAt).format('YYYY-MM-DD HH:mm:ss') }}</span>
							</t-space>
						</template>
						<div class="html-render markdown-body" v-html="post?.bodyHtml"></div>
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


<style lang="scss">
@import 'highlight.js/scss/github.scss';

html[theme-mode=dark] {
	@import 'highlight.js/scss/github-dark.scss';
}
</style> 