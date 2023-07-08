<script lang="tsx" setup>
import { marked } from 'marked'
import DOMPurify from 'isomorphic-dompurify';
marked.use({
  mangle: false,
  headerIds: false,
});

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
})
const emits = defineEmits(['update:modelValue'])


const activeTab = ref('first')
const currentValue = computed({
  get: () => props.modelValue || '',
  set: nv => emits('update:modelValue', nv)
})
const htmlText = ref('')

watchEffect(() => {
  htmlText.value = DOMPurify.sanitize(marked.parse(
    // eslint-disable-next-line no-misleading-character-class
    currentValue.value.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, '')
  ))
})

</script>

<template>
  <main class="markdown-editor w-full flex gap-5">
    <t-tabs class="w-full" v-model="activeTab">
      <t-tab-panel value="first" label="输入">
        <div class="py-5">
          <t-textarea class="editor flex-1" v-model="currentValue" :autosize="{ minRows: 10 }"
            placeholder="请输入markdown语法"></t-textarea>
        </div>

      </t-tab-panel>
      <t-tab-panel value="second" label="预览">
        <div class="py-5">
          <div class="preview flex-1" v-html="htmlText"></div>
        </div>

      </t-tab-panel>
    </t-tabs>
  </main>
</template>

<style lang="scss" scoped></style>