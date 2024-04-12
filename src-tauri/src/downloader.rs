use crate::config;
use anyhow::{anyhow, Result};
use chrono::Local;
use image::{self, ImageFormat};
use reqwest::Client;
use rusty_ytdl::Video;
use serde::Deserialize;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    items: Vec<ApiItem>,
}

#[derive(Debug, Deserialize)]
struct ApiItem {
    #[serde(rename = "title")]
    title: Option<String>,
    #[serde(rename = "uploaderName")]
    uploader_name: Option<String>,
    #[serde(rename = "thumbnail")]
    thumbnail: Option<String>,
    #[serde(rename = "duration")]
    duration: Option<u32>,
}

#[tauri::command]
pub async fn download(url: String, name: String) -> Result<()> {
    let video = Video::new(url.clone()).map_err(|e| anyhow!(e.to_string()))?;

    let client = Client::new();

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
    if !base_path.exists() {
        fs::create_dir_all(&base_path)?;
    }

    let mut path = base_path.clone();
    path.push("Songs");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    path.push(&name);

    video
        .download(&path)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    let api_url = format!(
        "https://wireway.ch/api/musicAPI/search/?q={}",
        url.trim_start_matches("https://youtube.com/watch?v=")
    );
    let resp_body = reqwest::get(&api_url).await?.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&resp_body)?;

    let mut cover_path = String::new();

    if let Some(first_item) = api_response.items.first() {
        let title = first_item
            .title
            .as_ref()
            .unwrap_or(&String::from("Unknown Title"))
            .clone();
        let artist = first_item
            .uploader_name
            .as_ref()
            .unwrap_or(&String::from("Unknown Artist"))
            .clone();
        let cover = first_item.thumbnail.clone().unwrap_or_default();
        let length = first_item.duration.unwrap_or(0);

        let video_id = url.trim_start_matches("https://youtube.com/watch?v=");
        let date_added = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        if !cover.is_empty() {
            let response = client.get(&cover).send().await?;
            if response.status().is_success() {
                let body = response.bytes().await?;
                let image = image::load_from_memory(&body)?;
                let covers_path = base_path.join("Covers");
                if !covers_path.exists() {
                    fs::create_dir_all(&covers_path)?;
                }
                let cover_file_name = format!("{}.png", video_id);
                let cover_file_path = covers_path.join(&cover_file_name);
                let mut file = File::create(&cover_file_path)?;
                image.write_to(&mut file, ImageFormat::Png)?;
                cover_path = format!("/Covers/{}", cover_file_name);
            }
        }

        config::write_song(
            video_id.to_string(),
            title,
            artist,
            length,
            cover_path,
            date_added,
        )
        .map_err(|e| anyhow!(e.to_string()))?;
    }

    println!("Downloaded and tagged: {}", path.display());
    Ok(())
}
