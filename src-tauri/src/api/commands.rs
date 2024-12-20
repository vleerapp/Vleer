use anyhow::anyhow;
use reqwest::Client;
use std::path::PathBuf;
use tauri::Result as TauriResult;
use tokio::time::Instant;
use tokio::task::JoinHandle;
use std::fs::{self, File};
use std::io::copy;
use std::result::Result;

#[tauri::command]
pub async fn download_from_backend(
    id: String,
    quality: String,
    url: String,
) -> Result<(), tauri::Error> {
    let client = Client::new();
    let response = client
        .get(format!("{}/download?id={}&quality={}", url, id, quality))
        .send()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    let base_path = get_music_path();

    let mut path = base_path.clone();
    path.push("Songs");
    let extension = if quality == "compressed" { "mp3" } else { "flac" };
    path.push(format!("{}.{}", id, extension));

    let mut file = File::create(&path).map_err(|e| anyhow!(e.to_string()))?;
    let content = response.bytes().await.map_err(|e| anyhow!(e.to_string()))?;
    copy(&mut content.as_ref(), &mut file).map_err(|e| anyhow!(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn get_music_path() -> PathBuf {
    let mut path = PathBuf::new();
    match std::env::consts::OS {
        "macos" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            path.push(format!("/users/{}/Music/Vleer", username));
        }
        "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            path.push(format!("/home/{}/Music/Vleer", username));
        }
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
        }
        _ => {}
    }
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create music directory");
    }
    path.push("Songs");
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create Songs directory");
    }
    path.pop();

    path.push("Covers");
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create Covers directory");
    }
    path.pop();
    return path;
}

#[tauri::command]
pub fn _get_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    match std::env::consts::OS {
        "macos" => {
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| format!("/Users/{}", std::env::var("USER").unwrap_or_else(|_| "default".into())));
            path.push(format!("{}/Library/Application Support/app.vleer", home_dir));
        }
        "linux" => {
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| format!("/home/{}", std::env::var("USER").unwrap_or_else(|_| "default".into())));
            path.push(format!("{}/.config/app.vleer", home_dir));
        }
        "windows" => {
            let app_data = std::env::var("APPDATA").unwrap_or_else(|_| format!("C:\\Users\\{}\\AppData\\Roaming", std::env::var("USERNAME").unwrap_or("default".into())));
            path.push(format!("{}\\app.vleer", app_data));
        }
        _ => {}
    }
    return path;
}

#[tauri::command]
pub async fn ping_urls(urls: Vec<String>) -> TauriResult<Vec<(String, u128)>> {
    ping_urls_helper(&urls).await.map_err(|e| e.into())
}

async fn ping_urls_helper(
    urls: &[String],
) -> Result<Vec<(String, u128)>, anyhow::Error> {
    let mut handles: Vec<JoinHandle<Result<(String, u128), anyhow::Error>>> = vec![];

    for url in urls.iter() {
        let url_clone = url.clone();

        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let result = Client::new().head(&url_clone).send().await;
            let latency = start.elapsed().as_millis();
            match result {
                Ok(_) => Ok((url_clone, latency)),
                Err(e) => Err(anyhow!(e.to_string())),
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Ok(result)) = handle.await {
            results.push(result);
        }
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(results)
}