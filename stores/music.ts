import type { MusicStore, SongsConfig, Song, Playlist } from "~/types/types";
import Database from "@tauri-apps/plugin-sql";
import {
  readFile,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";

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
      eqFilters: []
    },
    lastUpdated: Date.now(),
    db: null as Database | null,
  } as MusicStore),

  actions: {
    async init() {
      this.db = await Database.load("sqlite:data.db");
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
    async setSongFromBuffer(buffer: any, format: 'wav' | 'webm' = 'wav') {
      const mimeType = format === 'wav' ? 'audio/wav' : 'audio/webm';
      const blob = new Blob([buffer], { type: mimeType });
      const url = URL.createObjectURL(blob);
      this.player.audio.currentTime = 0;
      this.player.audio.src = url;
      await this.player.audio.load();
      this.player.audio.addEventListener("error", (e) => {
        console.error("Error with audio element:", e);
      });
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
    async setSong(id: string, contents: any) {
      this.player.currentSongId = id;
      await this.setSongFromBuffer(contents);      
      const currentTime = new Date().toISOString();
      await this.db.execute("UPDATE songs SET last_played = ? WHERE id = ?", [currentTime, id]);

      this.lastUpdated = Date.now();
    },
    async ensureAudioContextAndFilters() {
      if (!this.player.audioContext) {
        this.player.audioContext = new AudioContext();
        this.player.sourceNode =
          this.player.audioContext.createMediaElementSource(
            this.player.audio!
          );
        this.player.eqFilters = this.createEqFilters();
        this.connectEqFilters();
        await this.applyEqSettings(this.player.eqFilters);
        if (this.player.audioContext.state === "suspended") {
          await this.player.audioContext.resume();
        }
      } else if (this.player.audioContext.state === "suspended") {
        await this.player.audioContext.resume();
      }
    },
    createEqFilters(): BiquadFilterNode[] {
      const frequencies = [
        32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000,
      ];
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
      let lastNode: AudioNode = this.player.sourceNode!;
      this.player.eqFilters.forEach((filter) => {
        lastNode.connect(filter);
        lastNode = filter;
      });
      lastNode.connect(this.player.audioContext!.destination);
    },
    async applyEqSettings(eqSettings: any) {
      this.player.eqFilters.forEach((filter, index) => {
        const gain =
          eqSettings[filter.frequency.value.toString() as keyof EQSettings];
        if (gain !== undefined) {
          this.setEqGain(index, parseInt(gain));
        }
      });
    },
    setEqGain(filterIndex: number, gain: number): void {
      if (this.player.eqFilters[filterIndex]) {
        this.player.eqFilters[filterIndex].gain.value = gain;

        this.ensureAudioContextAndFilters();
      }
    },
    play() {
      this.player.audio.play()
    },
    getCurrentSong() {
      return this.player.currentSongId;
    }
  },
});