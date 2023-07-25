import { useSSRContext as useSSRContextVue } from "vue"

export const useSSRContext = () => {
	return import.meta.env.SSR ? useSSRContextVue() : null
}