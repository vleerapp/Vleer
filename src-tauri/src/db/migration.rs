use crate::utils::commands;
use regex::Regex;
use serde_json::Value;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs;
use std::path::PathBuf;
use tauri_plugin_sql::{Migration, MigrationKind};

pub async fn insert_data(app_data_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("data.db");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM songs")
        .fetch_one(&pool)
        .await?;

    if count.0 > 0 {
        return Ok(());
    }

    let songs_sql = generate_songs_insert_sql();
    for statement in songs_sql.split(';') {
        if !statement.trim().is_empty() {
            sqlx::query(statement).execute(&pool).await?;
        }
    }

    let playlists_sql = generate_playlists_insert_sql();
    for statement in playlists_sql.split(';') {
        if !statement.trim().is_empty() {
            sqlx::query(statement).execute(&pool).await?;
        }
    }

    let settings_sql = generate_settings_insert_sql();
    for statement in settings_sql.split(';') {
        if !statement.trim().is_empty() {
            sqlx::query(statement).execute(&pool).await?;
        }
    }

    Ok(())
}

pub fn generate_songs_insert_sql() -> String {
    let path = commands::get_music_path().join("songs.json");
    if !path.exists() {
        return String::new();
    }
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut inserts = Vec::new();
    if let Some(songs) = json["songs"].as_object() {
        for (id, song) in songs {
            let title = song["title"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let artist = song["artist"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let length = song["length"].as_i64().unwrap_or_default();
            let cover = song["cover"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let cover = Regex::new(r"^https?://[^/]+")
                .unwrap()
                .replace(&cover, "")
                .to_string();
            let date_added = song["date_added"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let last_played = song["lastPlayed"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");

            inserts.push(format!(
                "INSERT INTO songs (id, title, artist, length, cover, date_added, last_played) VALUES ('{}', '{}', '{}', {}, '{}', '{}', '{}');",
                id, title, artist, length, cover, date_added, last_played
            ));
        }
    }

    inserts.join("\n")
}

pub fn generate_playlists_insert_sql() -> String {
    let path = commands::get_music_path().join("songs.json");
    if !path.exists() {
        return String::new();
    }
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut inserts = Vec::new();
    if let Some(playlists) = json["playlists"].as_object() {
        for (id, playlist) in playlists {
            let name = playlist["name"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let date = playlist["date"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let cover = playlist["cover"]
                .as_str()
                .unwrap_or_default()
                .replace("'", "''");
            let song_ids = playlist["songs"]
                .as_array()
                .map(|songs| {
                    songs
                        .iter()
                        .filter_map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(",")
                })
                .unwrap_or_default();

            inserts.push(format!(
                "INSERT INTO playlists (id, name, date, cover, songs) VALUES ('{}', '{}', '{}', '{}', '{}');",
                id, name, date, cover, song_ids
            ));
        }
    }

    inserts.join("\n")
}

fn load_settings_json() -> Option<Value> {
    let path = commands::get_config_path().join("settings.json");
    if !path.exists() {
        return None;
    }
    let data = fs::read_to_string(path).expect("Unable to read settings file");
    Some(serde_json::from_str(&data).expect("Unable to parse settings JSON"))
}

pub fn generate_settings_insert_sql() -> String {
    if let Some(json) = load_settings_json() {
        let player_settings = json["playerSettings"]
            .as_object()
            .expect("Expected 'playerSettings' to be an object");

        let mut inserts = Vec::new();
        let mut eq_settings = serde_json::Map::new();

        for (key, value) in player_settings {
            if key == "eq" {
                if let Some(eq) = value.as_object() {
                    for (freq, val) in eq {
                        eq_settings.insert(freq.clone(), val.clone());
                    }
                }
            } else {
                let value_str = match value {
                    Value::String(s) => s.replace("'", "''"),
                    _ => value.to_string().replace("'", "''"),
                };
                inserts.push(format!(
                    "INSERT INTO settings (key, value) VALUES ('{}', '{}');",
                    key, value_str
                ));
            }
        }

        if !eq_settings.is_empty() {
            let eq_json = serde_json::to_string(&eq_settings)
                .unwrap()
                .replace("'", "''");
            inserts.push(format!(
                "INSERT INTO settings (key, value) VALUES ('eq', '{}');",
                eq_json
            ));
        }

        inserts.join("\n")
    } else {
        String::new()
    }
}
pub fn load_migrations() -> Vec<Migration> {
    let migration_v1 = r#"
    CREATE TABLE IF NOT EXISTS songs (
        id TEXT PRIMARY KEY,
        title TEXT,
        artist TEXT,
        length INTEGER,
        cover TEXT,
        date_added TEXT,
        last_played TEXT
    );
    CREATE TABLE IF NOT EXISTS playlists (
        id TEXT PRIMARY KEY,
        name TEXT,
        date TEXT,
        cover TEXT,
        songs TEXT
    );
    "#;
    let migration_v2 = r#"
    CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT
    );
    "#;

    let migration_v3 = r#"
    INSERT OR IGNORE INTO settings (key, value) VALUES ('apiURL', 'https://pipedapi.wireway.ch');
    INSERT OR IGNORE INTO settings (key, value) VALUES ('volume', '50');
    "#;

    let migration_v4 = r#"
    INSERT OR IGNORE INTO settings (key, value) VALUES ('lossless', 0);
    INSERT OR IGNORE INTO settings (key, value) VALUES ('streaming', 1);
    "#;

    vec![
        Migration {
            version: 1,
            description: "create_songs_and_playlists_tables",
            sql: migration_v1,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_settings_table",
            sql: migration_v2,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "insert_default_apiURL",
            sql: migration_v3,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "insert_default_lossless_and_streaming",
            sql: migration_v4,
            kind: MigrationKind::Up,
        },
    ]
}
