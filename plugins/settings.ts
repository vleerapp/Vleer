import { getDb, saveDb } from '~/services/db'
import type { Settings, Song } from '~/types/types'

export default defineNuxtPlugin(async (nuxtApp) => {
  const db = await getDb()

  const defaultSettings: Settings = {
    api_url: "https://api.vleer.app",
    current_song: null,
    eq: {
      "1000": "0.0",
      "125": "0.0",
      "16000": "0.0",
      "2000": "0.0",
      "250": "0.0",
      "32": "0.0",
      "4000": "0.0",
      "500": "0.0",
      "64": "0.0",
      "8000": "0.0",
    },
    lossless: true,
    loop: false,
    muted: false,
    queue: [],
    shuffle: false,
    streaming: true,
    volume: 0.5,
  }

  const settingsCollection = db.getCollection('settings')
  if (settingsCollection.count() === 0) {
    settingsCollection.insert(defaultSettings)
    await saveDb()
  }

  function getSettings(): Settings {
    const existingSettings = settingsCollection.findOne({}) as Settings
    if (existingSettings) {
      return { ...defaultSettings, ...existingSettings }
    }
    return defaultSettings
  }

  async function updateSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
    const settingsCollection = db.getCollection('settings');
    const settings = getSettings(); // Use getSettings to ensure we have all fields
    settings[key] = value;
    settingsCollection.update(settings);
    await saveDb();
  }

  const settings = {
    getApiUrl(): string {
      return getSettings().api_url
    },
    getCurrentSong(): Song | null {
      return getSettings().current_song
    },
    getEq(): { [key: string]: string } {
      return getSettings().eq
    },
    getLossless(): boolean {
      return Boolean(getSettings().lossless)
    },
    getLoop(): boolean {
      return Boolean(getSettings().loop)
    },
    getMuted(): boolean {
      return Boolean(getSettings().muted)
    },
    getQueue(): Song[] {
      return getSettings().queue
    },
    getShuffle(): boolean {
      return Boolean(getSettings().shuffle)
    },
    getStreaming(): boolean {
      return Boolean(getSettings().streaming)
    },
    getVolume(): number {
      return Number(getSettings().volume)
    },
    async setApiUrl(api_url: string) {
      await updateSetting('api_url', api_url)
    },
    async setCurrentSong(current_song: Song | null) {
      await updateSetting('current_song', current_song)
    },
    async setEq(eq: { [key: string]: string }) {
      await updateSetting('eq', eq)
    },
    async setLossless(lossless: boolean) {
      await updateSetting('lossless', lossless)
    },
    async setLoop(loop: boolean) {
      await updateSetting('loop', loop)
    },
    async setMuted(muted: boolean) {
      await updateSetting('muted', muted)
    },
    async setQueue(queue: Song[]) {
      await updateSetting('queue', queue)
    },
    async setShuffle(shuffle: boolean) {
      await updateSetting('shuffle', shuffle)
    },
    async setStreaming(streaming: boolean) {
      await updateSetting('streaming', streaming)
    },
    async setVolume(volume: number) {
      await updateSetting('volume', Math.max(0, Math.min(1, volume)))
    },
  }

  return {
    provide: {
      settings,
    }
  }
})