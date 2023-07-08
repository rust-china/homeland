<script lang="tsx" setup>
import { useForm } from '@/utils/hooks/useForm';
import CategoryCascader from '@/components/selects/CategoryCascader.vue'
import MarkdownEditor from '@/components/shared/MarkdownEditor.vue'
import { useMutation, gql } from '@/utils/graphql'
import { MessagePlugin } from 'tdesign-vue-next';

const { mutate: createPost } = useMutation(gql`
	mutation CreatePost($input: CreatePost!) {
		createPost(input: $input)
	}
`)

const formState = useForm({
  model: {
    body: ''
  },
  async onSubmit() {
    await createPost?.({
      input: formState.model
    })
    MessagePlugin.success("创建成功")
  }
})

</script>

<template>
  <main class="posts-new">
    <t-form :ref="formState.setFormRef" :data="formState.model" :rules="formState.rules" :requiredMark="false"
      :labelWidth="0" @reset="formState.onReset" @submit="formState.onSubmit">
      <div class="flex gap-5">
        <t-form-item name="categoryId" :rules="[{ required: true, number: true, message: '分类必填' }]">
          <CategoryCascader v-model="formState.model.categoryId"></CategoryCascader>
        </t-form-item>
        <t-form-item class="flex-1" name="title" :rules="[{ required: true, message: '标题必填' }]">
          <t-input v-model="formState.model.title" placeholder="标题"></t-input>
        </t-form-item>
      </div>

      <t-form-item name="body" :rules="[{ required: true, message: '内容必填' }]">
        <MarkdownEditor v-model="formState.model.body"></MarkdownEditor>
      </t-form-item>
      <t-form-item>
        <t-space size="small">
          <t-button theme="primary" type="submit">提交</t-button>
          <t-button theme="default" variant="base" type="reset">重置</t-button>
          <t-button theme="default" variant="base" @click=" formState.onClear">清空校验结果</t-button>
        </t-space>
      </t-form-item>
    </t-form>
  </main>
</template>

<style lang="scss" scoped></style>