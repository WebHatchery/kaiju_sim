//! Roster view screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::components::{draw_kaiju_card, CardAction, CardState};
use crate::ui::shell::*;
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_roster_view(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Roster);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    draw_panel(c, "ROSTER ARCHIVE");
    draw_text(
        &format!(
            "{} registered kaiju | {} living | highest generation {}",
            state.roster.len(),
            state.living_count(),
            state.player.stats.highest_generation
        ),
        c.x + 14.0,
        c.y + 52.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let cols = ((c.w - 28.0 + PANEL_GAP) / (CARD_WIDTH + PANEL_GAP))
        .floor()
        .max(1.0) as usize;
    let start_x = c.x + 14.0;
    let start_y = c.y + 74.0;

    for (i, kaiju) in state.roster.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        let x = start_x + col as f32 * (CARD_WIDTH + PANEL_GAP);
        let y = start_y + row as f32 * (CARD_HEIGHT + PANEL_GAP);
        if y > c.y + c.h {
            continue;
        }

        let card_state = if state.selected_kaiju == Some(kaiju.id) {
            CardState::Selected
        } else {
            CardState::Normal
        };

        if let Some(action) = draw_kaiju_card(x, y, kaiju, card_state, assets) {
            return Some(match action {
                CardAction::Select => UiAction::SelectKaiju(kaiju.id),
                CardAction::ViewDetails => UiAction::ViewKaijuDetails(kaiju.id),
            });
        }
    }

    if state.roster.is_empty() {
        draw_text_centered(
            "No kaiju in your roster.",
            c.x + c.w / 2.0,
            c.y + c.h / 2.0,
            FONT_MEDIUM,
            dark::TEXT_MUTED,
        );
    }

    None
}
