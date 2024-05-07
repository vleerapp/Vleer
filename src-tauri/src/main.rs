#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{anyhow, Result};
use rusty_ytdl::Video;
use std::path::PathBuf;
use tauri::Error as TauriError;
use reqwest::Client;
use tokio::time::Instant;

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

    println!("Downloaded: {}", path.display());
    Ok(())
}

#[tauri::command]
async fn ping_urls(urls: Vec<String>) -> Result<Vec<(String, u128)>, TauriError> {
    let client = Client::new();
    let results = ping_urls_helper(&client, &urls).await?;
    Ok(results)
}

async fn ping_urls_helper(client: &Client, urls: &[String]) -> Result<Vec<(String, u128)>, TauriError> {
    let mut handles = vec![];

    for url in urls.iter() {
        let url_clone = url.clone();
        let client_clone = client.clone();

        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let result = client_clone.head(&url_clone).send().await;
            let latency = start.elapsed().as_millis();
            match result {
                Ok(_) => (url_clone, latency), // Use the cloned URL here
                Err(_) => (url_clone, u128::MAX), // Use the cloned URL here
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(results)
}

fn main() {
    env_logger::init();
    let _ = discord_rpc::connect_rpc();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            download,
            ping_urls
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
