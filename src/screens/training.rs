//! Local MVP training screen.

use crate::data::{Kaiju, TrainingFocus};
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

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
    let left_w = c.w * 0.68;
    let list_panel = Rect::new(c.x, c.y, left_w, c.h);
    let status_panel = Rect::new(c.x + left_w + PANEL_GAP, c.y, c.w - left_w - PANEL_GAP, c.h);
    draw_panel(list_panel, "TRAINING ROSTER");
    draw_panel(status_panel, "PROGRAM STATUS");

    draw_text(
        &format!("Training cost: {} gold per session", training_cost),
        list_panel.x + 14.0,
        list_panel.y + 52.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let mut y = list_panel.y + 78.0;
    for kaiju in state.roster.iter().filter(|k| k.alive) {
        if let Some(action) = draw_training_row(
            Rect::new(list_panel.x + 14.0, y, list_panel.w - 28.0, 96.0),
            kaiju,
            assets,
            training_cost,
            state.player.gold >= training_cost,
        ) {
            return Some(action);
        }
        y += 110.0;
    }

    draw_program_status(status_panel);
    None
}

fn draw_training_row(
    rect: Rect,
    kaiju: &Kaiju,
    assets: &AssetManager,
    cost: i64,
    can_afford: bool,
) -> Option<UiAction> {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.060, 0.082, 0.94),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER);
    draw_portrait(
        Rect::new(rect.x + 10.0, rect.y + 10.0, 76.0, 76.0),
        kaiju,
        assets,
    );
    draw_text(
        &kaiju.name,
        rect.x + 102.0,
        rect.y + 28.0,
        FONT_MEDIUM,
        dark::ACCENT,
    );
    draw_text(
        &format!(
            "GEN {} | XP {} | RATING {} | COST {}",
            kaiju.generation,
            kaiju.experience,
            kaiju.battle_rating(),
            cost
        ),
        rect.x + 102.0,
        rect.y + 52.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        &format!(
            "HP {}  ATK {}  DEF {}  SPD {}",
            kaiju.stats.hp, kaiju.stats.attack, kaiju.stats.defense, kaiju.stats.speed
        ),
        rect.x + 102.0,
        rect.y + 76.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );

    let button_w = 86.0;
    let start_x = rect.x + rect.w - (button_w * 4.0 + 8.0 * 3.0) - 12.0;
    for (index, focus) in TrainingFocus::all().iter().enumerate() {
        let b = Rect::new(
            start_x + index as f32 * (button_w + 8.0),
            rect.y + 30.0,
            button_w,
            34.0,
        );
        if draw_button(b, focus.stat_label(), focus_color(*focus), can_afford) {
            return Some(UiAction::TrainKaiju {
                kaiju_id: kaiju.id,
                focus: *focus,
            });
        }
    }
    None
}

fn draw_program_status(rect: Rect) {
    let programs = [
        ("ENDURANCE", "HP growth and recovery drills", dark::HP_COLOR),
        (
            "POWER",
            "Attack pressure and strike output",
            dark::ATK_COLOR,
        ),
        (
            "GUARD",
            "Defense control and armored posture",
            dark::DEF_COLOR,
        ),
        (
            "REFLEX",
            "Speed timing and initiative work",
            dark::SPD_COLOR,
        ),
    ];
    let mut y = rect.y + 58.0;
    for (name, desc, color) in programs {
        draw_text(name, rect.x + 16.0, y, FONT_SMALL, color);
        draw_text(
            desc,
            rect.x + 16.0,
            y + 22.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
        y += 64.0;
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
