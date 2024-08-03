#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;
mod db;

use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_plugin_prevent_default::Flags;

fn main() {
    let _ = utils::discord_rpc::connect_rpc();

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
    INSERT INTO settings (key, value) VALUES ('apiURL', 'https://pipedapi.wireway.ch');
    INSERT INTO settings (key, value) VALUES ('volume', '50');
    "#;

    let migration_v1_data = format!(
        "{}\n{}",
        db::migration::generate_songs_insert_sql(),
        db::migration::generate_playlists_insert_sql()
    );

    let migration_v2_data = db::migration::generate_settings_insert_sql();

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
            description: "insert_default_apiURL",
            sql: migration_v3,
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_prevent_default::Builder::new()
  .with_flags(Flags::all().difference(Flags::CONTEXT_MENU))
  .build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:data.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            utils::discord_rpc::update_activity,
            utils::discord_rpc::clear_activity,
            utils::commands::download,
            utils::commands::ping_urls,
            utils::commands::get_music_path
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let _ = utils::updater::check_for_updates(app.handle().clone()).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
