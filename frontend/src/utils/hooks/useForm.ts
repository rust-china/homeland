import type { Ref, UnwrapNestedRefs } from 'vue'
import type { FormInstanceFunctions, FormResetParams, SubmitContext, FormValidateParams, FormValidateResult, Data } from 'tdesign-vue-next'
import { Form as TForm } from 'tdesign-vue-next'

export function useForm(params: FormParams) {
	const formState: UnwrapNestedRefs<Form> = reactive<Form>({
		...params,
		setFormRef: (formRef: any) => {
			formState.formRef = formRef as any as TFormType
		},
		formRef: undefined,
		model: params.model ?? {},
		rules: {},
		submitLoading: false,
		async validate(params?: FormValidateParams) {
			return formState.formRef?.validate(params)
		},
		onClear() {
			formState.formRef?.clearValidate()
		},
		onReset(params?: FormResetParams<Data>) {
			formState.formRef?.reset(params)
			// form.value.reset();
			// form.value.reset({ type: 'initial' });
			// form.value.reset({ type: 'empty' });
			// form.value.reset({ type: 'initial', fields: ['name'] });
			// form.value.reset({ type: 'empty', fields: ['name'] });
		},
		async onSubmit({ firstError }: SubmitContext, ...args: any[]) {
			if (firstError) {
				return console.log('Errors: ', firstError);
			}
			formState.submitLoading = true
			try {
				await params.onSubmit(...args)
			} finally {
				formState.submitLoading = false
			}
		}
	})
	return formState
}

export type TFormType = InstanceType<typeof TForm> & FormInstanceFunctions;

export interface FormParams extends Record<string, any> {
	model?: Record<string, any>;
	rules?: Record<string, any[]>;
	onSubmit: (...args: any[]) => Promise<void>
}

export interface Form extends Record<string, any> {
	model: Record<string, any>;
	rules: Record<string, any[]>;
	setFormRef: (formRef: any) => void;
	formRef: Ref<TFormType> | undefined;
	submitLoading: boolean;
	validate: (params?: FormValidateParams) => Promise<FormValidateResult<Data> | undefined>;
	onClear: () => void;
	onReset: (params?: FormResetParams<Data>) => void;
	onSubmit: (context: SubmitContext, ...args: any[]) => Promise<void>;
}