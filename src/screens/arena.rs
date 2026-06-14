//! Arena screen.

use crate::data::Kaiju;
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub fn draw_arena_screen(
    state: &GameState,
    assets: &AssetManager,
    entry_fee: i64,
) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Arena);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let banner_h = 142.0;
    let body_h = (c.h - banner_h - PANEL_GAP * 2.0) * 0.62;
    let roster_h = c.h - banner_h - body_h - PANEL_GAP * 2.0;
    let banner = Rect::new(c.x, c.y, c.w, banner_h);
    let fighter_w = c.w * 0.55;
    let fighter_panel = Rect::new(c.x, c.y + banner_h + PANEL_GAP, fighter_w, body_h);
    let event_panel = Rect::new(
        c.x + fighter_w + PANEL_GAP,
        fighter_panel.y,
        c.w - fighter_w - PANEL_GAP,
        body_h,
    );
    let roster_panel = Rect::new(c.x, fighter_panel.y + body_h + PANEL_GAP, c.w, roster_h);

    draw_arena_banner(banner, state, entry_fee);
    let active = selected_living_subject(state);
    if let Some(action) = draw_active_fighter(
        fighter_panel,
        active,
        assets,
        state.player.gold >= entry_fee,
    ) {
        return Some(action);
    }
    draw_event_panel(event_panel, entry_fee);
    if let Some(action) = draw_fighter_roster(roster_panel, state, assets) {
        return Some(action);
    }

    None
}

fn draw_arena_banner(rect: Rect, state: &GameState, entry_fee: i64) {
    draw_panel_with_accent(rect, "ARENA EVENT", dark::WARNING);
    draw_ui_text(
        "FRIDAY COLOSSEUM",
        rect.x + 22.0,
        rect.y + 72.0,
        FONT_TITLE,
        dark::WARNING,
    );
    draw_ui_text(
        "Fight. Record. Improve.",
        rect.x + 24.0,
        rect.y + 106.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_metric_tile(
        Rect::new(rect.x + rect.w - 410.0, rect.y + 48.0, 122.0, 70.0),
        "ENTRY",
        &format!("{}", entry_fee),
        dark::WARNING,
    );
    draw_metric_tile(
        Rect::new(rect.x + rect.w - 274.0, rect.y + 48.0, 122.0, 70.0),
        "WINS",
        &state.player.stats.total_battles_won.to_string(),
        dark::POSITIVE,
    );
    draw_metric_tile(
        Rect::new(rect.x + rect.w - 138.0, rect.y + 48.0, 122.0, 70.0),
        "SEASON",
        "SECURE",
        dark::ACCENT,
    );
}

fn draw_active_fighter(
    rect: Rect,
    kaiju: Option<&Kaiju>,
    assets: &AssetManager,
    can_pay: bool,
) -> Option<UiAction> {
    draw_panel_with_accent(rect, "COMMITTED FIGHTER", dark::WARNING);
    let Some(kaiju) = kaiju else {
        draw_empty_state(
            rect,
            "No active fighter",
            "Select a living kaiju from the roster.",
        );
        return None;
    };

    let portrait = Rect::new(rect.x + 18.0, rect.y + 56.0, 178.0, 178.0);
    draw_portrait(portrait, kaiju, assets);
    let info_x = portrait.x + portrait.w + 24.0;
    draw_ui_text(
        &ellipsize(&kaiju.name, rect.w - 250.0, FONT_LARGE),
        info_x,
        rect.y + 78.0,
        FONT_LARGE,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "GEN {} | wins {} | rating {}",
            kaiju.generation,
            kaiju.tournaments_won,
            kaiju.battle_rating()
        ),
        info_x,
        rect.y + 106.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_stat_meter(
        info_x,
        rect.y + 146.0,
        rect.w - info_x + rect.x - 22.0,
        "HP",
        kaiju.stats.hp,
        2500,
        dark::HP_COLOR,
    );
    draw_stat_meter(
        info_x,
        rect.y + 174.0,
        rect.w - info_x + rect.x - 22.0,
        "ATK",
        kaiju.stats.attack,
        300,
        dark::ATK_COLOR,
    );
    draw_stat_meter(
        info_x,
        rect.y + 202.0,
        rect.w - info_x + rect.x - 22.0,
        "SPD",
        kaiju.stats.speed,
        300,
        dark::SPD_COLOR,
    );

    draw_ui_text(
        "ARENA NOTICE",
        rect.x + 18.0,
        rect.y + rect.h - 66.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_ui_text(
        "Environment locks on entry.",
        rect.x + 18.0,
        rect.y + rect.h - 40.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    if draw_button(
        Rect::new(rect.x + rect.w - 126.0, rect.y + rect.h - 54.0, 104.0, 34.0),
        "FIGHT",
        dark::WARNING,
        can_pay,
    ) {
        return Some(UiAction::StartBattle(kaiju.id));
    }

    None
}

fn draw_event_panel(rect: Rect, entry_fee: i64) {
    draw_panel_with_accent(rect, "RIVAL BRIEFING", dark::ACCENT);
    draw_ui_text(
        "UNKNOWN AI CONTENDER",
        rect.x + 16.0,
        rect.y + 62.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        "Rival profile follows fighter rating.",
        rect.x + 16.0,
        rect.y + 92.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let y = rect.y + 174.0;
    draw_ui_text("MODIFIERS", rect.x + 16.0, y, FONT_TINY, dark::TEXT_MUTED);
    draw_status_pill(
        Rect::new(rect.x + 16.0, y + 18.0, 108.0, 26.0),
        "SEEDED ARENA",
        dark::ACCENT,
    );
    draw_status_pill(
        Rect::new(rect.x + 136.0, y + 18.0, 104.0, 26.0),
        "RECORDED",
        dark::POSITIVE,
    );
    draw_ui_text(
        &format!("Entry: {} gold.", entry_fee),
        rect.x + 16.0,
        rect.y + rect.h - 28.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
}

fn draw_fighter_roster(rect: Rect, state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    draw_panel_with_accent(rect, "FIGHTER ROSTER", dark::TEXT_SECONDARY);
    let mut x = rect.x + 16.0;
    let y = rect.y + 48.0;
    let row_w = 250.0;
    for kaiju in state.roster.iter().filter(|k| k.alive).take(4) {
        let row = Rect::new(x, y, row_w, rect.h - 62.0);
        if let Some(action) =
            draw_fighter_chip(row, kaiju, assets, state.selected_kaiju == Some(kaiju.id))
        {
            return Some(action);
        }
        x += row_w + 12.0;
    }

    if state.living_count() == 0 {
        draw_empty_state(
            rect,
            "No eligible fighters",
            "The arena requires one living kaiju.",
        );
    }

    None
}

fn draw_fighter_chip(
    rect: Rect,
    kaiju: &Kaiju,
    assets: &AssetManager,
    selected: bool,
) -> Option<UiAction> {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    let color = if selected {
        dark::WARNING
    } else {
        dark::ACCENT
    };
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.030, 0.055, 0.074, 0.92),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(
            color.r,
            color.g,
            color.b,
            if selected || hovered { 0.74 } else { 0.28 },
        ),
    );
    draw_portrait(
        Rect::new(rect.x + 10.0, rect.y + 10.0, 58.0, 58.0),
        kaiju,
        assets,
    );
    draw_ui_text(
        &ellipsize(&kaiju.name, 128.0, FONT_SMALL),
        rect.x + 80.0,
        rect.y + 30.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!("W{} | R{}", kaiju.tournaments_won, kaiju.battle_rating()),
        rect.x + 80.0,
        rect.y + 54.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        return Some(UiAction::SelectKaiju(kaiju.id));
    }
    None
}

fn selected_living_subject(state: &GameState) -> Option<&Kaiju> {
    state
        .selected_kaiju
        .and_then(|id| state.get_kaiju(id))
        .filter(|kaiju| kaiju.alive)
        .or_else(|| state.roster.iter().find(|kaiju| kaiju.alive))
}
