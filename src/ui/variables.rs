use gpui::*;

pub struct Variables {
    pub background: Rgba,
    pub element: Rgba,
    pub element_hover: Rgba,
    pub border: Rgba,
    pub accent: Rgba,
    pub text: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,

    pub default_padding: f32,
    pub small_padding: f32,
    pub large_padding: f32,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            background: rgb(0x121212),
            element: rgb(0x1A1A1A),
            element_hover: rgb(0x242424),
            border: rgb(0x535353),
            accent: rgb(0xA058FF),
            text: rgb(0xE6E6E6),
            text_secondary: rgb(0xABABAB),
            text_muted: rgb(0x303030),

            default_padding: 16.0,
            small_padding: 8.0,
            large_padding: 24.0,
        }
    }
}

impl Global for Variables {}

impl Variables {
    pub fn init(cx: &mut App) {
        cx.set_global(Variables::default());
    }
}
