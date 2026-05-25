//! Local MVP battle result screen.

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
        draw_text_centered(
            "No battle result yet.",
            c.x + c.w / 2.0,
            c.y + c.h / 2.0,
            FONT_LARGE,
            dark::TEXT_MUTED,
        );
        return None;
    };

    let headline_h = 128.0;
    let headline_panel = Rect::new(c.x, c.y, c.w, headline_h);
    draw_panel(headline_panel, "RESULT SUMMARY");
    draw_result_header(headline_panel, report);

    let body_y = c.y + headline_h + PANEL_GAP;
    let left_w = c.w * 0.38;
    let fighters = Rect::new(c.x, body_y, left_w, c.h - headline_h - PANEL_GAP);
    let log = Rect::new(
        c.x + left_w + PANEL_GAP,
        body_y,
        c.w - left_w - PANEL_GAP,
        fighters.h,
    );
    draw_fighter_panel(fighters, state, report, assets);
    draw_log_panel(log, report);

    None
}

fn draw_result_header(rect: Rect, report: &LastBattleReport) {
    let color = if report.won {
        dark::POSITIVE
    } else {
        dark::NEGATIVE
    };
    draw_text(
        if report.won { "VICTORY" } else { "DEFEAT" },
        rect.x + 18.0,
        rect.y + 72.0,
        FONT_HERO,
        color,
    );
    draw_text(
        &format!(
            "{} defeated {} in {} turns",
            report.result.winner, report.result.loser, report.result.turns_elapsed
        ),
        rect.x + 330.0,
        rect.y + 58.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        &format!(
            "+{} gold | +{} XP | {} | seed {}",
            report.gold_reward, report.xp_reward, report.result.environment, report.result.seed
        ),
        rect.x + 330.0,
        rect.y + 90.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
}

fn draw_fighter_panel(
    rect: Rect,
    state: &GameState,
    report: &LastBattleReport,
    assets: &AssetManager,
) {
    draw_panel(rect, "FIGHTERS");
    if let Some(player) = state.get_kaiju(report.player_kaiju_id) {
        draw_fighter(
            rect.x + 16.0,
            rect.y + 52.0,
            rect.w - 32.0,
            "YOUR KAIJU",
            player,
            assets,
        );
    }
    draw_fighter(
        rect.x + 16.0,
        rect.y + 190.0,
        rect.w - 32.0,
        "OPPONENT",
        &report.opponent,
        assets,
    );

    let mut y = rect.y + 350.0;
    draw_text(
        "BATTLE NOTES",
        rect.x + 16.0,
        y,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    y += 28.0;
    for insight in report.result.insights.iter().take(4) {
        draw_text(insight, rect.x + 16.0, y, FONT_TINY, dark::TEXT_SECONDARY);
        y += 22.0;
    }
}

fn draw_log_panel(rect: Rect, report: &LastBattleReport) {
    draw_panel(rect, "TURN LOG");
    let mut y = rect.y + 54.0;
    for entry in report.result.battle_log.iter().rev().take(20).rev() {
        draw_text(
            &format!(
                "TURN {:02}  {} -> {}  DMG {:03}  HP {}",
                entry.turn,
                entry.attacker_name,
                entry.defender_name,
                entry.damage,
                entry.hp_remaining
            ),
            rect.x + 16.0,
            y,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        y += 24.0;
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
    draw_portrait(Rect::new(x, y + 14.0, 94.0, 94.0), kaiju, assets);
    draw_text(&kaiju.name, x + 112.0, y + 44.0, FONT_MEDIUM, dark::ACCENT);
    draw_text(
        &format!(
            "RATING {} | GEN {}",
            kaiju.battle_rating(),
            kaiju.generation
        ),
        x + 112.0,
        y + 72.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        &format!(
            "HP {}  ATK {}  DEF {}  SPD {}",
            kaiju.stats.hp, kaiju.stats.attack, kaiju.stats.defense, kaiju.stats.speed
        ),
        x + 112.0,
        y + 96.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_line(x, y + 126.0, x + w, y + 126.0, 1.0, dark::BORDER);
}
