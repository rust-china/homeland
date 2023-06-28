const fs = require('fs')
const path = require('path')
const koaStatic = require('koa-static')

const projectPath = path.resolve(__dirname, '../../');
module.exports = async function useSSR(app) {
	app.use(koaStatic(path.resolve(projectPath, 'dist/client'), {
		maxAge: 1000 * 60 * 60 * 24 * 7,
		gzip: true,
		index: false
	}))

	const { render } = await import(path.resolve(projectPath, 'dist/server/entry-server.mjs'))
	const manifest = require(path.resolve(projectPath, 'dist/client/ssr-manifest.json'))
	const indexTemplate = fs.readFileSync(path.resolve(projectPath, 'dist/client/index.html'), 'utf-8')

	app.use(async (ctx, next) => {
		await next()
		if (ctx.body) return;
		
		ctx.ssrData ||= ctx
		const {renderedHtml, preloadLinks, ssrGlobalState} = await render(ctx, manifest)
		const html = indexTemplate.replace('<!--ssr-outlet-->', renderedHtml)
			.replace('<!--ssr-preload-links-->', preloadLinks + `<script>window.__SSR_GLOBAL_STATE__ = ${ssrGlobalState};</script>`)
		ctx.type = 'text/html'
		ctx.body = html
	})
}