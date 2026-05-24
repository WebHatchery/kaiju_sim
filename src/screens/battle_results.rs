//! Local MVP battle result screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_battle_results(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    clear_background(dark::BACKGROUND);

    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("BATTLE RESULT", 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);

    if draw_button(sw - 300.0, 15.0, 130.0, 30.0, "Arena", dark::WARNING) {
        return Some(UiAction::GoToTournament);
    }
    if draw_button(sw - 150.0, 15.0, 130.0, 30.0, "Laboratory", dark::BUTTON_BG) {
        return Some(UiAction::GoToLaboratory);
    }

    let Some(report) = &state.last_battle else {
        draw_text_centered(
            "No battle result yet.",
            sw / 2.0,
            sh / 2.0,
            FONT_LARGE,
            dark::TEXT_MUTED,
        );
        return None;
    };

    let result_color = if report.won {
        dark::POSITIVE
    } else {
        dark::NEGATIVE
    };
    let headline = if report.won { "VICTORY" } else { "DEFEAT" };
    draw_text_centered(headline, sw / 2.0, 118.0, FONT_HERO, result_color);
    draw_text_centered(
        &format!(
            "{} defeated {} in {} turns",
            report.result.winner, report.result.loser, report.result.turns_elapsed
        ),
        sw / 2.0,
        154.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text_centered(
        &format!(
            "Rewards: +{} gold, +{} XP | Environment: {} | Seed: {}",
            report.gold_reward, report.xp_reward, report.result.environment, report.result.seed
        ),
        sw / 2.0,
        182.0,
        FONT_NORMAL,
        dark::TEXT_SECONDARY,
    );

    let left_x = 40.0;
    let panel_y = 220.0;
    let panel_w = sw * 0.34;
    let panel_h = sh - panel_y - 40.0;
    draw_summary_panel(left_x, panel_y, panel_w, panel_h, state, report, assets);
    draw_log_panel(
        left_x + panel_w + 28.0,
        panel_y,
        sw - panel_w - 108.0,
        panel_h,
        report,
    );

    None
}

fn draw_summary_panel(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    state: &GameState,
    report: &crate::state::LastBattleReport,
    assets: &AssetManager,
) {
    draw_rectangle(x, y, w, h, dark::SURFACE);
    draw_rectangle_lines(x, y, w, h, 1.0, dark::BORDER);
    draw_text("Fighters", x + 18.0, y + 34.0, FONT_MEDIUM, WHITE);

    let player = state.get_kaiju(report.player_kaiju_id);
    if let Some(kaiju) = player {
        draw_fighter(x + 18.0, y + 60.0, w - 36.0, "Your Kaiju", kaiju, assets);
    }
    draw_fighter(
        x + 18.0,
        y + 190.0,
        w - 36.0,
        "Opponent",
        &report.opponent,
        assets,
    );

    let mut insight_y = y + 332.0;
    draw_text("Battle Notes", x + 18.0, insight_y, FONT_MEDIUM, WHITE);
    insight_y += 28.0;
    for insight in report.result.insights.iter().take(4) {
        draw_text(
            insight,
            x + 18.0,
            insight_y,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        insight_y += 22.0;
    }
}

fn draw_log_panel(x: f32, y: f32, w: f32, h: f32, report: &crate::state::LastBattleReport) {
    draw_rectangle(x, y, w, h, dark::SURFACE);
    draw_rectangle_lines(x, y, w, h, 1.0, dark::BORDER);
    draw_text("Turn Log", x + 18.0, y + 34.0, FONT_MEDIUM, WHITE);

    let mut log_y = y + 66.0;
    for entry in report.result.battle_log.iter().rev().take(18).rev() {
        draw_text(
            &format!(
                "Turn {:02}: {} hit {} for {} ({} HP left)",
                entry.turn,
                entry.attacker_name,
                entry.defender_name,
                entry.damage,
                entry.hp_remaining
            ),
            x + 18.0,
            log_y,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        log_y += 22.0;
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
    draw_text(label, x, y, FONT_SMALL, dark::TEXT_MUTED);
    draw_portrait(x, y + 14.0, 82.0, kaiju, assets);
    draw_text(&kaiju.name, x + 98.0, y + 42.0, FONT_MEDIUM, WHITE);
    draw_text(
        &format!(
            "Rating {} | Gen {}",
            kaiju.battle_rating(),
            kaiju.generation
        ),
        x + 98.0,
        y + 68.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        &format!(
            "HP {}  ATK {}  DEF {}  SPD {}",
            kaiju.stats.hp, kaiju.stats.attack, kaiju.stats.defense, kaiju.stats.speed
        ),
        x + 98.0,
        y + 91.0,
        FONT_SMALL,
        dark::TEXT_MUTED,
    );
    draw_line(x, y + 118.0, x + w, y + 118.0, 1.0, dark::BORDER);
}

fn draw_portrait(x: f32, y: f32, size: f32, kaiju: &crate::data::Kaiju, assets: &AssetManager) {
    let mut drawn = false;
    if let Some(uri) = &kaiju.image_uri {
        let key = assets.get_filename_from_url(uri);
        if let Some(tex) = assets.get_texture(&key) {
            draw_texture_ex(
                tex,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );
            drawn = true;
        }
    }
    if !drawn {
        draw_rectangle(x, y, size, size, dark::PANEL);
    }
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str, accent: Color) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    draw_rectangle(x, y, w, h, if hovered { accent } else { dark::BUTTON_BG });
    draw_text_centered(text, x + w / 2.0, y + h / 2.0 + 5.0, FONT_SMALL, WHITE);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
