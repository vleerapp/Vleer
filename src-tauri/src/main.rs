#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{anyhow, Result};
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use tauri::Error as TauriError;

mod discord_rpc;

#[tauri::command]
async fn download(url: String) -> Result<(), TauriError> {

    let ytId = url.trim_start_matches("https://youtube.com/watch?v=");

    let client = reqwest::Client::new();
    let response = client.get(format!("https://wave.wireway.ch/api/extract/music?q=/watch?v={}", ytId))
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to download video: {}", response.status()).into());
    }
    
    let video_data = response.json::<serde_json::Value>().await.map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
    let video_url = video_data["url"].as_str().ok_or_else(|| anyhow!("URL not found in response"))?;

    let video_response = client.get(video_url)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to download video: {}", e))?;

    if !video_response.status().is_success() {
        return Err(anyhow!("Failed to download video: {}", video_response.status()).into());
    }

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
    path.push(format!("{}.webm", ytId));

    let mut file = tokio::fs::File::create(&path).await.map_err(|e| anyhow!("Failed to create file: {}", e))?;
    let content = video_response.bytes().await.map_err(|e| anyhow!("Failed to read video bytes: {}", e))?;
    file.write_all(&content).await.map_err(|e| anyhow!("Failed to write video to file: {}", e))?;

    println!("Downloaded: {}", path.display());
    Ok(())
}

fn main() {
    env_logger::init();
    let _ = discord_rpc::connect_rpc();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
