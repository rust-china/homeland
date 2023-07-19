import {
	getCurrentInstance, 
	createVNode, 
	defineComponent,
	reactive,
	ref,
	computed,
	provide,
	inject
} from 'vue'
import { Dialog as TDialog } from 'tdesign-vue-next'

const DialogComponent = defineComponent({
	props: {
		options: {
			type: Object,
			required: true
		},
		configProvider: [Object]
	},
	emits: ['update:modelValue'],
	setup(props) {
		const { slots = {}, ...options } = props.options as DialogOptions
		const dialogState = reactive({
			...options,
			'onUpdate:modelValue'(value: boolean) {
				dialogState.modelValue = value
				if (typeof options['onUpdate:modelValue'] === 'function') {
					options['onUpdate:modelValue'](value)
				}
			},
			onClose() {
				dialogState['onUpdate:modelValue'](false)
			}
		})
		const dialogSlots = reactive(slots)
		provide(
			'dialogState',
			new Proxy(dialogState, {
				get(target, key, ...args) {
					if (key === 'close') {
						return () => target['onUpdate:modelValue'](false)
					}
					return Reflect.get(target, key, ...args)
				}
			})
		)
		provide('configProvider', props.configProvider)
		provide('dialogSlots', dialogSlots)

		return {
			dialogState,
			dialogSlots
		}
	},
	render() {
		const dialog = createVNode(TDialog, this.dialogState, this.dialogSlots)
		return dialog
	}
})


export const useDialog = (defaultOptions: DialogOptions | {} = {}) => {
	const instance = getCurrentInstance()
	console.assert(!!instance, 'getCurrentInstance无法获取到实例，请检查')
	const configProvider = inject('configProvider', null)
	const app = instance!.appContext.app

	return {
		create(options: DialogOptions) {
			const div = document.createElement('div')
			div.setAttribute('class', 'use-dialog')
			document.body.appendChild(div)

			const modelValue = ref(true)
			const slotsCache = {}

			const modalVNode = createVNode(DialogComponent, {
				options: {
					destroyOnClose: true,
					...defaultOptions,
					...options,
					modelValue,
					visible: computed(() => modelValue.value),
					slots: Object.assign(
						{},
						...Object.keys(options.slots).map((key) => {
							return {
								[key]: () => {
									if (typeof (options.slots as any)?.[key] === 'function') {
										; (slotsCache as any)[key] ||= (options.slots as any)[key]()
									}
									return (slotsCache as any)[key]
								}
							}
						})
					),
					onClosed() {
						app.render(null, div)
						div.parentNode?.removeChild(div)
						if (typeof options.onClosed === 'function') {
							options.onClosed()
						}
					}
				},
				configProvider
			})
			modalVNode.appContext = instance!.appContext
			app.render(modalVNode, div)

			return {
				close() {
					modelValue.value = false
					// modalVNode.component.setupState.close()
				}
			}
		}
	}
}

export interface DialogOptions extends Record<string, any> {
	modelValue?: boolean
}