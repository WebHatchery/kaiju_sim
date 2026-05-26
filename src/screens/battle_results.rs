//! Battle result screen.

use crate::state::{GameState, LastBattleReport};
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_battle_results(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Results);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let Some(report) = &state.last_battle else {
        draw_panel(c, "BATTLE RESULT");
        draw_empty_state(
            c,
            "No battle result yet",
            "Enter the arena to generate a report.",
        );
        return None;
    };

    let headline_h = 136.0;
    let headline_panel = Rect::new(c.x, c.y, c.w, headline_h);
    if let Some(action) = draw_result_header(headline_panel, report) {
        return Some(action);
    }

    let body_y = c.y + headline_h + PANEL_GAP;
    let left_w = c.w * 0.36;
    let fighters = Rect::new(c.x, body_y, left_w, c.h - headline_h - PANEL_GAP);
    let right_x = c.x + left_w + PANEL_GAP;
    let key_h = 218.0;
    let moments = Rect::new(right_x, body_y, c.w - left_w - PANEL_GAP, key_h);
    let log = Rect::new(
        right_x,
        body_y + key_h + PANEL_GAP,
        c.w - left_w - PANEL_GAP,
        c.h - headline_h - key_h - PANEL_GAP * 2.0,
    );
    draw_fighter_panel(fighters, state, report, assets);
    draw_key_moments(moments, report);
    draw_log_panel(log, report);

    None
}

fn draw_result_header(rect: Rect, report: &LastBattleReport) -> Option<UiAction> {
    let color = if report.won {
        dark::POSITIVE
    } else {
        dark::NEGATIVE
    };
    draw_panel_with_accent(rect, "RESULT SUMMARY", color);
    draw_text(
        if report.won { "VICTORY" } else { "DEFEAT" },
        rect.x + 20.0,
        rect.y + 86.0,
        FONT_HERO,
        color,
    );
    draw_text(
        &format!("{} defeated {}", report.result.winner, report.result.loser),
        rect.x + 330.0,
        rect.y + 56.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        &format!(
            "{} turns | {} | seed {}",
            report.result.turns_elapsed, report.result.environment, report.result.seed
        ),
        rect.x + 330.0,
        rect.y + 84.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_status_pill(
        Rect::new(rect.x + 330.0, rect.y + 98.0, 96.0, 26.0),
        &format!("+{} GOLD", report.gold_reward),
        dark::WARNING,
    );
    draw_status_pill(
        Rect::new(rect.x + 438.0, rect.y + 98.0, 82.0, 26.0),
        &format!("+{} XP", report.xp_reward),
        dark::ACCENT,
    );

    if draw_button(
        Rect::new(rect.x + rect.w - 252.0, rect.y + 82.0, 104.0, 34.0),
        "RECORD",
        dark::ACCENT,
        true,
    ) {
        return Some(UiAction::ViewKaijuDetails(report.player_kaiju_id));
    }
    if draw_button(
        Rect::new(rect.x + rect.w - 132.0, rect.y + 82.0, 104.0, 34.0),
        "ARENA",
        dark::WARNING,
        true,
    ) {
        return Some(UiAction::GoToTournament);
    }

    None
}

fn draw_fighter_panel(
    rect: Rect,
    state: &GameState,
    report: &LastBattleReport,
    assets: &AssetManager,
) {
    draw_panel_with_accent(rect, "FIGHTERS", dark::TEXT_SECONDARY);
    if let Some(player) = state.get_kaiju(report.player_kaiju_id) {
        draw_fighter(
            rect.x + 16.0,
            rect.y + 56.0,
            rect.w - 32.0,
            "YOUR KAIJU",
            player,
            assets,
        );
    }
    draw_fighter(
        rect.x + 16.0,
        rect.y + 208.0,
        rect.w - 32.0,
        "OPPONENT",
        &report.opponent,
        assets,
    );

    let y = rect.y + 374.0;
    draw_text(
        "POST-FIGHT NOTES",
        rect.x + 16.0,
        y,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    let mut note_y = y + 26.0;
    for insight in report.result.insights.iter().take(2) {
        draw_text_wrapped(
            insight,
            rect.x + 16.0,
            note_y,
            rect.w - 32.0,
            17.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
            1,
        );
        note_y += 38.0;
    }
    if report.result.insights.is_empty() {
        draw_text(
            "No notes filed.",
            rect.x + 16.0,
            note_y,
            FONT_TINY,
            dark::TEXT_MUTED,
        );
    }
}

fn draw_key_moments(rect: Rect, report: &LastBattleReport) {
    draw_panel_with_accent(rect, "KEY MOMENTS", dark::WARNING);
    let mut entries = report.result.battle_log.iter().collect::<Vec<_>>();
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.damage));

    let mut y = rect.y + 58.0;
    for entry in entries.into_iter().take(3) {
        draw_moment_row(
            rect.x + 16.0,
            y,
            rect.w - 32.0,
            &format!("TURN {:02}", entry.turn),
            &format!(
                "{} hit {} for {} damage",
                entry.attacker_name, entry.defender_name, entry.damage
            ),
            dark::WARNING,
        );
        y += 44.0;
    }

    if let Some(final_hit) = report.result.battle_log.last() {
        draw_moment_row(
            rect.x + 16.0,
            y,
            rect.w - 32.0,
            "FINAL SHIFT",
            &format!(
                "{} ended the fight with {} HP remaining",
                report.result.winner,
                report.result.hp_remaining.max(final_hit.hp_remaining)
            ),
            if report.won {
                dark::POSITIVE
            } else {
                dark::NEGATIVE
            },
        );
    }
}

fn draw_log_panel(rect: Rect, report: &LastBattleReport) {
    draw_panel_with_accent(rect, "TURN LOG", dark::ACCENT);
    let mut y = rect.y + 54.0;
    for entry in report.result.battle_log.iter().rev().take(12).rev() {
        if y + 22.0 > rect.y + rect.h {
            break;
        }
        let color = if entry.damage >= 45 {
            dark::WARNING
        } else {
            dark::TEXT_SECONDARY
        };
        draw_text(
            &format!("T{:02}", entry.turn),
            rect.x + 16.0,
            y,
            FONT_TINY,
            color,
        );
        draw_text(
            &ellipsize(
                &format!(
                    "{} -> {}  dmg {}  hp {}",
                    entry.attacker_name, entry.defender_name, entry.damage, entry.hp_remaining
                ),
                rect.w - 92.0,
                FONT_SMALL,
            ),
            rect.x + 58.0,
            y,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        y += 26.0;
        for effect in entry.special_effects.iter().take(2) {
            draw_text(
                &ellipsize(effect, rect.w - 96.0, FONT_TINY),
                rect.x + 58.0,
                y,
                FONT_TINY,
                dark::ACCENT,
            );
            y += 18.0;
        }
    }
}

fn draw_fighter(
    x: f32,
    y: f32,
    w: f32,
    label: &str,
    kaiju: &crate::data::Kaiju,
    assets: &AssetManager,
) {
    draw_text(label, x, y, FONT_TINY, dark::TEXT_MUTED);
    draw_portrait(Rect::new(x, y + 16.0, 104.0, 104.0), kaiju, assets);
    draw_text(
        &ellipsize(&kaiju.name, w - 128.0, FONT_MEDIUM),
        x + 122.0,
        y + 48.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        &format!(
            "RATING {} | GEN {}",
            kaiju.battle_rating(),
            kaiju.generation
        ),
        x + 122.0,
        y + 76.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        &format!(
            "HP {}  ATK {}  DEF {}  SPD {}",
            kaiju.stats.hp, kaiju.stats.attack, kaiju.stats.defense, kaiju.stats.speed
        ),
        x + 122.0,
        y + 100.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_line(x, y + 138.0, x + w, y + 138.0, 1.0, dark::BORDER_SOFT);
}

fn draw_moment_row(x: f32, y: f32, w: f32, label: &str, body: &str, color: Color) {
    draw_rectangle(x, y - 20.0, w, 36.0, Color::new(0.030, 0.055, 0.074, 0.82));
    draw_rectangle(x, y - 20.0, 2.0, 36.0, color);
    draw_text(label, x + 12.0, y, FONT_TINY, color);
    draw_text(
        &ellipsize(body, w - 122.0, FONT_SMALL),
        x + 104.0,
        y,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
}
