use serde_json::Value;
use std::fs;
use regex::Regex;
use crate::utils::commands;

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
            let title = song["title"].as_str().unwrap_or_default().replace("'", "''");
            let artist = song["artist"].as_str().unwrap_or_default().replace("'", "''");
            let length = song["length"].as_i64().unwrap_or_default();
            let cover = song["cover"].as_str().unwrap_or_default().replace("'", "''");
            let cover = Regex::new(r"^https?://[^/]+").unwrap().replace(&cover, "").to_string();
            let date_added = song["date_added"].as_str().unwrap_or_default().replace("'", "''");
            let last_played = song["lastPlayed"].as_str().unwrap_or_default().replace("'", "''");

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
            let name = playlist["name"].as_str().unwrap_or_default().replace("'", "''");
            let date = playlist["date"].as_str().unwrap_or_default().replace("'", "''");
            let cover = playlist["cover"].as_str().unwrap_or_default().replace("'", "''");
            let song_ids = playlist["songs"].as_array()
                .map(|songs| songs.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>().join(","))
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
        let player_settings = json["playerSettings"].as_object().expect("Expected 'playerSettings' to be an object");

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
            let eq_json = serde_json::to_string(&eq_settings).unwrap().replace("'", "''");
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
