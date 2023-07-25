<script lang="tsx" setup>
import { watchEffect, nextTick } from 'vue'
import { marked } from 'marked'
// import { markedHighlight } from "marked-highlight";
// import '@/assets/stylesheets/syntect-highlight-code/syntect-highlight-code.scss'
import DOMPurify from 'isomorphic-dompurify';
import mermaid from 'mermaid'
import hljs from 'highlight.js'
import 'github-markdown-css/github-markdown.css'

marked.use({
	mangle: false,
	headerIds: false,
});
// marked.use(markedHighlight({
//   langPrefix: 'hljs language-',
//   highlight(code, lang) {
//     const language = hljs.getLanguage(lang) ? lang : 'plaintext';
//     return hljs.highlight(code, { language }).value;
//   }
// }));

const props = defineProps({
	html: String,
	markdown: String
})

const htmlText = ref('')
const htmlRef = ref<any>()

watchEffect(() => {
	if (props.html) {
		// eslint-disable-next-line no-misleading-character-class
		htmlText.value = DOMPurify.sanitize(props.html.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, ''))
	} else if (props.markdown) {
		// eslint-disable-next-line no-misleading-character-class
		htmlText.value = DOMPurify.sanitize(marked.parse(props.markdown.replace(/^[\u200B\u200C\u200D\u200E\u200F\uFEFF]/, '')))
	}

	nextTick(() => {
		if (htmlRef.value) {
			htmlRef.value.querySelectorAll('pre code').forEach((el: HTMLElement) => {
				if (!el.classList.contains('language-mermaid')) {
					hljs.highlightElement(el)
				}
			})
			mermaid.initialize({ startOnLoad: false })
			mermaid.run({
				nodes: htmlRef.value.querySelectorAll('pre code.language-mermaid'),
			});
		}
	})
})

</script>

<template>
	<main class="html-render markdown-body">
		<div ref="htmlRef" v-html="htmlText"></div>
	</main>
</template>

<style lang="scss">
@import 'highlight.js/scss/github.scss';

html[theme-mode=dark] {
	@import 'highlight.js/scss/github-dark.scss';
}
</style> 