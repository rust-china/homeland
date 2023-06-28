const Routers = require('../controllers')
const ApplicationController = require('../controllers/ApplicationController')
const isObject = (v) => typeof v === 'object'
const isFunction = (v) => typeof v === 'function'

module.exports = async function useRouter(app) {
  registerAppRouter(app, Routers)
}

function registerAppRouter(app, Routers) {
  if (isObject(Routers)) {
    for (const controllerName in Routers) {
      if (isFunction(Routers[controllerName])) {
        let controller = new Routers[controllerName]()
        if (controller instanceof ApplicationController) {
          app.use(controller.routes()).use(controller.allowedMethods())
        }
      } else {
        registerAppRouter(app, Routers[controllerName])  
      }
    }
  }
}