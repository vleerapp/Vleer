use anyhow::Result;
use id3::frame::PictureType;
use id3::TagLike;
use id3::{Tag, Version};
use reqwest;
use rusty_ytdl::Video;
use serde::Deserialize;
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
}

#[tauri::command]
pub async fn download(url: String, name: String) -> Result<()> {
    let video = Video::new(url.clone()).unwrap();

    let watch_id = name.trim_end_matches(".mp3");

    let api_url = format!("https://wireway.ch/api/musicAPI/search/?q={}", watch_id);
    let resp_body = reqwest::get(&api_url).await?.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&resp_body)?;

    let mut path = PathBuf::new();
    match std::env::consts::OS {
        "macos" | "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            path.push(format!("/users/{}/Music/Vleer", username));
            if !path.exists() {
                std::fs::create_dir_all(&path).unwrap();
            }
        }
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
            if !path.exists() {
                std::fs::create_dir_all(&path).unwrap();
            }
        }
        _ => {}
    }
    path.push(&name);

    video.download(&path).await.unwrap();

    let mut tag = Tag::new();
    // Use the first item from the ApiResponse
    if let Some(first_item) = api_response.items.first() {
        tag.set_artist(
            first_item
                .uploader_name
                .as_ref()
                .unwrap_or(&String::from("Unknown Artist")),
        );
        tag.set_title(
            first_item
                .title
                .as_ref()
                .unwrap_or(&String::from("Unknown Title")),
        );

        if let Some(thumbnail_url) = &first_item.thumbnail {
            let response = reqwest::get(thumbnail_url).await?;
            if response.status().is_success() {
                let mut content: Vec<u8> = Vec::new();
                let body = response.bytes().await?;
                content.extend_from_slice(&body);
                let mime = "image/webp".to_string(); // Adjust based on actual MIME type if necessary
                tag.add_frame(id3::frame::Picture {
                    mime_type: mime,
                    picture_type: PictureType::CoverFront,
                    description: "Cover image".to_string(),
                    data: content,
                });
            }
        }
    }

    tag.write_to_path(&path, Version::Id3v23)?;

    println!(" {}", path.display());
    Ok(())
}
