//! Local MVP arena screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_arena_screen(
    state: &GameState,
    assets: &AssetManager,
    entry_fee: i64,
) -> Option<UiAction> {
    let sw = screen_width();
    clear_background(dark::BACKGROUND);

    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("ARENA", 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);

    if draw_button(sw - 105.0, 15.0, 85.0, 30.0, "< Back", dark::BUTTON_BG) {
        return Some(UiAction::GoToLaboratory);
    }

    draw_text(
        &format!(
            "Pick a kaiju for an instant seeded fight. Entry {} gold. Wins pay out and grant more XP.",
            entry_fee
        ),
        24.0,
        86.0,
        FONT_NORMAL,
        dark::TEXT_SECONDARY,
    );

    let mut y = 120.0;
    for kaiju in state.roster.iter().filter(|k| k.alive) {
        if let Some(action) = draw_fighter_row(24.0, y, sw - 48.0, kaiju, assets) {
            return Some(action);
        }
        y += 104.0;
    }

    None
}

fn draw_fighter_row(
    x: f32,
    y: f32,
    w: f32,
    kaiju: &crate::data::Kaiju,
    assets: &AssetManager,
) -> Option<UiAction> {
    draw_rectangle(x, y, w, 86.0, dark::SURFACE);
    draw_rectangle_lines(x, y, w, 86.0, 1.0, dark::BORDER);

    draw_portrait(x + 12.0, y + 10.0, 66.0, kaiju, assets);
    draw_text(&kaiju.name, x + 92.0, y + 30.0, FONT_MEDIUM, WHITE);
    draw_text(
        &format!(
            "Rating {} | Wins {} | HP {} ATK {} DEF {} SPD {}",
            kaiju.battle_rating(),
            kaiju.tournaments_won,
            kaiju.stats.hp,
            kaiju.stats.attack,
            kaiju.stats.defense,
            kaiju.stats.speed
        ),
        x + 92.0,
        y + 58.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    if draw_button(x + w - 140.0, y + 24.0, 112.0, 38.0, "Fight", dark::WARNING) {
        return Some(UiAction::StartBattle(kaiju.id));
    }

    None
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
        draw_text_centered("?", x + size / 2.0, y + size / 2.0 + 8.0, FONT_LARGE, GRAY);
    }
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str, accent: Color) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    draw_rectangle(x, y, w, h, if hovered { accent } else { dark::BUTTON_BG });
    draw_rectangle_lines(x, y, w, h, 1.0, if hovered { WHITE } else { dark::BORDER });
    draw_text_centered(text, x + w / 2.0, y + h / 2.0 + 6.0, FONT_NORMAL, WHITE);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
