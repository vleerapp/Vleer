use discord_ipc_rp::{activity, DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref DRPC_CLIENT: Mutex<DiscordIpcClient> = Mutex::new(DiscordIpcClient::new("1194990403963858984"));
}

#[tauri::command]
pub fn connect_rpc() {
    let mut drpc = DRPC_CLIENT.lock().unwrap();
    drpc.connect().expect("Failed to connect to Discord IPC");
}

#[tauri::command]
pub fn clear_activity() {
    let mut drpc = DRPC_CLIENT.lock().unwrap();
    drpc.clear_activity().expect("Failed to clear activity");
}

#[tauri::command]
pub fn update_activity(
    state: String,
    details: String,
    large_image: String,
    large_image_text: String
) {
    thread::spawn(move || {
        let mut drpc = DRPC_CLIENT.lock().unwrap();

        let activity = activity::Activity::new().state(&state).details(&details).assets(
            activity::Assets::new()
                .large_image(&large_image)
                .large_text(&large_image_text)
        );

        drpc.set_activity(activity)
            .expect("Failed to set activity");
    });
}