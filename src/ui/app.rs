use anyhow::Ok;
use gpui::*;
use gpui_component::*;
use std::fs;
use tracing::debug;

use crate::{
    data::{
        db::{Database, create_pool},
        settings::Settings,
    },
    media::{PlaybackContext, Queue, scanner::{MusicScanner, MusicWatcher, expand_scan_paths}},
    ui::{
        assets::VleerAssetSource,
        layout::{library::Library, navbar::Navbar, player::Player},
        variables::Variables,
        views::{AppView, HomeView, SongsView},
    },
};

pub struct ViewState {
    current_view: AppView,
}

impl Global for ViewState {}

impl ViewState {
    pub fn new() -> Self {
        Self {
            current_view: AppView::default(),
        }
    }

    pub fn current(&self) -> AppView {
        self.current_view
    }

    pub fn set(&mut self, view: AppView) {
        self.current_view = view;
    }
}

struct MainWindow {
    library: Entity<Library>,
    navbar: Entity<Navbar>,
    player: Entity<Player>,
    home_view: Entity<HomeView>,
    songs_view: Entity<SongsView>,
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let variables = cx.global::<Variables>();
        let current_view = cx.global::<ViewState>().current();

        let content: AnyElement = match current_view {
            AppView::Home => self.home_view.clone().into_any_element(),
            AppView::Songs => self.songs_view.clone().into_any_element(),
        };

        let mut element = v_flex()
            .gap(px(variables.default_padding))
            .paddings(16.0)
            .size_full()
            .bg(variables.background)
            .child(
                h_flex()
                    .flex_1()
                    .gap(px(variables.default_padding))
                    .child(
                        div()
                            .w(px(300.0))
                            .h_full()
                            .on_mouse_move(cx.listener(
                                |this, _event: &MouseMoveEvent, _window: &mut Window, cx| {
                                    this.library.update(cx, |library, cx| {
                                        library.hovered = true;
                                        cx.notify();
                                    });
                                    this.navbar.update(cx, |navbar, cx| {
                                        navbar.hovered = false;
                                        cx.notify();
                                    });
                                    this.home_view.update(cx, |home, cx| {
                                        home.hovered = false;
                                        cx.notify();
                                    });
                                    this.songs_view.update(cx, |songs, cx| {
                                        songs.hovered = false;
                                        cx.notify();
                                    });
                                    this.player.update(cx, |player, cx| {
                                        player.hovered = false;
                                        cx.notify();
                                    });
                                },
                            ))
                            .child(self.library.clone()),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .h_full()
                            .gap(px(variables.default_padding))
                            .child(
                                div()
                                    .h(px(48.0))
                                    .w_full()
                                    .on_mouse_move(cx.listener(
                                        |this,
                                         _event: &MouseMoveEvent,
                                         _window: &mut Window,
                                         cx| {
                                            this.library.update(cx, |library, cx| {
                                                library.hovered = false;
                                                cx.notify();
                                            });
                                            this.navbar.update(cx, |navbar, cx| {
                                                navbar.hovered = true;
                                                cx.notify();
                                            });
                                            this.home_view.update(cx, |home, cx| {
                                                home.hovered = false;
                                                cx.notify();
                                            });
                                            this.songs_view.update(cx, |songs, cx| {
                                                songs.hovered = false;
                                                cx.notify();
                                            });
                                            this.player.update(cx, |player, cx| {
                                                player.hovered = false;
                                                cx.notify();
                                            });
                                        },
                                    ))
                                    .child(self.navbar.clone()),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .w_full()
                                    .on_mouse_move(cx.listener(
                                        |this,
                                         _event: &MouseMoveEvent,
                                         _window: &mut Window,
                                         cx| {
                                            this.library.update(cx, |library, cx| {
                                                library.hovered = false;
                                                cx.notify();
                                            });
                                            this.navbar.update(cx, |navbar, cx| {
                                                navbar.hovered = false;
                                                cx.notify();
                                            });
                                            let current_view = cx.global::<ViewState>().current();
                                            match current_view {
                                                AppView::Home => {
                                                    this.home_view.update(cx, |home, cx| {
                                                        home.hovered = true;
                                                        cx.notify();
                                                    });
                                                    this.songs_view.update(cx, |songs, cx| {
                                                        songs.hovered = false;
                                                        cx.notify();
                                                    });
                                                }
                                                AppView::Songs => {
                                                    this.home_view.update(cx, |home, cx| {
                                                        home.hovered = false;
                                                        cx.notify();
                                                    });
                                                    this.songs_view.update(cx, |songs, cx| {
                                                        songs.hovered = true;
                                                        cx.notify();
                                                    });
                                                }
                                            }
                                            this.player.update(cx, |player, cx| {
                                                player.hovered = false;
                                                cx.notify();
                                            });
                                        },
                                    ))
                                    .child(content),
                            ),
                    ),
            )
            .child(
                div()
                    .h(px(100.0))
                    .w_full()
                    .on_mouse_move(cx.listener(
                        |this, _event: &MouseMoveEvent, _window: &mut Window, cx| {
                            this.library.update(cx, |library, cx| {
                                library.hovered = false;
                                cx.notify();
                            });
                            this.navbar.update(cx, |navbar, cx| {
                                navbar.hovered = false;
                                cx.notify();
                            });
                            this.home_view.update(cx, |home, cx| {
                                home.hovered = false;
                                cx.notify();
                            });
                            this.songs_view.update(cx, |songs, cx| {
                                songs.hovered = false;
                                cx.notify();
                            });
                            this.player.update(cx, |player, cx| {
                                player.hovered = true;
                                cx.notify();
                            });
                        },
                    ))
                    .child(self.player.clone()),
            );

        let text_styles = element.text_style();
        *text_styles = Some(TextStyleRefinement {
            color: Some(Hsla::from(variables.text)),
            font_family: Some(SharedString::new("Feature Mono")),
            font_size: Some(AbsoluteLength::Pixels(px(14.0))),
            line_height: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(13.0)))),
            ..Default::default()
        });

        element
    }
}

pub fn find_fonts(cx: &mut App) -> gpui::Result<()> {
    let paths = cx.asset_source().list("!bundled:fonts")?;
    let mut fonts = vec![];
    for path in paths {
        if (path.ends_with(".ttf") || path.ends_with(".otf"))
            && let Some(v) = cx.asset_source().load(&path)?
        {
            fonts.push(v);
        }
    }

    let results = cx.text_system().add_fonts(fonts);
    debug!("loaded fonts: {:?}", cx.text_system().all_font_names());
    results
}

#[tokio::main]
pub async fn run() -> anyhow::Result<()> {
    let data_dir = dirs::data_dir()
        .expect("couldn't get data directory")
        .join("vleer");
    let config_dir = dirs::config_dir()
        .expect("couldn't get config directory")
        .join("vleer");

    fs::create_dir_all(&data_dir).inspect_err(|error| {
        tracing::error!(
            ?error,
            "couldn't create data directory '{}'",
            data_dir.display(),
        )
    })?;

    let pool = create_pool(data_dir.join("library.db")).await?;

    Application::new()
        .with_assets(VleerAssetSource::new())
        .run(move |cx| {
            gpui_component::init(cx);
            Database::init(cx, pool).expect("unable to initizalize database");
            Settings::init(cx, config_dir).expect("unable to initizalize settings");
            PlaybackContext::init(cx).expect("unable to initizalize playback context");
            Queue::init(cx);
            Variables::init(cx);
            cx.set_global(ViewState::new());

            let settings = cx.global::<Settings>();
            let scan_paths = expand_scan_paths(&settings.config().scan.paths);
            let db = cx.global::<Database>().clone();

            let scanner = std::sync::Arc::new(MusicScanner::new(scan_paths));
            let scanner_clone = scanner.clone();

            match MusicWatcher::new(scanner.clone(), std::sync::Arc::new(db.clone())) {
                std::result::Result::Ok((watcher, mut rx)) => {
                    tokio::spawn(async move {
                        let _watcher = watcher; 
                        while let Some(stats) = rx.recv().await {
                            tracing::info!(
                                "Library scan completed - Added: {}, Updated: {}, Removed: {}",
                                stats.added,
                                stats.updated,
                                stats.removed
                            );
                        }
                    });

                    let db_clone = cx.global::<Database>().clone();
                    tokio::spawn(async move {
                        tracing::info!("Starting initial library scan...");
                        match scanner_clone.scan_and_save(&db_clone).await {
                            std::result::Result::Ok(stats) => {
                                tracing::info!(
                                    "Initial scan complete - Added: {}, Updated: {}, Removed: {}",
                                    stats.added,
                                    stats.updated,
                                    stats.removed
                                );
                            }
                            Err(e) => {
                                tracing::error!("Initial scan failed: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to initialize music watcher: {}", e);
                }
            }

            find_fonts(cx)
                .inspect_err(|e| tracing::error!(?e, "Failed to load fonts"))
                .expect("unable to load fonts");

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::new("Vleer")),
                        appears_transparent: true,
                        traffic_light_position: None,
                    }),
                    app_id: Some("app.vleerapp.vleer".to_string()),
                    kind: gpui::WindowKind::Normal,
                    ..Default::default()
                },
                |window, cx| {
                    let library = cx.new(|_| Library::new());
                    let navbar = cx.new(|_| Navbar::new());
                    let player = cx.new(|_| Player::new());
                    let home_view = cx.new(|_| HomeView::new());
                    let songs_view = cx.new(|_| SongsView::new());

                    let view = cx.new(|_| MainWindow {
                        library,
                        navbar,
                        player,
                        home_view,
                        songs_view,
                    });
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();
        });

    Ok(())
}
