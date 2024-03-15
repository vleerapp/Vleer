#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod discord_rpc;
mod downloader;
mod music_handler;

use crate::discord_rpc::DiscordRpc;
use music_handler::MusicHandlerWrapper;
use std::sync::{Arc, Mutex};
use tauri::{command, Manager};

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

    let mut builder = tauri::Builder::default().plugin(tauri_plugin_os::init());

    builder = builder
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity_rpc,
            discord_rpc::disconnect_rpc,
            download_wrapper,
            play_music,
            stop_music,
            pause_music,
            resume_music
        ])
        .setup(move |app| {
            let app_handle = app.app_handle().clone();

            tauri::async_runtime::spawn(async move {
                match DiscordRpc::initialize_rpc("1194990403963858988").await {
                    Ok(rpc) => {
                        app_handle.manage(rpc);
                        println!("Discord RPC initialized successfully.");
                    },
                    Err(e) => println!("Failed to initialize Discord RPC: {}", e),
                }
            });

            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
