#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{anyhow, Result};
use rusty_ytdl::Video;
use std::path::PathBuf;
use tauri::Error as TauriError;

mod discord_rpc;

#[tauri::command]
async fn download(url: String, name: String) -> Result<(), TauriError> {
    let video = Video::new(url.clone()).map_err(|e| anyhow!(e.to_string()))?;

    let mut base_path = PathBuf::new();
    match std::env::consts::OS {
        "macos" | "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            base_path.push(format!("/users/{}/Music/Vleer", username));
        }
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            base_path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
        }
        _ => {}
    }

    let mut path = base_path.clone();
    path.push("Songs");
    path.push(&name);

    video
        .download(&path)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    println!("Downloaded and tagged: {}", path.display());
    Ok(())
}

fn main() {
    env_logger::init();
    let _ = discord_rpc::connect_rpc();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            download
        ])
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
