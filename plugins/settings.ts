import re from "~/dist/_nuxt/BJ90lMuY";
import { useSettingsStore } from "~/stores/settings";
import type { EQSettings } from "~/types/types";

export default defineNuxtPlugin(async (nuxtApp) => {
  const store = useSettingsStore();
  const { $music } = useNuxtApp();

  await store.getSettings();
  await $music.init();

  if (store.settings.playerSettings.currentSong != "") {
    $music.setSong(store.settings.playerSettings.currentSong);
  }

  const settings = {
    getVolume(): number {
      return store.settings.playerSettings.volume;
    },
    setVolume(volume: number) {
      store.settings.playerSettings.volume = volume;
      store.saveSettings();
    },
    getEq(): EQSettings {
      return store.settings.playerSettings.eq;
    },
    setEq(eq: EQSettings) {
      store.settings.playerSettings.eq = eq;
      store.saveSettings();
    },
    getCurrentSong(): string {
      return store.settings.playerSettings.currentSong;
    },
    setCurrentSong(song: string) {
      store.settings.playerSettings.currentSong = song;
      store.saveSettings();
    },
    setApiURL(url: string) {
      store.settings.apiURL = url;
      store.saveSettings();
    },
    getApiURL() {
      return store.settings.apiURL;
    },
    async searchApiURL() {
      try {
        const response = await fetch('https://piped-instances.kavin.rocks/');
        const instances = await response.json();
        const urls = instances.map((instance: { api_url: string }) => instance.api_url);

        const results = await window.__TAURI__.core.invoke('ping_urls', { urls });
        console.log(results[0][0]);
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
  };

  return {
    provide: {
      settings,
    },
  };
});
