#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod discord_rpc;
mod migration;

use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

fn main() {
    let _ = discord_rpc::connect_rpc();

    let migration_v1 = format!(
        r#"
    CREATE TABLE songs (
        id TEXT PRIMARY KEY,
        title TEXT,
        artist TEXT,
        length INTEGER,
        cover TEXT,
        date_added TEXT,
        last_played TEXT
    );
    CREATE TABLE playlists (
        id TEXT PRIMARY KEY,
        name TEXT,
        date TEXT,
        cover TEXT,
        songs TEXT
    );
    {}
    {}
    "#,
        migration::generate_songs_insert_sql(),
        migration::generate_playlists_insert_sql(),
    );

    let migration_v2 = format!(
        r#"
    CREATE TABLE settings (
        key TEXT PRIMARY KEY,
        value TEXT
    );
    {}
    "#,
        migration::generate_settings_insert_sql(),
    );

    let sql_commands_v1 = Box::leak(Box::new(migration_v1));
    let sql_commands_v2 = Box::leak(Box::new(migration_v2));

    let migrations = vec![
        Migration {
            version: 1,
            description: "create_songs_and_playlists_tables",
            sql: sql_commands_v1,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_settings_table",
            sql: sql_commands_v2,
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:data.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            commands::download,
            commands::ping_urls,
            commands::get_music_path
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let _ = commands::check_for_updates(app.handle().clone()).await;

                if let Some(window) = app.get_window("main") {
                    let _ = window.restore_state(StateFlags::all());
                }
            });
            Ok(())
        })
        .on_window_event(|app, event| match event {
            tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed => {
                let _ = AppHandleExt::save_window_state(app.app_handle(), StateFlags::all());
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
