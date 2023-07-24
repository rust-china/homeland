<script lang="tsx" setup>
import Vditor from 'vditor'
import { usePreferredDark } from '@vueuse/core'
import { useDialog } from '@/utils/hooks/useDialog'
import MarkdownHelp from './MarkdownHelp.vue'
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

const dialog = useDialog()

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
			{
				name: '排版',
				tip: '排版说明',
				className: 'left',
				icon: '<svg t="1690201489775" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3200" width="200" height="200"><path d="M512 272c-35.3 0-64-28.7-64-64s28.7-64 64-64 64 28.7 64 64-28.7 64-64 64z m0 64c70.7 0 128-57.3 128-128S582.7 80 512 80s-128 57.3-128 128 57.3 128 128 128zM576 480v384c0 8.8-7.2 16-16 16h-96c-8.8 0-16-7.2-16-16V480c0-8.8 7.2-16 16-16h96c8.8 0 16 7.2 16 16z m-192-16v416c0 35.3 28.7 64 64 64h128c35.3 0 64-28.7 64-64V464c0-35.3-28.7-64-64-64H448c-35.3 0-64 28.7-64 64z" p-id="3201"></path></svg>',
				click() {
					dialog.create({
						header: '排版说明',
						width: '80%',
						cancelBtn: null,
						confirmBtn: null,
						slots: {
							default: () => (
								<div>
									<MarkdownHelp></MarkdownHelp>
								</div>
							)
						}
					})
				}
			},
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