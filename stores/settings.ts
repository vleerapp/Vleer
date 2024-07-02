import { defineStore } from 'pinia';
import Database from '@tauri-apps/plugin-sql';
import type { EQSettings } from "~/types/types";

export const useSettingsStore = defineStore("settingsStore", {
  state: () => ({
    settings: {
      volume: 100,
      currentSong: "",
      eq: {
        "32": "0.0",
        "64": "0.0",
        "125": "0.0",
        "250": "0.0",
        "500": "0.0",
        "1000": "0.0",
        "2000": "0.0",
        "4000": "0.0",
        "8000": "0.0",
        "16000": "0.0"
      },
      apiURL: "",
      queue: [] as string[]
    }
  }),
  actions: {
    async getSettings() {
      const db = await Database.load("sqlite:data.db");
      const results = await db.select<any[]>("SELECT key, value FROM settings");
      results.forEach(({ key, value }) => {
        try {
          this.settings[key] = JSON.parse(value);
        } catch {
          this.settings[key] = value;
        }
      });
    },
    async saveSettings() {
      const db = await Database.load("sqlite:data.db");
      await db.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", ['eq', JSON.stringify(this.settings.eq)]);
      await db.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", ['volume', this.settings.volume.toString()]);
      await db.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", ['apiURL', this.settings.apiURL]);
      await db.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", ['currentSong', this.settings.currentSong]);
      await db.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", ['queue', JSON.stringify(this.settings.queue)]);
    },
    getVolume(): number {
      return this.settings.volume;
    },
    setVolume(volume: number) {
      this.settings.volume = volume;
      this.saveSettings();
    },
    getEq(): EQSettings {
      return this.settings.eq;
    },
    setEq(eq: EQSettings) {
      this.settings.eq = eq;
      this.saveSettings();
    },
    getCurrentSong(): string {
      return this.settings.currentSong;
    },
    setCurrentSong(song: string) {
      this.settings.currentSong = song;
      this.saveSettings();
    },
    setApiURL(url: string) {
      this.settings.apiURL = url;
      this.saveSettings();
    },
    getApiURL(): string {
      return this.settings.apiURL;
    },
    setQueue(queue: string[]) {
      this.settings.queue = queue;
      this.saveSettings();
    },
    getQueue(): string[] {
      return this.settings.queue;
    },
    async searchApiURL() {
      try {
        const response = await fetch('https://piped-instances.kavin.rocks/');
        const instances = await response.json();
        const urls = instances.map((instance: { api_url: string }) => instance.api_url);

        const results = await window.__TAURI__.core.invoke('ping_urls', { urls });
        this.setApiURL(results[0][0]);
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
  },
});

