import { createPinia } from 'pinia';
import { useMusicStore } from "~/stores/music";

export default defineNuxtPlugin((nuxtApp) => {
  const pinia = createPinia();
  nuxtApp.vueApp.use(pinia);

  const store = useMusicStore();

  const music = {
    increase() {
      store.increase()
    }, 
    getCounter(): number {
      return store.num
    }
  }
  
  return {
    provide: {
      music,
    },
  };
});