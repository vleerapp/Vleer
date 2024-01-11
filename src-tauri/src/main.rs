// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_rpc_client::models::rich_presence::{Activity, ActivityAssets};
use discord_rpc_client::Client;
use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        let mut drpc = Client::new(1194990403963858984);
        drpc.start();

        loop {
            let activity = Activity::new()
                .state("Your state message")
                .details("Your details message")
                .assets(|_| {
                    ActivityAssets::new()
                        .large_image("logo")
                        .large_text("Text shown when hovering over large image")
                        .small_image("small_logo")
                        .small_text("Text shown when hovering over small image")
                });

            drpc.set_activity(|_| activity)
                .expect("Failed to set activity");
            thread::sleep(Duration::from_secs(10));
        }
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
