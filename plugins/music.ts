import { getDb, initDb } from '~/services/db'
import type { History, Playlist, Song } from '~/types/types'
import { BaseDirectory, writeFile, readFile } from '@tauri-apps/plugin-fs'

export default defineNuxtPlugin(async (nuxtApp) => {
  await initDb()
  const db = getDb()

  return {
    provide: {
      music: {
        async addPlaylist(playlist: Playlist) {
          await db.execute(`
            INSERT INTO playlists (id, date_created, name, songs)
            VALUES ($1, $2, $3, $4)
          `, [playlist.id, playlist.date_created.toISOString(), playlist.name, JSON.stringify(playlist.songs)])
        },
        async addSong(song: Song) {
          await db.execute(`
            INSERT INTO songs (id, album, artist, cover, date_added, duration, title)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
          `, [song.id, song.album, song.artist, song.cover, song.date_added.toISOString(), song.duration, song.title])

          const { $settings } = useNuxtApp()
          const apiUrl = await $settings.getApiUrl()
          if (song.cover && apiUrl) {
            try {
              const coverResponse = await fetch(`${apiUrl}/thumbnail?id=${song.id}`);
              if (coverResponse.ok) {
                const coverBlob = await coverResponse.blob();
                const coverArrayBuffer = await coverBlob.arrayBuffer();
                const coverUint8Array = new Uint8Array(coverArrayBuffer);

                writeFile(`Vleer/Covers/${song.id}.png`, coverUint8Array, { baseDir: BaseDirectory.Audio });
              } else {
                console.error('Failed to fetch cover image:', coverResponse.statusText);
              }
            } catch (error) {
              console.error('Error saving cover image:', error);
            }
          }

        },
        async addSongToHistory(song: Song) {
          const history: History = {
            date_played: new Date(),
            id: Date.now().toString(),
            song: song,
          }
          await db.execute(`
            INSERT INTO history (id, date_played, song_id)
            VALUES ($1, $2, $3)
          `, [history.id, history.date_played.toISOString(), song.id])
        },
        async addSongToPlaylist(playlistId: string, song: Song) {
          const playlist = await this.getPlaylist(playlistId)
          if (playlist) {
            playlist.songs.push(song)
            await db.execute('UPDATE playlists SET songs = $1 WHERE id = $2',
              [JSON.stringify(playlist.songs), playlistId])
          }
        },
        async clearHistory() {
          await db.execute('DELETE FROM history')
        },
        async getHistory(): Promise<History[]> {
          const rows: History[] = await db.select(`
            SELECT h.id, h.date_played, s.*
            FROM history h
            JOIN songs s ON h.song_id = s.id
          `)
          return rows.map(row => ({
            date_played: new Date(row.date_played),
            id: row.id,
            song: row.song
          }))
        },
        async getPlaylist(id: string): Promise<Playlist | null> {
          const rows: Playlist[] = await db.select('SELECT * FROM playlists WHERE id = $1', [id])
          if (rows.length > 0) {
            const row = rows[0]
            return {
              date_created: new Date(row.date_created),
              id: row.id,
              name: row.name,
              songs: row.songs
            }
          }
          return null
        },
        async getPlaylists(): Promise<Playlist[]> {
          const rows: Playlist[] = await db.select('SELECT * FROM playlists')
          return rows.map(row => ({
            date_created: new Date(row.date_created),
            id: row.id,
            name: row.name,
            songs: row.songs
          }))
        },
        async getSong(id: string): Promise<Song | null> {
          const rows: Song[] = await db.select('SELECT * FROM songs WHERE id = $1', [id])
          if (rows.length > 0) {
            const row = rows[0]
            return {
              album: row.album,
              artist: row.artist,
              cover: await this.getSongCover(id),
              date_added: new Date(row.date_added),
              duration: row.duration,
              id: row.id,
              title: row.title
            }
          }
          return null
        },
        async getSongs(): Promise<Song[]> {
          const rows: Song[] = await db.select('SELECT * FROM songs')
          return Promise.all(rows.map(async row => ({
            album: row.album,
            artist: row.artist,
            cover: await this.getSongCover(row.id),
            date_added: new Date(row.date_added),
            duration: row.duration,
            id: row.id,
            title: row.title
          })))
        },
        async removeSong(songId: string) {
          await db.execute('DELETE FROM songs WHERE id = $1', [songId])
        },
        async removeSongFromHistory(songId: string) {
          await db.execute('DELETE FROM history WHERE song_id = $1', [songId])
        },
        async removeSongFromPlaylist(playlistId: string, songId: string) {
          const playlist = await this.getPlaylist(playlistId)
          if (playlist) {
            playlist.songs = playlist.songs.filter(song => song.id !== songId)
            await db.execute('UPDATE playlists SET songs = $1 WHERE id = $2',
              [JSON.stringify(playlist.songs), playlistId])
          }
        },
        async getSongCover(id: string): Promise<string> {
          const coverData = await readFile(`Vleer/Covers/${id}.png`, { baseDir: BaseDirectory.Audio });
          const blob = new Blob([coverData], { type: 'image/png' });
          return URL.createObjectURL(blob);
        }
      }
    }
  }
})