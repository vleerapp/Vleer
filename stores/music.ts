import {
  BaseDirectory,
  writeTextFile,
} from "@tauri-apps/plugin-fs";
import type { MusicStore, SongsConfig, Song, Playlist } from "~/types/types";
import Database from "@tauri-apps/plugin-sql";

const db = await Database.load("sqlite:songs.db");

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
      // console.log(db);
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
      if (!this.songsConfig.playlists) {
        this.songsConfig.playlists = {};
      }
      this.songsConfig.playlists[playlist.id] = playlist;
      writeTextFile("Vleer/songs.json", JSON.stringify(this.songsConfig, null, 2), {
        baseDir: BaseDirectory.Audio,
      });
    },
    getPlaylistByID(id: string): Playlist | null {
      return this.songsConfig?.playlists?.[id] ?? null;
    },
    addSongToPlaylist(playlistId: string, songId: string) {
      const playlist = this.songsConfig.playlists[playlistId];
      if (playlist && this.songsConfig.songs[songId]) {
        playlist.songs.push(songId);
        writeTextFile("Vleer/songs.json", JSON.stringify(this.songsConfig, null, 2), {
          baseDir: BaseDirectory.Audio,
        });
      }
    },
    renamePlaylist(playlistId: string, newName: string) {
      const playlist = this.songsConfig.playlists[playlistId];
      if (playlist) {
        playlist.name = newName;
        writeTextFile("Vleer/songs.json", JSON.stringify(this.songsConfig, null, 2), {
          baseDir: BaseDirectory.Audio,
        });
      }
    },
    updatePlaylistCover(playlistId: string, newCoverPath: string) {
      const playlist = this.songsConfig.playlists[playlistId];
      if (playlist) {
        playlist.cover = newCoverPath;
        writeTextFile("Vleer/songs.json", JSON.stringify(this.songsConfig, null, 2), {
          baseDir: BaseDirectory.Audio,
        });
      }
    },
  },
});