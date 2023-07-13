import type { InternalAxiosRequestConfig, AxiosResponse, AxiosError } from 'axios'
import axios from 'axios'
import { MessagePlugin } from 'tdesign-vue-next';

// 创建 apiAxios 实例
const apiAxios = new Proxy(
  axios.create({
    withCredentials: true,
    // https://cn.vitejs.dev/guide/env-and-mode.html
    baseURL: import.meta.env.VITE_APP_API_BASE_URL || '/',
    timeout: 1000 * 60,
    // paramsSerializer: (params) => qs.stringify(params, { indices: false })
  }),
  {
    get(target, ...args) {
      return Reflect.get(target, ...args) || Reflect.get(axios, ...args)
    }
  }
)

// 设置 post 请求头
apiAxios.defaults.headers.get['Content-Type'] = 'application/json;charset=UTF-8'
apiAxios.defaults.headers.post['Content-Type'] = 'application/json;charset=UTF-8'
// axios.defaults.headers.post['Content-Type'] = 'application/x-www-form-urlencoded'

// 请求拦截 - default
apiAxios.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  return config
})

apiAxios.interceptors.response.use(
  (response: AxiosResponse<any>) => {
    if (response.data.errors instanceof Array) {
      MessagePlugin.error(response.data.errors.map((err: Error) => err.message).join('\n'))
      return Promise.reject(response.data.errors)
    }
    return Promise.resolve(response)
  },
  (error: AxiosError) => {
    return Promise.reject(error)
  }
)

export default apiAxios