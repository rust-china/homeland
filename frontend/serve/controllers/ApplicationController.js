const KoaRouter = require('koa-router')
class ApplicationController extends KoaRouter {
	constructor () {
		super(...arguments)
		this.use(this.authorize)
	}

	async authorize (ctx, next) {
		await next()
	}
}
module.exports = ApplicationController