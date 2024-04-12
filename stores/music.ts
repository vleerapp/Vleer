import { defineStore } from 'pinia'

export const useMusicStore = defineStore('musicStore', {
  state: () => ({
    num: 1
  }),
  actions: {
    increase() {
      this.num++
    },
  }
})