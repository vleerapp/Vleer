import { createPinia } from "pinia";
import { useMusicStore } from "~/stores/music";
import {
  readFile,
  exists,
  BaseDirectory,
  mkdir,
  writeTextFile,
  readTextFile,
} from "@tauri-apps/plugin-fs";
import type { SongsConfig } from "~/types/music";

export default defineNuxtPlugin((nuxtApp) => {
  const pinia = createPinia();
  nuxtApp.vueApp.use(pinia);

  const store = useMusicStore();

  const music = {
    async init() {
      const baseDirExists = await exists("Vleer", {
        baseDir: BaseDirectory.Audio,
      });
      const songJsonExists = await exists("Vleer/songs.json", {
        baseDir: BaseDirectory.Audio,
      });

      if (!baseDirExists) {
        await mkdir("Vleer", { baseDir: BaseDirectory.Audio });
      }

      if (!songJsonExists) {
        await writeTextFile("Vleer/songs.json", "{}", {
          baseDir: BaseDirectory.Audio,
          createNew: true,
        });
      }

      const songsConfig = JSON.parse(
        await readTextFile("Vleer/songs.json", { baseDir: BaseDirectory.Audio })
      ) as SongsConfig;
      store.init(songsConfig);
    },
    getSongs() {
      return store.songsConfig;
    },
    async setSong(id: string) {
      const contents = await readFile(`Vleer/Songs/${id}.webm`, {
        baseDir: BaseDirectory.Audio,
      });
      store.player.currentSongId = id;
      await store.setSongFromBuffer(contents);
    },
    async getCoverURLFromID(id: string): Promise<string> {
      const contents = await readFile(`Vleer/Covers/${id}.png`, {
        baseDir: BaseDirectory.Audio,
      });
      const blob = new Blob([contents], { type: "image/png" });
      const coverObjectURL = URL.createObjectURL(blob);
      return coverObjectURL;
    },
    play() {
      const audio = store.getAudio();
      audio.play();
    },
    pause() {
      const audio = store.getAudio();
      audio.pause();
    },
    getAudio(): HTMLAudioElement {
      return store.getAudio();
    },
    setVolume(volume: number) {
      const audio = store.getAudio();
      if (volume == 0) {
        audio.volume = 0;
        return;
      }

      const minVolume = 1;
      const maxVolume = 100;
      volume = Math.max(minVolume, Math.min(maxVolume, volume));

      const minp = 0;
      const maxp = 100;

      const minv = Math.log(0.01);
      const maxv = Math.log(1);

      const scale = (maxv - minv) / (maxp - minp);

      audio.volume = Math.exp(minv + scale * (volume - minp));
    },
    getCurrentSong() {
      return store.getSongByID(store.player.currentSongId);
    }
  };

  return {
    provide: {
      music,
    },
  };
});
