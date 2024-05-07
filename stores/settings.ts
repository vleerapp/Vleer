import { BaseDirectory, exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { defaultSettings, type PlayerSettings, type UserSettings } from "~/types/types";

export const useSettingsStore = defineStore("settingsStore", {
  state: () => ({
    init: false,
    settings: {
      playerSettings: {
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
          "16000": "0.0",
        },
      },
      apiURL: ""
    }
  }),
  actions: {
    async checkForDefaultFiles() {
      const configDirExists = await exists("", {
        baseDir: BaseDirectory.AppConfig,
      });

      const settingsFileExists = await exists("settings.json", {
        baseDir: BaseDirectory.AppConfig,
      });

      if (!configDirExists) {
        await mkdir("", { baseDir: BaseDirectory.AppConfig });
      }

      if (!settingsFileExists) {
        await writeTextFile("settings.json", JSON.stringify(defaultSettings, null, 2), {
          baseDir: BaseDirectory.AppConfig,
          createNew: true,
        });
      }
    },
    async getSettings(): Promise<PlayerSettings> {
      if (this.init) return this.settings.playerSettings;

      await this.checkForDefaultFiles();

      const settings = JSON.parse(
        await readTextFile("settings.json", { baseDir: BaseDirectory.AppConfig })
      ) as UserSettings;

      this.settings.playerSettings = settings.playerSettings;
      this.settings.apiURL = settings.apiURL;

      this.init = true;

      return settings.playerSettings;
    },
    async updateSettings(settings: PlayerSettings) {
      this.settings.playerSettings = settings;
      await this.saveSettings();
    },
    async saveSettings() {
      await writeTextFile("settings.json", JSON.stringify(this.settings, null, 2), {
        baseDir: BaseDirectory.AppConfig
      });
    }
  },
});
