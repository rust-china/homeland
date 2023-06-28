const fs = require('fs')
const path = require('path')
const vite = require('vite')
const koaConnect = require('koa-connect')

const projectPath = path.resolve(__dirname, '../../');

module.exports = async function useSSR(app) {
	// 以中间件模式创建 Vite 应用，这将禁用 Vite 自身的 HTML 服务逻辑
	// 并让上级服务器接管控制
	const viteServer = await vite.createServer({
		root: projectPath,
		logLevel: 'info',
		server: {
			middlewareMode: true,
			watch: {
				// During tests we edit the files too fast and sometimes chokidar
				// misses change events, so enforce polling for consistency
				usePolling: true,
				interval: 100
			},
		},
		appType: 'custom'
	})
	app.use(koaConnect(viteServer.middlewares))

	// 1. 读取 index.html
  const indexTemplate = fs.readFileSync(path.resolve(projectPath, 'index.html'), 'utf-8')
	app.use(async (ctx, next) => {
    await next()
    if (ctx.body) { return; }

    ctx.ssrData ||= ctx
    try {
      // 2. 应用 Vite HTML 转换。这将会注入 Vite HMR 客户端，
      //    同时也会从 Vite 插件应用 HTML 转换。
      //    例如：@vitejs/plugin-react 中的 global preambles
      let template = await viteServer.transformIndexHtml(ctx.path, indexTemplate)
      // 3. 加载服务器入口。vite.ssrLoadModule 将自动转换
      //    你的 ESM 源码使之可以在 Node.js 中运行！无需打包
      //    并提供类似 HMR 的根据情况随时失效。
      const { render } = await viteServer.ssrLoadModule('/src/entry-server.ts')
      // 4. 渲染应用的 HTML。这假设 entry-server.js 导出的 `render`
      //    函数调用了适当的 SSR 框架 API。
      //    例如 ReactDOMServer.renderToString()
      const { renderedHtml, preloadLinks, ssrState } = await render(ctx, {})
      // 5. 注入渲染后的应用程序 HTML 到模板中。
			const html = template.replace('<!--ssr-outlet-->', renderedHtml)
        .replace('<!--ssr-preload-links-->', preloadLinks + `<script>window.__SSR_GLOBAL_STATE__ = ${ssrState};</script>`)
      ctx.type = 'text/html'
      ctx.body = html
    } catch (e) {
      // 如果捕获到了一个错误，让 Vite 来修复该堆栈，这样它就可以映射回
      // 你的实际源码中。
      viteServer.ssrFixStacktrace(e)
      ctx.throw(500, e.stack)
    }
  })
}