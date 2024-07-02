import type { MusicStore, SongsConfig, Song, Playlist } from "~/types/types";
import Database from "@tauri-apps/plugin-sql";
import {
  readFile,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";
import { defineStore } from 'pinia';
import { useSettingsStore } from './settings';
import { computed } from 'vue';

export const useMusicStore = defineStore("musicStore", {
  state: () => ({
    songsConfig: {
      songs: {},
      playlists: {},
    },
    player: {
      audio: new Audio(),
      currentSongId: "",
      audioContext: null,
      sourceNode: null,
      analyser: null,
      eqFilters: []
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
        song.coverURL = URL.createObjectURL(new Blob([contents]))
      });

      const playlists = await this.db.select<Playlist[]>("SELECT * FROM playlists");
      playlists.forEach(playlist => {
        this.songsConfig.playlists[playlist.id] = {
          ...playlist,
          songs: playlist.songs.split(',')
        };
      });

      this.player.audio.volume = 1;
      this.player.audio.preload = "auto";
    },
    async addSongData(song: Song) {
      await this.db.execute("INSERT INTO songs (id, title, artist, length, cover, date_added, last_played) VALUES (?, ?, ?, ?, ?, ?, ?)", [
        song.id, song.title, song.artist, song.length, song.cover, song.date_added, song.lastPlayed
      ]);
      this.songsConfig.songs[song.id] = song;
      this.lastUpdated = Date.now();
    },
    getSongsData(): SongsConfig {
      return this.songsConfig;
    },
    async setSongFromBuffer(buffer: any) {
      const blob = new Blob([buffer], { type: "audio/mp3" });
      const url = URL.createObjectURL(blob);
      this.player.audio.currentTime = 0;
      this.player.audio.src = url;
      this.player.audio.load();
      this.lastUpdated = Date.now();
    },
    getAudio(): HTMLAudioElement {
      return this.player.audio;
    },
    getSongByID(id: string): Song {
      return this.songsConfig.songs[id] ?? null;
    },
    async updateLastPlayed(songId: string, lastPlayed: string) {
      await this.db.execute("UPDATE songs SET last_played = ? WHERE id = ?", [lastPlayed, songId]);
      if (this.songsConfig.songs[songId]) {
        this.songsConfig.songs[songId].lastPlayed = lastPlayed;
      }
    },
    async createPlaylist(playlist: Playlist) {
      await this.db.execute("INSERT INTO playlists (id, name, cover, songs) VALUES (?, ?, ?, ?)", [
        playlist.id, playlist.name, playlist.cover, playlist.songs.join(',')
      ]);
      this.songsConfig.playlists[playlist.id] = playlist;
      this.lastUpdated = Date.now();
    },
    getPlaylistByID(id: string): Playlist {
      return this.songsConfig.playlists[id] ?? null;
    },
    async renamePlaylist(playlistId: string, newName: string) {
      await this.db.execute("UPDATE playlists SET name = ? WHERE id = ?", [newName, playlistId]);
      if (this.songsConfig.playlists[playlistId]) {
        this.songsConfig.playlists[playlistId].name = newName;
      }
    },
    async updatePlaylistCover(playlistId: string, newCoverPath: string) {
      await this.db.execute("UPDATE playlists SET cover = ? WHERE id = ?", [newCoverPath, playlistId]);
      if (this.songsConfig.playlists[playlistId]) {
        this.songsConfig.playlists[playlistId].cover = newCoverPath;
      }
    },
    async addSongToPlaylist(playlistId: string, songId: string) {
      const playlist = this.songsConfig.playlists[playlistId];
      if (playlist && this.songsConfig.songs[songId]) {
        playlist.songs.push(songId);
        await this.db.execute("UPDATE playlists SET songs = ? WHERE id = ?", [playlist.songs.join(','), playlistId]);
      }
      this.lastUpdated = Date.now();
    },
    getLastUpdated() {
      return this.lastUpdated;
    },
    setVolume(volume: number) {
      this.player.audio.volume = volume;
      this.lastUpdated = Date.now();
    },
    async setSong(id: string) {
      const contents = await readFile(`Vleer/Songs/${id}.mp3`, {
        baseDir: BaseDirectory.Audio,
      });

      this.player.currentSongId = id;
      await this.setSongFromBuffer(contents);
      const currentTime = new Date().toISOString();
      await this.db.execute("UPDATE songs SET last_played = ? WHERE id = ?", [currentTime, id]);
      if (this.songsConfig.songs[id]) {
        this.songsConfig.songs[id].lastPlayed = currentTime;
      }

      this.lastUpdated = Date.now();
    },
    createEqFilters(): BiquadFilterNode[] {
      const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];
      return frequencies.map((freq) => {
        const filter = this.player.audioContext!.createBiquadFilter();
        filter.type = "peaking";
        filter.frequency.value = freq;
        filter.Q.value = 1;
        filter.gain.value = 0;
        return filter;
      });
    },

    connectEqFilters(): void {
      if (!this.player.sourceNode || !this.player.audioContext) return;

      this.player.sourceNode.disconnect();
      let lastNode: AudioNode = this.player.sourceNode;
      this.player.eqFilters.forEach((filter) => {
        lastNode.connect(filter);
        lastNode = filter;
      });
      lastNode.connect(this.player.audioContext.destination);
    },

    async applyEqSettings(eqSettings: any) {
      if (!this.player.audioContext) {
        this.player.audioContext = new AudioContext();
        this.player.sourceNode = this.player.audioContext.createMediaElementSource(this.player.audio);
        this.player.eqFilters = this.createEqFilters();
      }

      this.player.eqFilters.forEach((filter, index) => {
        const freq = filter.frequency.value;
        const gain = eqSettings[freq.toString()];
        if (gain !== undefined) {
          filter.gain.setValueAtTime(parseFloat(gain), this.player.audioContext!.currentTime);
        }
      });

      this.connectEqFilters();
    },
    setEqGain(filterIndex: number, gain: number): void {
      if (this.player.eqFilters[filterIndex]) {
        this.player.eqFilters[filterIndex].gain.value = gain;
      }
    },
    play() {
      this.player.audio.play()
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
        this.play();
      }
    },
    async skip() {
      if (this.currentQueueIndex < this.queue.length - 1) {
        this.currentQueueIndex++;
        await this.setSong(this.queue[this.currentQueueIndex]);
        this.play();
      }
    },
    async rewind() {
      if (this.currentQueueIndex > 0) {
        this.currentQueueIndex--;
        await this.setSong(this.queue[this.currentQueueIndex]);
        this.play();
      }
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