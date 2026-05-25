use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_leaderboard(state: &GameState) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Leaderboard);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    draw_panel(c, "LOCAL LEADERBOARD");
    draw_text(
        "Ranked by arena wins, then battle rating. V2 will include real player kaiju.",
        c.x + 14.0,
        c.y + 52.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_table_header(
        c.x + 14.0,
        c.y + 78.0,
        c.w - 28.0,
        &[
            ("RANK", 16.0),
            ("KAIJU", 100.0),
            ("GEN", 330.0),
            ("WINS", 430.0),
            ("RATING", 540.0),
            ("EVENTS", 660.0),
        ],
    );

    let mut ranked = state.roster.iter().collect::<Vec<_>>();
    ranked.sort_by_key(|kaiju| std::cmp::Reverse((kaiju.tournaments_won, kaiju.battle_rating())));

    let mut y = c.y + 124.0;
    for (index, kaiju) in ranked.iter().enumerate() {
        draw_rectangle(
            c.x + 14.0,
            y - 22.0,
            c.w - 28.0,
            38.0,
            Color::new(0.035, 0.060, 0.082, 0.94),
        );
        let rank_color = if index == 0 {
            dark::WARNING
        } else {
            dark::TEXT_PRIMARY
        };
        draw_text(
            &(index + 1).to_string(),
            c.x + 30.0,
            y,
            FONT_SMALL,
            rank_color,
        );
        draw_text(&kaiju.name, c.x + 114.0, y, FONT_SMALL, dark::ACCENT);
        draw_text(
            &kaiju.generation.to_string(),
            c.x + 344.0,
            y,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text(
            &kaiju.tournaments_won.to_string(),
            c.x + 448.0,
            y,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text(
            &kaiju.battle_rating().to_string(),
            c.x + 554.0,
            y,
            FONT_SMALL,
            dark::WARNING,
        );
        draw_text(
            &kaiju.history.len().to_string(),
            c.x + 678.0,
            y,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        y += 46.0;
    }

    None
}
