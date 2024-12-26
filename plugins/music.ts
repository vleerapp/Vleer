import { invoke } from '@tauri-apps/api/core'
import type { History, Playlist, Song, Album } from '~/types/types'

export default defineNuxtPlugin((nuxtApp) => {
  return {
    provide: {
      music: {
        async addPlaylist(playlist: Playlist) {
          return await invoke('add_playlist', { playlist })
        },
        async addSong(song: Song) {
          return await invoke('add_song', { song })
        },
        async addSongToHistory(song: Song) {
          return await invoke('add_song_to_history', { song })
        },
        async addSongToPlaylist(playlistId: string, song: Song) {
          return await invoke('add_song_to_playlist', { playlistId, song })
        },
        async clearHistory() {
          return await invoke('clear_history')
        },
        async getHistory(): Promise<History[]> {
          return await invoke('get_history')
        },
        async getPlaylist(id: string): Promise<Playlist | null> {
          return await invoke('get_playlist', { id })
        },
        async getPlaylists(): Promise<Playlist[]> {
          return await invoke('get_playlists')
        },
        async getSong(id: string): Promise<Song | null> {
          return await invoke('get_song', { id })
        },
        async getSongs(): Promise<Song[]> {
          return await invoke('get_songs')
        },
        async removeSong(songId: string) {
          return await invoke('remove_song', { songId })
        },
        async removeSongFromHistory(songId: string) {
          return await invoke('remove_song_from_history', { songId })
        },
        async removeSongFromPlaylist(playlistId: string, songId: string) {
          return await invoke('remove_song_from_playlist', { playlistId, songId })
        },
        async removePlaylist(playlistId: string) {
          return await invoke('remove_playlist', { playlistId })
        },
        async removeAlbum(albumId: string) {
          return await invoke('remove_album', { albumId })
        },
        async addAlbum(album: Album) {
          return await invoke('add_album', { album })
        },
        async getAlbum(id: string): Promise<Album | null> {
          return await invoke('get_album', { id })
        }
      }
    }
  }
})
