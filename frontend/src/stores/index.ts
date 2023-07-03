import { createPinia as createVuePinia } from 'pinia'
import { createPersistedState } from 'pinia-plugin-persistedstate'
import { customLocalStorage } from '@/utils/storage'

export const createPinia = () => {
  const pinia = createVuePinia()
  if (!import.meta.env.SSR) {
    pinia.use(
      createPersistedState({
        storage: customLocalStorage,
        key: (id) => `__persisted__${id}`,
        beforeRestore: (ctx) => {
          if (import.meta.env.DEV) {
            console.info(`beforeRestore: ${ctx.store.$id}`)
          }
        },
        afterRestore: (ctx) => {
          if (import.meta.env.DEV) {
            console.info(`afterRestore: ${ctx.store.$id}`)
          }
        },
        debug: import.meta.env.DEV
      })
    )
  }
  return pinia
}

export default createPinia()
