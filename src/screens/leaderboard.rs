use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub fn draw_leaderboard(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Leaderboard);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let top_h = 178.0;
    let hero = Rect::new(c.x, c.y, c.w, top_h);
    let table = Rect::new(c.x, c.y + top_h + PANEL_GAP, c.w, c.h - top_h - PANEL_GAP);

    let mut ranked = state.roster.iter().collect::<Vec<_>>();
    ranked.sort_by_key(|kaiju| std::cmp::Reverse((kaiju.tournaments_won, kaiju.battle_rating())));

    draw_records_hero(hero, state, ranked.first().copied(), assets);
    if let Some(action) = draw_records_table(table, ranked, assets) {
        return Some(action);
    }

    None
}

fn draw_records_hero(
    rect: Rect,
    state: &GameState,
    champion: Option<&crate::data::Kaiju>,
    assets: &AssetManager,
) {
    draw_panel_with_accent(rect, "LOCAL RECORDS", dark::WARNING);
    draw_ui_text(
        "SEASON: CLASSIFIED",
        rect.x + 22.0,
        rect.y + 62.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_ui_text(
        "PROVEN BLOODLINES",
        rect.x + 22.0,
        rect.y + 100.0,
        FONT_TITLE,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        "Ranked by arena wins, then rating.",
        rect.x + 24.0,
        rect.y + 132.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    draw_metric_tile(
        Rect::new(rect.x + rect.w - 430.0, rect.y + 58.0, 120.0, 76.0),
        "BATTLES",
        &state.player.stats.total_battles_fought.to_string(),
        dark::WARNING,
    );
    draw_metric_tile(
        Rect::new(rect.x + rect.w - 296.0, rect.y + 58.0, 120.0, 76.0),
        "HATCHES",
        &state.player.stats.total_kaiju_bred.to_string(),
        dark::ACCENT,
    );

    if let Some(champion) = champion {
        let champ = Rect::new(rect.x + rect.w - 156.0, rect.y + 42.0, 116.0, 116.0);
        draw_portrait(champ, champion, assets);
        draw_status_pill(
            Rect::new(rect.x + rect.w - 168.0, rect.y + 132.0, 140.0, 28.0),
            "CURRENT CHAMPION",
            dark::WARNING,
        );
    }
}

fn draw_records_table(
    rect: Rect,
    ranked: Vec<&crate::data::Kaiju>,
    assets: &AssetManager,
) -> Option<UiAction> {
    draw_panel_with_accent(rect, "ARCHIVED RANKINGS", dark::ACCENT);
    draw_table_header(
        rect.x + 16.0,
        rect.y + 48.0,
        rect.w - 32.0,
        &[
            ("RANK", 18.0),
            ("KAIJU", 92.0),
            ("BADGE", 332.0),
            ("GEN", 478.0),
            ("WINS", 568.0),
            ("RATING", 666.0),
            ("EVENTS", 782.0),
        ],
    );

    let mut y = rect.y + 102.0;
    for (index, kaiju) in ranked.iter().take(8).enumerate() {
        if y + 56.0 > rect.y + rect.h {
            break;
        }
        let row = Rect::new(rect.x + 16.0, y - 34.0, rect.w - 32.0, 64.0);
        if draw_record_row(row, index + 1, kaiju, assets) {
            return Some(UiAction::ViewKaijuDetails(kaiju.id));
        }
        y += 72.0;
    }

    if ranked.is_empty() {
        draw_empty_state(
            rect,
            "No records yet",
            "Arena results and hatches appear here.",
        );
    }

    None
}

fn draw_record_row(
    rect: Rect,
    rank: usize,
    kaiju: &crate::data::Kaiju,
    assets: &AssetManager,
) -> bool {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    let rank_color = if rank == 1 {
        dark::WARNING
    } else {
        dark::TEXT_SECONDARY
    };
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hovered {
            Color::new(0.050, 0.088, 0.110, 0.96)
        } else {
            Color::new(0.030, 0.055, 0.074, 0.90)
        },
    );
    draw_rectangle(
        rect.x,
        rect.y,
        2.0,
        rect.h,
        Color::new(rank_color.r, rank_color.g, rank_color.b, 0.70),
    );
    draw_ui_text(
        &rank.to_string(),
        rect.x + 18.0,
        rect.y + 38.0,
        FONT_MEDIUM,
        rank_color,
    );
    draw_portrait(
        Rect::new(rect.x + 72.0, rect.y + 8.0, 48.0, 48.0),
        kaiju,
        assets,
    );
    draw_ui_text(
        &ellipsize(&kaiju.name, 190.0, FONT_SMALL),
        rect.x + 132.0,
        rect.y + 28.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!("Legacy #{}", kaiju.token_id),
        rect.x + 132.0,
        rect.y + 48.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_status_pill(
        Rect::new(rect.x + 330.0, rect.y + 18.0, 118.0, 28.0),
        badge_label(rank, kaiju),
        badge_color(rank, kaiju),
    );
    draw_ui_text(
        &kaiju.generation.to_string(),
        rect.x + 480.0,
        rect.y + 38.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &kaiju.tournaments_won.to_string(),
        rect.x + 572.0,
        rect.y + 38.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &kaiju.battle_rating().to_string(),
        rect.x + 670.0,
        rect.y + 38.0,
        FONT_SMALL,
        dark::WARNING,
    );
    draw_ui_text(
        &kaiju.history.len().to_string(),
        rect.x + 786.0,
        rect.y + 38.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn badge_label(rank: usize, kaiju: &crate::data::Kaiju) -> &'static str {
    if rank == 1 {
        "CHAMPION"
    } else if kaiju.tournaments_won > 0 {
        "PROVEN"
    } else if kaiju.generation > 0 {
        "LINEAGE"
    } else {
        "BASELINE"
    }
}

fn badge_color(rank: usize, kaiju: &crate::data::Kaiju) -> Color {
    if rank == 1 {
        dark::WARNING
    } else if kaiju.tournaments_won > 0 {
        dark::POSITIVE
    } else if kaiju.generation > 0 {
        dark::ACCENT
    } else {
        dark::TEXT_SECONDARY
    }
}
