#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod discord_rpc;
mod downloader;

use base64::{engine::general_purpose::STANDARD as BASE64_ENGINE, Engine};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

#[tauri::command]
async fn download_wrapper(url: String, name: String) -> Result<(), String> {
    downloader::download(url, name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_path() -> PathBuf {
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
    path
}

#[tauri::command]
fn get_cover_base64(id: String) -> Result<String, tauri::Error> {
    let path = get_path().join(format!("Covers/{}.png", id));
    println!("{}", path.display());
    let mut file = fs::File::open(path).map_err(tauri::Error::from)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(tauri::Error::from)?;
    Ok(BASE64_ENGINE.encode(&buffer))
}

#[tauri::command]
fn write_song(
    id: String,
    title: String,
    artist: String,
    length: u32,
    cover: String,
    date_added: String,
) -> Result<(), String> {
    config::write_song(id, title, artist, length, cover, date_added).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_songs() -> Result<config::SongsConfig, String> {
    config::read_songs().map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_song_data(song_id: String) -> Result<Vec<u8>, String> {
    let path = get_path().join(format!("Songs/{}.webm", song_id));
    fs::read(path).map_err(|e| e.to_string())
}

fn main() {
    env_logger::init();
    discord_rpc::connect_rpc();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            write_song,
            read_songs,
            get_path,
            get_cover_base64,
            download_wrapper,
            get_song_data,
        ])
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
