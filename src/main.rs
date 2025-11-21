#![cfg_attr(
    all(not(test), not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ui;
mod data;
mod media;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting application");

    crate::ui::app::run()
}
