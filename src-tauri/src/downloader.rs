use anyhow::Result;
use id3::{Tag, Version, frame::PictureType, TagLike};
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Write};

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
    let watch_id = url.trim_start_matches("https://www.youtube.com/watch?v=");

    let client = Client::new();
    let url = "https://wave.wireway.ch/api/transcode/download/music?q=/watch?v=06JYzej_NJ0";
    let response = client.get(url).send().await?;

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

    let mut file = File::create(&path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    let api_url = format!("https://wireway.ch/api/musicAPI/search/?q={}", watch_id);
    let resp_body = reqwest::get(&api_url).await?.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&resp_body)?;

    let mut tag = Tag::new();
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
                let mime = "image/jpeg".to_string(); // Assuming JPEG for broader compatibility
                tag.add_frame(id3::frame::Picture {
                    mime_type: mime,
                    picture_type: PictureType::CoverFront,
                    description: "Cover image".to_string(),
                    data: content,
                });
            }
        }
    }

    // Write ID3 tags to the downloaded file
    tag.write_to_path(&path, Version::Id3v24)?;

    println!("Downloaded and tagged: {}", path.display());
    Ok(())
}