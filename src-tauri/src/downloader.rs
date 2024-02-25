use rusty_ytdl::{Video, VideoSearchOptions, VideoOptions, VideoQuality};

#[tauri::command]
pub async fn download(url: String) {
  let video = Video::new(url.clone()).unwrap();

  let stream = video.stream().await.unwrap();

  while let Some(chunk) = stream.chunk().await.unwrap() {
    println!("{:#?}", chunk);
  }

  let path = std::path::Path::new(r"test.mp3");

  video.download(path).await.unwrap();

  let video_options = VideoOptions {
    quality: VideoQuality::Lowest,
    filter: VideoSearchOptions::Audio,
    ..Default::default()
  };

  let video = Video::new_with_options(url, video_options).unwrap();

  let stream = video.stream().await.unwrap();

  while let Some(chunk) = stream.chunk().await.unwrap() {
    println!("{:#?}", chunk);
  }

  let path = std::path::Path::new(r"test.mp3");

  video.download(path).await.unwrap();
}