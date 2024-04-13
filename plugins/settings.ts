import { useSettingsStore } from "~/stores/settings";
import type { EQSettings } from "~/types/types";

export default defineNuxtPlugin(async (nuxtApp) => {
  const store = useSettingsStore();
  const { $music } = useNuxtApp();

  await store.getSettings();

  if (store.settings.playerSettings.currentSong != "") {
    $music.setSong(store.settings.playerSettings.currentSong)
  }
  
  const settings = {
    getVolume(): number {
      return store.settings.playerSettings.volume;
    },
    setVolume(volume: number) {
      store.settings.playerSettings.volume = volume;
      store.saveSettings()
    },
    initEQ() {
      
    },
    getEq(): EQSettings {
      return store.settings.playerSettings.eq;
    },
    setEq(eq: EQSettings) {
      store.settings.playerSettings.eq = eq;
      store.saveSettings()
    },
    getCurrentSong(): string {
      return store.settings.playerSettings.currentSong;
    },
    setCurrentSong(song: string) {
      store.settings.playerSettings.currentSong = song;
      store.saveSettings()
    }
  };

  return {
    provide: {
      settings,
    },
  };
});
