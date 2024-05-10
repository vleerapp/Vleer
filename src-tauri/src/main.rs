#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod discord_rpc;

use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

fn main() {
    env_logger::init();
    let _ = discord_rpc::connect_rpc();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = commands::show_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            commands::download,
            commands::ping_urls
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let _ = commands::check_for_updates(app.handle().clone()).await;
                if let Some(window) = app.get_window("main") {
                    let _ = window.restore_state(StateFlags::all());
                }
            });
            Ok(())
        })
        .on_window_event(|app, event| match event {
            tauri::WindowEvent::CloseRequested { .. } |
            tauri::WindowEvent::Destroyed => {
                let _ = AppHandleExt::save_window_state(app.app_handle(), StateFlags::all());
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
