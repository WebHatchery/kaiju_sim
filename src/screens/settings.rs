//! Local settings and system status screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_settings_screen(state: &GameState) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Settings);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let left_w = c.w * 0.56;
    let settings = Rect::new(c.x, c.y, left_w, c.h);
    let system = Rect::new(c.x + left_w + PANEL_GAP, c.y, c.w - left_w - PANEL_GAP, c.h);

    if let Some(action) = draw_settings_panel(settings, state) {
        return Some(action);
    }
    draw_system_panel(system, state);

    None
}

fn draw_settings_panel(rect: Rect, state: &GameState) -> Option<UiAction> {
    draw_panel_with_accent(rect, "FACILITY SETTINGS", dark::ACCENT);
    draw_text(
        "Facility controls.",
        rect.x + 18.0,
        rect.y + 60.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let mut y = rect.y + 112.0;
    draw_setting_row(
        rect.x + 18.0,
        y,
        rect.w - 36.0,
        "Autosave",
        if state.player.settings.auto_save_enabled {
            "ENABLED"
        } else {
            "DISABLED"
        },
        dark::POSITIVE,
    );
    y += 58.0;
    draw_setting_row(
        rect.x + 18.0,
        y,
        rect.w - 36.0,
        "Battle speed",
        &format!("{:.1}x", state.player.settings.battle_speed),
        dark::WARNING,
    );
    y += 58.0;
    draw_setting_row(
        rect.x + 18.0,
        y,
        rect.w - 36.0,
        "Hints",
        if state.player.settings.show_hints {
            "VISIBLE"
        } else {
            "HIDDEN"
        },
        dark::ACCENT,
    );
    y += 58.0;
    draw_setting_row(
        rect.x + 18.0,
        y,
        rect.w - 36.0,
        "Music volume",
        &format!("{:.0}%", state.player.settings.music_volume * 100.0),
        dark::TEXT_SECONDARY,
    );
    y += 58.0;
    draw_setting_row(
        rect.x + 18.0,
        y,
        rect.w - 36.0,
        "SFX volume",
        &format!("{:.0}%", state.player.settings.sfx_volume * 100.0),
        dark::TEXT_SECONDARY,
    );

    let button_y = rect.y + rect.h - 62.0;
    if draw_button(
        Rect::new(rect.x + 18.0, button_y, 118.0, 36.0),
        "SAVE NOW",
        dark::POSITIVE,
        true,
    ) {
        return Some(UiAction::SaveGame);
    }
    if draw_button(
        Rect::new(rect.x + 150.0, button_y, 122.0, 36.0),
        "LABORATORY",
        dark::ACCENT,
        true,
    ) {
        return Some(UiAction::GoToLaboratory);
    }
    if draw_button(
        Rect::new(rect.x + rect.w - 158.0, button_y, 140.0, 36.0),
        "TITLE SCREEN",
        dark::WARNING,
        true,
    ) {
        return Some(UiAction::GoToMenu);
    }

    None
}

fn draw_system_panel(rect: Rect, state: &GameState) {
    draw_panel_with_accent(rect, "SYSTEM STATUS", dark::POSITIVE);
    draw_metric_tile(
        Rect::new(rect.x + 16.0, rect.y + 58.0, rect.w - 32.0, 72.0),
        "SAVE FORMAT",
        &format!("V{}", state.save_version),
        dark::ACCENT,
    );
    draw_metric_tile(
        Rect::new(rect.x + 16.0, rect.y + 146.0, rect.w - 32.0, 72.0),
        "ROSTER",
        &format!("{} / 50", state.roster.len()),
        dark::POSITIVE,
    );
    draw_metric_tile(
        Rect::new(rect.x + 16.0, rect.y + 234.0, rect.w - 32.0, 72.0),
        "GAME TICKS",
        &state.game_time.total_ticks.to_string(),
        dark::WARNING,
    );

    draw_text(
        "SYSTEM",
        rect.x + 16.0,
        rect.y + 350.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_status_pill(
        Rect::new(rect.x + 16.0, rect.y + 370.0, 104.0, 28.0),
        "FACILITY",
        dark::POSITIVE,
    );
    draw_status_pill(
        Rect::new(rect.x + 132.0, rect.y + 370.0, 104.0, 28.0),
        "OFFLINE SAVE",
        dark::ACCENT,
    );
    draw_status_pill(
        Rect::new(rect.x + 248.0, rect.y + 370.0, 104.0, 28.0),
        "ISOLATED",
        dark::TEXT_SECONDARY,
    );
}

fn draw_setting_row(x: f32, y: f32, w: f32, label: &str, value: &str, color: Color) {
    draw_rectangle(x, y - 26.0, w, 44.0, Color::new(0.030, 0.055, 0.074, 0.82));
    draw_text(label, x + 12.0, y, FONT_SMALL, dark::TEXT_PRIMARY);
    draw_status_pill(
        Rect::new(x + w - 122.0, y - 22.0, 104.0, 28.0),
        value,
        color,
    );
}
