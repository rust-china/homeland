<script lang="tsx" setup>
import Vditor from 'vditor'
import { usePreferredDark } from '@vueuse/core'
import 'vditor/dist/index.css'
import 'vditor/dist/css/content-theme/light.css'
import 'vditor/dist/css/content-theme/dark.css'

const props = defineProps({
	modelValue: {
		type: String,
		required: true,
	},
	options: [Object],
})
const emits = defineEmits(['update:modelValue'])

let vditor: Vditor | null = null
const vditorRef = ref()
const isDarkTheme = usePreferredDark()

onMounted(() => {
	vditor = new Vditor(vditorRef.value, {
		// icon: 'material',
		width: '100%',
		height: '100%',
		mode: 'ir',
		theme: isDarkTheme.value ? 'dark' : 'classic',
		tab: '\t',
		placeholder: '请输入Markdown语法',
		cache: {
			enable: false,
		},
		counter: {
			enable: false,
		},
		resize: {
			enable: true
		},
		toolbar: [
			"emoji",
			"headings",
			"bold",
			"italic",
			"strike",
			"link",
			"|",
			"list",
			"ordered-list",
			"check",
			"outdent",
			"indent",
			"|",
			"quote",
			"line",
			"code",
			"inline-code",
			"insert-before",
			"insert-after",
			"|",
			"upload",
			"record",
			"table",
			"|",
			"undo",
			"redo",
			"|",
			"fullscreen",
			"edit-mode",
			// {
			// 	name: "more",
			// 	toolbar: [
			// 		"both",
			// 		"code-theme",
			// 		"content-theme",
			// 		"export",
			// 		"outline",
			// 		"preview",
			// 		"devtools",
			// 		"info",
			// 		"help",
			// 	],
			// },
		],
		toolbarConfig: {
			pin: true,
		},
		outline: {
			enable: true,
			position: 'right'
		},
		upload: {
			url: '/api/storage/upload',
			multiple: false,
			fieldName: 'file',
			format: (files: File[], response: string) => {
				let json = JSON.parse(response)
				json.data.succMap = { [json.data.filename]: json.data.path }
				return JSON.stringify(json)
			}
		},
		preview: {
			delay: 300,
			actions: [],
			hljs: {
				defaultLang: 'Rust'
			}
		},
		after: () => {
			// vditor.value is a instance of Vditor now and thus can be safely used here
			vditor!.setValue(props.modelValue);
		},
		input: (value: string) => {
			emits('update:modelValue', value)
		},
		...props.options
	})

	nextTick(() => {
		setTimeout(() => {
			watchEffect(() => {
				vditor?.setTheme(isDarkTheme.value ? 'dark' : 'classic')
				if (props.modelValue !== vditor!.getValue()) {
					vditor?.setValue(props.modelValue)
				}
			})
		}, 500)
	})


})
</script>

<template>
	<main class="markdown-vditor html-render w-full">
		<div class="editor" ref="vditorRef"></div>
	</main>
</template>

<style lang="scss" scoped>
.markdown-vditor {
	::v-deep(.vditor-reset) {
		color: #24292e;
	}

	::v-deep(.vditor--dark) {
		.vditor-reset {
			color: #d1d5da;
		}
	}
}
</style>