#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod discord_rpc;
mod downloader;

use tauri::command;
use serde::{Serialize,Deserialize};

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

fn main() {
    discord_rpc::initialize_rpc();
    env_logger::init();

    let mut builder = tauri::Builder::default();

    builder = builder
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            download_wrapper,
            get_os
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
