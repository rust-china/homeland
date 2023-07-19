import type { VNode } from 'vue'
import { createSSRApp, render } from 'vue'
import { createPinia } from './stores'
import router from './router'

import App from './App.vue'

export const createApp = () => {
	const app = createSSRApp({
		setup() {},
		render: () => h(App)
	})

	app.render = (vnode: VNode | null, rootContainer: Element): void => {
		if (vnode && !vnode.appContext) vnode.appContext = app._context
		render(vnode, rootContainer)
	}

	// const router = createRouter()
	const pinia = createPinia()

	app.use(router)
	app.use(pinia)

	return { app, router, pinia }
}