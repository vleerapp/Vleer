import { Howl, Howler } from 'howler';
import { useMusicStore } from "~/stores/music";
import { useSettingsStore } from "~/stores/settings";
import {
  readFile,
  writeFile,
  exists,
  BaseDirectory,
  remove,
} from "@tauri-apps/plugin-fs";
import type { Song, SongsConfig, Playlist } from "~/types/types";
import Database from "@tauri-apps/plugin-sql";

export default defineNuxtPlugin(async (nuxtApp) => {
  const db = await Database.load("sqlite:data.db");
  const musicStore = useMusicStore();
  const settingsStore = useSettingsStore();

  const music = {
    howl: null as Howl | null,
    analyzer: null as AnalyserNode | null,
    equalizer: null as BiquadFilterNode[] | null,

    async init() {
      await musicStore.init();
      this.setVolume(settingsStore.getVolume());
    },
    getSongs() {
      return musicStore.getSongs();
    },
    getPlaylists() {
      return musicStore.getPlaylists();
    },
    async addSongData(song: Song) {
      await musicStore.addSongData(song);
    },
    async updatePlaylistCover(playlistId: string, coverPath: any) {
      if (!coverPath || typeof coverPath !== 'object' || typeof coverPath.path !== 'string') {
        console.error('Invalid coverPath:', coverPath);
        throw new TypeError('coverPath must be an object with a path string');
      }
      const extension = coverPath.path.split('.').pop();
      const newCoverName = `${playlistId}.${extension}`;
      const newCoverPath = `Vleer/Covers/${newCoverName}`;

      const existingExtensions = ['png', 'jpg', 'jpeg', 'gif'];
      for (let ext of existingExtensions) {
        const oldCoverPath = `Vleer/Covers/${playlistId}.${ext}`;
        const coverExists = await exists(oldCoverPath, { baseDir: BaseDirectory.Audio });
        if (coverExists) {
          await remove(oldCoverPath, { baseDir: BaseDirectory.Audio });
        }
      }

      try {
        const data = await readFile(coverPath.path, { baseDir: BaseDirectory.Audio });
        await writeFile(newCoverPath, data, { baseDir: BaseDirectory.Audio });
        await musicStore.updatePlaylistCover(playlistId, newCoverPath);
      } catch (error) {
        console.error('Failed to update playlist cover:', error);
        throw new Error('Failed to update playlist cover due to path or permission issues.');
      }
    },
    async getCoverURLFromID(playlistId: string): Promise<string> {
      const extensions = ['png', 'jpg', 'jpeg', 'gif'];
      for (let ext of extensions) {
        const coverExists = await exists(`Vleer/Covers/${playlistId}.${ext}`, {
          baseDir: BaseDirectory.Audio,
        });
        if (coverExists) {
          const contents = await readFile(`Vleer/Covers/${playlistId}.${ext}`, {
            baseDir: BaseDirectory.Audio,
          });
          const blob = new Blob([contents]);
          return URL.createObjectURL(blob);
        }
      }
      return "/cover.png";
    },
    async getCoverFromID(songId: string) {
      const contents = await readFile(`Vleer/Covers/${songId}.png`, {
        baseDir: BaseDirectory.Audio,
      });
      return URL.createObjectURL(new Blob([contents]));
    },
    async setSong(id: string) {
      const song = await musicStore.getSongByID(id);
      if (song) {
        if (this.howl) {
          this.howl.unload();
        }
        try {
          const isLossless = settingsStore.getLossless();
          const fileExtension = isLossless ? 'flac' : 'mp3';
          const fileContent = await readFile(`Vleer/Songs/${id}.${fileExtension}`, { baseDir: BaseDirectory.Audio });
          const blob = new Blob([fileContent], { type: isLossless ? 'audio/flac' : 'audio/mp3' });
          const url = URL.createObjectURL(blob);

          this.howl = new Howl({
            src: [url],
            format: [fileExtension],
            html5: true,
            volume: settingsStore.getVolume() / 100,
            onload: () => {
              console.log('Audio loaded successfully');
              this.setupEqualizer();
            },
            onloaderror: (id, error) => {
              console.error('Error loading audio:', error);
            },
            onplay: () => {
              console.log('Audio started playing');
            },
            onplayerror: (id, error) => {
              console.error('Error playing audio:', error);
            },
            onend: () => {
              this.skip();
              URL.revokeObjectURL(url);
            },
          });

          await musicStore.setSong(id);
        } catch (error) {
          console.error('Error reading audio file:', error);
        }
      }
    },
    play() {
      if (this.howl) {
        this.howl.play();
        console.log('Attempting to play audio');
      } else {
        console.error('No audio loaded');
      }
    },
    pause() {
      if (this.howl) {
        this.howl.pause();
      }
    },
    playPause() {
      if (this.howl) {
        if (this.howl.playing()) {
          this.howl.pause();
        } else {
          this.howl.play();
        }
      } else if (musicStore.getCurrentSong()) {
        this.setSong(musicStore.getCurrentSong()).then(() => {
          this.play();
        });
      }
    },
    setVolume(volume: number) {
      if (this.howl) {
        this.howl.volume(volume / 100);
      }
      Howler.volume(volume / 100);
    },
    async getCurrentSong(): Promise<Song | null> {
      return await musicStore.getCurrentSong();
    },
    async applyEqSettings() {
      const eqSettings = settingsStore.getEq();
      if (this.equalizer) {
        Object.entries(eqSettings).forEach(([freq, gain], index) => {
          this.setEqGain(index, parseFloat(gain));
        });
      }
    },
    setEqGain(filterIndex: number, gain: number): void {
      if (this.equalizer && this.equalizer[filterIndex]) {
        this.equalizer[filterIndex].gain.setValueAtTime(gain, Howler.ctx.currentTime);
      }
    },
    async setQueue(songIds: string[]) {
      await musicStore.setQueue(songIds);
    },
    async skip() {
      try {
        await musicStore.skip();
        const currentSong = await musicStore.getCurrentSong();
        if (currentSong) {
          await this.setSong(currentSong);
          this.play();
        }
      } catch (error) {
        console.error('Error skipping song:', error);
      }
    },
    async rewind() {
      try {
        await musicStore.rewind();
        const currentSong = await musicStore.getCurrentSong();
        if (currentSong) {
          await this.setSong(currentSong);
          this.play();
        }
      } catch (error) {
        console.error('Error rewinding song:', error);
      }
    },
    async getSongsData(): Promise<SongsConfig> {
      return await musicStore.getSongsData();
    },
    async createPlaylist(playlist: Playlist) {
      await musicStore.createPlaylist(playlist);
    },
    async getPlaylistByID(id: string): Promise<Playlist> {
      return await musicStore.getPlaylistByID(id);
    },
    async getSongByID(id: string): Promise<Song> {
      return await musicStore.getSongByID(id);
    },
    async addSongToPlaylist(playlistId: string, songId: string) {
      await musicStore.addSongToPlaylist(playlistId, songId);
    },
    async renamePlaylist(playlistId: string, newName: string) {
      await musicStore.renamePlaylist(playlistId, newName);
    },
    async getLastUpdated() {
      return await musicStore.getLastUpdated();
    },
    async searchCoverByPlaylistId(playlistId: string): Promise<string> {
      const extensions = ['png', 'jpg', 'jpeg', 'gif'];
      for (let ext of extensions) {
        const coverExists = await exists(`Vleer/Covers/${playlistId}.${ext}`, {
          baseDir: BaseDirectory.Audio,
        });
        if (coverExists) {
          const contents = await readFile(`Vleer/Covers/${playlistId}.${ext}`, {
            baseDir: BaseDirectory.Audio,
          });
          const blob = new Blob([contents]);
          return URL.createObjectURL(blob);
        }
      }
      return "/cover.png";
    },
    setupEqualizer() {
      if (!this.howl) return;

      const node = this.howl._sounds[0]._node; // HTMLAudioElement
      const ctx = Howler.ctx;

      this.analyzer = ctx.createAnalyser();
      const sourceNode = ctx.createMediaElementSource(node);

      // Create equalizer bands
      const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];
      this.equalizer = frequencies.map(freq => {
        const filter = ctx.createBiquadFilter();
        filter.type = 'peaking';
        filter.frequency.value = freq;
        filter.Q.value = 1;
        filter.gain.value = 0;
        return filter;
      });

      // Connect nodes
      sourceNode.connect(this.equalizer[0]);
      this.equalizer.reduce((prev, curr) => {
        prev.connect(curr);
        return curr;
      });
      this.equalizer[this.equalizer.length - 1].connect(this.analyzer);
      this.analyzer.connect(ctx.destination);
    },
  };

  return {
    provide: {
      music,
    },
  };
});