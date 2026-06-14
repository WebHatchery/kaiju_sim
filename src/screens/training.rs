//! Training screen.

use crate::data::{Kaiju, TrainingFocus};
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub fn draw_training_screen(
    state: &GameState,
    assets: &AssetManager,
    training_cost: i64,
) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Training);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let top_h = (c.h * 0.68).clamp(400.0, 430.0);
    let subject_w = (c.w * 0.36).clamp(340.0, 430.0);
    let subject_panel = Rect::new(c.x, c.y, subject_w, top_h);
    let program_panel = Rect::new(
        c.x + subject_w + PANEL_GAP,
        c.y,
        c.w - subject_w - PANEL_GAP,
        top_h,
    );
    let roster_panel = Rect::new(c.x, c.y + top_h + PANEL_GAP, c.w, c.h - top_h - PANEL_GAP);

    let active = selected_living_subject(state);
    draw_training_subject(subject_panel, active, assets, training_cost);
    if let Some(action) = draw_program_cards(
        program_panel,
        active,
        state.player.gold >= training_cost,
        training_cost,
    ) {
        return Some(action);
    }
    if let Some(action) = draw_training_roster(roster_panel, state, assets) {
        return Some(action);
    }

    None
}

fn draw_training_subject(rect: Rect, kaiju: Option<&Kaiju>, assets: &AssetManager, cost: i64) {
    draw_panel_with_accent(rect, "TRAINING SUBJECT", dark::POSITIVE);
    let Some(kaiju) = kaiju else {
        draw_empty_state(
            rect,
            "No living kaiju",
            "Training requires an active specimen.",
        );
        return;
    };

    draw_ui_text(
        &ellipsize(&kaiju.name, rect.w - 32.0, FONT_LARGE),
        rect.x + 16.0,
        rect.y + 66.0,
        FONT_LARGE,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "GEN {} | XP {} | SESSION COST {}",
            kaiju.generation, kaiju.experience, cost
        ),
        rect.x + 16.0,
        rect.y + 92.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    let portrait_size = (rect.h - 250.0).clamp(120.0, (rect.w - 32.0).min(190.0));
    draw_portrait(
        Rect::new(rect.x + 16.0, rect.y + 112.0, portrait_size, portrait_size),
        kaiju,
        assets,
    );

    let stat_x = rect.x + 16.0;
    let y = rect.y + rect.h - 112.0;
    draw_ui_text("CURRENT LOADOUT", stat_x, y, FONT_TINY, dark::TEXT_MUTED);
    draw_stat_meter(
        stat_x,
        y + 28.0,
        rect.w - 32.0,
        "HP",
        kaiju.stats.hp,
        2500,
        dark::HP_COLOR,
    );
    draw_stat_meter(
        stat_x,
        y + 56.0,
        rect.w - 32.0,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_stat_meter(
        stat_x,
        y + 84.0,
        rect.w - 32.0,
        "DEF",
        kaiju.stats.defense,
        300,
        dark::DEF_COLOR,
    );
}

fn draw_program_cards(
    rect: Rect,
    kaiju: Option<&Kaiju>,
    can_afford: bool,
    cost: i64,
) -> Option<UiAction> {
    draw_panel_with_accent(rect, "GROWTH PROGRAMS", dark::ACCENT);
    draw_ui_text(
        "Pick one controlled stress program for the active subject.",
        rect.x + 16.0,
        rect.y + 56.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let card_gap = 14.0;
    let card_w = (rect.w - 46.0) / 2.0;
    let card_h = (rect.h - 110.0 - card_gap) / 2.0;
    let start_y = rect.y + 82.0;
    for (index, focus) in TrainingFocus::all().iter().enumerate() {
        let col = index % 2;
        let row = index / 2;
        let card = Rect::new(
            rect.x + 16.0 + col as f32 * (card_w + card_gap),
            start_y + row as f32 * (card_h + card_gap),
            card_w,
            card_h,
        );
        if let Some(action) = draw_program_card(card, *focus, kaiju, can_afford, cost) {
            return Some(action);
        }
    }
    None
}

fn draw_program_card(
    rect: Rect,
    focus: TrainingFocus,
    kaiju: Option<&Kaiju>,
    can_afford: bool,
    cost: i64,
) -> Option<UiAction> {
    let color = focus_color(focus);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.065, 0.082, 0.92),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        2.0,
        Color::new(color.r, color.g, color.b, 0.62),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(color.r, color.g, color.b, 0.28),
    );

    draw_ui_text(
        focus.label().to_uppercase().as_str(),
        rect.x + 14.0,
        rect.y + 28.0,
        FONT_MEDIUM,
        color,
    );
    draw_ui_text(
        program_theme(focus),
        rect.x + 14.0,
        rect.y + 54.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    draw_ui_text(
        &format!("Estimated: focused {} growth", focus.stat_label()),
        rect.x + 14.0,
        rect.y + 84.0,
        FONT_TINY,
        dark::TEXT_PRIMARY,
    );
    draw_status_pill(
        Rect::new(rect.x + rect.w - 118.0, rect.y + 14.0, 102.0, 24.0),
        risk_label(focus),
        risk_color(focus),
    );
    draw_ui_text(
        &format!("{} gold", cost),
        rect.x + 14.0,
        rect.y + rect.h - 18.0,
        FONT_TINY,
        if can_afford {
            dark::TEXT_SECONDARY
        } else {
            dark::NEGATIVE
        },
    );

    if let Some(kaiju) = kaiju {
        if draw_button(
            Rect::new(rect.x + rect.w - 102.0, rect.y + rect.h - 40.0, 86.0, 28.0),
            "RUN",
            color,
            can_afford,
        ) {
            return Some(UiAction::TrainKaiju {
                kaiju_id: kaiju.id,
                focus,
            });
        }
    } else {
        draw_button(
            Rect::new(rect.x + rect.w - 102.0, rect.y + rect.h - 40.0, 86.0, 28.0),
            "RUN",
            color,
            false,
        );
    }

    None
}

fn draw_training_roster(rect: Rect, state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    draw_panel_with_accent(rect, "AVAILABLE SUBJECTS", dark::TEXT_SECONDARY);
    let mut x = rect.x + 16.0;
    let y = rect.y + 52.0;
    let row_w = 250.0;
    for kaiju in state.roster.iter().filter(|k| k.alive).take(4) {
        let row = Rect::new(x, y, row_w, rect.h - 68.0);
        if let Some(action) =
            draw_subject_chip(row, kaiju, assets, state.selected_kaiju == Some(kaiju.id))
        {
            return Some(action);
        }
        x += row_w + 12.0;
    }

    if state.living_count() == 0 {
        draw_empty_state(
            rect,
            "No active subjects",
            "The roster has no living kaiju available for training.",
        );
    }

    None
}

fn draw_subject_chip(
    rect: Rect,
    kaiju: &Kaiju,
    assets: &AssetManager,
    selected: bool,
) -> Option<UiAction> {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    let color = if selected {
        dark::WARNING
    } else {
        dark::ACCENT
    };
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.030, 0.055, 0.074, 0.92),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(
            color.r,
            color.g,
            color.b,
            if selected || hovered { 0.74 } else { 0.28 },
        ),
    );
    draw_portrait(
        Rect::new(rect.x + 10.0, rect.y + 10.0, 62.0, 62.0),
        kaiju,
        assets,
    );
    draw_ui_text(
        &ellipsize(&kaiju.name, 128.0, FONT_SMALL),
        rect.x + 84.0,
        rect.y + 32.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "GEN {} | rating {}",
            kaiju.generation,
            kaiju.battle_rating()
        ),
        rect.x + 84.0,
        rect.y + 56.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        return Some(UiAction::SelectKaiju(kaiju.id));
    }
    None
}

fn selected_living_subject(state: &GameState) -> Option<&Kaiju> {
    state
        .selected_kaiju
        .and_then(|id| state.get_kaiju(id))
        .filter(|kaiju| kaiju.alive)
        .or_else(|| state.roster.iter().find(|kaiju| kaiju.alive))
}

fn program_theme(focus: TrainingFocus) -> &'static str {
    match focus {
        TrainingFocus::Endurance => "Hydraulic resistance and recovery tanks",
        TrainingFocus::Power => "Impact towers and pressure strikes",
        TrainingFocus::Guard => "Armor bracing and shield posture",
        TrainingFocus::Reflex => "Strobe timing and predator response",
    }
}

fn risk_label(focus: TrainingFocus) -> &'static str {
    match focus {
        TrainingFocus::Endurance => "LOW STRAIN",
        TrainingFocus::Power => "HIGH STRAIN",
        TrainingFocus::Guard => "LOW RISK",
        TrainingFocus::Reflex => "MED RISK",
    }
}

fn risk_color(focus: TrainingFocus) -> Color {
    match focus {
        TrainingFocus::Power | TrainingFocus::Reflex => dark::WARNING,
        _ => dark::POSITIVE,
    }
}

fn focus_color(focus: TrainingFocus) -> Color {
    match focus {
        TrainingFocus::Endurance => dark::HP_COLOR,
        TrainingFocus::Power => dark::ATK_COLOR,
        TrainingFocus::Guard => dark::DEF_COLOR,
        TrainingFocus::Reflex => dark::SPD_COLOR,
    }
}
