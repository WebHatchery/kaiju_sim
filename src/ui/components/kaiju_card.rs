//! Kaiju card component matching the dashboard visual standard.

use crate::data::Kaiju;
use crate::ui::assets::AssetManager;
use crate::ui::colors::{dark, trait_color};
use crate::ui::shell::{
    draw_button, draw_portrait, draw_progress_bar, draw_status_pill, draw_trait_chip,
};
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Normal,
    Hovered,
    Selected,
    Dead,
}

#[derive(Debug, Clone, Copy)]
pub enum CardAction {
    Select,
    ViewDetails,
}

pub fn draw_kaiju_card(
    x: f32,
    y: f32,
    kaiju: &Kaiju,
    state: CardState,
    assets: &AssetManager,
) -> Option<CardAction> {
    let mouse = mouse_position();
    let card = Rect::new(x, y, CARD_WIDTH, CARD_HEIGHT);
    let hovered = card.contains(vec2(mouse.0, mouse.1));
    let effective_state = if !kaiju.alive {
        CardState::Dead
    } else if state == CardState::Selected {
        CardState::Selected
    } else if hovered {
        CardState::Hovered
    } else {
        CardState::Normal
    };

    let accent = card_accent(kaiju, effective_state);
    draw_rectangle(
        x + 5.0,
        y + 7.0,
        CARD_WIDTH,
        CARD_HEIGHT,
        Color::new(0.0, 0.0, 0.0, 0.24),
    );
    draw_rectangle(
        x,
        y,
        CARD_WIDTH,
        CARD_HEIGHT,
        Color::new(0.030, 0.055, 0.074, 0.96),
    );
    draw_rectangle(
        x,
        y,
        CARD_WIDTH,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.80),
    );
    draw_rectangle_lines(
        x,
        y,
        CARD_WIDTH,
        CARD_HEIGHT,
        1.0,
        Color::new(
            accent.r,
            accent.g,
            accent.b,
            if hovered { 0.85 } else { 0.38 },
        ),
    );

    draw_ui_text(
        &ellipsize(&kaiju.name, 142.0, FONT_MEDIUM),
        x + 12.0,
        y + 28.0,
        FONT_MEDIUM,
        accent,
    );
    draw_status_pill(
        Rect::new(x + CARD_WIDTH - 74.0, y + 10.0, 60.0, 24.0),
        &format!("G{}", kaiju.generation),
        rarity_color(kaiju),
    );

    let portrait = Rect::new(x + 12.0, y + 46.0, CARD_WIDTH - 24.0, 150.0);
    draw_portrait(portrait, kaiju, assets);

    let rarity = rarity_label(kaiju);
    draw_ui_text(rarity, x + 14.0, y + 218.0, FONT_TINY, rarity_color(kaiju));
    draw_text_right(
        &format!("RATING {}", kaiju.battle_rating()),
        x + CARD_WIDTH - 14.0,
        y + 218.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    let stat_y = y + 240.0;
    draw_micro_stat(x + 14.0, stat_y, "HP", kaiju.stats.hp, 2500, dark::HP_COLOR);
    draw_micro_stat(
        x + 14.0,
        stat_y + 18.0,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_micro_stat(
        x + 14.0,
        stat_y + 36.0,
        "DEF",
        kaiju.stats.defense,
        300,
        dark::DEF_COLOR,
    );
    draw_micro_stat(
        x + 14.0,
        stat_y + 54.0,
        "SPD",
        kaiju.stats.speed,
        300,
        dark::SPD_COLOR,
    );

    let chip_y = y + 306.0;
    let first_chip_w = if let Some(first) = kaiju.traits.first() {
        draw_trait_chip(x + 12.0, chip_y, &first.name, trait_color(&first.category))
    } else {
        draw_trait_chip(x + 12.0, chip_y, "Trait sealed", dark::HIDDEN)
    };
    if first_chip_w < 116.0 {
        if let Some(second) = kaiju.traits.get(1) {
            let second_x = x + 18.0 + first_chip_w;
            let max_w = x + CARD_WIDTH - second_x - 12.0;
            if max_w > 72.0 {
                draw_trait_chip(
                    second_x,
                    chip_y,
                    &second.name,
                    trait_color(&second.category),
                );
            }
        }
    } else if kaiju.traits.len() > 1 {
        draw_trait_chip(x + CARD_WIDTH - 72.0, chip_y, "+1", dark::TEXT_SECONDARY);
    }

    if !kaiju.alive {
        draw_rectangle(
            x,
            y,
            CARD_WIDTH,
            CARD_HEIGHT,
            Color::new(0.0, 0.0, 0.0, 0.58),
        );
        draw_text_centered(
            "DECEASED",
            x + CARD_WIDTH / 2.0,
            y + CARD_HEIGHT / 2.0,
            FONT_MEDIUM,
            dark::DEAD,
        );
        return None;
    }

    let select_rect = Rect::new(x + 12.0, y + CARD_HEIGHT - 30.0, 96.0, 24.0);
    let record_rect = Rect::new(x + CARD_WIDTH - 108.0, y + CARD_HEIGHT - 30.0, 96.0, 24.0);
    if draw_button(select_rect, "SELECT", accent, true) {
        return Some(CardAction::Select);
    }
    if draw_button(record_rect, "RECORD", accent, true) {
        return Some(CardAction::ViewDetails);
    }

    None
}

fn draw_micro_stat(x: f32, y: f32, label: &str, value: i32, max: i32, color: Color) {
    draw_ui_text(label, x, y, FONT_TINY, dark::TEXT_MUTED);
    draw_progress_bar(
        Rect::new(x + 34.0, y - 8.0, 112.0, 5.0),
        value as f32 / max as f32,
        color,
    );
    draw_text_right(
        &value.to_string(),
        x + 206.0,
        y,
        FONT_TINY,
        dark::TEXT_PRIMARY,
    );
}

fn card_accent(kaiju: &Kaiju, state: CardState) -> Color {
    if matches!(state, CardState::Dead) {
        return dark::DEAD;
    }
    if matches!(state, CardState::Selected) {
        return dark::WARNING;
    }

    if let Some(trait_def) = kaiju.traits.first() {
        return trait_color(&trait_def.category);
    }

    if matches!(state, CardState::Hovered) {
        dark::ACCENT
    } else {
        Color::new(0.22, 0.46, 0.56, 1.0)
    }
}

fn rarity_label(kaiju: &Kaiju) -> &'static str {
    if kaiju.generation >= 3 {
        "DEEP LINEAGE"
    } else if kaiju.traits.iter().any(|trait_def| trait_def.power >= 15) {
        "RARE TRAIT"
    } else if kaiju.tournaments_won > 0 {
        "PROVEN"
    } else if kaiju.traits.len() >= 2 {
        "ADAPTED"
    } else {
        "BASELINE"
    }
}

fn rarity_color(kaiju: &Kaiju) -> Color {
    match rarity_label(kaiju) {
        "DEEP LINEAGE" => dark::WARNING,
        "RARE TRAIT" => dark::MUTATION,
        "PROVEN" => dark::POSITIVE,
        "ADAPTED" => dark::ACCENT,
        _ => dark::TEXT_SECONDARY,
    }
}
