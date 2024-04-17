import {
  BaseDirectory,
  writeTextFile,
} from "@tauri-apps/plugin-fs";
import type { MusicStore, SongsConfig, Song, Playlist } from "~/types/types";

export const useMusicStore = defineStore("musicStore", {
  state: () =>
  ({
    songsConfig: {
      songs: {},
      playlists: {},
    },
    player: {
      audio: new Audio(),
      currentSongId: "",
      audioContext: null,
      sourceNode: null,
      eqFilters: []
    },
    lastUpdated: Date.now(),
  } as MusicStore),

  actions: {
    init(songs: SongsConfig) {
      this.songsConfig = songs;
      this.player.audio.volume = 1;
      this.player.audio.preload = "auto";
    },
    replaceConfig(songs: SongsConfig) {
      this.songsConfig = songs
    },
    addSongData(song: Song) {
      this.songsConfig.songs[song.id] = song;
      this.lastUpdated = Date.now();
    },
    getSongsData(): SongsConfig {
      return this.songsConfig;
    },
    async setSongFromBuffer(buffer: any) {
      const blob = new Blob([buffer], { type: "audio/webm" });
      const url = URL.createObjectURL(blob);
      console.log(url);
      this.player.audio!.currentTime = 0;
      this.player.audio!.src = url;
      await this.player.audio!.load();
      this.player.audio!.addEventListener("error", (e) => {
        console.error("Error with audio element:", e);
      });
    },
    getAudio(): HTMLAudioElement {
      return this.player.audio!;
    },
    getSongByID(id: string): Song | null {
      return this.songsConfig?.songs?.[id] ?? null;
    },
    updateLastPlayed(songId: string, lastPlayed: string) {
      if (this.songsConfig.songs[songId]) {
        this.songsConfig.songs[songId].lastPlayed = lastPlayed;
        writeTextFile("Vleer/songs.json", JSON.stringify(this.songsConfig, null, 2), {
          baseDir: BaseDirectory.Audio,
        });
      }
    },
    createPlaylist(playlist: Playlist) {
      this.songsConfig.playlists[playlist.id] = playlist;
      writeTextFile("Vleer/songs.json", JSON.stringify(this.songsConfig, null, 2), {
        baseDir: BaseDirectory.Audio,
      });
    },
    getPlaylistByID(id: string): Playlist | null {
      return this.songsConfig?.playlists?.[id] ?? null;
    },
  },
});
