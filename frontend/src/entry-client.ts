import { createApp } from './main'

import './assets/stylesheets/application.scss'
import 'tdesign-vue-next/es/style/index.css';

const { app, router, pinia } = createApp()

router.isReady().then(() => {
	app.mount('#app', true)
})

// 初始化 global state
if (window.__SSR_STATE__) {
	const ssrState = Object.assign({
		piniaState: null
	}, window.__SSR_STATE__)
	if (ssrState.piniaState) { // 初始化pinia
		// console.log('init ssr pinia state', ssrState.piniaState)
		pinia.state.value = ssrState.piniaState
	}
}


const setThemeMode = () => {
	if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
		document.documentElement.setAttribute('theme-mode', 'dark');
	} else {
		document.documentElement.setAttribute('theme-mode', 'light');
	}
}
window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', setThemeMode)
setThemeMode()