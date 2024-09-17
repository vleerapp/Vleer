#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;

use tauri_plugin_prevent_default::Flags;

fn main() {
    let _ = api::discord_rpc::connect_rpc();

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
            let app_handle = app.handle();
            let config_path = api::commands::get_config_path();

            let update_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                api::updater::check_for_updates(update_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::discord_rpc::update_activity,
            api::discord_rpc::clear_activity,
            api::commands::download,
            api::commands::ping_urls,
            api::commands::get_music_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
