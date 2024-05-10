use serde_json::Value;
use std::fs;
use crate::commands;

pub fn generate_songs_insert_sql() -> String {
    let path = commands::get_path().join("songs.json");
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut inserts = Vec::new();
    if let Some(songs) = json["songs"].as_object() {
        for (id, song) in songs {
            let title = song["title"].as_str().unwrap_or_default().replace("'", "''");
            let artist = song["artist"].as_str().unwrap_or_default().replace("'", "''");
            let length = song["length"].as_i64().unwrap_or_default();
            let cover = song["cover"].as_str().unwrap_or_default().replace("'", "''");
            let date_added = song["date_added"].as_str().unwrap_or_default().replace("'", "''");
            let cover_url = song["coverURL"].as_str().unwrap_or_default().replace("'", "''");
            let last_played = song["lastPlayed"].as_str().unwrap_or_default().replace("'", "''");

            inserts.push(format!(
                "INSERT INTO songs (id, title, artist, length, cover, date_added, cover_url, last_played) VALUES ('{}', '{}', '{}', {}, '{}', '{}', '{}', '{}');",
                id, title, artist, length, cover, date_added, cover_url, last_played
            ));
        }
    }

    inserts.join("\n")
}

pub fn generate_playlists_insert_sql() -> String {
    let path = commands::get_path().join("songs.json");
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut inserts = Vec::new();
    if let Some(playlists) = json["playlists"].as_object() {
        for (id, playlist) in playlists {
            let name = playlist["name"].as_str().unwrap_or_default().replace("'", "''");
            let date = playlist["date"].as_str().unwrap_or_default().replace("'", "''");
            let cover = playlist["cover"].as_str().unwrap_or_default().replace("'", "''");

            inserts.push(format!(
                "INSERT INTO playlists (id, name, date, cover) VALUES ('{}', '{}', '{}', '{}');",
                id, name, date, cover
            ));
        }
    }

    inserts.join("\n")
}

pub fn generate_playlist_songs_insert_sql() -> String {
    let path = commands::get_path().join("songs.json");
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut inserts = Vec::new();
    if let Some(playlists) = json["playlists"].as_object() {
        for (playlist_id, playlist) in playlists {
            if let Some(song_ids) = playlist["songs"].as_array() {
                for song_id in song_ids {
                    if let Some(song_id_str) = song_id.as_str() {
                        inserts.push(format!(
                            "INSERT INTO playlist_songs (playlist_id, song_id) VALUES ('{}', '{}');",
                            playlist_id, song_id_str
                        ));
                    }
                }
            }
        }
    }

    inserts.join("\n")
}