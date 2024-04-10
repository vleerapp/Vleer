use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Error;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Song {
    title: String,
    artist: String,
    length: u32,
    cover: String,
    date_added: String,
}

#[derive(Serialize, Deserialize)]
pub struct SongsConfig {
    songs: HashMap<String, Song>,
}

impl SongsConfig {
    fn new() -> Self {
        SongsConfig {
            songs: HashMap::new(),
        }
    }

    fn load(path: &PathBuf) -> Result<Self, Error> {
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;
        Ok(config)
    }

    fn save(&self, path: &PathBuf) -> Result<(), Error> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    fn add_song(&mut self, id: String, title: String, artist: String, length: u32, cover: String, date_added: String) {
        let song = Song {
            title,
            artist,
            length,
            cover,
            date_added,
        };
        self.songs.insert(id, song);
    }
}

fn get_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    match std::env::consts::OS {
        "macos" | "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            path.push(format!("/users/{}/Music/Vleer", username));
        }
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
        }
        _ => {}
    }
    path.push("songs.json");
    path
}

pub fn write_song(id: String, title: String, artist: String, length: u32, cover: String, date_added: String) -> Result<(), Error> {
    let path = get_config_path();
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
        File::create(&path)?;
        SongsConfig::new().save(&path)?;
    }
    let mut config = SongsConfig::load(&path)?;
    config.add_song(id, title, artist, length, cover, date_added);
    config.save(&path)
}

pub fn read_songs() -> Result<SongsConfig, Error> {
    let path = get_config_path();
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
        File::create(&path)?;
        SongsConfig::new().save(&path)?;
    }
    SongsConfig::load(&path)
}