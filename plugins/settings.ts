import { useSettingsStore } from "~/stores/settings";
import type { EQSettings } from "~/types/types";

export default defineNuxtPlugin(async (nuxtApp) => {
  const store = useSettingsStore();
  const { $music } = useNuxtApp();

  await store.getSettings();
  await $music.init();

  if (store.settings.currentSong != "") {
    $music.setSong(store.settings.currentSong);
  }

  const settings = {
    getVolume(): number {
      return store.settings.volume;
    },
    setVolume(volume: number) {
      store.settings.volume = volume;
      store.saveSettings();
    },
    getEq(): EQSettings {
      return store.settings.eq;
    },
    setEq(eq: EQSettings) {
      store.settings.eq = eq;
      store.saveSettings();
    },
    getCurrentSong(): string {
      return store.settings.currentSong;
    },
    setCurrentSong(id: string) {
      console.log(id);
      store.settings.currentSong = id;
      store.saveSettings();
    },
    setApiURL(url: string) {
      store.settings.apiURL = url;
      store.saveSettings();
    },
    getApiURL() {
      if (!store.settings.apiURL) {
        return "https://pipedapi.r4fo.com";
      }
      return store.settings.apiURL;
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