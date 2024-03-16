#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod discord_rpc;
mod downloader;
mod music_handler;

use music_handler::MusicHandlerWrapper;
use std::sync::{Arc, Mutex};
use tauri::command;

lazy_static! {
    static ref MUSIC_HANDLER: Arc<Mutex<MusicHandlerWrapper>> =
        Arc::new(Mutex::new(MusicHandlerWrapper::new()));
}

#[command]
async fn download_wrapper(url: String, name: String) -> Result<(), String> {
    downloader::download(url, name)
        .await
        .map_err(|e| e.to_string())
}

#[command]
fn play_music(path: String) {
    let handler = MUSIC_HANDLER.lock().unwrap();
    handler.play(path);
}

#[command]
fn stop_music() {
    let handler = MUSIC_HANDLER.lock().unwrap();
    handler.stop();
}

#[command]
fn pause_music() {
    let handler = MUSIC_HANDLER.lock().unwrap();
    handler.pause();
}

#[command]
fn resume_music() {
    let handler = MUSIC_HANDLER.lock().unwrap();
    handler.resume();
}

fn main() {
    env_logger::init();
    discord_rpc::initialize_rpc();

    let mut builder = tauri::Builder::default().plugin(tauri_plugin_os::init());

    builder = builder
        .invoke_handler(tauri::generate_handler![
            download_wrapper,
            play_music,
            stop_music,
            pause_music,
            resume_music,
            discord_rpc::update_activity
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
