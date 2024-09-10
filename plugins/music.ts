import { getDb, initDb } from '~/services/db'
import type { History, Playlist, Song } from '~/types/types'

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
              cover: row.cover,
              date_added: new Date(row.date_added),
              duration: row.duration,
              id: row.id,
              title: row.title
            }
          }
          return null
        },
        async getSongs(): Promise<Song[]> {
          const rows: Song[]  = await db.select('SELECT * FROM songs')
          return rows.map(row => ({
            album: row.album,
            artist: row.artist,
            cover: row.cover,
            date_added: new Date(row.date_added),
            duration: row.duration,
            id: row.id,
            title: row.title
          }))
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
      }
    }
  }
})