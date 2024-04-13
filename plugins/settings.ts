import {
  exists,
  BaseDirectory,
  mkdir,
  writeTextFile,
  readTextFile,
} from "@tauri-apps/plugin-fs";
import { defaultSettings, type PlayerSettings, type UserSettings } from "~/types/definitions";

export default defineNuxtPlugin((nuxtApp) => {
  const settings = {
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
    async getSettings() {
      await this.checkForDefaultFiles();
    },
  };

  return {
    provide: {
      settings,
    },
  };
});
