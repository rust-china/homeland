<script lang="tsx" setup>
import { ref, computed } from 'vue'
import MarkdownRender from './MarkdownRender.vue'
import axios from 'axios'

const helpText = ref('')
axios.get('/help.md').then(({ data: rData }) => {
  helpText.value = rData
})
const currentValue = computed(() => {
  return helpText.value
})


</script>

<template>
  <main class="markdown-editor w-full flex flex-col">
    <div class="flex-1 flex gap-5 w-full">
      <div class="flex flex-1 flex-shrink-0">
        <t-textarea class="editor h-full" v-model="currentValue" readonly placeholder="请输入markdown语法"></t-textarea>
      </div>
      <div ref="markdownRef" class="flex-1 border border-sky-500 overflow-auto p-3">
        <MarkdownRender :markdown="currentValue"></MarkdownRender>
      </div>
    </div>
  </main>
</template>

<style lang="scss" scoped>
.markdown-editor {
  min-height: 700px;

  ::v-deep(textarea) {
    height: 100%;
  }
}
</style>