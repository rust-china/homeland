import { basename } from 'node:path'
import { createApp } from './main'
import { renderToString } from '@vue/server-renderer'

// if (import.meta.env.SSR) {
// 	// ... 仅在服务端执行的逻辑
// }

export const render = async (ctx: Record<string, any>, manifest: Record<string, any>) => {
	const { app, router, pinia } = createApp()

	await router.push(ctx.path)
	await router.isReady()

	const ssrState = JSON.stringify({
		piniaState: pinia.state.value
	})

	// 注入vue ssr中的上下文对象，获取方法useSSRContext()
	const renderContext = {
		ssrPath: ctx.path,
		ssrData: ctx.ssrData
	} as Record<string, any>
	const renderedHtml = await renderToString(app, renderContext)
	const preloadLinks = renderPreloadLinks(renderContext.modules, manifest)
	return { renderedHtml, preloadLinks, ssrState };
}

function renderPreloadLinks(modules: Record<string, any>, manifest: Record<string, any>) {
	let links = ''
	const seen = new Set()
	modules.forEach((id: string) => {
		const files = manifest[id]
		if (files) {
			files.forEach((file: string) => {
				if (!seen.has(file)) {
					seen.add(file)
					const filename = basename(file)
					if (manifest[filename]) {
						for (const depFile of manifest[filename]) {
							links += renderPreloadLink(depFile)
							seen.add(depFile)
						}
					}
					links += renderPreloadLink(file)
				}
			})
		}
	})
	return links
}

function renderPreloadLink(file: string) {
	if (file.endsWith('.js')) {
		return `<link rel="modulepreload" crossorigin href="${file}">`
	} else if (file.endsWith('.css')) {
		return `<link rel="stylesheet" href="${file}">`
	} else if (file.endsWith('.woff')) {
		return ` <link rel="preload" href="${file}" as="font" type="font/woff" crossorigin>`
	} else if (file.endsWith('.woff2')) {
		return ` <link rel="preload" href="${file}" as="font" type="font/woff2" crossorigin>`
	} else if (file.endsWith('.gif')) {
		return ` <link rel="preload" href="${file}" as="image" type="image/gif">`
	} else if (file.endsWith('.jpg') || file.endsWith('.jpeg')) {
		return ` <link rel="preload" href="${file}" as="image" type="image/jpeg">`
	} else if (file.endsWith('.png')) {
		return ` <link rel="preload" href="${file}" as="image" type="image/png">`
	} else {
		// TODO
		return ''
	}
}