use gpui::{div, px, IntoElement, ParentElement, RenderOnce, SharedString, Styled};

use crate::ui::variables::Variables;

#[derive(IntoElement)]
pub struct Title {
    label: SharedString,
    hovered: bool,
}

impl Title {
    pub fn new(label: impl Into<SharedString>, hovered: bool) -> Self {
        Self {
            label: label.into(),
            hovered,
        }
    }
}

impl RenderOnce for Title {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let variables = cx.global::<Variables>();

        let color = if self.hovered {
            variables.accent
        } else {
            variables.border
        };

        div()
            .absolute()
            .top(px(-6.0))
            .left(px(6.0))
            .px(px(2.0))
            .bg(variables.background)
            .text_color(color)
            .child(self.label)
    }
}
