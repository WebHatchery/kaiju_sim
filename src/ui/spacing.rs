//! Layout spacing system.

/// Spacing constants (8px grid)
pub const SPACING_TINY: f32 = 4.0;
pub const SPACING_SMALL: f32 = 8.0;
pub const SPACING_NORMAL: f32 = 16.0;
pub const SPACING_MEDIUM: f32 = 24.0;
pub const SPACING_LARGE: f32 = 32.0;
pub const SPACING_HUGE: f32 = 48.0;

/// Border radius constants
pub const RADIUS_SMALL: f32 = 4.0;
pub const RADIUS_MEDIUM: f32 = 8.0;
pub const RADIUS_LARGE: f32 = 12.0;

/// Standard component sizes
pub const BUTTON_HEIGHT: f32 = 40.0;
pub const BUTTON_WIDTH: f32 = 120.0;
pub const CARD_WIDTH: f32 = 180.0;
pub const CARD_HEIGHT: f32 = 240.0;
pub const PANEL_HEADER: f32 = 50.0;
pub const STAT_BAR_HEIGHT: f32 = 20.0;

/// Screen margins
pub const SCREEN_MARGIN: f32 = 20.0;
pub const CONTENT_PADDING: f32 = 16.0;

/// Grid layout helper
pub fn grid_cols(screen_width: f32, item_width: f32, gap: f32) -> (usize, f32) {
    let available = screen_width - 2.0 * SCREEN_MARGIN;
    let cols = ((available + gap) / (item_width + gap)).floor() as usize;
    let total_width = cols as f32 * (item_width + gap) - gap;
    let start_x = (screen_width - total_width) / 2.0;
    (cols.max(1), start_x)
}

/// Calculate grid position
pub fn grid_position(index: usize, cols: usize, start_x: f32, start_y: f32, item_width: f32, item_height: f32, gap: f32) -> (f32, f32) {
    let row = index / cols;
    let col = index % cols;
    let x = start_x + col as f32 * (item_width + gap);
    let y = start_y + row as f32 * (item_height + gap);
    (x, y)
}
