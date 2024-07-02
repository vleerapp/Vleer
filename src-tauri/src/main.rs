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
use serde::Serialize;

#[derive(Clone, Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn main() {
    let _ = discord_rpc::connect_rpc();

    let migration_v1 = r#"
    CREATE TABLE IF NOT EXISTS songs (
        id TEXT PRIMARY KEY,
        title TEXT,
        artist TEXT,
        length INTEGER,
        cover TEXT,
        date_added TEXT,
        last_played TEXT
    );
    CREATE TABLE IF NOT EXISTS playlists (
        id TEXT PRIMARY KEY,
        name TEXT,
        date TEXT,
        cover TEXT,
        songs TEXT
    );
    "#;

    let migration_v2 = r#"
    CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT
    );
    "#;

    let migration_v3 = r#"
    INSERT INTO settings (key, value) VALUES ('api_url', 'https://pipedapi.wireway.ch');
    "#;

    let migration_v1_data = format!(
        "{}\n{}",
        migration::generate_songs_insert_sql(),
        migration::generate_playlists_insert_sql()
    );

    let migration_v2_data = migration::generate_settings_insert_sql();

    let migrations = vec![
        Migration {
            version: 1,
            description: "create_songs_and_playlists_tables",
            sql: Box::leak(format!("{}\n{}", migration_v1, migration_v1_data).into_boxed_str()),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_settings_table",
            sql: Box::leak(format!("{}\n{}", migration_v2, migration_v2_data).into_boxed_str()),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "insert_default_api_url",
            sql: migration_v3,
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
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .invoke_handler(tauri::generate_handler![
            discord_rpc::update_activity,
            discord_rpc::clear_activity,
            commands::download,
            commands::ping_urls,
            commands::get_music_path
        ])
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                let _ = window.restore_state(StateFlags::all());
                window.show().unwrap();
            }

            tauri::async_runtime::block_on(async {
                let _ = commands::check_for_updates(app.handle().clone()).await;
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
