//! Main menu screen.

use crate::state::persistence::save_exists;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use macroquad::prelude::*;

/// Draw main menu and return action if button pressed.
pub fn draw_main_menu(assets: &AssetManager) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();

    draw_title_background(assets, sw, sh);

    let button_w = 260.0;
    let button_h = 44.0;
    let gap = 14.0;
    let start_x = sw / 2.0 - button_w / 2.0;
    let start_y = (sh * 0.70).min(sh - 240.0).max(390.0);
    let has_save = save_exists();

    if draw_menu_button(start_x, start_y, button_w, button_h, "New Game", true) {
        return Some(UiAction::NewGame);
    }

    if draw_menu_button(
        start_x,
        start_y + (button_h + gap),
        button_w,
        button_h,
        "Load Game",
        has_save,
    ) {
        return Some(UiAction::ContinueGame);
    }

    if draw_menu_button(
        start_x,
        start_y + (button_h + gap) * 2.0,
        button_w,
        button_h,
        "Settings",
        true,
    ) {
        return Some(UiAction::GoToSettings);
    }

    if draw_menu_button(
        start_x,
        start_y + (button_h + gap) * 3.0,
        button_w,
        button_h,
        "Exit Game",
        true,
    ) {
        return Some(UiAction::ExitGame);
    }

    draw_text(
        "v0.1.0",
        16.0,
        sh - 16.0,
        FONT_TINY,
        Color::new(0.65, 0.70, 0.78, 0.72),
    );

    None
}

fn draw_title_background(assets: &AssetManager, sw: f32, sh: f32) {
    clear_background(dark::BACKGROUND);

    let Some(texture) = assets.get_texture("title_page") else {
        return;
    };

    let tw = texture.width();
    let th = texture.height();
    let scale = (sw / tw).max(sh / th);
    let draw_w = tw * scale;
    let draw_h = th * scale;
    let x = (sw - draw_w) / 2.0;
    let y = (sh - draw_h) / 2.0;

    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(draw_w, draw_h)),
            ..Default::default()
        },
    );

    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.08));
}

fn draw_menu_button(x: f32, y: f32, w: f32, h: f32, text: &str, enabled: bool) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h && enabled;

    let bg = if !enabled {
        Color::new(0.02, 0.03, 0.05, 0.42)
    } else if hovered {
        Color::new(0.18, 0.37, 0.78, 0.68)
    } else {
        Color::new(0.02, 0.04, 0.08, 0.54)
    };
    let border = if hovered {
        Color::new(0.38, 0.64, 1.0, 0.95)
    } else {
        Color::new(0.30, 0.50, 0.78, 0.58)
    };
    let text_color = if enabled {
        Color::new(0.88, 0.94, 1.0, 0.96)
    } else {
        Color::new(0.45, 0.50, 0.56, 0.70)
    };

    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.5, border);
    draw_text_centered(
        text,
        x + w / 2.0,
        y + h / 2.0 + 7.0,
        FONT_MEDIUM,
        text_color,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
