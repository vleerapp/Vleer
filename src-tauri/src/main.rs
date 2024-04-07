#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod discord_rpc;
mod downloader;
mod music_handler;

// use music_handler::MusicHandler;
// use rodio::OutputStream;
// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};
// use std::thread;
use tauri::command;

// enum AudioCommand {
//     Play(String), // Play a file
//     Stop,
//     SetVolume(f32),
// }

#[command]
async fn download_wrapper(url: String, name: String) -> Result<(), String> {
    downloader::download(url, name)
        .await
        .map_err(|e| e.to_string())
}

// #[tauri::command]
// async fn play_music(file_path: String) -> Result<(), String> {
//     let mut handler = MUSIC_HANDLER.lock().map_err(|e| e.to_string())?;
//     handler.play(&file_path);
//     Ok(())
// }

fn main() {
    // let (tx, rx) = mpsc::channel();

    // // Audio playback thread
    // thread::spawn(move || {
    //     let mut handler = MusicHandler::new();
    //     for cmd in rx {
    //         match cmd {
    //             AudioCommand::Play(file_path) => handler.play(&file_path)
    //         }
    //     }
    // });

    env_logger::init();
    discord_rpc::connect_rpc();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_wrapper,
            // play_music,
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
        ])
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
