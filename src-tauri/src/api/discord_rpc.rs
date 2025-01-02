use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref DRPC_CLIENT: Mutex<Option<DiscordIpcClient>> =
        Mutex::new(Some(DiscordIpcClient::new("1194990403963858984").unwrap()));
}

fn is_discord_rpc_disabled() -> bool {
    let value = env::var("disable_discord_rpc").unwrap_or_else(|_| "not set".to_string());
    value == "1"
}

#[tauri::command]
pub fn connect_rpc() -> Result<(), String> {
    if is_discord_rpc_disabled() {
        return Err("Discord RPC is disabled".to_string());
    }

    let mut drpc = DRPC_CLIENT.lock().map_err(|e| e.to_string())?;
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
    if is_discord_rpc_disabled() {
        return Err("Discord RPC is disabled".to_string());
    }

    let mut drpc = DRPC_CLIENT.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut client) = *drpc {
        match client.clear_activity() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Discord IPC client not initialized".to_string())
    }
}

#[tauri::command]
pub fn update_activity(
    state: String,
    details: String,
    large_image: String,
    youtube_url: Option<String>,
) -> Result<(), String> {
    if is_discord_rpc_disabled() {
        return Err("Discord RPC is disabled".to_string());
    }

    thread::spawn(move || {
        let drpc = DRPC_CLIENT.lock().map_err(|e| e.to_string());
        if let Ok(mut drpc) = drpc {
            if let Some(ref mut client) = *drpc {
                let start_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() as i64;

                let mut activity_builder = activity::Activity::new()
                    .state(&state)
                    .details(&details)
                    .assets(
                        activity::Assets::new()
                            .large_image(&large_image)
                    )
                    .timestamps(activity::Timestamps::new().start(start_timestamp))
                    .activity_type(activity::ActivityType::Listening);

                if let Some(ref url) = youtube_url {
                    let youtube_button = activity::Button::new("YouTube", url);
                    activity_builder = activity_builder.buttons(vec![youtube_button]);
                }

                match client.set_activity(activity_builder) {
                    Ok(_) => (),
                    Err(e) => panic!("Failed to set activity: {}", e),
                }
            }
        }
    });
    Ok(())
}
