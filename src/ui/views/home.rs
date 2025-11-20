use gpui::*;
use gpui_component::*;

use crate::ui::{components::title::Title, variables::Variables};

pub struct HomeView {
    pub hovered: bool,
}

impl HomeView {
    pub fn new() -> Self {
        Self { hovered: false }
    }
}

impl Render for HomeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let variables = cx.global::<Variables>();

        let border_color = if self.hovered {
            variables.accent
        } else {
            variables.border
        };

        div()
            .relative()
            .size_full()
            .child(
                v_flex()
                    .border(px(1.0))
                    .border_color(border_color)
                    .h_full()
                    .paddings(px(variables.large_padding)),
            )
            .child(Title::new("Home", self.hovered))
    }
}
