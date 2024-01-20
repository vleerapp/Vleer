use std::fs::File;
use std::io::Write;
use std::path::Path;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[tauri::command]
pub async fn download_youtube_video_as_mp3(
    url: String,
    output_path: String,
) -> Result<String, String> {
    println!("URL received: {}", url);
    // Run yt-dlp and parse its JSON output
    let output = YoutubeDl::new(&url)
        .socket_timeout("15")
        .run_async()
        .await
        .map_err(|e| e.to_string())?;

    let output_string = format!("{:?}", output);
    println!("{}", output_string);

    // Check if the output contains video information
    if let YoutubeDlOutput::SingleVideo(video) = output {
        // Extract the audio URL from the video information
        let audio_url = video
            .url
            .ok_or("Failed to extract audio URL from video information")?;

        // Download the audio file
        let response = reqwest::get(audio_url)
            .await
            .map_err(|e| e.to_string())?
            .bytes()
            .await
            .map_err(|e| e.to_string())?;

        // Write the audio file to the specified output path
        let file_path = Path::new(&output_path);
        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        file.write_all(&response).map_err(|e| e.to_string())?;

        Ok(file_path.to_string_lossy().to_string())
    } else {
        Err("Failed to download video as MP3".to_string())
    }
}
