//! Future multiplayer tournament lobby.
//!
//! The local MVP uses the arena screen for deterministic solo battles. This
//! screen remains as a V2 placeholder so server-backed real-player brackets can
//! be added without diluting the offline loop.

use crate::state::GameState;
use crate::ui::*;
use macroquad::prelude::*;

pub fn reset_state() {}

pub fn draw_tournament_lobby(state: &mut GameState) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Arena);
    if let Some(action) = frame.nav_action {
        return Some(action);
    }

    let content = frame.content;
    let left = Rect::new(content.x, content.y, content.w * 0.42, content.h);
    let right = Rect::new(
        left.x + left.w + PANEL_GAP,
        content.y,
        content.w - left.w - PANEL_GAP,
        content.h,
    );

    draw_panel(left, "MULTIPLAYER LOBBY");
    draw_text(
        "V2 REAL-PLAYER FEATURE",
        left.x + 20.0,
        left.y + 66.0,
        FONT_MEDIUM,
        dark::ACCENT,
    );
    draw_text(
        "Player-vs-player brackets and cross-roster breeding are planned after the V1 local game is stable.",
        left.x + 20.0,
        left.y + 104.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        "For V1, use Arena for seeded solo fights that record battle history on each kaiju.",
        left.x + 20.0,
        left.y + 134.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    if draw_button(
        Rect::new(left.x + 20.0, left.y + left.h - 56.0, 160.0, 36.0),
        "OPEN ARENA",
        dark::ACCENT,
        true,
    ) {
        return Some(UiAction::GoToTournament);
    }

    draw_panel(right, "ELIGIBLE ROSTER");
    let mut y = right.y + 54.0;
    for kaiju in state.roster.iter().filter(|k| k.alive).take(8) {
        draw_rectangle(
            right.x + 16.0,
            y - 20.0,
            right.w - 32.0,
            38.0,
            Color::new(0.04, 0.06, 0.08, 0.85),
        );
        draw_text(
            &kaiju.name,
            right.x + 28.0,
            y + 5.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text_right(
            &format!("PWR {}", kaiju.stats.power_level()),
            right.x + right.w - 28.0,
            y + 5.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
        y += 46.0;
    }

    if state.roster.iter().all(|k| !k.alive) {
        draw_text(
            "No living kaiju are available.",
            right.x + 20.0,
            right.y + 66.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
    }

    None
}
