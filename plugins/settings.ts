import { useSettingsStore } from "~/stores/settings";
import type { EQSettings } from "~/types/types";

export default defineNuxtPlugin(async (nuxtApp) => {
  const store = useSettingsStore();
  const { $music } = useNuxtApp();

  await store.getSettings();

  const settings = {
    async init() {
      if (store.settings.currentSong != "") {
        await $music.setSong(store.settings.currentSong);
      }
    },
    getVolume(): number {
      return store.getVolume()
    },
    setVolume(volume: number) {
      store.setVolume(volume)
    },
    getEq(): EQSettings {
      return store.getEq();
    },
    setEq(eq: EQSettings) {
      store.setEq(eq)
    },
    getCurrentSong(): string {
      return store.getCurrentSong();
    },
    setCurrentSong(id: string) {
      store.setCurrentSong(id)
    },
    setApiURL(url: string) {
      store.setApiURL(url)
    },
    getApiURL() {
      if (!store.getApiURL()) {
        return "https://pipedapi.wireway.ch";
      }
      return store.getApiURL();
    },
    async searchApiURL() {
      try {
        const response = await fetch('https://piped-instances.kavin.rocks/');
        const instances = await response.json();
        const urls = instances.map((instance: { api_url: string }) => instance.api_url);

        const results = await window.__TAURI__.core.invoke('ping_urls', { urls });
        this.setApiURL(results[0][0])
        return results;
      } catch (error) {
        console.error('Failed to fetch API URLs:', error);
        return [];
      }
    },
    async getApiURLs() {
      try {
        const response = await fetch('https://piped-instances.kavin.rocks/');
        const instances = await response.json();
        const urls = instances.map((instance: { api_url: string }) => instance.api_url);
        return urls;
      } catch (error) {
        console.error('Failed to fetch API URLs:', error);
        return [];
      }
    },
    async saveSettings(){
      await store.saveSettings();
    }
  };

  return {
    provide: {
      settings,
    },
  };
});