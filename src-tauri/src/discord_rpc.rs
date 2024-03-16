use discord_rpc_client::Client;
use discord_rpc_client::models::rich_presence::{Activity, ActivityAssets};
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::thread;

lazy_static! {
    static ref DRPC_CLIENT: Mutex<Client> = Mutex::new(Client::new(1194990403963858984));
}

pub fn initialize_rpc() {
    let mut drpc = DRPC_CLIENT.lock().unwrap();
    drpc.start();
}

#[tauri::command]
pub fn update_activity(state: String, details: String, large_image: String, large_image_text: String, small_image: String, small_image_text: String) {
    thread::spawn(move || {
        let mut drpc = DRPC_CLIENT.lock().unwrap();

        let activity = Activity::new()
            .state(state)
            .details(details)
            .assets(|_| ActivityAssets::new()
                .large_image(large_image)
                .large_text(large_image_text)
                .small_image(small_image)
                .small_text(small_image_text)
            );

        drpc.set_activity(|_| activity).expect("Failed to set activity");
    });
}