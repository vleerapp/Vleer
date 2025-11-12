pub fn rgba_hex(hex: u32) -> gpui::Rgba {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;

    gpui::Rgba { r, g, b, a: 1.0 }
}
