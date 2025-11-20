use gpui::*;

use crate::ui::variables::Variables;

#[derive(IntoElement)]
pub struct Icon {
    svg: Svg,
    icon: SharedString,
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        self.svg.style()
    }
}

impl RenderOnce for Icon {
    fn render(mut self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let variables = cx.global::<Variables>();

        if self.svg.text_style().as_ref().and_then(|s| s.color).is_none() {
            self.svg = self.svg.text_color(variables.text_secondary);
        }

        self.svg
            .path(self.icon)
            .w(px(16.0))
            .h(px(16.0))
            .flex_shrink_0()
    }
}

pub fn icon(icon: impl Into<SharedString>) -> Icon {
    Icon {
        svg: svg(),
        icon: icon.into(),
    }
}
