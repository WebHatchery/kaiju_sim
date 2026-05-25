//! Marketplace Screen
//! V2-facing exchange screen. The local MVP remains fully playable without it.

use crate::server_bridge::{self, MarketplaceItem};
use crate::ui::assets::AssetManager;
use crate::ui::*;
use macroquad::prelude::*;

/// State for marketplace screen.
pub struct MarketplaceState {
    pub items: Vec<MarketplaceItem>,
    pub loaded: bool,
    pub error: Option<String>,
}

impl Default for MarketplaceState {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            loaded: false,
            error: None,
        }
    }
}

impl MarketplaceState {
    pub fn load(&mut self) {
        if self.loaded {
            return;
        }

        match server_bridge::list_marketplace() {
            Ok(items) => {
                self.items = items;
                self.loaded = true;
                self.error = None;
            }
            Err(e) => {
                self.error = Some(e);
                self.loaded = true;
            }
        }
    }
}

pub fn draw_marketplace(
    state: &mut MarketplaceState,
    player_gold: i64,
    assets: &AssetManager,
) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    draw_background(sw, sh);

    draw_header(sw, player_gold);

    let content = Rect::new(48.0, 104.0, sw - 96.0, sh - 164.0);
    let left = Rect::new(content.x, content.y, content.w * 0.36, content.h);
    let right = Rect::new(
        left.x + left.w + PANEL_GAP,
        content.y,
        content.w - left.w - PANEL_GAP,
        content.h,
    );

    draw_panel(left, "EXCHANGE STATUS");
    draw_text(
        "V2 NETWORK FEATURE",
        left.x + 20.0,
        left.y + 66.0,
        FONT_MEDIUM,
        dark::ACCENT,
    );
    draw_text(
        "Real-player trading and external breeding access are intentionally kept out of the V1 local loop.",
        left.x + 20.0,
        left.y + 104.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    draw_text(
        "Local play already supports training, arena battles, breeding, and permanent kaiju history records.",
        left.x + 20.0,
        left.y + 136.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    if draw_button(
        Rect::new(left.x + 20.0, left.y + left.h - 58.0, 170.0, 36.0),
        "BACK TO LAB",
        dark::ACCENT,
        true,
    ) {
        return Some(UiAction::GoToLaboratory);
    }

    draw_panel(right, "CATALOG PREVIEW");
    if !state.loaded {
        draw_text(
            "Catalog connection is dormant for the MVP build.",
            right.x + 20.0,
            right.y + 62.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        draw_text(
            "The saved kaiju roster is the source of truth for V1.",
            right.x + 20.0,
            right.y + 90.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        return None;
    }

    if let Some(ref err) = state.error {
        draw_text(
            &format!("Connection unavailable: {}", err),
            right.x + 20.0,
            right.y + 62.0,
            FONT_SMALL,
            dark::NEGATIVE,
        );
        return None;
    }

    let mut x = right.x + 18.0;
    let y = right.y + 48.0;
    let card_w = 200.0;
    let card_h = (right.h - 72.0).min(320.0);
    for item in state.items.iter().take(4) {
        if let Some(action) =
            draw_market_card(Rect::new(x, y, card_w, card_h), item, player_gold, assets)
        {
            return Some(action);
        }
        x += card_w + 14.0;
    }

    None
}

fn draw_header(sw: f32, player_gold: i64) {
    draw_rectangle(
        0.0,
        0.0,
        sw,
        TOP_BAR_H,
        Color::new(0.025, 0.035, 0.045, 0.98),
    );
    draw_line(0.0, TOP_BAR_H, sw, TOP_BAR_H, 1.0, dark::BORDER);
    draw_text(
        "KAIJU BREEDING SIMULATOR",
        24.0,
        31.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text("MARKETPLACE", 24.0, 56.0, FONT_SMALL, dark::ACCENT);
    draw_text_right(
        &format!("Gold: {}", player_gold),
        sw - 32.0,
        42.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
}

fn draw_market_card(
    rect: Rect,
    item: &MarketplaceItem,
    player_gold: i64,
    assets: &AssetManager,
) -> Option<UiAction> {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.045, 0.070, 0.095, 0.94),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER);

    let portrait = Rect::new(rect.x + 10.0, rect.y + 10.0, rect.w - 20.0, rect.h * 0.46);
    draw_rectangle(
        portrait.x,
        portrait.y,
        portrait.w,
        portrait.h,
        Color::new(0.02, 0.04, 0.06, 1.0),
    );
    let filename = item.image_url.split('/').last().unwrap_or("unknown.png");
    if let Some(tex) = assets.get_texture(filename) {
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
    }
    draw_rectangle_lines(
        portrait.x,
        portrait.y,
        portrait.w,
        portrait.h,
        1.0,
        dark::BORDER,
    );

    let y = portrait.y + portrait.h + 28.0;
    draw_text(
        &item.name,
        rect.x + 14.0,
        y,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        &format!("Power {}", item.stats.power_level()),
        rect.x + 14.0,
        y + 24.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    let can_afford = player_gold >= item.price as i64;
    draw_text(
        &format!("{} gold", item.price),
        rect.x + 14.0,
        y + 50.0,
        FONT_SMALL,
        if can_afford {
            dark::ACCENT
        } else {
            dark::NEGATIVE
        },
    );

    let button = Rect::new(rect.x + 14.0, rect.y + rect.h - 46.0, rect.w - 28.0, 32.0);
    if draw_button(button, "RESERVE", dark::ACCENT, can_afford) {
        return Some(UiAction::PurchaseKaiju(item.id.clone()));
    }

    None
}
