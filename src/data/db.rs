use std::path::Path;

use gpui::{App, Global};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
};
use tracing::debug;

use crate::data::types::{Album, Artist, Cuid, EventContext, Playlist, Song};

pub async fn create_pool(path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    debug!("Creating database pool at {:?}", path.as_ref());

    let options = SqliteConnectOptions::new()
        .filename(path)
        .optimize_on_close(true, None)
        .synchronous(SqliteSynchronous::Normal)
        .journal_mode(SqliteJournalMode::Wal)
        .statement_cache_capacity(0)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Global for Database {}

impl Database {
    pub fn init(cx: &mut App, pool: SqlitePool) {
        cx.set_global(Database { pool });
    }

    pub async fn insert_song(
        &self,
        title: String,
        artist_id: Option<Cuid>,
        album_id: Option<Cuid>,
        genre: Option<String>,
        date: Option<String>,
        duration: Option<i32>,
        cover: Option<String>,
        track_number: Option<i32>,
    ) -> Result<Cuid, sqlx::Error> {
        let id = Cuid::new();
        sqlx::query(
                "INSERT INTO songs (id, title, artist_id, album_id, genre, date, duration, cover, track_number)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&title)
            .bind(&artist_id)
            .bind(&album_id)
            .bind(&genre)
            .bind(&date)
            .bind(&duration)
            .bind(&cover)
            .bind(&track_number)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn get_all_songs(&self) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as::<_, Song>("SELECT * FROM songs")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_song(&self, id: &Cuid) -> Result<Song, sqlx::Error> {
        sqlx::query_as::<_, Song>("SELECT * FROM songs WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert_artist(
        &self,
        name: String,
        image: Option<String>,
    ) -> Result<Cuid, sqlx::Error> {
        let id = Cuid::new();
        sqlx::query("INSERT INTO artists (id, name, image) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&name)
            .bind(&image)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn get_all_artists(&self) -> Result<Vec<Artist>, sqlx::Error> {
        sqlx::query_as::<_, Artist>("SELECT * FROM artists")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_artist(&self, id: &Cuid) -> Result<Artist, sqlx::Error> {
        sqlx::query_as::<_, Artist>("SELECT * FROM artists WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert_album(
        &self,
        title: String,
        artist: Option<Cuid>,
        cover: Option<String>,
    ) -> Result<Cuid, sqlx::Error> {
        let id = Cuid::new();
        sqlx::query("INSERT INTO albums (id, title, artist, cover) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&title)
            .bind(&artist)
            .bind(&cover)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn get_all_albums(&self) -> Result<Vec<Album>, sqlx::Error> {
        sqlx::query_as::<_, Album>("SELECT * FROM albums")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_album(&self, id: &Cuid) -> Result<Album, sqlx::Error> {
        sqlx::query_as::<_, Album>("SELECT * FROM albums WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert_playlist(
        &self,
        name: String,
        description: Option<String>,
        image: Option<String>,
    ) -> Result<Cuid, sqlx::Error> {
        let id = Cuid::new();
        sqlx::query("INSERT INTO playlists (id, name, description, image) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&name)
            .bind(&description)
            .bind(&image)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn get_all_playlists(&self) -> Result<Vec<Playlist>, sqlx::Error> {
        sqlx::query_as::<_, Playlist>("SELECT * FROM playlists")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_playlist(&self, id: &Cuid) -> Result<Playlist, sqlx::Error> {
        sqlx::query_as::<_, Playlist>("SELECT * FROM playlists WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert_event(
        &self,
        event_type: crate::data::types::EventType,
        context_id: Option<Cuid>,
    ) -> Result<Cuid, sqlx::Error> {
        let id = Cuid::new();
        sqlx::query("INSERT INTO events (id, event_type, context_id) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&event_type)
            .bind(&context_id)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn insert_event_context(
        &self,
        song_id: Option<Cuid>,
        playlist_id: Option<Cuid>,
    ) -> Result<Cuid, sqlx::Error> {
        let id = Cuid::new();
        sqlx::query("INSERT INTO event_contexts (id, song_id, playlist_id) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&song_id)
            .bind(&playlist_id)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn get_event_context(&self, id: &Cuid) -> Result<EventContext, sqlx::Error> {
        sqlx::query_as::<_, EventContext>("SELECT * FROM event_contexts WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
