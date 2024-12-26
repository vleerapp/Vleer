use crate::db::types::{Album, History, Playlist, Song};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use sqlx::Row;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;
use tauri::State;

pub struct MusicDatabase {
    pub pool: SqlitePool,
}

impl MusicDatabase {
    fn get_song_cover(&self, id: &str) -> String {
        let cover_path = Path::new("Vleer")
            .join("Covers")
            .join(format!("{}.png", id));

        if cover_path.exists() {
            fs::read(cover_path)
                .ok()
                .map(|data| BASE64_STANDARD.encode(data))
                .unwrap_or_default()
        } else {
            String::new()
        }
    }
}

#[tauri::command]
pub async fn add_playlist(
    music_db: State<'_, MusicDatabase>,
    playlist: Playlist,
) -> Result<(), String> {
    sqlx::query("INSERT INTO playlists (id, name, date_created) VALUES (?, ?, ?)")
        .bind(playlist.id)
        .bind(playlist.name)
        .bind(playlist.date_created.to_rfc3339())
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_song(music_db: State<'_, MusicDatabase>, song: Song) -> Result<(), String> {
    let song_id = song.id.clone();
    let cover_data = song.cover.clone();

    sqlx::query(
        "INSERT INTO songs (id, title, artist, album, cover, date_added, duration) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&song_id)
    .bind(&song.title)
    .bind(&song.artist)
    .bind(&song.album)
    .bind(&cover_data)
    .bind(song.date_added.to_rfc3339())
    .bind(song.duration)
    .execute(&music_db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let cover_path = Path::new("Vleer")
        .join("Covers")
        .join(format!("{}.png", song_id));
    fs::write(cover_path, cover_data).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn add_song_to_history(
    music_db: State<'_, MusicDatabase>,
    song: Song,
) -> Result<(), String> {
    let history = History {
        id: Utc::now().timestamp_millis().to_string(),
        date_played: Utc::now(),
        song,
    };
    sqlx::query("INSERT INTO history (id, date_played, song_id) VALUES (?, ?, ?)")
        .bind(history.id)
        .bind(history.date_played.to_rfc3339())
        .bind(history.song.id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_song_to_playlist(
    music_db: State<'_, MusicDatabase>,
    playlist_id: String,
    song: Song,
) -> Result<(), String> {
    sqlx::query("INSERT INTO playlist_songs (playlist_id, song_id) VALUES (?, ?)")
        .bind(playlist_id)
        .bind(song.id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn clear_history(music_db: State<'_, MusicDatabase>) -> Result<(), String> {
    sqlx::query("DELETE FROM history")
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_history(music_db: State<'_, MusicDatabase>) -> Result<Vec<History>, String> {
    let rows = sqlx::query(
        "SELECT id, date_played, song_id FROM history ORDER BY date_played DESC LIMIT 5",
    )
    .fetch_all(&music_db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut history = Vec::new();
    for row in rows {
        let song = get_song(music_db.clone(), row.get("song_id"))
            .await?
            .unwrap();
        history.push(History {
            id: row.get("id"),
            date_played: row.get::<String, _>("date_played").parse().unwrap(),
            song,
        });
    }

    Ok(history)
}

#[tauri::command]
pub async fn get_playlist(
    music_db: State<'_, MusicDatabase>,
    id: String,
) -> Result<Option<Playlist>, String> {
    let row = sqlx::query("SELECT id, name, date_created FROM playlists WHERE id = ?")
        .bind(id)
        .fetch_optional(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(row) = row {
        let songs = get_songs_in_playlist(music_db.clone(), row.get("id")).await?;
        Ok(Some(Playlist {
            id: row.get("id"),
            name: row.get("name"),
            date_created: row.get::<String, _>("date_created").parse().unwrap(),
            songs,
        }))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn get_playlists(music_db: State<'_, MusicDatabase>) -> Result<Vec<Playlist>, String> {
    let rows = sqlx::query("SELECT id, name, date_created FROM playlists")
        .fetch_all(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut playlists = Vec::new();
    for row in rows {
        let songs = get_songs_in_playlist(music_db.clone(), row.get("id")).await?;
        playlists.push(Playlist {
            id: row.get("id"),
            name: row.get("name"),
            date_created: row.get::<String, _>("date_created").parse().unwrap(),
            songs,
        });
    }

    Ok(playlists)
}

#[tauri::command]
pub async fn get_song(
    music_db: State<'_, MusicDatabase>,
    id: String,
) -> Result<Option<Song>, String> {
    let row = sqlx::query(
        "SELECT id, title, artist, album, cover, date_added, duration FROM songs WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&music_db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(row) = row {
        let mut song = Song {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            cover: row.get("cover"),
            date_added: row.get::<String, _>("date_added").parse().unwrap(),
            duration: row.get("duration"),
        };
        song.cover = music_db.get_song_cover(&song.id);
        Ok(Some(song))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn get_songs(music_db: State<'_, MusicDatabase>) -> Result<Vec<Song>, String> {
    let rows = sqlx::query(
        "SELECT id, title, artist, album, cover, date_added, duration FROM songs ORDER BY title",
    )
    .fetch_all(&music_db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut songs = Vec::new();
    for row in rows {
        let mut song = Song {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            cover: row.get("cover"),
            date_added: row.get::<String, _>("date_added").parse().unwrap(),
            duration: row.get("duration"),
        };
        song.cover = music_db.get_song_cover(&song.id);
        songs.push(song);
    }

    Ok(songs)
}

#[tauri::command]
pub async fn remove_song(
    music_db: State<'_, MusicDatabase>,
    song_id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM songs WHERE id = ?")
        .bind(song_id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_song_from_history(
    music_db: State<'_, MusicDatabase>,
    song_id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM history WHERE song_id = ?")
        .bind(song_id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_song_from_playlist(
    music_db: State<'_, MusicDatabase>,
    playlist_id: String,
    song_id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM playlist_songs WHERE playlist_id = ? AND song_id = ?")
        .bind(playlist_id)
        .bind(song_id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_playlist(
    music_db: State<'_, MusicDatabase>,
    playlist_id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM playlists WHERE id = ?")
        .bind(playlist_id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_album(
    music_db: State<'_, MusicDatabase>,
    album_id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM albums WHERE id = ?")
        .bind(album_id)
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_album(music_db: State<'_, MusicDatabase>, album: Album) -> Result<(), String> {
    sqlx::query("INSERT INTO albums (id, name, artist, cover, date_added) VALUES (?, ?, ?, ?, ?)")
        .bind(album.id)
        .bind(album.name)
        .bind(album.artist)
        .bind(album.cover)
        .bind(album.date_added.to_rfc3339())
        .execute(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_album(
    music_db: State<'_, MusicDatabase>,
    id: String,
) -> Result<Option<Album>, String> {
    let row = sqlx::query("SELECT id, name, artist, cover, date_added FROM albums WHERE id = ?")
        .bind(id)
        .fetch_optional(&music_db.pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(row) = row {
        let songs = get_songs_in_album(music_db.clone(), row.get("id")).await?;
        Ok(Some(Album {
            id: row.get("id"),
            name: row.get("name"),
            artist: row.get("artist"),
            cover: row.get("cover"),
            date_added: row.get::<String, _>("date_added").parse().unwrap(),
            songs,
        }))
    } else {
        Ok(None)
    }
}

async fn get_songs_in_playlist(
    music_db: State<'_, MusicDatabase>,
    playlist_id: String,
) -> Result<Vec<Song>, String> {
    let rows = sqlx::query(
        "SELECT s.id, s.title, s.artist, s.album, s.cover, s.date_added, s.duration
         FROM songs s
         JOIN playlist_songs ps ON s.id = ps.song_id
         WHERE ps.playlist_id = ?",
    )
    .bind(playlist_id)
    .fetch_all(&music_db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut songs = Vec::new();
    for row in rows {
        let mut song = Song {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            cover: row.get("cover"),
            date_added: row.get::<String, _>("date_added").parse().unwrap(),
            duration: row.get("duration"),
        };
        song.cover = music_db.get_song_cover(&song.id);
        songs.push(song);
    }

    Ok(songs)
}

async fn get_songs_in_album(
    music_db: State<'_, MusicDatabase>,
    album_id: String,
) -> Result<Vec<Song>, String> {
    let rows = sqlx::query(
        "SELECT s.id, s.title, s.artist, s.album, s.cover, s.date_added, s.duration
         FROM songs s
         JOIN album_songs as ON s.id = as.song_id
         WHERE as.album_id = ?",
    )
    .bind(album_id)
    .fetch_all(&music_db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut songs = Vec::new();
    for row in rows {
        let mut song = Song {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            cover: row.get("cover"),
            date_added: row.get::<String, _>("date_added").parse().unwrap(),
            duration: row.get("duration"),
        };
        song.cover = music_db.get_song_cover(&song.id);
        songs.push(song);
    }

    Ok(songs)
}
