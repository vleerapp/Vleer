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

  musicStore.player.audio.addEventListener("error", (e) => {
    const mediaError = e.target.error;
    if (mediaError) {
      console.error("Error with audio element:", mediaError);
      console.error("MediaError code:", mediaError.code);
    }
  });

  musicStore.player.audio.onplay = () => music.ensureAudioContextAndFilters();

  const music = {
    queue: [] as string[],
    currentQueueIndex: 0,
    async init() {
      musicStore.init();
    },
    getSongs() {
      return musicStore.songsConfig;
    },
    async addSongData(song: Song) {
      await db.execute(`INSERT INTO songs (id, title, artist, length, cover, date_added, cover_url, last_played) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`, [
        song.id, song.title, song.artist, song.length, song.cover, song.date_added, song.coverURL, song.lastPlayed
      ]);

      musicStore.addSongData(song);
    },
    async setSong(id: string) {
      const songExists = await exists(`Vleer/Songs/${id}.webm`, {
        baseDir: BaseDirectory.Audio,
      });

      if (songExists) {
        const contents = await readFile(`Vleer/Songs/${id}.webm`, {
          baseDir: BaseDirectory.Audio,
        });
        musicStore.player.currentSongId = id;
        await musicStore.setSongFromBuffer(contents);
        await this.ensureAudioContextAndFilters();
        const currentTime = new Date().toISOString();
        await db.execute("UPDATE songs SET last_played = ? WHERE id = ?", [currentTime, id]);
      } else {
        settingsStore.setCurrentSong("");
        await settingsStore.saveSettings();
      }
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
        await db.execute("UPDATE playlists SET cover = ? WHERE id = ?", [newCoverPath, playlistId]);
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
    play() {
      const audio = musicStore.getAudio();
      settingsStore.setCurrentSong(musicStore.player.currentSongId);
      settingsStore.saveSettings();
      audio.play();
    },
    pause() {
      const audio = musicStore.getAudio();
      audio.pause();
    },
    playPause() {
      const audio = musicStore.getAudio();
      if (audio.paused) {
        audio.play();
      } else {
        audio.pause();
      }
    },
    getAudio(): HTMLAudioElement {
      return musicStore.getAudio();
    },
    setVolume(volume: number) {
      const audio = musicStore.getAudio();
      if (volume == 0) {
        audio.volume = 0;
        return;
      }

      const minVolume = 1;
      const maxVolume = 100;
      volume = Math.max(minVolume, Math.min(maxVolume, volume));

      const minp = 0;
      const maxp = 100;

      const minv = Math.log(0.001);
      const maxv = Math.log(1);

      const scale = (maxv - minv) / (maxp - minp);

      audio.volume = Math.exp(minv + scale * (volume - minp));
    },
    getCurrentSong(): Song | null {
      const song = musicStore.getSongByID(musicStore.player.currentSongId);
      if (song) {
        return song;
      }
      return null;
    },
    createEqFilters(): BiquadFilterNode[] {
      const frequencies = [
        32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000,
      ];
      return frequencies.map((freq) => {
        const filter = musicStore.player.audioContext!.createBiquadFilter();
        filter.type = "peaking";
        filter.frequency.value = freq;
        filter.Q.value = 1;
        filter.gain.value = 0;
        return filter;
      });
    },
    connectEqFilters(): void {
      let lastNode: AudioNode = musicStore.player.sourceNode!;
      musicStore.player.eqFilters.forEach((filter) => {
        lastNode.connect(filter);
        lastNode = filter;
      });
      lastNode.connect(musicStore.player.audioContext!.destination);
    },
    async applyEqSettings() {
      const eqSettings = settingsStore.getEq();
      musicStore.player.eqFilters.forEach((filter, index) => {
        const gain =
          eqSettings[filter.frequency.value.toString() as keyof EQSettings];
        if (gain !== undefined) {
          this.setEqGain(index, parseInt(gain));
        }
      });
    },
    setEqGain(filterIndex: number, gain: number): void {
      if (musicStore.player.eqFilters[filterIndex]) {
        musicStore.player.eqFilters[filterIndex].gain.value = gain;

        this.ensureAudioContextAndFilters();
      }
    },
    async ensureAudioContextAndFilters() {
      if (!musicStore.player.audioContext) {
        musicStore.player.audioContext = new AudioContext();
        musicStore.player.sourceNode =
          musicStore.player.audioContext.createMediaElementSource(
            musicStore.player.audio!
          );
        musicStore.player.eqFilters = this.createEqFilters();
        this.connectEqFilters();
        await this.applyEqSettings();
        if (musicStore.player.audioContext.state === "suspended") {
          await musicStore.player.audioContext.resume();
        }
      } else if (musicStore.player.audioContext.state === "suspended") {
        await musicStore.player.audioContext.resume();
      }
    },
    async setQueue(songIds: string[]) {
      this.queue = songIds;
      this.currentQueueIndex = 0;
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
    getSongsData(): SongsConfig {
      return musicStore.getSongsData();
    },
    async createPlaylist(playlist: Playlist) {
      musicStore.createPlaylist(playlist);
    },
    getPlaylistByID(id: string): Playlist {
      return musicStore.getPlaylistByID(id);
    },
    getSongByID(id: string): Song {
      return musicStore.getSongByID(id);
    },
    addSongToPlaylist(playlistId: string, songId: string){
      musicStore.addSongToPlaylist(playlistId, songId);
    },
    renamePlaylist(playlistId: string, newName: string){
      musicStore.renamePlaylist(playlistId, newName);
    },
    getLastUpdated() {
      return musicStore.getLastUpdated();
    }
  };

  musicStore.player.audio.addEventListener('ended', () => {
    music.skip();
  });

  return {
    provide: {
      music,
    },
  };
});

