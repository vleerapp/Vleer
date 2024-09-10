import { getDb, initDb } from '~/services/db'
import type { Settings, Song } from '~/types/types'

export default defineNuxtPlugin(async (nuxtApp) => {
  await initDb()
  const db = getDb()

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
    lossless: false,
    loop: false,
    muted: false,
    queue: [],
    shuffle: false,
    streaming: true,
    volume: 0.5,
  }

  let settingsInitialized = false
  let cachedSettings: Settings | null = null

  async function initializeSettings() {
    for (const [key, value] of Object.entries(defaultSettings)) {
      const result = await db.select<Array<{ key: string; value: string }>>('SELECT * FROM settings WHERE key = ?', [key])
      if (result.length === 0) {
        await db.execute('INSERT INTO settings (key, value) VALUES (?, ?)', [
          key,
          typeof value === 'object' ? JSON.stringify(value) : value.toString()
        ])
      }
    }
    settingsInitialized = true
    console.log('Settings initialized')
    
    cachedSettings = await fetchSettingsFromDb()
  }

  async function fetchSettingsFromDb(): Promise<Settings> {
    const result = await db.select<Array<{ key: string; value: string }>>('SELECT * FROM settings')
    const settings: Partial<Settings> = {}

    for (const row of result) {
      let value: any = row.value
      if (typeof value === 'string') {
        if (value.startsWith('{') || value.startsWith('[')) {
          value = JSON.parse(value)
        } else if (value === 'true' || value === 'false') {
          value = value === 'true'
        } else if (!isNaN(Number(value))) {
          value = Number(value)
        }
      }
      settings[row.key as keyof Settings] = value
    }

    return { ...defaultSettings, ...settings }
  }

  async function getSettings(): Promise<Settings> {
    if (!settingsInitialized) {
      await initializeSettings()
    }

    if (cachedSettings) {
      return cachedSettings
    }

    cachedSettings = await fetchSettingsFromDb()
    return cachedSettings
  }

  async function updateSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
    const stringValue = typeof value === 'object' ? JSON.stringify(value) : value.toString()
    await db.execute('UPDATE settings SET value = ? WHERE key = ?', [stringValue, key])
    cachedSettings = null
  }

  const settings = {
    async getApiUrl(): Promise<string> {
      const settings = await getSettings()
      return settings.api_url
    },
    async getCurrentSong(): Promise<Song | null> {
      const settings = await getSettings()
      return settings.current_song
    },
    async getEq(): Promise<{ [key: string]: string }> {
      const settings = await getSettings()
      return settings.eq
    },
    async getLossless(): Promise<boolean> {
      const settings = await getSettings()
      return Boolean(settings.lossless)
    },
    async getLoop(): Promise<boolean> {
      const settings = await getSettings()
      return Boolean(settings.loop)
    },
    async getMuted(): Promise<boolean> {
      const settings = await getSettings()
      return Boolean(settings.muted)
    },
    async getQueue(): Promise<Song[]> {
      const settings = await getSettings()
      return settings.queue
    },
    async getShuffle(): Promise<boolean> {
      const settings = await getSettings()
      return Boolean(settings.shuffle)
    },
    async getStreaming(): Promise<boolean> {
      const settings = await getSettings()
      return Boolean(settings.streaming)
    },
    async getVolume(): Promise<number> {
      const settings = await getSettings()
      return Number(settings.volume)
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