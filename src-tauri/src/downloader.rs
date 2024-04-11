use crate::config;
use anyhow::{anyhow, Result};
use chrono::Local;
// use id3::{frame::PictureType, Tag, TagLike, Version};
// use image::{self, ImageFormat};
// use reqwest::Client;
use serde::Deserialize;
// use std::fs::File;
// use std::io::{Cursor, Write};
use std::path::PathBuf;
use rusty_ytdl::Video;

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

    let video = Video::new(url.clone()).unwrap();

    // let client = Client::new();
    // let url = format!(
    //     "https://wave.wireway.ch/api/transcode/download/music?q={}",
    //     url.trim_start_matches("https://www.youtube.com")
    // );
    // let response = client.get(&url).send().await?;

    let mut path = PathBuf::new();
    match std::env::consts::OS {
        "macos" | "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            path.push(format!("/users/{}/Music/Vleer/Songs", username));
            if !path.exists() {
                std::fs::create_dir_all(&path).unwrap();
            }
        }
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            path.push(format!("C:\\Users\\{}\\Music\\Vleer\\Songs", username));
            if !path.exists() {
                std::fs::create_dir_all(&path).unwrap();
            }
        }
        _ => {}
    }
    path.push(&name);

    video.download(&path).await.unwrap();

    // let mut file = File::create(&path)?;
    // let content = response.bytes().await?;
    // file.write_all(&content)?;

    let api_url = format!(
        "https://wireway.ch/api/musicAPI/search/?q={}",
        url.trim_start_matches(
            "https://youtube.com/watch?v="
        )
    );
    let resp_body = reqwest::get(&api_url).await?.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&resp_body)?;

    // let mut tag = Tag::new();
    // if let Some(first_item) = api_response.items.first() {
    //     tag.set_artist(
    //         first_item
    //             .uploader_name
    //             .as_ref()
    //             .unwrap_or(&String::from("Unknown Artist")),
    //     );
    //     tag.set_title(
    //         first_item
    //             .title
    //             .as_ref()
    //             .unwrap_or(&String::from("Unknown Title")),
    //     );

    //     if let Some(thumbnail_url) = &first_item.thumbnail {
    //         let response = reqwest::get(thumbnail_url).await?;
    //         if response.status().is_success() {
    //             let body = response.bytes().await?;
    //             let image = image::load_from_memory(&body)?;
    //             let mut buffer = Cursor::new(Vec::new());
    //             image.write_to(&mut buffer, ImageFormat::Png)?;
    //             let png_data = buffer.into_inner();
    //             tag.add_frame(id3::frame::Picture {
    //                 mime_type: "image/png".to_string(),
    //                 picture_type: PictureType::CoverFront,
    //                 description: "Cover image".to_string(),
    //                 data: png_data,
    //             });
    //         }
    //     }
    // }

    // tag.write_to_path(&path, Version::Id3v24)?;

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

        let video_id = url.trim_start_matches(
            "https://youtube.com/watch?v=",
        );
        let date_added = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        config::write_song(
            video_id.to_string(),
            title,
            artist,
            length,
            cover,
            date_added,
        )
        .map_err(|e| anyhow!(e.to_string()))?;
    }

    println!("Downloaded and tagged: {}", path.display());
    Ok(())
}
