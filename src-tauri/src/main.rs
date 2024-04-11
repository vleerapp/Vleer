#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod discord_rpc;
mod downloader;
mod config;

use tauri::command;
use std::path::PathBuf;

#[command]
async fn download_wrapper(url: String, name: String) -> Result<(), String> {
    downloader::download(url, name)
        .await
        .map_err(|e| e.to_string())
}

#[command]
fn write_song_wrapper(id: String, title: String, artist: String, length: u32, cover: String, date_added: String) -> Result<(), String> {
    config::write_song(id, title, artist, length, cover, date_added)
        .map_err(|e| e.to_string())
}

#[command]
fn read_songs_wrapper() -> Result<config::SongsConfig, String> {
    config::read_songs()
        .map_err(|e| e.to_string())
}

#[command]
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

fn main() {
    env_logger::init();
    discord_rpc::connect_rpc();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_wrapper,
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            write_song_wrapper,
            read_songs_wrapper,
            get_path
        ])
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}