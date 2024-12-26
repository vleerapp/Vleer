use anyhow::{Ok, Result};
use serde::Serialize;
use tauri::State;
use sqlx::SqlitePool;
use crate::db::types::{Settings, Song, EQSettings};

pub struct SettingsDatabase {
    pub pool: SqlitePool,
}

impl SettingsDatabase {
    pub async fn get_default_settings() -> Settings {
        Settings::default()
    }

    pub async fn get_setting(&self, key: &str) -> Result<String> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT value FROM settings WHERE key = ?"
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some((value,)) => Ok(value),
            None => {
                let default_settings = Self::get_default_settings().await;
                self.initialize_default_settings(&default_settings).await?;
                Box::pin(self.get_setting(key)).await
            }
        }
    }

    pub async fn update_setting<T: Serialize>(&self, key: &str, value: T) -> Result<()> {
        let value_str = match serde_json::to_value(&value)? {
            serde_json::Value::String(s) => s,
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Number(n) => n.to_string(),
            v => v.to_string().trim_matches('"').to_string(),
        };

        sqlx::query(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)"
        )
        .bind(key)
        .bind(value_str)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn initialize_default_settings(&self, settings: &Settings) -> Result<()> {
        let settings_map = [
            ("api_url", settings.api_url.clone()),
            ("current_song", serde_json::to_string(&settings.current_song)?),
            ("eq", serde_json::to_string(&settings.eq)?),
            ("lossless", settings.lossless.to_string()),
            ("loop", settings.r#loop.to_string()),
            ("muted", settings.muted.to_string()),
            ("queue", serde_json::to_string(&settings.queue)?),
            ("shuffle", settings.shuffle.to_string()),
            ("streaming", settings.streaming.to_string()),
            ("volume", settings.volume.to_string()),
        ];

        for (key, value) in settings_map {
            self.update_setting(key, value).await?;
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn get_api_url(settings_db: State<'_, SettingsDatabase>) -> Result<String, String> {
    settings_db
        .get_setting("api_url")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_song(settings_db: State<'_, SettingsDatabase>) -> Result<Option<Song>, String> {
    settings_db
        .get_setting("current_song")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_eq(settings_db: State<'_, SettingsDatabase>) -> Result<EQSettings, String> {
    settings_db
        .get_setting("eq")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_lossless(settings_db: State<'_, SettingsDatabase>) -> Result<bool, String> {
    settings_db
        .get_setting("lossless")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_loop(settings_db: State<'_, SettingsDatabase>) -> Result<bool, String> {
    settings_db
        .get_setting("loop")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_muted(settings_db: State<'_, SettingsDatabase>) -> Result<bool, String> {
    settings_db
        .get_setting("muted")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_queue(settings_db: State<'_, SettingsDatabase>) -> Result<Vec<Song>, String> {
    settings_db
        .get_setting("queue")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_shuffle(settings_db: State<'_, SettingsDatabase>) -> Result<bool, String> {
    settings_db
        .get_setting("shuffle")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_streaming(settings_db: State<'_, SettingsDatabase>) -> Result<bool, String> {
    settings_db
        .get_setting("streaming")
        .await
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
}

#[tauri::command]
pub async fn get_volume(settings_db: State<'_, SettingsDatabase>) -> Result<f64, String> {
    let value = settings_db.get_setting("volume").await
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&value)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_api_url(settings_db: State<'_, SettingsDatabase>, api_url: String) -> Result<(), String> {
    settings_db
        .update_setting("api_url", api_url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_current_song(
    settings_db: State<'_, SettingsDatabase>,
    current_song: Option<Song>,
) -> Result<(), String> {
    settings_db
        .update_setting("current_song", current_song)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_eq(settings_db: State<'_, SettingsDatabase>, eq: EQSettings) -> Result<(), String> {
    settings_db
        .update_setting("eq", eq)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_lossless(settings_db: State<'_, SettingsDatabase>, lossless: bool) -> Result<(), String> {
    settings_db
        .update_setting("lossless", lossless)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_loop(settings_db: State<'_, SettingsDatabase>, r#loop: bool) -> Result<(), String> {
    settings_db
        .update_setting("loop", r#loop)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_muted(settings_db: State<'_, SettingsDatabase>, muted: bool) -> Result<(), String> {
    settings_db
        .update_setting("muted", muted)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_queue(settings_db: State<'_, SettingsDatabase>, queue: Vec<Song>) -> Result<(), String> {
    settings_db
        .update_setting("queue", queue)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_shuffle(settings_db: State<'_, SettingsDatabase>, shuffle: bool) -> Result<(), String> {
    settings_db
        .update_setting("shuffle", shuffle)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_streaming(settings_db: State<'_, SettingsDatabase>, streaming: bool) -> Result<(), String> {
    settings_db
        .update_setting("streaming", streaming)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_volume(settings_db: State<'_, SettingsDatabase>, volume: f64) -> Result<(), String> {
    let clamped_volume = volume.max(0.0).min(1.0);
    settings_db
        .update_setting("volume", clamped_volume)
        .await
        .map_err(|e| e.to_string())
}

pub async fn initialize_settings(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let default_settings = Settings::default();

    sqlx::query(
        "INSERT INTO settings (api_url, current_song, eq, lossless, loop, muted, queue, shuffle, streaming, volume) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&default_settings.api_url)
    .bind(serde_json::to_string(&default_settings.current_song).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?)
    .bind(serde_json::to_string(&default_settings.eq).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?)
    .bind(default_settings.lossless)
    .bind(default_settings.r#loop)
    .bind(default_settings.muted)
    .bind(serde_json::to_string(&default_settings.queue).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?)
    .bind(default_settings.shuffle)
    .bind(default_settings.streaming)
    .bind(default_settings.volume)
    .execute(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Result::<(), Box<dyn std::error::Error>>::Ok(())
}
