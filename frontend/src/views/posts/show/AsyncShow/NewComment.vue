<script lang="tsx" setup>
import MarkdownVditor from '@/components/shared/MarkdownVditor.vue'
import { useForm } from '@/utils/hooks/useForm';
import { graphqlApi } from '@/api'
import { MessagePlugin } from 'tdesign-vue-next';
import { emitter } from '@/utils/emitter'

const route = useRoute()
const formState = useForm({
	model: {
		postUuid: route.params.uuid,
		body: '',
	},
	rules: {
		body: [{ required: true, message: '请输入Markdownd内容' }]
	},
	async onSubmit() {
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		const { data: rData } = await graphqlApi({
        query: `
        mutation GCommentCreate($input: GraCommentCreate!) {
          commentCreate(input: $input)
        }
      `,
        variables: {
          input: formState.model,
        }
      })
			emitter.emit('newComment', rData.data.commentCreate)
			formState.onReset()
      MessagePlugin.success("创建成功")
	}
})
</script>

<template>
	<t-form :ref="formState.setFormRef" :data="formState.model" :rules="formState.rules" label-align="top" :labelWidth="0" @reset="formState.onReset" @submit="formState.onSubmit">
		<t-form-item label="回复" name="body">
			<MarkdownVditor v-model="formState.model.body" :options="{ minHeight: 400 }"></MarkdownVditor>
		</t-form-item>
		<t-form-item>
			<t-space>
				<t-button theme="primary" type="submit" :loading="formState.submitLoading">提交回复</t-button>
				<!-- <t-button theme="default" variant="base" type="reset">重置</t-button> -->
			</t-space>
		</t-form-item>
	</t-form>
</template>

<style lang="scss" scoped></style>