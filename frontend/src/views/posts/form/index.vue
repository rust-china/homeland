<script lang="tsx" setup>
import { useForm } from '@/utils/hooks/useForm';
import CategoryCascader from '@/components/selects/CategoryCascader.vue'
// import MarkdownEditor from '@/components/shared/MarkdownEditor.vue'
import MarkdownVditor from '@/components/shared/MarkdownVditor.vue'
import { graphqlApi } from '@/api'
import { MessagePlugin } from 'tdesign-vue-next';
import { onMounted } from 'vue';

const route = useRoute()
const router = useRouter()
const uuid = computed(() => route.params.uuid)
const formState = useForm({
  model: {
    body: ''
  },
  async onSubmit() {
    if (uuid.value) {
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const { data: rData } = await graphqlApi({
        query: `
        mutation GPostUpdate($input: GraPostUpdate!) {
          postUpdate(input: $input)
        }
      `,
        variables: {
          input: formState.model
        }
      })
      MessagePlugin.success("更新成功")
      router.push({ name: 'posts/show', params: { uuid: uuid.value } })

    } else {
      const { data: rData } = await graphqlApi({
        query: `
        mutation GPostCreate($input: GraPostCreate!) {
          postCreate(input: $input)
        }
      `,
        variables: {
          input: formState.model
        }
      })
      MessagePlugin.success("创建成功")
      router.push({ name: 'posts/show', params: { uuid: rData.data.postCreate } })
    }

  }
})

const fetchPost = async (uuid: string) => {
  const { data: rData } = await graphqlApi({
    query: `
					query PostQuery($uuid: String!) {
						post(uuid: $uuid) {
							uuid,
							title,
              body,
							categoryId,
						}
					}
				`,
    variables: { uuid }
  })
  const postData = rData.data.post
  formState.model = postData
}

onMounted(() => {
  if (uuid.value) {
    fetchPost(uuid.value as string)
  }
})


</script>

<template>
  <main class="posts-form main-container h-full pb-5">
    <t-card class="card h-full">
      <t-form class="h-full flex flex-col" :ref="formState.setFormRef" :data="formState.model" :rules="formState.rules"
        :requiredMark="false" :labelWidth="0" @reset="formState.onReset" @submit="formState.onSubmit">
        <div class="flex gap-5">
          <t-form-item name="categoryId" :rules="[{ required: true, number: true, message: '分类必填' }]">
            <CategoryCascader v-model="formState.model.categoryId"></CategoryCascader>
          </t-form-item>
          <t-form-item class="flex-1" name="title" :rules="[{ required: true, message: '标题必填' }]">
            <t-input v-model="formState.model.title" placeholder="标题"></t-input>
          </t-form-item>
        </div>

        <t-form-item class="body-form-tem flex-1" name="body" :rules="[{ required: true, message: '内容必填' }]">
          <!-- <MarkdownEditor class="h-full" v-model="formState.model.body"></MarkdownEditor> -->
          <MarkdownVditor class="h-full" v-model="formState.model.body"></MarkdownVditor>
        </t-form-item>
        <t-form-item>
          <t-space size="small">
            <t-button theme="primary" type="submit">提交</t-button>
            <t-button theme="default" variant="base" type="reset">重置</t-button>
            <t-button theme="default" variant="base" @click=" formState.onClear">清空校验结果</t-button>
          </t-space>
        </t-form-item>
      </t-form>
    </t-card>
  </main>
</template>

<style lang="scss" scoped>
.card {
  ::v-deep(.t-card__body) {
    @apply h-full;
  }
}

.body-form-tem {
  ::v-deep(.t-form__controls) {
    @apply h-full;

    .t-form__controls-content {
      @apply h-full;
    }
  }
}
</style>