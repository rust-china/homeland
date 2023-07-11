<script lang="tsx">
import { graphqlApi } from '@/api'
import DOMPurify from 'isomorphic-dompurify';
import 'github-markdown-css/github-markdown.css'

export default defineComponent({
	async setup(_ctx) {
		const route = useRoute();
		console.log('setup', route.name)
		const post = ref<any>(null)
		const fetchPost = async () => {
			const { data: rData } = await graphqlApi({
				query: `
					query PostQuery($uuid: String!) {
						post(uuid: $uuid) {
							uuid,
							title,
							user,
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

		await fetchPost()
		return { route, post }
	}
})
</script>

<template>
	<main class="post">
		<div class="post-body main-container px-0 mt-3 md:px-4 lg:px-0">
			<div class="flex flex-col gap-4 lg:flex-row">
				<div class="grow lg:w-3/4">
					<t-card bordered class="card">
						<template #title>
							{{ post?.title }}
						</template>
						<template #description>
							{{ post?.user }}
						</template>
						<div class="html-render markdown-body" v-html="post?.bodyHtml"></div>
					</t-card>
				</div>
				<div class="w-full lg:w-1/4">
					<div class="mb-4">
						<t-card>
							{{ post?.user }}
						</t-card>
					</div>
					<div class="mb-4">
						<t-card>
							点赞
						</t-card>
					</div>
				</div>
			</div>
		</div>
	</main>
</template>

<style lang="scss" scoped>
.card {
	@apply border-none bg-white shadow-sm sm:rounded-lg;
	@apply bg-white;
	@apply dark:bg-gray-900 dark:border dark:border-solid dark:border-gray-800;
}
</style>