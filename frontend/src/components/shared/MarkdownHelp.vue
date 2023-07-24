<script lang="tsx" setup>
import { ref, computed, watchEffect } from 'vue'
import { marked } from 'marked'
import 'github-markdown-css/github-markdown.css'
import DOMPurify from 'isomorphic-dompurify';
import { markedHighlight } from "marked-highlight";
import mermaid from 'mermaid'
import hljs from 'highlight.js'
import axios from 'axios'

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

// const props = defineProps({
//   modelValue: {
//     type: String,
//     required: true,
//   },
// })
// const emits = defineEmits(['update:modelValue'])
// const currentValue = computed({
//   get: () => props.modelValue || '',
//   set: nv => emits('update:modelValue', nv)
// })
const helpText = ref('')
axios.get('/help.md').then(({ data: rData }) => {
  helpText.value = rData
})
const currentValue = computed(() => {
  return helpText.value
})

const htmlText = ref('')
const markdownRef = ref<any>()
watchEffect(() => {
  // eslint-disable-next-line no-misleading-character-class
  htmlText.value = DOMPurify.sanitize(marked.parse(currentValue.value.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, '')))

  nextTick(() => {
    mermaid.initialize({ startOnLoad: false })
    mermaid.run({
      nodes: markdownRef.value?.querySelectorAll('pre code.language-mermaid'),
    });
  })
})

</script>

<template>
  <main class="markdown-editor w-full flex flex-col">
    <div class="flex-1 flex gap-5 w-full">
      <div class="flex flex-1 flex-shrink-0">
        <t-textarea class="editor h-full" v-model="currentValue" readonly placeholder="请输入markdown语法"></t-textarea>
      </div>
      <div ref="markdownRef" class="html-render markdown-body flex-1 border border-sky-500 overflow-auto p-3" v-html="htmlText"></div>
    </div>
  </main>
</template>

<style lang="scss">
@import 'highlight.js/scss/github.scss';

html[theme-mode=dark] {
  @import 'highlight.js/scss/github-dark.scss';
}
</style> 

<style lang="scss" scoped>
.markdown-editor {
  min-height: 700px;

  ::v-deep(textarea) {
    height: 100%;
  }
}
</style>