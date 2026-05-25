//! Local MVP arena screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

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
    let list_w = c.w * 0.66;
    let list_panel = Rect::new(c.x, c.y, list_w, c.h);
    let info_panel = Rect::new(c.x + list_w + PANEL_GAP, c.y, c.w - list_w - PANEL_GAP, c.h);
    draw_panel(list_panel, "ARENA FIGHTERS");
    draw_panel(info_panel, "NEXT ARENA FIGHT");

    draw_text(
        &format!(
            "Entry fee: {} gold | Battles are seeded and recorded.",
            entry_fee
        ),
        list_panel.x + 14.0,
        list_panel.y + 52.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let can_pay = state.player.gold >= entry_fee;
    let mut y = list_panel.y + 78.0;
    for kaiju in state.roster.iter().filter(|k| k.alive) {
        if let Some(action) = draw_fighter_row(
            Rect::new(list_panel.x + 14.0, y, list_panel.w - 28.0, 92.0),
            kaiju,
            assets,
            can_pay,
        ) {
            return Some(action);
        }
        y += 106.0;
    }

    draw_arena_info(info_panel, entry_fee);
    None
}

fn draw_fighter_row(
    rect: Rect,
    kaiju: &crate::data::Kaiju,
    assets: &AssetManager,
    can_pay: bool,
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
        Rect::new(rect.x + 10.0, rect.y + 10.0, 72.0, 72.0),
        kaiju,
        assets,
    );
    draw_text(
        &kaiju.name,
        rect.x + 98.0,
        rect.y + 30.0,
        FONT_MEDIUM,
        dark::ACCENT,
    );
    draw_text(
        &format!(
            "RATING {} | WINS {} | HP {} ATK {} DEF {} SPD {}",
            kaiju.battle_rating(),
            kaiju.tournaments_won,
            kaiju.stats.hp,
            kaiju.stats.attack,
            kaiju.stats.defense,
            kaiju.stats.speed
        ),
        rect.x + 98.0,
        rect.y + 58.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    if draw_button(
        Rect::new(rect.x + rect.w - 112.0, rect.y + 29.0, 92.0, 34.0),
        "FIGHT",
        dark::WARNING,
        can_pay,
    ) {
        return Some(UiAction::StartBattle(kaiju.id));
    }
    None
}

fn draw_arena_info(rect: Rect, entry_fee: i64) {
    draw_text(
        "FRIDAY COLOSSEUM",
        rect.x + 16.0,
        rect.y + 64.0,
        FONT_MEDIUM,
        dark::WARNING,
    );
    draw_text(
        "A rotating AI challenge bracket for V1 local play.",
        rect.x + 16.0,
        rect.y + 94.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        &format!("ENTRY: {} gold", entry_fee),
        rect.x + 16.0,
        rect.y + 146.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        "REWARD: gold, XP, and a battle history entry",
        rect.x + 16.0,
        rect.y + 176.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        "V2: real player kaiju matchmaking",
        rect.x + 16.0,
        rect.y + 226.0,
        FONT_SMALL,
        dark::ACCENT,
    );
}
