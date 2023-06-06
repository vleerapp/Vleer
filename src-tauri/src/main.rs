#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use base64::{engine::general_purpose, Engine as _};
use id3::{Tag, TagLike};
use mp3_duration;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Track {
    pub source: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_art: Option<String>,
    pub duration: Option<u128>,
}

#[tauri::command]
fn get_metadata(source: &str) -> Option<Track> {
    match Tag::read_from_path(source) {
        Ok(tag) => {
            let d = match mp3_duration::from_path(source) {
                Ok(d) => Some(d),
                Err(_e) => None,
            };
            Some(Track {
                source: String::from(source),
                title: to_str(tag.title()),
                artist: to_str(tag.artist()),
                album: to_str(tag.album()),
                album_art: match tag.pictures().next() {
                    None => None,
                    Some(p) => {
                        let mut str = "data:".to_owned();
                        str.push_str(&p.mime_type);
                        str.push_str(";base64, ");
                        let base64_str = general_purpose::STANDARD_NO_PAD.encode(&p.data);
                        str.push_str(&base64_str);
                        Some(str)
                    }
                },
                duration: match d {
                    Some(d) => Some(d.as_millis()),
                    None => None,
                },
            })
        }
        Err(_e) => None,
    }
}

fn to_str(var: Option<&str>) -> Option<String> {
    var.map(|s: &str| s.to_owned())
        .or(Some(String::from("Unknown")))
}

fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_fs_extra::init())
    .invoke_handler(tauri::generate_handler![
      get_metadata,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
