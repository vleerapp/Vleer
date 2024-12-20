#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod db;
mod utils;

use crate::db::{music::MusicDatabase, settings::SettingsDatabase};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::fs;
use tauri::Manager;
use tauri_plugin_prevent_default::Flags;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_prevent_default::Builder::new()
                .with_flags(Flags::all().difference(Flags::CONTEXT_MENU))
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap();
            utils::logger::init_logger(&app_data_dir).expect("Failed to initialize logger");

            let db_path = app_data_dir.join("data.db");
            let is_new_db = !db_path.exists();
            if is_new_db {
                fs::File::create(&db_path).expect("Failed to create database file");
            }

            let db_url = format!("sqlite:{}", db_path.to_str().unwrap());
            
            let app_handle = app.handle().clone();
            let update_handle = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                api::updater::check_for_updates(update_handle).await;

                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("Failed to create pool");

                let music_db = MusicDatabase { pool: pool.clone() };
                let settings_db = SettingsDatabase { pool };

                app_handle.manage(music_db);
                app_handle.manage(settings_db);
            });

            let _ = db::database::setup(app);
            api::discord_rpc::connect_rpc().ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::music::add_playlist,
            db::music::add_song,
            db::music::add_song_to_history,
            db::music::add_song_to_playlist,
            db::music::clear_history,
            db::music::get_history,
            db::music::get_playlist,
            db::music::get_playlists,
            db::music::get_song,
            db::music::get_songs,
            db::music::remove_song,
            db::music::remove_song_from_history,
            db::music::remove_song_from_playlist,
            db::music::remove_playlist,
            db::music::remove_album,
            db::music::add_album,
            db::music::get_album,
            db::settings::get_api_url,
            db::settings::get_current_song,
            db::settings::get_eq,
            db::settings::get_lossless,
            db::settings::get_loop,
            db::settings::get_muted,
            db::settings::get_queue,
            db::settings::get_shuffle,
            db::settings::get_streaming,
            db::settings::get_volume,
            db::settings::set_api_url,
            db::settings::set_current_song,
            db::settings::set_eq,
            db::settings::set_lossless,
            db::settings::set_loop,
            db::settings::set_muted,
            db::settings::set_queue,
            db::settings::set_shuffle,
            db::settings::set_streaming,
            db::settings::set_volume,
            api::commands::download_from_backend,
            api::commands::get_music_path,
            api::commands::ping_urls,
            api::discord_rpc::clear_activity,
            api::discord_rpc::update_activity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
