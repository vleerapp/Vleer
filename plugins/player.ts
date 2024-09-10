import { Howl } from 'howler';
import { BaseDirectory } from '@tauri-apps/api/path';
import { defineNuxtPlugin } from 'nuxt/app';
import { readFile } from '@tauri-apps/plugin-fs';
import { ref } from 'vue';
import type { Song } from '~/types/types';

export default defineNuxtPlugin((nuxtApp) => {
  let sound: Howl | null = null;

  const currentSong = ref<Song | null>(null);
  const duration = ref(0);
  const looping = ref(false);
  const muted = ref(false);
  const paused = ref(true);
  const progress = ref(0);
  const time = ref(0);
  const volume = ref(50);

  const player = {
    currentSong,
    duration,
    looping,
    muted,
    paused,
    progress,
    time,
    volume,

    async loadSong(song: Song) {
      if (sound) {
        sound.unload();
      }

      this.currentSong.value = song;

      const { $settings } = useNuxtApp();

      const [lossless, streaming] = await Promise.all([
        $settings.getLossless(),
        $settings.getStreaming()
      ]);
      const fileExtension = lossless ? 'flac' : 'mp3';
      const fileContent = await readFile(`Vleer/Songs/${song.id}.${fileExtension}`, { baseDir: BaseDirectory.Audio });
      const blob = new Blob([fileContent], { type: lossless ? 'audio/flac' : 'audio/mp3' });
      const url = URL.createObjectURL(blob);
      
      sound = new Howl({
        src: [url],
        format: [fileExtension],
        html5: true,
        onend: () => {
          if (this.looping.value) {
            sound!.play();
          } else {
            this.skip();
          }
        },
        onload: () => {
          this.duration.value = sound!.duration();
        },
        onloaderror: (id, error) => {
          console.error('Error loading audio:', error);
        },
        onpause: () => {
          this.paused.value = true;
        },
        onplay: () => {
          this.paused.value = false;
          this.updateProgress();
        },
        onseek: () => {
          this.updateProgress();
        },
      });

      const updateProgressInterval = setInterval(() => {
        if (sound && !this.paused.value) {
          const seek = sound.seek() as number;
          this.progress.value = (seek / this.duration.value) * 100;
          this.time.value = seek;
        }
      }, 1000);

      sound.on('end', () => {
        clearInterval(updateProgressInterval);
      });

      sound.volume(this.volume.value / 100);
      sound.mute(this.muted.value);

      try {
        sound.on('unload', () => {
          URL.revokeObjectURL(url);
        });
      } catch (error) {
        console.error('Error attaching unload event:', error);
      }
    },
    updateProgress() {
      if (sound && !this.paused) {
        const seek = sound.seek() as number;
        this.progress.value = (seek / this.duration.value) * 100;
        this.time.value = seek;
        requestAnimationFrame(() => this.updateProgress());
      }
    },
    play() {
      if (sound) {
        sound.play();
      }
    },
    pause() {
      if (sound) {
        sound.pause();
      }
    },
    async skip() {
      const { $settings } = useNuxtApp();
      const queue = await $settings.getQueue();
      if (queue.length > 0) {
        const nextSong = queue.shift();
        if (nextSong) {
          await $settings.setQueue(queue);
          await this.loadSong(nextSong);
          this.play();
        }
      }
    },
    rewind() {
      if (sound) {
        sound.seek(0);
      }
    },
    skipTo(percentage: number) {
      if (sound) {
        const seekTime = (percentage / 100) * this.duration.value;
        sound.seek(seekTime);
      }
    },
    async setVolume(value: number) {
      this.volume.value = value;
      if (sound) {
        sound.volume(this.volume.value / 100);
      }
      const { $settings } = useNuxtApp();
      await $settings.setVolume(this.volume.value / 100);
    },
    async mute() {
      this.muted.value = !this.muted.value;
      if (sound) {
        sound.mute(this.muted.value);
      }
      const { $settings } = useNuxtApp();
      await $settings.setMuted(this.muted.value);
    },
    async toggleLoop() {
      this.looping.value = !this.looping.value;
      const { $settings } = useNuxtApp();
      await $settings.setLoop(this.looping.value);
    },
    playPause() {
      if (sound) {
        if (this.paused.value) {
          this.play();
        } else {
          this.pause();
        }
      }
    }
  };

  return {
    provide: {
      player
    }
  };
});