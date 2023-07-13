<script lang="tsx" setup>
import { marked } from 'marked'
import 'github-markdown-css/github-markdown.css'
import DOMPurify from 'isomorphic-dompurify';
import { markedHighlight } from "marked-highlight";
import hljs from 'highlight.js'

marked.use({
  mangle: false,
  headerIds: false,
});
marked.use(markedHighlight({
  langPrefix: 'hljs language-',
  highlight(code, lang) {
    const language = hljs.getLanguage(lang) ? lang : 'plaintext';
    return hljs.highlight(code, { language }).value;
  }
}));

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
})
const emits = defineEmits(['update:modelValue'])


const activeTab = ref('editor')
const currentValue = computed({
  get: () => props.modelValue || '',
  set: nv => emits('update:modelValue', nv)
})
const htmlText = ref('')

watchEffect(() => {
  // eslint-disable-next-line no-misleading-character-class
  htmlText.value = DOMPurify.sanitize(marked.parse(currentValue.value.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, '')))
})

</script>

<template>
  <main class="markdown-editor w-full flex gap-5">
    <t-tabs class="w-full" v-model="activeTab" theme="card">
      <t-tab-panel value="editor" label="输入" destroyOnHide>
        <div class="py-5">
          <t-textarea class="editor flex-1" v-model="currentValue"
            placeholder="请输入markdown语法"></t-textarea>
        </div>

      </t-tab-panel>
      <t-tab-panel value="preview" label="预览" destroyOnHide>
        <div class="py-5">
          <div class="html-render flex-1 markdown-body" v-html="htmlText"></div>
        </div>
      </t-tab-panel>
      <t-tab-panel value="editorAndPreview" label="编辑/预览" destroyOnHide>
        <div class="py-5 flex gap-5">
          <t-textarea class="editor flex-1" v-model="currentValue"
            placeholder="请输入markdown语法"></t-textarea>
          <div class="html-render flex-1 markdown-body border border-sky-500" v-html="htmlText"></div>
        </div>
      </t-tab-panel>
    </t-tabs>
  </main>
</template>

<style lang="scss">
@import 'highlight.js/scss/github.scss';

html[theme-mode=dark] {
  @import 'highlight.js/scss/github-dark.scss';
}

.html-render {
  padding: 15px;
}
</style> 