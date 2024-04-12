import { defineStore } from "pinia";
import type { MusicStore, SongsConfig, Song } from "~/types/music";

export const useMusicStore = defineStore("musicStore", {
  state: () =>
    ({
      songsConfig: {
        songs: {},
      },
      player: {
        audio: new Audio(),
        currentSongId: "",
      },
    } as MusicStore),

  actions: {
    init(songs: SongsConfig) {
      this.songsConfig = songs;
      this.player.audio.volume = 1;
      this.player.audio.preload = "auto";
    },
    async setSongFromBuffer(buffer: any) {
      const blob = new Blob([buffer], { type: "audio/webm" });
      const url = URL.createObjectURL(blob);
      this.player.audio.currentTime = 0;
      this.player.audio.src = url;
      await this.player.audio.load();
      this.player.audio.addEventListener("error", (e) => {
        console.error("Error with audio element:", e);
      });
    },
    getAudio(): HTMLAudioElement {
      return this.player.audio;
    },
    getSongByID(id: string): Song {
      return this.songsConfig.songs[id];
    },
  },
});
