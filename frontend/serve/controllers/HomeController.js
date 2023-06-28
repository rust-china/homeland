const ApplicationController = require('./ApplicationController')
class HomeController extends ApplicationController {
	constructor () {
		super({prefix: '/'})

		// this.get('/', async (ctx) => {
		// 	ctx.body = 'hello'
		// })
	}
}
module.exports = HomeController