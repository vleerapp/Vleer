use rusty_ytdl::Video;
use std::path::PathBuf;

#[tauri::command]
pub async fn download(url: String, name: String) {
    let video = Video::new(url.clone()).unwrap();

    let mut path = PathBuf::new();
    match std::env::consts::OS {
        "macos" | "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            path.push(format!("/users/{}/Music/Vleer", username));
        },
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
        },
        _ => {}
    }
    path.push(&name);

    println!("{}", &path.display());

    video.download(&path).await.unwrap();
}
