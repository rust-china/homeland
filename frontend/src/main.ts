import { createSSRApp } from 'vue'
import { createPinia } from './stores'
import router from './router'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { client } from '@/utils/graphql'

import App from './App.vue'

export const createApp = () => {
	const app = createSSRApp({
		setup() {
			provide(DefaultApolloClient, client)
		},
		render: () => h(App)
	})
	// const router = createRouter()
	const pinia = createPinia()

	app.use(router)
	app.use(pinia)

	return { app, router, pinia }
}