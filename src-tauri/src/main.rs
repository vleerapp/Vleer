// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod discord_rpc;
mod downloader;

fn main() {
    discord_rpc::initialize_rpc();
    env_logger::init();

    // Start the Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            downloader::download_youtube_video_as_mp3
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
