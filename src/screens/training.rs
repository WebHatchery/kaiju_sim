//! Local MVP training screen.

use crate::data::{Kaiju, TrainingFocus};
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_training_screen(
    state: &GameState,
    assets: &AssetManager,
    training_cost: i64,
) -> Option<UiAction> {
    let sw = screen_width();
    clear_background(dark::BACKGROUND);

    draw_header("TRAINING RING", sw);
    if draw_button(sw - 105.0, 15.0, 85.0, 30.0, "< Back", dark::BUTTON_BG) {
        return Some(UiAction::GoToLaboratory);
    }

    draw_text(
        &format!(
            "Gold: {} | Cost: {} per session",
            state.player.gold, training_cost
        ),
        24.0,
        86.0,
        FONT_NORMAL,
        dark::TEXT_SECONDARY,
    );

    let mut y = 115.0;
    for kaiju in state.roster.iter().filter(|k| k.alive) {
        if let Some(action) = draw_training_row(24.0, y, sw - 48.0, kaiju, assets) {
            return Some(action);
        }
        y += 118.0;
    }

    if state.living_count() == 0 {
        draw_text_centered(
            "No living kaiju available for training.",
            sw / 2.0,
            screen_height() / 2.0,
            FONT_MEDIUM,
            dark::TEXT_MUTED,
        );
    }

    None
}

fn draw_training_row(
    x: f32,
    y: f32,
    w: f32,
    kaiju: &Kaiju,
    assets: &AssetManager,
) -> Option<UiAction> {
    let h = 96.0;
    draw_rectangle(x, y, w, h, dark::SURFACE);
    draw_rectangle_lines(x, y, w, h, 1.0, dark::BORDER);

    draw_portrait(x + 12.0, y + 12.0, 72.0, kaiju, assets);
    draw_text(
        &kaiju.name,
        x + 100.0,
        y + 30.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        &format!(
            "Gen {} | XP {} | Rating {}",
            kaiju.generation,
            kaiju.experience,
            kaiju.battle_rating()
        ),
        x + 100.0,
        y + 54.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        &format!(
            "HP {}   ATK {}   DEF {}   SPD {}",
            kaiju.stats.hp, kaiju.stats.attack, kaiju.stats.defense, kaiju.stats.speed
        ),
        x + 100.0,
        y + 76.0,
        FONT_SMALL,
        dark::TEXT_MUTED,
    );

    let button_w = 118.0;
    let button_gap = 10.0;
    let start_x = x + w - (button_w * 4.0 + button_gap * 3.0) - 14.0;
    for (index, focus) in TrainingFocus::all().iter().enumerate() {
        let bx = start_x + index as f32 * (button_w + button_gap);
        let label = format!("{} +{}", focus.label(), focus.stat_label());
        if draw_button(bx, y + 29.0, button_w, 38.0, &label, focus_color(*focus)) {
            return Some(UiAction::TrainKaiju {
                kaiju_id: kaiju.id,
                focus: *focus,
            });
        }
    }

    None
}

fn draw_header(title: &str, sw: f32) {
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text(title, 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);
}

fn draw_portrait(x: f32, y: f32, size: f32, kaiju: &Kaiju, assets: &AssetManager) {
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
        draw_text_centered("?", x + size / 2.0, y + size / 2.0 + 8.0, FONT_LARGE, GRAY);
    }
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str, accent: Color) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    let bg = if hovered { accent } else { dark::BUTTON_BG };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, if hovered { WHITE } else { dark::BORDER });
    draw_text_centered(text, x + w / 2.0, y + h / 2.0 + 6.0, FONT_SMALL, WHITE);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn focus_color(focus: TrainingFocus) -> Color {
    match focus {
        TrainingFocus::Endurance => dark::HP_COLOR,
        TrainingFocus::Power => dark::ATK_COLOR,
        TrainingFocus::Guard => dark::DEF_COLOR,
        TrainingFocus::Reflex => dark::SPD_COLOR,
    }
}
