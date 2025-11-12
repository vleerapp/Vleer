use gpui::*;
use gpui_component::*;

use crate::ui::utils::rgba_hex;

pub struct Navbar {
    pub hovered: bool,
}

impl Navbar {
    pub fn new() -> Self {
        Self { hovered: false }
    }
}

impl Render for Navbar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let color = if self.hovered {
            rgba_hex(0xA058FF)
        } else {
            rgba_hex(0x535353)
        };

        div()
            .relative()
            .size_full()
            .child(
                h_flex()
                    .border(px(1.0))
                    .border_color(color)
                    .h_full()
                    .pt(px(10.0))
                    .px(px(10.0)),
            )
            .child(
                div()
                    .absolute()
                    .top(px(-12.0))
                    .left(px(6.0))
                    .px(px(2.0))
                    .bg(rgba_hex(0x121212))
                    .text_color(color)
                    .child("NavBar"),
            )
    }
}
