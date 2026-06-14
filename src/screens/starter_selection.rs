//! Starter Selection Screen
//! Allows the user to choose their starting Kaiju element.

use crate::ui::assets::AssetManager;
use crate::ui::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

struct StarterOption<'a> {
    name: &'a str,
    element: &'a str,
    specialty: &'a str,
    image: &'a str,
    accent: Color,
}

pub async fn draw_starter_selection(assets: &AssetManager) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    draw_background(sw, sh);

    draw_header(sw);

    let content = Rect::new(48.0, 112.0, sw - 96.0, sh - 176.0);
    draw_panel(content, "STARTER DOSSIERS");
    draw_ui_text(
        "Choose the first kaiju in your documented lineage.",
        content.x + 22.0,
        content.y + 58.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let options = [
        StarterOption {
            name: "Ignis",
            element: "Fire",
            specialty: "High attack pressure",
            image: "kaiju_fire_elemental_1768091138860.png",
            accent: dark::ATK_COLOR,
        },
        StarterOption {
            name: "Glacies",
            element: "Ice",
            specialty: "Heavy defensive shell",
            image: "kaiju_ice_elemental_1768091156648.png",
            accent: dark::DEF_COLOR,
        },
        StarterOption {
            name: "Volt",
            element: "Electric",
            specialty: "Fast first-strike tempo",
            image: "kaiju_electric_elemental_1768091175509.png",
            accent: dark::SPD_COLOR,
        },
    ];

    let gap = 18.0;
    let card_w = ((content.w - 44.0 - gap * 2.0) / 3.0).clamp(210.0, 310.0);
    let card_h = (content.h - 116.0).clamp(350.0, 440.0);
    let total_w = card_w * 3.0 + gap * 2.0;
    let mut x = content.x + (content.w - total_w) / 2.0;
    let y = content.y + 86.0;

    for option in options {
        if draw_starter_card(Rect::new(x, y, card_w, card_h), &option, assets) {
            return Some(UiAction::SelectStarter(option.element.to_string()));
        }
        x += card_w + gap;
    }

    let back_rect = Rect::new(sw / 2.0 - 80.0, sh - 48.0, 160.0, 36.0);
    if draw_button(back_rect, "BACK", dark::TEXT_SECONDARY, true) {
        return Some(UiAction::GoToMenu);
    }

    None
}

fn draw_header(sw: f32) {
    draw_rectangle(0.0, 0.0, sw, 78.0, Color::new(0.025, 0.035, 0.045, 0.98));
    draw_line(0.0, 78.0, sw, 78.0, 1.0, dark::BORDER);
    draw_ui_text(
        "KAIJU BREEDING SIMULATOR",
        34.0,
        34.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text("STARTER SELECTION", 34.0, 60.0, FONT_SMALL, dark::ACCENT);
}

fn draw_starter_card(rect: Rect, option: &StarterOption, assets: &AssetManager) -> bool {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    let border = if hovered { option.accent } else { dark::BORDER };

    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.045, 0.070, 0.095, 0.94),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, border);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        2.0,
        Color::new(option.accent.r, option.accent.g, option.accent.b, 0.72),
    );

    let portrait = Rect::new(rect.x + 14.0, rect.y + 14.0, rect.w - 28.0, rect.h * 0.52);
    draw_rectangle(
        portrait.x,
        portrait.y,
        portrait.w,
        portrait.h,
        Color::new(0.02, 0.04, 0.06, 1.0),
    );
    if let Some(tex) = assets.get_texture(option.image) {
        draw_texture_ex(
            tex,
            portrait.x,
            portrait.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(portrait.w, portrait.h)),
                ..Default::default()
            },
        );
    } else {
        draw_text_centered(
            "NO IMAGE",
            portrait.x + portrait.w / 2.0,
            portrait.y + portrait.h / 2.0 + 6.0,
            FONT_SMALL,
            dark::TEXT_MUTED,
        );
    }
    draw_rectangle_lines(
        portrait.x,
        portrait.y,
        portrait.w,
        portrait.h,
        1.0,
        dark::BORDER,
    );

    let text_y = portrait.y + portrait.h + 36.0;
    draw_ui_text(
        option.name,
        rect.x + 18.0,
        text_y,
        FONT_LARGE,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        option.element,
        rect.x + 18.0,
        text_y + 28.0,
        FONT_MEDIUM,
        option.accent,
    );
    draw_ui_text(
        option.specialty,
        rect.x + 18.0,
        text_y + 56.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let button = Rect::new(rect.x + 18.0, rect.y + rect.h - 50.0, rect.w - 36.0, 34.0);
    if draw_button(button, "SELECT", option.accent, true) {
        return true;
    }

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
