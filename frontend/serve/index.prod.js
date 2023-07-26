const path = require('path')
const  { File, Blob } = require('web-file-polyfill')
require('dotenv').config({ path: path.resolve(process.cwd(), '.env.local') })
global.File = File
global.Blob = Blob

const Koa = require('koa')
// middlewares
const catchError = require('./middlewares/catchError')
const responseTime = require('./middlewares/responseTime')
const logger = require('./middlewares/logger')
const cookie = require('./middlewares/cookie')
const useRouter = require('./middlewares/useRouter')
const useProdSSR = require('./middlewares/useSSR.prod')

const app = new Koa()
async function startApp(port) {
	app.use(catchError)
	app.use(responseTime)
	app.use(logger)
	app.use(cookie)	
	await useRouter(app)
	await useProdSSR(app)
	
	app.listen(port, () => {
		console.log(`server is listening in http://localhost:${port}`)
	})
}

startApp(9000)