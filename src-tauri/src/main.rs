#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod discord_rpc;

fn main() {
    env_logger::init();
    let _ = discord_rpc::connect_rpc();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            commands::download,
            commands::ping_urls
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                // let app_handle = app.handle().clone();
                // let updater = app_handle.updater().unwrap();
                // let response = updater.check().await;
                // if let Err(e) = &response {
                //     println!("Error checking for updates: {:?}", e);
                // }
                let _ = commands::check_for_updates(app.handle().clone()).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
