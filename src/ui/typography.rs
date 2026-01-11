//! Typography and text rendering helpers.

use macroquad::prelude::*;

/// Font size constants
pub const FONT_TINY: f32 = 12.0;
pub const FONT_SMALL: f32 = 14.0;
pub const FONT_NORMAL: f32 = 16.0;
pub const FONT_MEDIUM: f32 = 20.0;
pub const FONT_LARGE: f32 = 28.0;
pub const FONT_TITLE: f32 = 40.0;
pub const FONT_HERO: f32 = 60.0;

/// Draw text with drop shadow
pub fn draw_text_shadow(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let shadow = Color::new(0.0, 0.0, 0.0, 0.6);
    draw_text(text, x + 2.0, y + 2.0, font_size, shadow);
    draw_text(text, x, y, font_size, color);
}

/// Draw bold text (simulated with double render)
pub fn draw_text_bold(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    draw_text(text, x, y, font_size, color);
    draw_text(text, x + 1.0, y, font_size, color);
}

/// Measure text dimensions
pub fn measure_text_size(text: &str, font_size: f32) -> (f32, f32) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    (dims.width, dims.height)
}

/// Draw centered text
pub fn draw_text_centered(text: &str, center_x: f32, y: f32, font_size: f32, color: Color) {
    let (width, _) = measure_text_size(text, font_size);
    draw_text(text, center_x - width / 2.0, y, font_size, color);
}

/// Draw right-aligned text
pub fn draw_text_right(text: &str, right_x: f32, y: f32, font_size: f32, color: Color) {
    let (width, _) = measure_text_size(text, font_size);
    draw_text(text, right_x - width, y, font_size, color);
}
