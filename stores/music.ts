import { defineStore } from 'pinia';
import { Howl } from 'howler';
import type { MusicStore, SongsConfig, Song, Playlist } from "~/types/types";
import Database from "@tauri-apps/plugin-sql";
import { readFile, BaseDirectory, exists } from "@tauri-apps/plugin-fs";
import { useSettingsStore } from './settings';
import { computed } from 'vue';

export const useMusicStore = defineStore("musicStore", {
  state: () => ({
    songsConfig: {
      songs: {} as Record<string, Song>,
      playlists: {} as Record<string, Playlist>,
    },
    player: {
      currentSongId: "",
      howl: null as Howl | null,
    },
    lastUpdated: Date.now(),
    db: null as Database | null,
    queue: [] as string[],
    currentQueueIndex: 0,
  } as MusicStore),

  actions: {
    async init() {
      this.db = await Database.load("sqlite:data.db");
      const settingsStore = useSettingsStore();
      await settingsStore.getSettings();
      this.queue = settingsStore.getQueue();
      if (this.queue.length > 0) {
        await this.setSong(this.queue[0]);
      }

      const songs = await this.db.select<Song[]>("SELECT * FROM songs");
      songs.forEach(async (song: Song) => {
        this.songsConfig.songs[song.id] = song;

        const contents = await readFile(`Vleer/Covers/${song.id}.png`, {
          baseDir: BaseDirectory.Audio,
        });
        song.coverURL = URL.createObjectURL(new Blob([contents]));
      });

      const playlists = await this.db.select<Playlist[]>("SELECT * FROM playlists");
      playlists.forEach(playlist => {
        this.songsConfig.playlists[playlist.id] = playlist;
      });
    },
    async addSongData(song: Song) {
      if (this.db) {
        await this.db.execute("INSERT INTO songs (id, title, artist, length, cover, date_added, last_played) VALUES (?, ?, ?, ?, ?, ?, ?)", [
          song.id, song.title, song.artist, song.length, song.cover, song.date_added, song.lastPlayed
        ]);
      }
      this.songsConfig.songs[song.id] = song;
      this.lastUpdated = Date.now();
    },
    getSongsData(): SongsConfig {
      return this.songsConfig;
    },
    getSongByID(id: string): Song {
      return this.songsConfig.songs[id] ?? null;
    },
    async updateLastPlayed(songId: string, lastPlayed: string) {
      if (this.db) {
        await this.db.execute("UPDATE songs SET last_played = ? WHERE id = ?", [lastPlayed, songId]);
      }
      if (this.songsConfig.songs[songId]) {
        this.songsConfig.songs[songId].lastPlayed = lastPlayed;
      }
    },
    async createPlaylist(playlist: Playlist) {
      if (this.db) {
        await this.db.execute("INSERT INTO playlists (id, name, cover, songs) VALUES (?, ?, ?, ?)", [
          playlist.id, playlist.name, playlist.cover, playlist.songs.join(',')
        ]);
      }
      this.songsConfig.playlists[playlist.id] = playlist;
      this.lastUpdated = Date.now();
    },
    getPlaylistByID(id: string): Playlist {
      return this.songsConfig.playlists[id] ?? null;
    },
    async renamePlaylist(playlistId: string, newName: string) {
      if (this.db) {
        await this.db.execute("UPDATE playlists SET name = ? WHERE id = ?", [newName, playlistId]);
      }
      if (this.songsConfig.playlists[playlistId]) {
        this.songsConfig.playlists[playlistId].name = newName;
      }
    },
    async updatePlaylistCover(playlistId: string, newCoverPath: string) {
      if (this.db) {
        await this.db.execute("UPDATE playlists SET cover = ? WHERE id = ?", [newCoverPath, playlistId]);
      }
      if (this.songsConfig.playlists[playlistId]) {
        this.songsConfig.playlists[playlistId].cover = newCoverPath;
      }
    },
    async addSongToPlaylist(playlistId: string, songId: string) {
      const playlist = this.songsConfig.playlists[playlistId];
      if (playlist && this.songsConfig.songs[songId]) {
        playlist.songs.push(songId);
        if (this.db) {
          await this.db.execute("UPDATE playlists SET songs = ? WHERE id = ?", [playlist.songs.join(','), playlistId]);
        }
      }
      this.lastUpdated = Date.now();
    },
    getLastUpdated() {
      return this.lastUpdated;
    },
    async setSong(id: string) {
      this.player.currentSongId = id;
      const currentTime = new Date().toISOString();
      if (this.db) {
        await this.db.execute("UPDATE songs SET last_played = ? WHERE id = ?", [currentTime, id]);
      }
      if (this.songsConfig.songs[id]) {
        this.songsConfig.songs[id].lastPlayed = currentTime;
      }
      this.lastUpdated = Date.now();
    },
    play() {
      if (this.player.howl) {
        this.player.howl.play();
      }
    },
    pause() {
      if (this.player.howl) {
        this.player.howl.pause();
      }
    },
    getCurrentSong() {
      return this.player.currentSongId;
    },
    async setQueue(queue: string[]) {
      this.queue = queue;
      this.currentQueueIndex = 0;
      const settingsStore = useSettingsStore();
      settingsStore.setQueue(queue);
      if (this.queue.length > 0) {
        await this.setSong(this.queue[0]);
      }
    },
    async skip() {
      if (this.currentQueueIndex < this.queue.length - 1) {
        this.currentQueueIndex++;
        await this.setSong(this.queue[this.currentQueueIndex]);
      }
    },
    async rewind() {
      if (this.currentQueueIndex > 0) {
        this.currentQueueIndex--;
        await this.setSong(this.queue[this.currentQueueIndex]);
      }
    },
    getSongs() {
      return Object.values(this.songsConfig.songs);
    },

    getPlaylists() {
      return Object.values(this.songsConfig.playlists);
    },
  },
  getters: {
    sortedRecentlyPlayed: (state) => {
      return computed(() => Object.values(state.songsConfig.songs)
        .sort((a, b) => {
          if (!a.lastPlayed) return 1;
          if (!b.lastPlayed) return -1;
          return new Date(b.lastPlayed).getTime() - new Date(a.lastPlayed).getTime();
        })
      );
    }
  },
});