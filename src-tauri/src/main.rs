// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod discord_rpc;

fn main() {
    // Initialize the Discord RPC client
    discord_rpc::initialize_rpc();

    // Start the Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![discord_rpc::update_activity])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
