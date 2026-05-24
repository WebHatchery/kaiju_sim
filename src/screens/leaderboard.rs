use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_leaderboard(state: &GameState) -> Option<UiAction> {
    let sw = screen_width();
    clear_background(dark::BACKGROUND);
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("LOCAL LEADERBOARD", 20.0, 40.0, FONT_LARGE, WHITE);

    if draw_button(sw - 105.0, 15.0, 85.0, 30.0, "< Back") {
        return Some(UiAction::GoToLaboratory);
    }

    let mut ranked = state.roster.iter().collect::<Vec<_>>();
    ranked.sort_by_key(|kaiju| std::cmp::Reverse((kaiju.tournaments_won, kaiju.battle_rating())));

    let mut y = 100.0;
    for (index, kaiju) in ranked.iter().enumerate() {
        draw_rectangle(28.0, y - 26.0, sw - 56.0, 46.0, dark::SURFACE);
        draw_text(
            &format!(
                "#{:<2} {:<18} Rating {:<4} Wins {:<3} Gen {}",
                index + 1,
                kaiju.name,
                kaiju.battle_rating(),
                kaiju.tournaments_won,
                kaiju.generation
            ),
            44.0,
            y,
            FONT_MEDIUM,
            if index == 0 {
                dark::WARNING
            } else {
                dark::TEXT_PRIMARY
            },
        );
        y += 56.0;
    }

    if is_key_pressed(KeyCode::Escape) {
        return Some(UiAction::GoToLaboratory);
    }
    None
}

fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    draw_rectangle(
        x,
        y,
        w,
        h,
        if hovered {
            dark::BUTTON_HOVER
        } else {
            dark::BUTTON_BG
        },
    );
    draw_text_centered(text, x + w / 2.0, y + h / 2.0 + 5.0, FONT_SMALL, WHITE);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
