use anyhow::{anyhow, Result};
use reqwest::Client;
use rusty_ytdl::Video;
use std::path::PathBuf;
use tauri::async_runtime;
use tauri::Error as TauriError;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::MessageDialogKind;
use tauri_plugin_updater::UpdaterExt;
use tokio::time::Instant;

#[tauri::command]
pub async fn download(url: String, name: String) -> Result<(), TauriError> {
    let video = Video::new(url.clone()).map_err(|e| anyhow!(e.to_string()))?;

    let mut base_path = PathBuf::new();
    match std::env::consts::OS {
        "macos" | "linux" => {
            let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
            base_path.push(format!("/users/{}/Music/Vleer", username));
        }
        "windows" => {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            base_path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
        }
        _ => {}
    }

    let mut path = base_path.clone();
    path.push("Songs");
    path.push(&name);

    video
        .download(&path)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    println!("Downloaded: {}", path.display());
    Ok(())
}

#[tauri::command]
pub async fn ping_urls(urls: Vec<String>) -> Result<Vec<(String, u128)>, TauriError> {
    let client = Client::new();
    let results = ping_urls_helper(&client, &urls).await?;
    Ok(results)
}

async fn ping_urls_helper(
    client: &Client,
    urls: &[String],
) -> Result<Vec<(String, u128)>, TauriError> {
    let mut handles = vec![];

    for url in urls.iter() {
        let url_clone = url.clone();
        let client_clone = client.clone();

        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let result = client_clone.head(&url_clone).send().await;
            let latency = start.elapsed().as_millis();
            match result {
                Ok(_) => (url_clone, latency),    // Use the cloned URL here
                Err(_) => (url_clone, u128::MAX), // Use the cloned URL here
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(results)
}

pub async fn check_for_updates(app: AppHandle) {
    println!("Checking for updates...");

    let updater = app.updater().unwrap();
    let response = updater.check().await;

    match response {
        Ok(Some(update)) => {
            let cur_ver = &update.current_version;
            let new_ver = &update.version;
            let mut msg = String::new();
            msg.extend([
                &format!("XXX {new_ver} is now available -- you have {cur_ver}\n\n"),
                "Would you like to install it now?\n\n",
                "Release notes can be found at XXX",
            ]);

            app.dialog()
                .message(msg)
                .title("A new version of XXX is available!")
                .ok_button_label("Yes")
                .cancel_button_label("No")
                .show(move |response| {
                    if !response {
                        return;
                    }
                    async_runtime::spawn(async move {
                        if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                            println!("Error installing new update: {:?}", e);
                            app.dialog().message(
                                "Failed to install new update. The new update can be downloaded from XXX"
                            ).kind(MessageDialogKind::Error).show(|_| {});
                        }
                    });
                });
        }
        Ok(None) => println!("No updates available."),
        Err(e) => {
            println!("Failed to check for updates: {:?}", e);
        }
    }
}

pub fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
