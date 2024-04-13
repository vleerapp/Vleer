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
import type { EQSettings, Song, SongsConfig } from "~/types/types";

export default defineNuxtPlugin((nuxtApp) => {
  const musicStore = useMusicStore();
  const settingsStore = useSettingsStore();

  musicStore.player.audio.onplay = () => music.ensureAudioContextAndFilters();

  const music = {
    async init() {
      const baseDirExists = await exists("Vleer", {
        baseDir: BaseDirectory.Audio,
      });

      const songsDirExists = await exists("Vleer/Songs", {
        baseDir: BaseDirectory.Audio,
      });

      const coverDirExists = await exists("Vleer/Covers", {
        baseDir: BaseDirectory.Audio,
      });

      const songJsonExists = await exists("Vleer/songs.json", {
        baseDir: BaseDirectory.Audio,
      });

      if (!baseDirExists) {
        await mkdir("Vleer", { baseDir: BaseDirectory.Audio });
      }

      if (!songsDirExists) {
        await mkdir("Vleer/Songs", { baseDir: BaseDirectory.Audio });
      }

      if (!coverDirExists) {
        await mkdir("Vleer/Covers", { baseDir: BaseDirectory.Audio });
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

      musicStore.init(songsConfig);
    },
    getSongs() {
      return musicStore.songsConfig;
    },
    async addSongData(song: Song) {
      const songsConfig = JSON.parse(
        await readTextFile("Vleer/songs.json", { baseDir: BaseDirectory.Audio })
      ) as SongsConfig;

      musicStore.replaceConfig(songsConfig);

      musicStore.addSongData(song);

      const data = musicStore.getSongsData();

      await writeTextFile("Vleer/songs.json", JSON.stringify(data, null, 2), {
        baseDir: BaseDirectory.Audio,
      });
    },
    async setSong(id: string) {
      const contents = await readFile(`Vleer/Songs/${id}.webm`, {
        baseDir: BaseDirectory.Audio,
      });
      musicStore.player.currentSongId = id;
      await musicStore.setSongFromBuffer(contents);
      await this.ensureAudioContextAndFilters(); 
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
      const audio = musicStore.getAudio();
      audio.play();
    },
    pause() {
      const audio = musicStore.getAudio();
      audio.pause();
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

      const minv = Math.log(0.01);
      const maxv = Math.log(1);

      const scale = (maxv - minv) / (maxp - minp);

      audio.volume = Math.exp(minv + scale * (volume - minp));
    },
    getCurrentSong() {
      return musicStore.getSongByID(musicStore.player.currentSongId);
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
      let lastNode = musicStore.player.sourceNode!;
      musicStore.player.eqFilters.forEach((filter) => {
        lastNode.connect(filter);
        lastNode = filter;
      });
      lastNode.connect(musicStore.player.audioContext!.destination);
    },
    async applyEqSettings() {
      const eqSettings = (await settingsStore.getSettings()).eq;
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
            musicStore.player.audio
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
  };

  return {
    provide: {
      music,
    },
  };
});
