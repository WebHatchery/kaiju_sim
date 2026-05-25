//! Kaiju detail and legacy record screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use uuid::Uuid;

pub fn draw_kaiju_detail(
    state: &GameState,
    kaiju_id: Uuid,
    assets: &AssetManager,
) -> Option<UiAction> {
    let kaiju = match state.get_kaiju(kaiju_id) {
        Some(k) => k,
        None => return Some(UiAction::Back),
    };

    let frame = draw_app_shell(state, AppSection::Detail);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let left_w = c.w * 0.36;
    let profile = Rect::new(c.x, c.y, left_w, c.h);
    let right_x = c.x + left_w + PANEL_GAP;
    let stats = Rect::new(right_x, c.y, c.w - left_w - PANEL_GAP, 220.0);
    let history = Rect::new(
        right_x,
        c.y + 220.0 + PANEL_GAP,
        c.w - left_w - PANEL_GAP,
        c.h - 220.0 - PANEL_GAP,
    );

    draw_profile(profile, kaiju, assets);
    draw_stats_and_traits(stats, kaiju);
    draw_history(history, kaiju);
    None
}

fn draw_profile(rect: Rect, kaiju: &crate::data::Kaiju, assets: &AssetManager) {
    draw_panel(rect, "KAIJU RECORD");
    draw_text(
        &kaiju.name,
        rect.x + 16.0,
        rect.y + 58.0,
        FONT_LARGE,
        dark::ACCENT,
    );
    draw_text(
        &format!("GEN {} | LEGACY ID {}", kaiju.generation, kaiju.token_id),
        rect.x + 16.0,
        rect.y + 88.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    let img_size = (rect.w - 32.0).min(330.0);
    draw_portrait(
        Rect::new(rect.x + 16.0, rect.y + 112.0, img_size, img_size),
        kaiju,
        assets,
    );
    let status = if kaiju.alive { "ACTIVE" } else { "DECEASED" };
    draw_text(
        status,
        rect.x + 16.0,
        rect.y + 136.0 + img_size,
        FONT_MEDIUM,
        if kaiju.alive {
            dark::POSITIVE
        } else {
            dark::NEGATIVE
        },
    );
    draw_text(
        &format!("History entries: {}", kaiju.history.len()),
        rect.x + 16.0,
        rect.y + 166.0 + img_size,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
}

fn draw_stats_and_traits(rect: Rect, kaiju: &crate::data::Kaiju) {
    draw_panel(rect, "STATS AND TRAITS");
    let stat_x = rect.x + 16.0;
    let stat_y = rect.y + 56.0;
    draw_stat_meter(
        stat_x,
        stat_y,
        280.0,
        "HP",
        kaiju.stats.hp,
        2500,
        dark::HP_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 28.0,
        280.0,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 56.0,
        280.0,
        "DEF",
        kaiju.stats.defense,
        300,
        dark::DEF_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 84.0,
        280.0,
        "SPD",
        kaiju.stats.speed,
        300,
        dark::SPD_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 112.0,
        280.0,
        "ENG",
        kaiju.stats.energy,
        250,
        dark::ACCENT,
    );

    let trait_x = rect.x + 340.0;
    draw_text(
        "VISIBLE TRAITS",
        trait_x,
        rect.y + 56.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    let mut x = trait_x;
    let mut y = rect.y + 88.0;
    if kaiju.traits.is_empty() {
        draw_text("No traits discovered.", x, y, FONT_SMALL, dark::TEXT_MUTED);
    }
    for trait_def in &kaiju.traits {
        draw_rectangle(x, y - 18.0, 132.0, 26.0, Color::new(0.06, 0.10, 0.14, 0.96));
        draw_rectangle_lines(x, y - 18.0, 132.0, 26.0, 1.0, dark::ACCENT);
        draw_text(&trait_def.name, x + 8.0, y, FONT_TINY, dark::ACCENT);
        x += 144.0;
        if x + 132.0 > rect.x + rect.w - 12.0 {
            x = trait_x;
            y += 36.0;
        }
    }
}

fn draw_history(rect: Rect, kaiju: &crate::data::Kaiju) {
    draw_panel(rect, "DOCUMENTED HISTORY");
    let mut y = rect.y + 54.0;
    for event in kaiju.history.iter().rev().take(12) {
        draw_rectangle(
            rect.x + 14.0,
            y - 22.0,
            rect.w - 28.0,
            48.0,
            Color::new(0.035, 0.060, 0.082, 0.94),
        );
        draw_text(
            event.kind.label(),
            rect.x + 28.0,
            y,
            FONT_TINY,
            dark::ACCENT,
        );
        draw_text(
            &event.title,
            rect.x + 150.0,
            y,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text(
            &event.details,
            rect.x + 150.0,
            y + 20.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
        y += 58.0;
    }
}
