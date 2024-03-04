#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod discord_rpc;
mod downloader;

#[cfg(target_os = "macos")]
mod window_ext;

use tauri::command;
use tauri::Manager;

#[command]
async fn download_wrapper(url: String, name: String) -> Result<(), String> {
    downloader::download(url, name)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    discord_rpc::initialize_rpc();
    env_logger::init();

    let mut builder = tauri::Builder::default();

    builder = builder
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            download_wrapper,
        ])
        .setup(|app| {
            let app_handle = app.app_handle(); // Obtain the AppHandle

            // let win = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            {
                use window_ext::WindowExt;
                win.set_transparent_titlebar(true);
                win.position_traffic_lights(9.0, 16.0);
            }

            // win.show().unwrap();

            Ok(())
        });

    #[cfg(target_os = "macos")]
    {
        use tauri::WindowEvent;
        builder = builder.on_window_event(|e| {
            use window_ext::WindowExt;
            if let WindowEvent::Resized(..) = e.event() {
                let win = e.window();
                win.position_traffic_lights(9.0, 16.0);
            }
        })
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
