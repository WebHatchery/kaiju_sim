//! Laboratory dashboard screen.

use crate::data::Kaiju;
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::{dark, trait_color};
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub fn draw_laboratory(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Laboratory);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let hero_h = (c.h * 0.58).clamp(330.0, 380.0);
    let left_w = c.w * 0.64;
    let active_panel = Rect::new(c.x, c.y, left_w, hero_h);
    let status_panel = Rect::new(
        c.x + left_w + PANEL_GAP,
        c.y,
        c.w - left_w - PANEL_GAP,
        hero_h,
    );
    let bottom_h = c.h - hero_h - PANEL_GAP;
    let activity_panel = Rect::new(c.x, c.y + hero_h + PANEL_GAP, left_w * 0.54, bottom_h);
    let bloodline_panel = Rect::new(
        activity_panel.x + activity_panel.w + PANEL_GAP,
        activity_panel.y,
        left_w - activity_panel.w - PANEL_GAP,
        bottom_h,
    );
    let directive_panel = Rect::new(status_panel.x, activity_panel.y, status_panel.w, bottom_h);

    if let Some(action) = draw_active_specimen(state, assets, active_panel) {
        return Some(action);
    }
    draw_facility_status(state, status_panel);
    draw_notification_rows(state, activity_panel);
    draw_bloodline_summary(state, bloodline_panel);
    draw_next_directive(state, directive_panel);

    None
}

fn draw_active_specimen(state: &GameState, assets: &AssetManager, rect: Rect) -> Option<UiAction> {
    draw_panel_with_accent(rect, "ACTIVE SPECIMEN", dark::ACCENT);
    let Some(kaiju) = selected_subject(state) else {
        draw_empty_state(
            rect,
            "No registered kaiju",
            "Start or load a facility roster.",
        );
        return None;
    };

    let portrait_size = (rect.h - 92.0).min(rect.w * 0.42).max(210.0);
    let portrait = Rect::new(rect.x + 22.0, rect.y + 56.0, portrait_size, portrait_size);
    draw_portrait(portrait, kaiju, assets);

    let info_x = portrait.x + portrait.w + 26.0;
    let info_w = rect.x + rect.w - info_x - 22.0;
    draw_ui_text(
        &ellipsize(&kaiju.name, info_w - 90.0, FONT_LARGE),
        info_x,
        rect.y + 70.0,
        FONT_LARGE,
        dark::TEXT_PRIMARY,
    );
    draw_status_pill(
        Rect::new(info_x, rect.y + 86.0, 92.0, 25.0),
        if kaiju.alive { "ACTIVE" } else { "DECEASED" },
        if kaiju.alive {
            dark::POSITIVE
        } else {
            dark::NEGATIVE
        },
    );
    draw_status_pill(
        Rect::new(info_x + 104.0, rect.y + 86.0, 78.0, 25.0),
        &format!("GEN {}", kaiju.generation),
        dark::ACCENT,
    );
    draw_status_pill(
        Rect::new(info_x + 194.0, rect.y + 86.0, 108.0, 25.0),
        &format!("RATING {}", kaiju.battle_rating()),
        dark::WARNING,
    );

    draw_ui_text(
        "KEY READOUTS",
        info_x,
        rect.y + 142.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_stat_meter(
        info_x,
        rect.y + 170.0,
        info_w,
        "HP",
        kaiju.stats.hp,
        2500,
        dark::HP_COLOR,
    );
    draw_stat_meter(
        info_x,
        rect.y + 198.0,
        info_w,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_stat_meter(
        info_x,
        rect.y + 226.0,
        info_w,
        "DEF",
        kaiju.stats.defense,
        300,
        dark::DEF_COLOR,
    );
    draw_stat_meter(
        info_x,
        rect.y + 254.0,
        info_w,
        "SPD",
        kaiju.stats.speed,
        300,
        dark::SPD_COLOR,
    );

    let mut chip_x = info_x;
    let chip_y = rect.y + 284.0;
    if kaiju.traits.is_empty() {
        draw_trait_chip(chip_x, chip_y, "Trait data sealed", dark::HIDDEN);
    } else {
        for trait_def in kaiju.traits.iter().take(2) {
            let w = draw_trait_chip(
                chip_x,
                chip_y,
                &trait_def.name,
                trait_color(&trait_def.category),
            );
            chip_x += w + 8.0;
            if chip_x > rect.x + rect.w - 120.0 {
                break;
            }
        }
    }

    let button_y = rect.y + rect.h - 50.0;
    if draw_button(
        Rect::new(info_x, button_y, 128.0, 34.0),
        "OPEN RECORD",
        dark::ACCENT,
        true,
    ) {
        return Some(UiAction::ViewKaijuDetails(kaiju.id));
    }
    if draw_button(
        Rect::new(info_x + 140.0, button_y, 96.0, 34.0),
        "TRAIN",
        dark::POSITIVE,
        kaiju.alive,
    ) {
        return Some(UiAction::GoToTraining);
    }
    if draw_button(
        Rect::new(info_x + 248.0, button_y, 96.0, 34.0),
        "ARENA",
        dark::WARNING,
        kaiju.alive,
    ) {
        return Some(UiAction::GoToTournament);
    }

    None
}

fn draw_facility_status(state: &GameState, rect: Rect) {
    draw_panel_with_accent(rect, "FACILITY STATUS", dark::POSITIVE);
    let tile_w = (rect.w - 44.0) / 2.0;
    let tile_h = 72.0;
    draw_metric_tile(
        Rect::new(rect.x + 16.0, rect.y + 52.0, tile_w, tile_h),
        "AVAILABLE GOLD",
        &state.player.gold.to_string(),
        dark::WARNING,
    );
    draw_metric_tile(
        Rect::new(rect.x + 28.0 + tile_w, rect.y + 52.0, tile_w, tile_h),
        "LIVING KAIJU",
        &format!("{}", state.living_count()),
        dark::POSITIVE,
    );
    draw_metric_tile(
        Rect::new(rect.x + 16.0, rect.y + 138.0, tile_w, tile_h),
        "MAX GENERATION",
        &state.player.stats.highest_generation.to_string(),
        dark::ACCENT,
    );
    draw_metric_tile(
        Rect::new(rect.x + 28.0 + tile_w, rect.y + 138.0, tile_w, tile_h),
        "BATTLES WON",
        &format!(
            "{}/{}",
            state.player.stats.total_battles_won, state.player.stats.total_battles_fought
        ),
        dark::POSITIVE,
    );

    let y = rect.y + 244.0;
    draw_ui_text(
        "CURRENT ACTIVITY",
        rect.x + 16.0,
        y,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_activity_row(
        rect.x + 16.0,
        y + 28.0,
        rect.w - 32.0,
        "Training rigs",
        0.64,
        dark::POSITIVE,
    );
    draw_activity_row(
        rect.x + 16.0,
        y + 62.0,
        rect.w - 32.0,
        "Incubators",
        incubation_load(state),
        dark::ACCENT,
    );
    draw_activity_row(
        rect.x + 16.0,
        y + 96.0,
        rect.w - 32.0,
        "Arena uplink",
        0.72,
        dark::WARNING,
    );
}

fn draw_bloodline_summary(state: &GameState, rect: Rect) {
    draw_panel_with_accent(rect, "BLOODLINE SNAPSHOT", dark::WARNING);
    let bred = state.player.stats.total_kaiju_bred;
    draw_ui_text(
        &format!("{} hatches recorded", bred),
        rect.x + 16.0,
        rect.y + 58.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        "Activity archived.",
        rect.x + 16.0,
        rect.y + 86.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    let mut ranked = state.roster.iter().collect::<Vec<_>>();
    ranked.sort_by_key(|kaiju| std::cmp::Reverse((kaiju.tournaments_won, kaiju.battle_rating())));

    let mut y = rect.y + 140.0;
    for (index, kaiju) in ranked.iter().take(3).enumerate() {
        draw_ui_text(
            &format!("#{}", index + 1),
            rect.x + 16.0,
            y,
            FONT_TINY,
            dark::WARNING,
        );
        draw_ui_text(
            &ellipsize(&kaiju.name, rect.w - 112.0, FONT_SMALL),
            rect.x + 52.0,
            y,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text_right(
            &kaiju.battle_rating().to_string(),
            rect.x + rect.w - 16.0,
            y,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
        y += 28.0;
    }
}

fn draw_next_directive(state: &GameState, rect: Rect) {
    draw_panel_with_accent(rect, "NEXT DIRECTIVE", dark::ACCENT);
    let directive = if state.roster.len() < 2 {
        (
            "Secure a second specimen",
            "Breeding requires two living kaiju in the roster.",
        )
    } else if state.player.stats.total_battles_fought == 0 {
        ("Test the active kaiju", "Enter one arena fight.")
    } else if state.player.stats.total_kaiju_bred == 0 {
        ("Start first lineage", "Hatch generation 1.")
    } else {
        ("Specialize the bloodline", "Train or fight.")
    };

    draw_ui_text(
        directive.0,
        rect.x + 16.0,
        rect.y + 62.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        directive.1,
        rect.x + 16.0,
        rect.y + 96.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let bar = Rect::new(rect.x + 16.0, rect.y + rect.h - 52.0, rect.w - 32.0, 8.0);
    let progress = if state.player.stats.total_kaiju_bred > 0 {
        0.86
    } else if state.player.stats.total_battles_fought > 0 {
        0.58
    } else {
        0.32
    };
    draw_progress_bar(bar, progress, dark::ACCENT);
    draw_ui_text(
        "LEGACY LOOP",
        rect.x + 16.0,
        rect.y + rect.h - 18.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
}

fn draw_activity_row(x: f32, y: f32, w: f32, label: &str, percent: f32, color: Color) {
    draw_ui_text(label, x, y, FONT_SMALL, dark::TEXT_SECONDARY);
    draw_progress_bar(
        Rect::new(x + 126.0, y - 9.0, w - 126.0, 7.0),
        percent,
        color,
    );
}

fn selected_subject(state: &GameState) -> Option<&Kaiju> {
    state
        .selected_kaiju
        .and_then(|id| state.get_kaiju(id))
        .or_else(|| state.roster.iter().find(|kaiju| kaiju.alive))
        .or_else(|| state.roster.first())
}

fn incubation_load(state: &GameState) -> f32 {
    if state.pending_breeding.is_some() {
        0.82
    } else if state.player.stats.total_kaiju_bred > 0 {
        0.36
    } else {
        0.18
    }
}
