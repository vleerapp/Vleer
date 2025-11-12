use anyhow::Ok;
use gpui::*;
use gpui_component::*;
use std::fs;
use tracing::debug;

use crate::{
    data::db::{Database, create_pool},
    ui::{
        assets::VleerAssetSource,
        layout::{Library, Navbar, Player},
        utils::rgba_hex,
        views::{AppView, HomeView},
    },
};

struct MainWindow {
    current_view: AppView,
    library: Entity<Library>,
    navbar: Entity<Navbar>,
    player: Entity<Player>,
    home_view: Entity<HomeView>,
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.current_view {
            AppView::Home => self.home_view.clone(),
        };

        v_flex()
            .gap(px(16.0))
            .paddings(16.0)
            .size_full()
            .bg(rgba_hex(0x121212))
            .text_color(rgba_hex(0xE6E6E6))
            .font_family("Feature Mono")
            .child(
                h_flex()
                    .flex_1()
                    .gap(px(16.0))
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
                            .gap(px(16.0))
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
                                            this.home_view.update(cx, |home, cx| {
                                                home.hovered = true;
                                                cx.notify();
                                            });
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
                            this.player.update(cx, |player, cx| {
                                player.hovered = true;
                                cx.notify();
                            });
                        },
                    ))
                    .child(self.player.clone()),
            )
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
    let dirs = directories::ProjectDirs::from("app", "vleerapp", "vleer")
        .expect("couldn't generate project dirs");
    let data_dir = dirs.data_dir().to_path_buf();
    fs::create_dir_all(&data_dir).inspect_err(|error| {
        tracing::error!(
            ?error,
            "couldn't create data directory '{}'",
            data_dir.display(),
        )
    })?;

    let pool = create_pool(data_dir.join("library.db")).await?;
    let database = Database::init(pool.clone());

    Application::new()
        .with_assets(VleerAssetSource::new())
        .run(move |cx| {
            cx.set_global(database);
            gpui_component::init(cx);
            find_fonts(cx).expect("unable to load fonts");

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

                    let view = cx.new(|_| MainWindow {
                        current_view: AppView::default(),
                        library,
                        navbar,
                        player,
                        home_view,
                    });
                    cx.new(|cx| Root::new(view.into(), window, cx))
                },
            )
            .unwrap();
        });

    Ok(())
}
