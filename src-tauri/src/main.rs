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
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{command, Manager};
use crate::discord_rpc::DiscordRpc;

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
fn get_os() -> OS {
    #[cfg(target_os = "windows")]
    let os = OS::Windows;
    #[cfg(target_os = "linux")]
    let os = OS::Linux;
    #[cfg(target_os = "macos")]
    let os = OS::MacOS;
    os
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
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
    // discord_rpc::initialize_rpc();
    env_logger::init();

    let mut builder = tauri::Builder::default();

    builder = builder
        .invoke_handler(tauri::generate_handler![
            discord_rpc::initialize_rpc,
            discord_rpc::update_activity_rpc, // Corrected command name
            discord_rpc::disconnect_rpc,
            download_wrapper,
            get_os,
            play_music,
            stop_music,
            pause_music,
            resume_music
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let rpc = DiscordRpc::new("1194990403963858984");
            app.manage(rpc); // Manage your DiscordRpc instance here
            tauri::async_runtime::spawn(async move {
                discord_rpc::initialize_rpc(app_handle).await;
            });
            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
