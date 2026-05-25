//! Kaiju card component matching the dashboard visual standard.

use crate::data::Kaiju;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

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
        x,
        y,
        CARD_WIDTH,
        CARD_HEIGHT,
        Color::new(0.035, 0.060, 0.082, 0.96),
    );
    draw_rectangle(x, y, CARD_WIDTH, 2.0, accent);
    draw_rectangle_lines(x, y, CARD_WIDTH, CARD_HEIGHT, 1.0, accent);

    let portrait = Rect::new(x + 12.0, y + 44.0, CARD_WIDTH - 24.0, 145.0);
    draw_portrait(portrait, kaiju, assets);

    draw_text(&kaiju.name, x + 12.0, y + 28.0, FONT_MEDIUM, accent);
    draw_text_right(
        &format!("GEN {}", kaiju.generation),
        x + CARD_WIDTH - 12.0,
        y + 27.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    let stat_y = y + 208.0;
    draw_stat_row(x + 14.0, stat_y, "HP", kaiju.stats.hp, 2500, dark::HP_COLOR);
    draw_stat_row(
        x + 14.0,
        stat_y + 22.0,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_stat_row(
        x + 14.0,
        stat_y + 44.0,
        "DEF",
        kaiju.stats.defense,
        300,
        dark::DEF_COLOR,
    );
    draw_stat_row(
        x + 14.0,
        stat_y + 66.0,
        "SPD",
        kaiju.stats.speed,
        300,
        dark::SPD_COLOR,
    );

    let trait_summary = kaiju
        .traits
        .first()
        .map(|trait_def| trait_def.name.as_str())
        .unwrap_or("Unrevealed");
    draw_text(
        &format!("TRAITS: {}", trait_summary),
        x + 14.0,
        y + 292.0,
        FONT_TINY,
        accent,
    );

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

    let select_rect = Rect::new(x + 12.0, y + CARD_HEIGHT - 38.0, 98.0, 28.0);
    let record_rect = Rect::new(x + CARD_WIDTH - 110.0, y + CARD_HEIGHT - 38.0, 98.0, 28.0);
    if draw_card_button(select_rect, "SELECT", accent) {
        return Some(CardAction::Select);
    }
    if draw_card_button(record_rect, "RECORD", accent) {
        return Some(CardAction::ViewDetails);
    }

    None
}

fn draw_portrait(rect: Rect, kaiju: &Kaiju, assets: &AssetManager) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.02, 0.04, 0.06, 1.0),
    );
    let mut drawn = false;
    if let Some(uri) = &kaiju.image_uri {
        let key = assets.get_filename_from_url(uri);
        if let Some(tex) = assets.get_texture(&key) {
            draw_texture_ex(
                tex,
                rect.x,
                rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rect.w, rect.h)),
                    ..Default::default()
                },
            );
            drawn = true;
        }
    }
    if !drawn {
        draw_text_centered(
            "?",
            rect.x + rect.w / 2.0,
            rect.y + rect.h / 2.0 + 10.0,
            FONT_LARGE,
            dark::TEXT_MUTED,
        );
    }
}

fn draw_stat_row(x: f32, y: f32, label: &str, value: i32, max: i32, color: Color) {
    draw_text(label, x, y, FONT_TINY, dark::TEXT_SECONDARY);
    let bar_x = x + 34.0;
    let bar_w = 128.0;
    draw_rectangle(
        bar_x,
        y - 7.0,
        bar_w,
        6.0,
        Color::new(0.12, 0.16, 0.20, 1.0),
    );
    draw_rectangle(
        bar_x,
        y - 7.0,
        bar_w * (value as f32 / max as f32).clamp(0.0, 1.0),
        6.0,
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

fn draw_card_button(rect: Rect, label: &str, accent: Color) -> bool {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hovered {
            Color::new(accent.r, accent.g, accent.b, 0.50)
        } else {
            Color::new(0.06, 0.09, 0.12, 0.90)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if hovered { accent } else { dark::BORDER },
    );
    draw_text_centered(
        label,
        rect.x + rect.w / 2.0,
        rect.y + 19.0,
        FONT_TINY,
        dark::TEXT_PRIMARY,
    );
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn card_accent(kaiju: &Kaiju, state: CardState) -> Color {
    if matches!(state, CardState::Dead) {
        return dark::DEAD;
    }
    if matches!(state, CardState::Selected) {
        return dark::ACCENT;
    }

    for trait_def in &kaiju.traits {
        if trait_def.name.contains("Fire") {
            return Color::new(0.95, 0.58, 0.12, 1.0);
        }
        if trait_def.name.contains("Electric") {
            return dark::ACCENT;
        }
        if trait_def.name.contains("Aqua") || trait_def.name.contains("Water") {
            return Color::new(0.22, 0.80, 0.84, 1.0);
        }
        if trait_def.name.contains("Armor") {
            return dark::POSITIVE;
        }
    }

    if matches!(state, CardState::Hovered) {
        dark::ACCENT
    } else {
        Color::new(0.28, 0.42, 0.55, 1.0)
    }
}
