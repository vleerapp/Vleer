import { invoke } from '@tauri-apps/api/core'
import type { Settings, Song, EQSettings } from '~/types/types'

export default defineNuxtPlugin((nuxtApp) => {
  return {
    provide: {
      settings: {
        async getApiUrl(): Promise<string> {
          return await invoke('get_api_url')
        },
        async getCurrentSong(): Promise<Song | null> {
          return await invoke('get_current_song')
        },
        async getEq(): Promise<EQSettings> {
          return await invoke('get_eq')
        },
        async getLossless(): Promise<boolean> {
          return await invoke('get_lossless')
        },
        async getLoop(): Promise<boolean> {
          return await invoke('get_loop')
        },
        async getMuted(): Promise<boolean> {
          return await invoke('get_muted')
        },
        async getQueue(): Promise<Song[]> {
          return await invoke('get_queue')
        },
        async getShuffle(): Promise<boolean> {
          return await invoke('get_shuffle')
        },
        async getStreaming(): Promise<boolean> {
          return await invoke('get_streaming')
        },
        async getVolume(): Promise<number> {
          return await invoke('get_volume')
        },
        async setApiUrl(apiUrl: string) {
          console.log(apiUrl)
          return await invoke('set_api_url', { apiUrl })
        },
        async setCurrentSong(currentSong: Song | null) {
          return await invoke('set_current_song', { currentSong })
        },
        async setEq(eq: EQSettings) {
          return await invoke('set_eq', { eq })
        },
        async setLossless(lossless: boolean) {
          return await invoke('set_lossless', { lossless })
        },
        async setLoop(loop: boolean) {
          return await invoke('set_loop', { loop })
        },
        async setMuted(muted: boolean) {
          return await invoke('set_muted', { muted })
        },
        async setQueue(queue: Song[]) {
          return await invoke('set_queue', { queue });
        },
        async setShuffle(shuffle: boolean) {
          return await invoke('set_shuffle', { shuffle })
        },
        async setStreaming(streaming: boolean) {
          return await invoke('set_streaming', { streaming })
        },
        async setVolume(volume: number) {
          return await invoke('set_volume', { volume })
        }
      }
    }
  }
})
