//! Roster view screen - kaiju collection display.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::components::{draw_kaiju_card, CardAction, CardState};
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

/// Draw roster view and return action if interaction
pub fn draw_roster_view(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();

    clear_background(dark::BACKGROUND);

    // Header
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("YOUR ROSTER", 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);

    // Back button
    if draw_back_button(sw - 100.0, 15.0) {
        return Some(UiAction::GoToLaboratory);
    }

    // Roster count
    draw_text(
        &format!(
            "{} Kaiju ({} alive)",
            state.roster.len(),
            state.living_count()
        ),
        20.0,
        80.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    // Kaiju cards grid
    let (cols, start_x) = grid_cols(sw, CARD_WIDTH, SPACING_NORMAL);
    let start_y = 100.0;

    for (i, kaiju) in state.roster.iter().enumerate() {
        let (x, y) = grid_position(
            i,
            cols,
            start_x,
            start_y,
            CARD_WIDTH,
            CARD_HEIGHT,
            SPACING_NORMAL,
        );

        // Don't draw if off screen
        if y > sh + CARD_HEIGHT {
            continue;
        }

        let card_state = if state.selected_kaiju == Some(kaiju.id) {
            CardState::Selected
        } else {
            CardState::Normal
        };

        if let Some(action) = draw_kaiju_card(x, y, kaiju, card_state, assets) {
            match action {
                CardAction::Select => return Some(UiAction::SelectKaiju(kaiju.id)),
                CardAction::ViewDetails => return Some(UiAction::ViewKaijuDetails(kaiju.id)),
            }
        }
    }

    // Empty state
    if state.roster.is_empty() {
        draw_text_centered(
            "No kaiju in your roster!",
            sw / 2.0,
            sh / 2.0,
            FONT_MEDIUM,
            dark::TEXT_SECONDARY,
        );
    }

    None
}

/// Draw back button
fn draw_back_button(x: f32, y: f32) -> bool {
    let w = 80.0;
    let h = 30.0;
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;

    let bg = if hovered {
        dark::BUTTON_HOVER
    } else {
        dark::BUTTON_BG
    };
    draw_rectangle(x, y, w, h, bg);
    draw_text_centered(
        "< Back",
        x + w / 2.0,
        y + 20.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
