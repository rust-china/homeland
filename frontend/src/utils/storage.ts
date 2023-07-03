export const customLocalStorage = {
	setItem(key: string, value: any) {
		return import.meta.env.SSR ? undefined : localStorage.setItem(key, value)
	},
	getItem(key: string) {
		return import.meta.env.SSR ? null : localStorage.getItem(key)
	},
}