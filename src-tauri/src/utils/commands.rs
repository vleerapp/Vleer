use anyhow::{anyhow, Result as AnyhowResult};
use reqwest::Client;
use std::path::PathBuf;
use std::fs;
use tauri::Result as TauriResult;
use tokio::time::Instant;
use tokio::task::JoinHandle;
use std::fs::File;
use std::io::copy;

#[tauri::command]
pub async fn download(id: String, quality: String) -> TauriResult<()> {
    let client = Client::new();
    let response = client
        .get(format!("https://api.vleer.app/download?id={}&quality={}", id, quality))
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
pub fn get_config_path() -> PathBuf {
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
    let client = Client::new();
    let results = ping_urls_helper(&client, &urls).await?;
    Ok(results)
}

async fn ping_urls_helper(
    client: &Client,
    urls: &[String],
) -> AnyhowResult<Vec<(String, u128)>> {
    let mut handles: Vec<JoinHandle<AnyhowResult<(String, u128)>>> = vec![];

    for url in urls.iter() {
        let url_clone = url.clone();
        let client_clone = client.clone();

        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let result = client_clone.head(&url_clone).send().await;
            let latency = start.elapsed().as_millis();
            match result {
                Ok(_) => Ok((url_clone, latency)),    // Use the cloned URL here
                Err(_) => Ok((url_clone, u128::MAX)), // Use the cloned URL here
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