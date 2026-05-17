//! KaijuCard component - compact kaiju display.

use crate::data::Kaiju;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

/// Card visual state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Normal,
    Hovered,
    Selected,
    Dead,
}

/// Card interaction result
#[derive(Debug, Clone, Copy)]
pub enum CardAction {
    Select,
    ViewDetails,
}

/// Draw a kaiju card and return action if interacted
pub fn draw_kaiju_card(
    x: f32,
    y: f32,
    kaiju: &Kaiju,
    state: CardState,
    assets: &AssetManager,
) -> Option<CardAction> {
    let mouse = mouse_position();
    let is_hovered =
        mouse.0 >= x && mouse.0 <= x + CARD_WIDTH && mouse.1 >= y && mouse.1 <= y + CARD_HEIGHT;

    let effective_state = if !kaiju.alive {
        CardState::Dead
    } else if state == CardState::Selected {
        CardState::Selected
    } else if is_hovered {
        CardState::Hovered
    } else {
        CardState::Normal
    };

    // Premium Background (Dark Glass style)
    let bg_color = match effective_state {
        CardState::Dead => Color::new(0.05, 0.05, 0.05, 0.9),
        CardState::Selected => Color::new(0.15, 0.15, 0.20, 1.0),
        CardState::Hovered => Color::new(0.12, 0.12, 0.15, 1.0),
        CardState::Normal => Color::new(0.08, 0.08, 0.10, 0.95), // Deep dark slightly transparent
    };
    draw_rectangle(x, y, CARD_WIDTH, CARD_HEIGHT, bg_color);

    // Border (Subtle glow for selected)
    let border_color = match effective_state {
        CardState::Selected => dark::ACCENT,
        CardState::Hovered => Color::new(0.6, 0.6, 0.6, 0.5),
        CardState::Dead => dark::DEAD,
        CardState::Normal => Color::new(0.3, 0.3, 0.3, 0.3), // Subtle border
    };
    let border_width = if effective_state == CardState::Selected {
        2.0
    } else {
        1.0
    };
    draw_rectangle_lines(x, y, CARD_WIDTH, CARD_HEIGHT, border_width, border_color);

    // Portrait (SQUARE)
    let portrait_size = CARD_WIDTH - 20.0;

    let mut drawn = false;
    if let Some(uri) = &kaiju.image_uri {
        // Fallback or real texture
        let key = assets.get_filename_from_url(uri);
        if let Some(tex) = assets.get_texture(&key) {
            draw_texture_ex(
                tex,
                x + 10.0,
                y + 10.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(portrait_size, portrait_size)),
                    ..Default::default()
                },
            );
            drawn = true;
        }
    }

    if !drawn {
        // Placeholder rectangle
        draw_rectangle(
            x + 10.0,
            y + 10.0,
            portrait_size,
            portrait_size,
            Color::new(0.15, 0.15, 0.15, 1.0),
        );
        draw_text_centered(
            "?",
            x + 10.0 + portrait_size / 2.0,
            y + 10.0 + portrait_size / 2.0,
            FONT_LARGE,
            GRAY,
        );
    }

    // Generation badge (Top Left overlay)
    draw_rectangle(
        x + 10.0,
        y + 10.0,
        50.0,
        20.0,
        Color::new(0.0, 0.0, 0.0, 0.7),
    );
    draw_text(
        &format!("Gen {}", kaiju.generation),
        x + 15.0,
        y + 24.0,
        FONT_TINY,
        WHITE,
    );

    // Trophy Badge (Top Right)
    if kaiju.tournaments_won > 0 {
        draw_rectangle(x + 10.0 + portrait_size - 60.0, y + 10.0, 60.0, 20.0, GOLD);
        draw_text(
            &format!("Wins: {}", kaiju.tournaments_won),
            x + 10.0 + portrait_size - 55.0,
            y + 24.0,
            FONT_TINY,
            BLACK,
        );
    }

    // Info Section
    let info_start_y = y + portrait_size + 20.0;

    // Name
    draw_text(&kaiju.name, x + 10.0, info_start_y, FONT_MEDIUM, WHITE);

    // Stats Compact
    let stats_y = info_start_y + 25.0;
    let stat_spacing = 15.0;

    draw_stat_row(x + 10.0, stats_y, "HP", kaiju.stats.hp, dark::HP_COLOR);
    draw_stat_row(
        x + 10.0,
        stats_y + stat_spacing,
        "ATK",
        kaiju.stats.attack,
        dark::ATK_COLOR,
    );
    draw_stat_row(
        x + 10.0,
        stats_y + stat_spacing * 2.0,
        "DEF",
        kaiju.stats.defense,
        dark::DEF_COLOR,
    );
    draw_stat_row(
        x + 10.0,
        stats_y + stat_spacing * 3.0,
        "SPD",
        kaiju.stats.speed,
        dark::SPD_COLOR,
    );

    // Status overlay for dead kaiju
    if !kaiju.alive {
        draw_rectangle(
            x,
            y,
            CARD_WIDTH,
            CARD_HEIGHT,
            Color::new(0.0, 0.0, 0.0, 0.6),
        );
        draw_text_centered(
            "DECEASED",
            x + CARD_WIDTH / 2.0,
            y + CARD_HEIGHT / 2.0,
            FONT_MEDIUM,
            dark::DEAD,
        );
    }

    // Buttons row at bottom
    let btn_y = y + CARD_HEIGHT - 35.0;
    let btn_w = (CARD_WIDTH - 25.0) / 2.0;

    let select_hovered = is_mouse_in_rect(x + 5.0, btn_y, btn_w, 28.0, mouse);
    let view_hovered = is_mouse_in_rect(x + btn_w + 15.0, btn_y, btn_w, 28.0, mouse);

    // Select button
    let select_bg = if select_hovered {
        dark::ACCENT
    } else {
        dark::BUTTON_BG
    };
    draw_rectangle(x + 5.0, btn_y, btn_w, 28.0, select_bg);
    draw_text_centered(
        "Select",
        x + 5.0 + btn_w / 2.0,
        btn_y + 19.0,
        FONT_SMALL,
        WHITE,
    );

    // View button
    let view_bg = if view_hovered {
        dark::ACCENT
    } else {
        dark::PANEL
    };
    draw_rectangle(x + btn_w + 15.0, btn_y, btn_w, 28.0, view_bg);
    draw_text_centered(
        "View",
        x + btn_w + 15.0 + btn_w / 2.0,
        btn_y + 19.0,
        FONT_SMALL,
        WHITE,
    );

    // Handle clicks
    if is_mouse_button_pressed(MouseButton::Left) && kaiju.alive {
        if select_hovered {
            return Some(CardAction::Select);
        }
        if view_hovered {
            return Some(CardAction::ViewDetails);
        }
    }

    None
}

/// Helper for compact stat row
fn draw_stat_row(x: f32, y: f32, label: &str, value: i32, color: Color) {
    // Label
    draw_text(label, x, y, FONT_TINY, GRAY);

    // Bar
    let bar_x = x + 30.0;
    let bar_w = 80.0;
    let max = 200; // soft cap for visualization
    let fill = (value as f32 / max as f32).min(1.0) * bar_w;

    draw_rectangle(bar_x, y - 6.0, bar_w, 6.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(bar_x, y - 6.0, fill, 6.0, color);

    // Value
    draw_text(&value.to_string(), bar_x + bar_w + 5.0, y, FONT_TINY, WHITE);
}

/// Draw mini stat bar
fn draw_stat_mini(x: f32, y: f32, label: &str, value: i32, max: i32, color: Color) {
    let bar_x = x + 30.0;
    let bar_w = CARD_WIDTH - 50.0;
    let bar_h = 12.0;

    draw_text(label, x, y + 10.0, FONT_TINY, dark::TEXT_SECONDARY);

    // Background
    draw_rectangle(bar_x, y, bar_w, bar_h, dark::PANEL);

    // Fill
    let fill_w = (value as f32 / max as f32).min(1.0) * bar_w;
    draw_rectangle(bar_x, y, fill_w, bar_h, color);

    // Value
    draw_text_right(
        &value.to_string(),
        x + CARD_WIDTH - 15.0,
        y + 10.0,
        FONT_TINY,
        dark::TEXT_PRIMARY,
    );
}

fn is_mouse_in_rect(x: f32, y: f32, w: f32, h: f32, mouse: (f32, f32)) -> bool {
    mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h
}
