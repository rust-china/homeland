import { createSSRApp } from 'vue'
import { createPinia } from './stores'
import router from './router'

import App from './App.vue'

export const createApp = () => {
	const app = createSSRApp(App)
	// const router = createRouter()
	const pinia = createPinia()

	app.use(router)
	app.use(pinia)

	return { app, router, pinia }
}