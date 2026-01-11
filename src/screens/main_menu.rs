//! Main menu screen.

use macroquad::prelude::*;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use crate::ui::actions::UiAction;
use crate::ui::spacing::*;
use crate::state::persistence::save_exists;

/// Draw main menu and return action if button pressed
pub fn draw_main_menu() -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    
    // Background
    clear_background(dark::BACKGROUND);
    
    // Title with shadow
    let title = "KAIJU SIMULATOR";
    draw_text_shadow(title, sw / 2.0 - 280.0, sh * 0.25, FONT_HERO, dark::ACCENT);
    
    // Tagline
    let tagline = "Breed. Battle. Build Legacy.";
    draw_text_centered(tagline, sw / 2.0, sh * 0.32, FONT_MEDIUM, dark::TEXT_SECONDARY);
    
    // Menu buttons
    let btn_width = 250.0;
    let btn_height = 50.0;
    let btn_x = sw / 2.0 - btn_width / 2.0;
    let mut btn_y = sh * 0.45;
    let btn_gap = 20.0;
    
    let mut result = None;
    
    // Continue button (only if save exists)
    if save_exists() {
        if draw_menu_button(btn_x, btn_y, btn_width, btn_height, "Continue", true) {
            result = Some(UiAction::ContinueGame);
        }
        btn_y += btn_height + btn_gap;
    }
    
    // New Game button
    if draw_menu_button(btn_x, btn_y, btn_width, btn_height, "New Game", true) {
        result = Some(UiAction::NewGame);
    }
    btn_y += btn_height + btn_gap;
    
    // Settings button
    if draw_menu_button(btn_x, btn_y, btn_width, btn_height, "Settings", true) {
        result = Some(UiAction::GoToSettings);
    }
    btn_y += btn_height + btn_gap;
    
    // Exit button
    if draw_menu_button(btn_x, btn_y, btn_width, btn_height, "Exit", true) {
        result = Some(UiAction::ExitGame);
    }
    
    // Version footer
    draw_text("v0.1.0 - Phase 6", 10.0, sh - 10.0, FONT_TINY, dark::TEXT_MUTED);
    
    result
}

/// Draw a menu button, returns true if clicked
fn draw_menu_button(x: f32, y: f32, w: f32, h: f32, text: &str, enabled: bool) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h && enabled;
    
    let bg = if !enabled {
        dark::PANEL
    } else if hovered {
        dark::BUTTON_HOVER
    } else {
        dark::BUTTON_BG
    };
    
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 2.0, if hovered { dark::ACCENT } else { dark::BORDER });
    
    let text_color = if enabled { dark::TEXT_PRIMARY } else { dark::TEXT_MUTED };
    draw_text_centered(text, x + w / 2.0, y + h / 2.0 + 7.0, FONT_MEDIUM, text_color);
    
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
