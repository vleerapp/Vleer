use discord_ipc_rp::{activity, DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref DRPC_CLIENT: Mutex<Option<DiscordIpcClient>> =
        Mutex::new(Some(DiscordIpcClient::new("1194990403963858984")));
}

#[tauri::command]
pub fn connect_rpc() -> Result<(), String> {
    let mut drpc = DRPC_CLIENT.lock().unwrap();
    if let Some(ref mut client) = *drpc {
        match client.connect() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to connect to Discord IPC: {}", e)),
        }
    } else {
        Err("Discord IPC client not initialized".to_string())
    }
}

#[tauri::command]
pub fn clear_activity() -> Result<(), String> {
    let mut drpc = DRPC_CLIENT.lock().unwrap();
    if let Some(ref mut client) = *drpc {
        client.clear_activity().map_err(|e| e.to_string())
    } else {
        Err("Discord IPC client not initialized".to_string())
    }
}

#[tauri::command]
pub fn update_activity(
    state: String,
    details: String,
    large_image: String,
    large_image_text: String,
    youtube_url: Option<String>,
) -> Result<(), String> {
    thread::spawn(move || {
        let mut drpc = DRPC_CLIENT.lock().unwrap();
        if let Some(ref mut client) = *drpc {
            let mut activity_builder = activity::Activity::new()
                .state(&state)
                .details(&details)
                .assets(
                    activity::Assets::new()
                        .large_image(&large_image)
                        .large_text(&large_image_text),
                );

            if let Some(ref url) = youtube_url {
                let youtube_button = activity::Button::new("YouTube", url); 
                activity_builder = activity_builder.buttons(vec![youtube_button]);
            }

            client
                .set_activity(activity_builder)
                .expect("Failed to set activity");
        }
    });
    Ok(())
}
