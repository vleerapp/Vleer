use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{command, Manager};

pub struct DiscordRpc {
    client: Arc<Mutex<DiscordIpcClient>>,
    connected: AtomicBool,
}

impl DiscordRpc {
    pub fn new(client_id: &str) -> Self {
        let mut client =
            DiscordIpcClient::new(client_id).expect("Failed to create DiscordIpcClient");
        let connected = client.connect().is_ok();

        DiscordRpc {
            client: Arc::new(Mutex::new(client)),
            connected: AtomicBool::new(connected),
        }
    }

    pub async fn initialize_rpc(client_id: &str) -> Result<Self, String> {
        let rpc = DiscordRpc::new(client_id);
        rpc.connect().await?;
        Ok(rpc)
    }

    pub async fn connect(&self) -> Result<(), String> {
        let mut client = self
            .client
            .lock()
            .expect("Failed to lock client for connection");

        client.connect().map_err(|e| e.to_string()).map(|_| {
            self.connected.store(true, Ordering::SeqCst);
        })
    }

    pub fn disconnect(&self) {
        if self.connected.load(Ordering::SeqCst) {
            let mut client = self
                .client
                .lock()
                .expect("Failed to lock client for disconnection");
            let _ = client.close();
            self.connected.store(false, Ordering::SeqCst);
        }
    }

    pub fn update_activity(&self, details: &str, state: &str) {
        println!("Updating Discord RPC activity: {} - {}", details, state);
        if !self.connected.load(Ordering::SeqCst) {
            println!("Discord RPC is not connected.");
            return;
        }

        let activity = Activity::new().details(details).state(state).assets(
            Assets::new()
                .large_image("logo")
                .large_text("Vleer")
                .small_image("search")
                .small_text("search"),
        );

        let mut client = self
            .client
            .lock()
            .expect("Failed to lock client for activity update");
        let _ = client.set_activity(activity);
    }
}

#[command]
pub async fn update_activity_rpc(app: tauri::AppHandle, details: String, state: String) {
    let rpc = app.state::<DiscordRpc>();
    rpc.update_activity(&details, &state);
}

#[command]
pub async fn disconnect_rpc(app: tauri::AppHandle) {
    let rpc = app.state::<DiscordRpc>();
    rpc.disconnect();
}
