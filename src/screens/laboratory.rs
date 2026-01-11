//! Laboratory hub screen - main gameplay hub.

use macroquad::prelude::*;
use crate::state::GameState;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use crate::ui::actions::UiAction;
use crate::ui::spacing::*;

/// Draw laboratory hub and return action if button pressed
pub fn draw_laboratory(state: &GameState) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    
    clear_background(dark::BACKGROUND);
    
    // Header
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("LABORATORY", 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);
    
    // Player info
    draw_text_right(
        &format!("Gold: {} | Roster: {}", state.player.gold, state.living_count()),
        sw - 20.0,
        40.0,
        FONT_NORMAL,
        dark::TEXT_SECONDARY,
    );
    
    // Main buttons grid
    let btn_size = 180.0;
    let gap = 30.0;
    let grid_w = btn_size * 3.0 + gap * 2.0;
    let start_x = (sw - grid_w) / 2.0;
    let start_y = 120.0;
    
    let mut result = None;
    
    // Row 1
    if draw_hub_button(start_x, start_y, btn_size, "ROSTER", "View your kaiju", dark::ACCENT) {
        result = Some(UiAction::GoToRoster);
    }
    
    if draw_hub_button(start_x + btn_size + gap, start_y, btn_size, "BREEDING", "Create offspring", dark::POSITIVE) {
        result = Some(UiAction::GoToBreeding);
    }
    
    if draw_hub_button(start_x + (btn_size + gap) * 2.0, start_y, btn_size, "TOURNAMENT", "Enter battles", dark::WARNING) {
        result = Some(UiAction::GoToTournament);
    }
    
    // Row 2
    let row2_y = start_y + btn_size + gap;
    
    if draw_hub_button(start_x, row2_y, btn_size, "LEADERBOARD", "Top kaiju", dark::RANKED) {
        result = Some(UiAction::GoToLeaderboard);
    }
    
    if draw_hub_button(start_x + btn_size + gap, row2_y, btn_size, "SETTINGS", "Options", dark::TEXT_SECONDARY) {
        result = Some(UiAction::GoToSettings);
    }
    
    if draw_hub_button(start_x + (btn_size + gap) * 2.0, row2_y, btn_size, "MENU", "Main menu", dark::NEGATIVE) {
        result = Some(UiAction::GoToMenu);
    }
    
    // Notifications panel at bottom
    let notif_y = sh - 100.0;
    draw_rectangle(20.0, notif_y, sw - 40.0, 80.0, dark::SURFACE);
    draw_text("Recent Activity", 30.0, notif_y + 20.0, FONT_SMALL, dark::TEXT_SECONDARY);
    
    for (i, notif) in state.notifications.iter().rev().take(3).enumerate() {
        draw_text(&notif.message, 30.0, notif_y + 40.0 + i as f32 * 18.0, FONT_SMALL, dark::TEXT_PRIMARY);
    }
    
    result
}

/// Draw a hub navigation button
fn draw_hub_button(x: f32, y: f32, size: f32, title: &str, subtitle: &str, accent: Color) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + size && mouse.1 >= y && mouse.1 <= y + size;
    
    let bg = if hovered { dark::PANEL } else { dark::SURFACE };
    
    draw_rectangle(x, y, size, size, bg);
    draw_rectangle(x, y, size, 4.0, accent); // Accent bar at top
    draw_rectangle_lines(x, y, size, size, 2.0, if hovered { accent } else { dark::BORDER });
    
    draw_text_centered(title, x + size / 2.0, y + size / 2.0, FONT_MEDIUM, dark::TEXT_PRIMARY);
    draw_text_centered(subtitle, x + size / 2.0, y + size / 2.0 + 25.0, FONT_SMALL, dark::TEXT_SECONDARY);
    
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
