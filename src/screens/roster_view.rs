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
use macroquad_toolkit::ui::draw_ui_text;

pub fn draw_roster_view(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Roster);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    draw_panel_with_accent(c, "KAIJU ARCHIVE", dark::ACCENT);
    draw_roster_header(state, c);

    let mut roster = state.roster.iter().collect::<Vec<_>>();
    roster.sort_by_key(|kaiju| {
        std::cmp::Reverse((
            state.selected_kaiju == Some(kaiju.id),
            kaiju.alive,
            kaiju.generation,
            kaiju.battle_rating(),
        ))
    });

    let cols = ((c.w - 32.0 + PANEL_GAP) / (CARD_WIDTH + PANEL_GAP))
        .floor()
        .max(1.0) as usize;
    let start_x = c.x + 16.0;
    let start_y = c.y + 116.0;

    for (i, kaiju) in roster.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        let x = start_x + col as f32 * (CARD_WIDTH + PANEL_GAP);
        let y = start_y + row as f32 * (CARD_HEIGHT + PANEL_GAP);
        if y + 42.0 > c.y + c.h {
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
        draw_empty_state(
            c,
            "No kaiju in archive",
            "Start a new game to register a starter.",
        );
    }

    None
}

fn draw_roster_header(state: &GameState, rect: Rect) {
    let summary = format!(
        "{} registered | {} living | highest generation {}",
        state.roster.len(),
        state.living_count(),
        state.player.stats.highest_generation
    );
    draw_ui_text(
        &summary,
        rect.x + 16.0,
        rect.y + 56.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let y = rect.y + 72.0;
    draw_status_pill(
        Rect::new(rect.x + 16.0, y, 118.0, 28.0),
        "ALL SPECIMENS",
        dark::ACCENT,
    );
    draw_status_pill(
        Rect::new(rect.x + 144.0, y, 104.0, 28.0),
        "LIVING FIRST",
        dark::POSITIVE,
    );
    draw_status_pill(
        Rect::new(rect.x + 258.0, y, 104.0, 28.0),
        "RATING SORT",
        dark::WARNING,
    );
    draw_status_pill(
        Rect::new(rect.x + 372.0, y, 98.0, 28.0),
        "EXPANDED",
        dark::TEXT_SECONDARY,
    );

    if let Some(selected) = state.selected_kaiju.and_then(|id| state.get_kaiju(id)) {
        draw_text_right(
            &format!(
                "Selected: {} | GEN {} | rating {}",
                selected.name,
                selected.generation,
                selected.battle_rating()
            ),
            rect.x + rect.w - 16.0,
            rect.y + 91.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    } else {
        draw_text_right(
            "Select a kaiju to assign training, arena, or breeding work.",
            rect.x + rect.w - 16.0,
            rect.y + 91.0,
            FONT_TINY,
            dark::TEXT_MUTED,
        );
    }
}
