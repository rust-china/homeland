import { defineStore } from 'pinia'
import { customLocalStorage } from '@/utils/storage'

export const useUserStore = defineStore('user', {
  state: (): any => ({
    userInfo: null,
  }),
  getters: {
    isLogin: (state) => !!state.authInfo,
  },
  actions: {
    setUserInfo(userInfo: any): void {
      this.userInfo = userInfo
    },
  },
  persist: [
    {
      paths: ['userInfo'],
      storage: customLocalStorage
    }
  ]
})
