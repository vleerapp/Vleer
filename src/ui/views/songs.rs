use gpui::{prelude::FluentBuilder, *};
use gpui_component::*;

use crate::{
    data::{
        db::Database,
        scanner::{MusicScanner, expand_scan_paths},
        settings::Settings,
    },
    media::{
        playback::PlaybackContext,
        queue::{Queue, QueueItem},
    },
    ui::{components::title::Title, variables::Variables},
};

pub struct SongsView {
    pub hovered: bool,
    is_scanning: bool,
}

impl SongsView {
    pub fn new() -> Self {
        Self {
            hovered: false,
            is_scanning: false,
        }
    }

    fn start_scan(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_scanning {
            return;
        }

        self.is_scanning = true;
        cx.notify();

        let settings = cx.global::<Settings>().clone();
        let db = cx.global::<Database>().clone();

        cx.spawn_in(
            window,
            |this: WeakEntity<Self>, cx: &mut AsyncWindowContext| {
                let mut cx = cx.clone();
                async move {
                    let scan_paths = expand_scan_paths(&settings.config().scan.paths);
                    let scanner = MusicScanner::new(scan_paths);

                    match scanner.scan_and_save(&db).await {
                        Ok(stats) => tracing::info!(
                            "Scan complete - Scanned: {}, Added: {}, Updated: {}, Removed: {}",
                            stats.scanned,
                            stats.added,
                            stats.updated,
                            stats.removed
                        ),
                        Err(e) => tracing::error!("Error scanning music library: {}", e),
                    }

                    this.update(&mut cx, |this, cx| {
                        this.is_scanning = false;
                        cx.notify();
                    })
                    .ok();
                }
            },
        )
        .detach();
    }

    fn load_all_songs(&self, window: &mut Window, cx: &mut Context<Self>) {
        let db = cx.global::<Database>().clone();

        cx.spawn_in(
            window,
            |_this: WeakEntity<Self>, cx: &mut AsyncWindowContext| {
                let mut cx = cx.clone();
                async move {
                    match db.get_all_songs().await {
                        Ok(songs) => {
                            tracing::info!("Loaded {} songs from database", songs.len());

                            cx.update_global::<Queue, _>(|queue, _window, _cx| {
                                queue.clear();
                                for song in songs {
                                    let item = QueueItem::with_metadata(
                                        song.file_path.into(),
                                        Some(song.id),
                                        Some(song.title),
                                        None,
                                        None,
                                        song.duration,
                                    );
                                    queue.add(item);
                                }
                            })
                            .ok();
                        }
                        Err(e) => tracing::error!("Error loading songs: {}", e),
                    }
                }
            },
        )
        .detach();
    }

    fn play_all(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.load_all_songs(window, cx);

        cx.spawn_in(
            window,
            |_this: WeakEntity<Self>, cx: &mut AsyncWindowContext| {
                let mut cx = cx.clone();
                async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                    cx.update(|_window, cx| {
                        let settings = cx.global::<Settings>().clone();

                        if let Some(queue) = cx.try_global::<Queue>() {
                            if let Some(first_item) = queue.current() {
                                let path = first_item.path.clone();

                                cx.update_global::<PlaybackContext, _>(|playback, _cx| {
                                    if let Err(e) = playback.load_file(&path, &settings) {
                                        tracing::error!("Failed to load file: {}", e);
                                    } else {
                                        playback.play();
                                    }
                                });
                            }
                        }
                    })
                    .ok();
                }
            },
        )
        .detach();
    }
}

impl Render for SongsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let variables = cx.global::<Variables>();

        let border_color = if self.hovered {
            variables.accent
        } else {
            variables.border
        };

        let scan_button_text = if self.is_scanning {
            "Scanning..."
        } else {
            "Scan Library"
        };

        div()
            .relative()
            .size_full()
            .child(
                v_flex()
                    .border(px(1.0))
                    .border_color(border_color)
                    .h_full()
                    .paddings(px(variables.default_padding))
                    .gap(px(16.0))
                    .child(
                        h_flex()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .border(px(1.0))
                                    .border_color(variables.border)
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(variables.element_hover).cursor_pointer())
                                    .when(!self.is_scanning, |s| {
                                        s.on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, window, cx| {
                                                this.start_scan(window, cx);
                                            }),
                                        )
                                    })
                                    .child(scan_button_text),
                            )
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .border(px(1.0))
                                    .border_color(variables.border)
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(variables.element_hover).cursor_pointer())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, window, cx| {
                                            this.play_all(window, cx);
                                        }),
                                    )
                                    .child("Play All"),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .child("Song list will appear here after scanning"),
                    ),
            )
            .child(Title::new("Songs", self.hovered))
    }
}
