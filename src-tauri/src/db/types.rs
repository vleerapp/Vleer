use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EQSettings {
    pub values: HashMap<String, String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover: String,
    #[serde_as(as = "DisplayFromStr")]
    pub date_added: DateTime<Utc>,
    pub duration: i64, 
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub cover: String,
    #[serde_as(as = "DisplayFromStr")]
    pub date_added: DateTime<Utc>,
    pub songs: Vec<Song>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub date_created: DateTime<Utc>,
    pub songs: Vec<Song>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
    pub id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub date_played: DateTime<Utc>,
    pub song: Song,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_url: String,
    pub current_song: Option<Song>,
    pub eq: EQSettings,
    pub lossless: bool,
    pub r#loop: bool,
    pub muted: bool,
    pub queue: Vec<Song>,
    pub shuffle: bool,
    pub streaming: bool,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRow {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover: String,
    pub date_added: String,  
    pub duration: i64, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRow {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub cover: String,
    pub date_added: String,  
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRow {
    pub id: String,
    pub name: String,
    pub date_created: String,  
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryRow {
    pub id: String,
    pub date_played: String,  
    pub song_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumSongRow {
    pub album_id: String,
    pub song_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistSongRow {
    pub playlist_id: String,
    pub song_id: String,
}

impl From<SongRow> for Song {
    fn from(row: SongRow) -> Self {
        Song {
            id: row.id,
            title: row.title,
            artist: row.artist,
            album: row.album,
            cover: row.cover,
            date_added: DateTime::parse_from_rfc3339(&row.date_added)
                .unwrap()
                .with_timezone(&Utc),
            duration: row.duration,
        }
    }
}

impl From<Song> for SongRow {
    fn from(song: Song) -> Self {
        SongRow {
            id: song.id,
            title: song.title,
            artist: song.artist,
            album: song.album,
            cover: song.cover,
            date_added: song.date_added.to_rfc3339(),
            duration: song.duration,
        }
    }
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            api_url: "https://api.vleer.app".to_string(),
            current_song: None,
            eq: EQSettings {
                values: [
                    ("32", "0.0"),
                    ("64", "0.0"),
                    ("125", "0.0"),
                    ("250", "0.0"),
                    ("500", "0.0"),
                    ("1000", "0.0"),
                    ("2000", "0.0"),
                    ("4000", "0.0"),
                    ("8000", "0.0"),
                    ("16000", "0.0"),
                ]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            },
            lossless: true,
            r#loop: false,
            muted: false,
            queue: Vec::new(),
            shuffle: false,
            streaming: true,
            volume: 0.5,
        }
    }
} 