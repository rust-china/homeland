import { defineStore } from 'pinia'
import { customLocalStorage } from '@/utils/storage'
import apiAxios from '@/utils/apiAxios'

export const useUserStore = defineStore('user', {
  state: (): any => ({
    userInfo: null,
  }),
  getters: {
    isLogin: (state) => !!state.authInfo,
  },
  actions: {
    async fetchUserInfo() {
      try {
        const { data: rData } = await apiAxios.get('/auth/user')
        this.setUserInfo(rData)
      } catch(e) {
        this.setUserInfo(null)
      }
      
    },
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
