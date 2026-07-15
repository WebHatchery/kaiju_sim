//! Typography and text rendering helpers.

use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

/// Truncate text with a `...` suffix so it fits within `max_width`.
pub use macroquad_toolkit::ui::truncate_text_to_width as ellipsize;

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
    macroquad_toolkit::ui::draw_text_shadow(
        text,
        x,
        y,
        macroquad_toolkit::ui::TextStyle::new(font_size, color),
        vec2(2.0, 2.0),
        shadow,
    );
}

/// Draw bold text (simulated with double render)
pub fn draw_text_bold(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    draw_ui_text(text, x, y, font_size, color);
    draw_ui_text(text, x + 1.0, y, font_size, color);
}

/// Measure text dimensions
pub fn measure_text_size(text: &str, font_size: f32) -> (f32, f32) {
    let dims = macroquad_toolkit::ui::measure_text_size(
        text,
        macroquad_toolkit::ui::TextStyle::new(font_size, WHITE),
    );
    (dims.width, dims.height)
}

/// Draw centered text
pub fn draw_text_centered(text: &str, center_x: f32, y: f32, font_size: f32, color: Color) {
    macroquad_toolkit::ui::draw_text_centered(
        text,
        center_x,
        y,
        macroquad_toolkit::ui::TextStyle::new(font_size, color),
    );
}

/// Draw right-aligned text
pub fn draw_text_right(text: &str, right_x: f32, y: f32, font_size: f32, color: Color) {
    macroquad_toolkit::ui::draw_text_right(
        text,
        right_x,
        y,
        macroquad_toolkit::ui::TextStyle::new(font_size, color),
    );
}

pub fn draw_text_wrapped(
    text: &str,
    x: f32,
    mut y: f32,
    max_width: f32,
    line_height: f32,
    font_size: f32,
    color: Color,
    max_lines: usize,
) -> f32 {
    for line in macroquad_toolkit::ui::wrap_text(text, max_width, font_size)
        .iter()
        .filter(|line| !line.is_empty())
        .take(max_lines)
    {
        draw_ui_text(line, x, y, font_size, color);
        y += line_height;
    }
    y
}
