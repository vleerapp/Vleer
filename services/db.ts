import Database from '@tauri-apps/plugin-sql'
import { appDataDir, join } from '@tauri-apps/api/path'

let db: Database

export async function initDb() {
  if (!db) {
    const appDataDirPath = await appDataDir()
    const dbPath = await join(appDataDirPath, 'data.db')

    db = await Database.load(`sqlite:${dbPath}`)

    await db.execute(`
      CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT
      );

      CREATE TABLE IF NOT EXISTS songs (
        id TEXT PRIMARY KEY,
        title TEXT,
        artist TEXT,
        album TEXT,
        cover TEXT,
        date_added TEXT,
        duration INTEGER
      );

      CREATE TABLE IF NOT EXISTS playlists (
        id TEXT PRIMARY KEY,
        date_created TEXT,
        name TEXT,
        songs TEXT
      );

      CREATE TABLE IF NOT EXISTS history (
        id TEXT PRIMARY KEY,
        date_played TEXT,
        song_id TEXT,
        FOREIGN KEY(song_id) REFERENCES songs(id)
      );
    `)
  }
}

export function getDb() {
  if (!db) {
    throw new Error('Database not initialized')
  }
  return db
}