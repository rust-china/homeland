const path = require('path')
require('dotenv').config({ path: path.resolve(process.cwd(), '.env.local') })

const Koa = require('koa')
// middlewares
const catchError = require('./middlewares/catchError')
const responseTime = require('./middlewares/responseTime')
const logger = require('./middlewares/logger')
const cookie = require('./middlewares/cookie')
const useRouter = require('./middlewares/useRouter')
const useDevSSR = require('./middlewares/useSSR.dev')

const koaConnect = require('koa-connect')
const { createProxyMiddleware } = require('http-proxy-middleware')

const app = new Koa()
async function startApp(port) {
	app.use(catchError)
	app.use(responseTime)
	app.use(logger)
	app.use(cookie)	
	await useRouter(app)
	await useDevSSR(app)
	
	app.use(async (ctx, next) => {
		if (ctx.url.startsWith('/api')) {
			ctx.respond = false;
			await koaConnect(
				createProxyMiddleware({
					target: 'http://localhost:3000',
					changeOrigin: true,
					ws: true, // 配置ws跨域
					secure: false, // https
					pathRewrite: (path) => path.replace('/api', '')
				})
			)(ctx, next)
		} else {
			await next()
		}
	})

	
	app.listen(port, () => {
		console.log(`server is listening in http://localhost:${port}`)
	})
}

startApp(9000)