use macroquad::prelude::*;
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::colors::dark;
use crate::ui::typography::*;

pub fn draw_leaderboard(_state: &GameState) -> Option<UiAction> {
    clear_background(dark::BACKGROUND);
    draw_text_centered("Leaderboard Coming Soon", screen_width()/2.0, screen_height()/2.0, FONT_LARGE, WHITE);
    
    if is_key_pressed(KeyCode::Escape) {
        return Some(UiAction::Back);
    }
    None
}
