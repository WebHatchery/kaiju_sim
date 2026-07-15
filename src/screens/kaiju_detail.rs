//! Kaiju detail and legacy record screen.

use crate::data::{Kaiju, KaijuEventKind};
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::{dark, trait_color};
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::colors::with_alpha;
use macroquad_toolkit::ui::draw_ui_text;
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
    let left_w = (c.w * 0.38).clamp(360.0, 440.0);
    let profile = Rect::new(c.x, c.y, left_w, c.h);
    let right_x = c.x + left_w + PANEL_GAP;
    let right_w = c.w - left_w - PANEL_GAP;
    let top_h = 252.0;
    let profile_stats = Rect::new(right_x, c.y, right_w, top_h);
    let lineage = Rect::new(right_x, c.y + top_h + PANEL_GAP, right_w, 142.0);
    let history = Rect::new(
        right_x,
        lineage.y + lineage.h + PANEL_GAP,
        right_w,
        c.h - top_h - lineage.h - PANEL_GAP * 2.0,
    );

    if let Some(action) = draw_profile(profile, kaiju, assets) {
        return Some(action);
    }
    draw_stats_and_traits(profile_stats, kaiju);
    draw_lineage_and_mutations(lineage, kaiju);
    draw_history(history, kaiju);
    None
}

fn draw_profile(rect: Rect, kaiju: &Kaiju, assets: &AssetManager) -> Option<UiAction> {
    draw_panel_with_accent(rect, "SUBJECT RECORD", dark::ACCENT);
    if draw_button(
        Rect::new(rect.x + 16.0, rect.y + 48.0, 82.0, 28.0),
        "BACK",
        dark::TEXT_SECONDARY,
        true,
    ) {
        return Some(UiAction::Back);
    }

    draw_ui_text(
        &ellipsize(&kaiju.name, rect.w - 32.0, FONT_LARGE),
        rect.x + 16.0,
        rect.y + 116.0,
        FONT_LARGE,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "LEGACY ID {} | GENOME {}",
            kaiju.token_id,
            ellipsize(&kaiju.genome_hash, 130.0, FONT_TINY)
        ),
        rect.x + 16.0,
        rect.y + 142.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    let img_size = (rect.w - 32.0).min(rect.h - 302.0).max(220.0);
    draw_portrait(
        Rect::new(rect.x + 16.0, rect.y + 164.0, img_size, img_size),
        kaiju,
        assets,
    );

    let y = rect.y + 190.0 + img_size;
    draw_status_pill(
        Rect::new(rect.x + 16.0, y, 96.0, 28.0),
        if kaiju.alive { "ACTIVE" } else { "DECEASED" },
        if kaiju.alive {
            dark::POSITIVE
        } else {
            dark::NEGATIVE
        },
    );
    draw_status_pill(
        Rect::new(rect.x + 124.0, y, 78.0, 28.0),
        &format!("GEN {}", kaiju.generation),
        dark::ACCENT,
    );
    draw_status_pill(
        Rect::new(rect.x + 214.0, y, 112.0, 28.0),
        &format!("RATING {}", kaiju.battle_rating()),
        dark::WARNING,
    );

    let battle_events = kaiju
        .history
        .iter()
        .filter(|event| event.kind == KaijuEventKind::Battle)
        .count();
    let breeding_events = kaiju
        .history
        .iter()
        .filter(|event| {
            event.kind == KaijuEventKind::Breeding || event.kind == KaijuEventKind::Offspring
        })
        .count();

    draw_metric_tile(
        Rect::new(
            rect.x + 16.0,
            rect.y + rect.h - 126.0,
            (rect.w - 48.0) / 2.0,
            72.0,
        ),
        "BATTLE FILES",
        &battle_events.to_string(),
        dark::WARNING,
    );
    draw_metric_tile(
        Rect::new(
            rect.x + 32.0 + (rect.w - 48.0) / 2.0,
            rect.y + rect.h - 126.0,
            (rect.w - 48.0) / 2.0,
            72.0,
        ),
        "LINEAGE FILES",
        &breeding_events.to_string(),
        dark::ACCENT,
    );
    draw_ui_text(
        &format!("{} total history entries", kaiju.history.len()),
        rect.x + 16.0,
        rect.y + rect.h - 24.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    None
}

fn draw_stats_and_traits(rect: Rect, kaiju: &Kaiju) {
    draw_panel_with_accent(rect, "COMBAT AND TRAITS", dark::POSITIVE);
    let stat_x = rect.x + 16.0;
    let stat_y = rect.y + 60.0;
    let stat_w = (rect.w * 0.48).max(300.0);
    draw_stat_meter(
        stat_x,
        stat_y,
        stat_w,
        "HP",
        kaiju.stats.hp,
        2500,
        dark::HP_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 30.0,
        stat_w,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 60.0,
        stat_w,
        "DEF",
        kaiju.stats.defense,
        300,
        dark::DEF_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 90.0,
        stat_w,
        "SPD",
        kaiju.stats.speed,
        300,
        dark::SPD_COLOR,
    );
    draw_stat_meter(
        stat_x,
        stat_y + 120.0,
        stat_w,
        "ENG",
        kaiju.stats.energy,
        250,
        dark::ACCENT,
    );

    let trait_x = rect.x + rect.w * 0.54;
    draw_ui_text(
        "VISIBLE TRAITS",
        trait_x,
        rect.y + 60.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    let mut x = trait_x;
    let mut y = rect.y + 82.0;
    if kaiju.traits.is_empty() {
        draw_trait_chip(x, y, "No traits revealed", dark::HIDDEN);
    }
    for trait_def in &kaiju.traits {
        let w = draw_trait_chip(x, y, &trait_def.name, trait_color(&trait_def.category));
        x += w + 8.0;
        if x + 90.0 > rect.x + rect.w - 14.0 {
            x = trait_x;
            y += 32.0;
        }
        if y > rect.y + rect.h - 34.0 {
            break;
        }
    }
}

fn draw_lineage_and_mutations(rect: Rect, kaiju: &Kaiju) {
    draw_panel_with_accent(rect, "LINEAGE AND MUTATIONS", dark::WARNING);
    let left_x = rect.x + 16.0;
    draw_ui_text(
        "PARENT RECORDS",
        left_x,
        rect.y + 58.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    if let Some((parent_a, parent_b)) = kaiju.parent_ids {
        draw_ui_text(
            &format!("PARENT A  #{}", parent_a),
            left_x,
            rect.y + 86.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_ui_text(
            &format!("PARENT B  #{}", parent_b),
            left_x,
            rect.y + 114.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
    } else {
        draw_ui_text(
            "Original facility starter or wild registry source.",
            left_x,
            rect.y + 88.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
    }

    let right_x = rect.x + rect.w * 0.52;
    draw_ui_text(
        "MUTATION WATCH",
        right_x,
        rect.y + 58.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    let mutation_count = kaiju
        .traits
        .iter()
        .filter(|trait_def| matches!(trait_def.category, crate::data::TraitCategory::Mutation))
        .count();
    draw_ui_text(
        &format!(
            "{} visible mutations | {} hidden markers",
            mutation_count,
            kaiju.hidden_traits.len()
        ),
        right_x,
        rect.y + 88.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        inheritance_label(kaiju),
        right_x,
        rect.y + 116.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
}

fn draw_history(rect: Rect, kaiju: &Kaiju) {
    draw_panel_with_accent(rect, "CHRONOLOGICAL EVENT LOG", dark::ACCENT);
    let mut y = rect.y + 58.0;
    for event in kaiju.history.iter().rev().take(4) {
        if y + 54.0 > rect.y + rect.h {
            break;
        }
        let color = event_color(&event.kind);
        draw_circle(rect.x + 24.0, y - 5.0, 4.0, color);
        draw_line(
            rect.x + 24.0,
            y,
            rect.x + 24.0,
            y + 44.0,
            1.0,
            with_alpha(color, 0.22),
        );
        draw_ui_text(event.kind.label(), rect.x + 40.0, y, FONT_TINY, color);
        draw_ui_text(
            &ellipsize(&event.title, rect.w - 220.0, FONT_SMALL),
            rect.x + 166.0,
            y,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text_wrapped(
            &event.details,
            rect.x + 166.0,
            y + 20.0,
            rect.w - 184.0,
            16.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
            1,
        );
        y += 58.0;
    }

    if kaiju.history.is_empty() {
        draw_empty_state(rect, "No events recorded", "No entries filed yet.");
    }
}

fn inheritance_label(kaiju: &Kaiju) -> &'static str {
    if kaiju.generation >= 3 {
        "Stabilized deep-lineage inheritance profile."
    } else if kaiju.traits.len() >= 2 {
        "Trait expression is active and worth preserving."
    } else {
        "Baseline inheritance profile, low volatility."
    }
}

fn event_color(kind: &KaijuEventKind) -> Color {
    match kind {
        KaijuEventKind::Battle => dark::WARNING,
        KaijuEventKind::Breeding | KaijuEventKind::Offspring => dark::ACCENT,
        KaijuEventKind::Training => dark::POSITIVE,
        KaijuEventKind::Death => dark::NEGATIVE,
        KaijuEventKind::Research => dark::SYNERGY,
        _ => dark::TEXT_SECONDARY,
    }
}
