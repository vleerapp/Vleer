import { getDb, saveDb } from '~/services/db'
import type { History, Playlist, Song } from '~/types/types'
import { BaseDirectory, writeFile, readFile } from '@tauri-apps/plugin-fs'

export default defineNuxtPlugin(async (nuxtApp) => {
  const db = await getDb()

  return {
    provide: {
      music: {
        async addPlaylist(playlist: Playlist) {
          db.getCollection('playlists').insert(playlist)
          await saveDb()
        },
        async addSong(song: Song) {
          db.getCollection('songs').insert(song)
          await saveDb()

          const { $settings } = useNuxtApp()
          const apiUrl = await $settings.getApiUrl()
          if (song.cover && apiUrl) {
            try {
              const coverResponse = await fetch(`${apiUrl}/thumbnail?id=${song.id}`)
              if (coverResponse.ok) {
                const coverBlob = await coverResponse.blob()
                const coverArrayBuffer = await coverBlob.arrayBuffer()
                const coverUint8Array = new Uint8Array(coverArrayBuffer)

                writeFile(`Vleer/Covers/${song.id}.png`, coverUint8Array, { baseDir: BaseDirectory.Audio })
              } else {
                console.error('Failed to fetch cover image:', coverResponse.statusText)
              }
            } catch (error) {
              console.error('Error saving cover image:', error)
            }
          }
        },
        async addSongToHistory(song: Song) {
          const history: History = {
            date_played: new Date(),
            id: Date.now().toString(),
            song: song,
          }
          db.getCollection('history').insert(history)
          await saveDb()
        },
        async addSongToPlaylist(playlistId: string, song: Song) {
          const playlist = await this.getPlaylist(playlistId)
          if (playlist) {
            playlist.songs.push(song)
            db.getCollection('playlists').update(playlist)
            await saveDb()
          }
        },
        async clearHistory() {
          db.getCollection('history').clear()
          await saveDb()
        },
        async getHistory(): Promise<History[]> {
          const historyCollection = db.getCollection('history')
          const history = historyCollection.find() as History[]
          
          const sortedHistory = Array.from(history).sort((a, b) => {
            return new Date(b.date_played).getTime() - new Date(a.date_played).getTime()
          })

          return sortedHistory.slice(0, 5)
        },
        async getPlaylist(id: string): Promise<Playlist | null> {
          return db.getCollection('playlists').findOne({ id }) as Playlist | null
        },
        async getPlaylists(): Promise<Playlist[]> {
          const playlistsCollection = db.getCollection('playlists')
          return (playlistsCollection.find() as Playlist[]).sort((a, b) => a.date_created.getTime() - b.date_created.getTime())
        },
        async getSong(id: string): Promise<Song | null> {
          const song = db.getCollection('songs').findOne({ id })
          if (song) {
            song.cover = await this.getSongCover(id)
          }
          return song as Song
        },
        async getSongs(): Promise<Song[]> {
          const songsCollection = db.getCollection('songs')
          const songs = (songsCollection.find() as Song[]).sort((a, b) => a.title.localeCompare(b.title))
          return Promise.all(songs.map(async (song: Song) => ({
            ...song,
            cover: await this.getSongCover(song.id)
          })))
        },
        async removeSong(songId: string) {
          db.getCollection('songs').findAndRemove({ id: songId })
          await saveDb()
        },
        async removeSongFromHistory(songId: string) {
          db.getCollection('history').findAndRemove({ 'song.id': songId })
          await saveDb()
        },
        async removeSongFromPlaylist(playlistId: string, songId: string) {
          const playlist = await this.getPlaylist(playlistId)
          if (playlist) {
            playlist.songs = playlist.songs.filter(song => song.id !== songId)
            db.getCollection('playlists').update(playlist)
            await saveDb()
          }
        },
        async getSongCover(id: string): Promise<string> {
          const coverData = await readFile(`Vleer/Covers/${id}.png`, { baseDir: BaseDirectory.Audio })
          const blob = new Blob([coverData], { type: 'image/png' })
          return URL.createObjectURL(blob)
        }
      }
    }
  }
})