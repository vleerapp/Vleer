#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;
mod utils;

use tauri_plugin_prevent_default::Flags;

fn main() {
    let _ = utils::discord_rpc::connect_rpc();

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
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:data.db", db::migration::load_migrations())
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle();

            
            let config_path = utils::commands::get_config_path();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = db::migration::insert_data(config_path).await {
                    eprintln!("Error inserting data: {:?}", e);
                }
            });

            
            let update_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                utils::updater::check_for_updates(update_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            utils::discord_rpc::update_activity,
            utils::discord_rpc::clear_activity,
            utils::commands::download,
            utils::commands::ping_urls,
            utils::commands::get_music_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}