import { BaseDirectory } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'
import { readFile } from '@tauri-apps/plugin-fs'
import { defineNuxtPlugin } from 'nuxt/app'
import { Howl, Howler } from 'howler'
import { ref } from 'vue'
import type { EQSettings, Song } from '~/types/types'
import { listen } from '@tauri-apps/api/event'
import { useNuxtApp } from '#app'

export default defineNuxtPlugin((nuxtApp) => {
  let sound: Howl | null = null
  let analyzer: AnalyserNode | null = null
  let equalizer: BiquadFilterNode[] | null = null

  const currentSong = ref<Song | null>(null)
  const duration = ref(0)
  const looping = ref(false)
  const muted = ref(false)
  const paused = ref(true)
  const progress = ref(0)
  const time = ref(0)
  const volume = ref(50)

  const player = {
    currentSong,
    duration,
    looping,
    muted,
    paused,
    progress,
    time,
    volume,

    applyEQ(eq: EQSettings) {
      if (!sound || !equalizer) return;

      Object.entries(eq).forEach(([freq, gain], index) => {
        const cleanGain = typeof gain === 'string' ? gain.replace(',', '.') : gain;
        const numericGain = parseFloat(cleanGain);

        if (Number.isFinite(numericGain)) {
          this.setEqGain(index, numericGain);
        } else {
          console.error('Non-finite gain encountered', gain, 'Converted to:', numericGain);
        }
      });
    },

    async loadSong(song: Song) {
      if (sound) {
        sound.unload()
      }

      this.currentSong.value = song

      const { $settings } = useNuxtApp()

      const [lossless, streaming, eq] = await Promise.all([
        $settings.getLossless(),
        $settings.getStreaming(),
        $settings.getEq()
      ])
      const fileExtension = lossless ? 'flac' : 'mp3'
      const fileContent = await readFile(`Vleer/Songs/${song.id}.${fileExtension}`, { baseDir: BaseDirectory.Audio })
      const blob = new Blob([fileContent], { type: lossless ? 'audio/flac' : 'audio/mp3' })
      const url = URL.createObjectURL(blob)

      sound = new Howl({
        src: [url],
        format: [fileExtension],
        html5: streaming,
        onend: () => {
          if (this.looping.value) {
            sound!.play()
          } else {
            this.skip()
          }
        },
        onload: () => {
          this.duration.value = sound!.duration()
          this.setupEqualizer()
          this.setupEqListener()
          this.applyEQ(eq)
        },
        onloaderror: (id, error) => {
          console.error('Error loading audio:', error)
        },
        onpause: async () => {
          this.paused.value = true
          await invoke('clear_activity')
        },
        onplay: async () => {
          this.paused.value = false
          this.updateProgress()
          await invoke('update_activity', {
            details: `by ${this.currentSong.value?.artist}`,
            largeImage: 'https://api.vleer.app/thumbnail?id=' + this.currentSong.value?.id,
            largeImageText: this.currentSong.value?.title,
            state: this.currentSong.value?.title,
            youtubeUrl: 'https://www.youtube.com/watch?v=' + this.currentSong.value?.id
          })
        },
        onseek: () => {
          this.updateProgress()
        }
      })

      const updateProgressInterval = setInterval(() => {
        if (sound && !this.paused.value) {
          const seek = sound.seek() as number
          this.progress.value = (seek / this.duration.value) * 100
          this.time.value = seek
        }
      }, 1000)

      sound.on('end', () => {
        clearInterval(updateProgressInterval)
      })

      sound.volume(this.volume.value / 100)
      sound.mute(this.muted.value)
    },

    getAnalyzerData() {
      if (!analyzer) return null

      const bufferLength = analyzer.frequencyBinCount
      const dataArray = new Uint8Array(bufferLength)
      analyzer.getByteFrequencyData(dataArray)
      return dataArray
    },

    mute() {
      this.muted.value = !this.muted.value
      if (sound) {
        sound.mute(this.muted.value)
      }
      const { $settings } = useNuxtApp()
      $settings.setMuted(this.muted.value)
    },

    pause() {
      if (sound) {
        sound.pause()
      }
    },

    play() {
      if (sound) {
        sound.play()
      }
    },

    playPause() {
      if (sound) {
        if (this.paused.value) {
          this.play()
        } else {
          this.pause()
        }
      }
    },

    rewind() {
      if (sound) {
        sound.seek(0)
      }
    },

    setEqGain(filterIndex: number, gain: number) {
      if (equalizer && equalizer[filterIndex]) {
        if (Number.isFinite(gain)) {
          equalizer[filterIndex].gain.setValueAtTime(gain, Howler.ctx.currentTime);
        } else {
          console.error('Attempted to set non-finite gain:', gain);
        }
      } else {
        console.error('Equalizer filter not found or invalid filter index:', filterIndex);
      }
    },

    async setVolume(value: number) {
      this.volume.value = value
      if (sound) {
        sound.volume(this.volume.value / 100)
      }
      const { $settings } = useNuxtApp()
      await $settings.setVolume(this.volume.value / 100)
    },

    setupEqualizer() {
      if (!sound) return

      const node = (sound as any)._sounds[0]._node
      const ctx = Howler.ctx

      if (!node.sourceNode) {
        analyzer = ctx.createAnalyser()
        node.sourceNode = ctx.createMediaElementSource(node)

        const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000]
        equalizer = frequencies.map(freq => {
          const filter = ctx.createBiquadFilter()
          filter.type = 'peaking'
          filter.frequency.value = freq
          filter.Q.value = 1
          filter.gain.value = 0
          return filter
        })

        node.sourceNode.connect(equalizer[0])
        equalizer.reduce((prev, curr) => {
          prev.connect(curr)
          return curr
        })
        equalizer[equalizer.length - 1].connect(analyzer)
        analyzer.connect(ctx.destination)
      }
    },

    setupEqListener() {
      listen('eq-change', (event: any) => {
        const newEq = event.payload as EQSettings
        this.applyEQ(newEq)
      })
    },

    async skip() {
      const { $settings } = useNuxtApp()
      const queue = await $settings.getQueue()
      if (queue.length > 0) {
        const nextSong = queue.shift()
        if (nextSong) {
          await $settings.setQueue(queue)
          await this.loadSong(nextSong)
          this.play()
        }
      }
    },

    skipTo(percentage: number) {
      if (sound) {
        const seekTime = (percentage / 100) * this.duration.value
        sound.seek(seekTime)
      }
    },

    async toggleLoop() {
      this.looping.value = !this.looping.value
      const { $settings } = useNuxtApp()
      await $settings.setLoop(this.looping.value)
    },

    updateProgress() {
      if (sound && !this.paused) {
        const seek = sound.seek() as number
        this.progress.value = (seek / this.duration.value) * 100
        this.time.value = seek
        requestAnimationFrame(() => this.updateProgress())
      }
    }
  }

  return {
    provide: {
      player
    }
  }
})
