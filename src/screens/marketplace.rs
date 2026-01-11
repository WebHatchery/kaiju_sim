//! Marketplace Screen
//! Browse and purchase Kaiju from the server-controlled catalog.

use macroquad::prelude::*;
use crate::ui::*;
use crate::ui::assets::AssetManager;
use crate::server_bridge::{self, MarketplaceItem};

/// State for marketplace screen
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
    assets: &AssetManager
) -> Option<UiAction> {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Load data if not loaded
    state.load();

    // Background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, dark::BACKGROUND);

    // Title
    draw_text_centered("Marketplace", screen_w / 2.0, 60.0, FONT_TITLE, dark::TEXT_PRIMARY);
    
    // Gold display
    let gold_text = format!("Gold: {}", player_gold);
    draw_text_right(&gold_text, screen_w - 30.0, 60.0, FONT_MEDIUM, dark::ACCENT);

    // Error handling
    if let Some(ref err) = state.error {
        draw_text_centered(err, screen_w / 2.0, screen_h / 2.0, FONT_MEDIUM, dark::NEGATIVE);
        
        // Back button
        if draw_back_button(screen_w, screen_h) {
            return Some(UiAction::GoToLaboratory);
        }
        return None;
    }

    // Items grid
    let card_w = 280.0;
    let card_h = 400.0;
    let gap = 30.0;
    let items_count = state.items.len() as f32;
    let total_w = (card_w * items_count) + (gap * (items_count - 1.0).max(0.0));
    let start_x = (screen_w - total_w) / 2.0;
    let start_y = 120.0;

    let mouse = mouse_position();

    for (i, item) in state.items.iter().enumerate() {
        let x = start_x + (card_w + gap) * i as f32;
        let y = start_y;
        
        // Card
        draw_rectangle(x, y, card_w, card_h, dark::SURFACE);
        draw_rectangle_lines(x, y, card_w, card_h, 2.0, dark::BORDER);

        // Image placeholder
        let img_h = 180.0;
        draw_rectangle(x + 10.0, y + 10.0, card_w - 20.0, img_h, dark::PANEL);
        
        // Try to get texture
        let filename = item.image_url.split('/').last().unwrap_or("unknown.png");
        if let Some(tex) = assets.get_texture(filename) {
            draw_texture_ex(
                tex,
                x + 10.0,
                y + 10.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(card_w - 20.0, img_h)),
                    ..Default::default()
                },
            );
        }

        // Name
        let content_y = y + img_h + 30.0;
        draw_text_centered(&item.name, x + card_w / 2.0, content_y, FONT_LARGE, dark::TEXT_PRIMARY);
        
        // Description
        draw_text_centered(&item.description, x + card_w / 2.0, content_y + 30.0, FONT_SMALL, dark::TEXT_SECONDARY);
        
        // Stats summary
        let stats_y = content_y + 60.0;
        let stats_text = format!("HP:{} ATK:{} DEF:{} SPD:{}", 
            item.stats.hp, item.stats.attack, item.stats.defense, item.stats.speed);
        draw_text_centered(&stats_text, x + card_w / 2.0, stats_y, FONT_TINY, dark::TEXT_SECONDARY);

        // Price
        let price_y = stats_y + 30.0;
        let price_text = format!("{} Gold", item.price);
        let can_afford = player_gold >= item.price as i64;
        let price_color = if can_afford { dark::ACCENT } else { dark::NEGATIVE };
        draw_text_centered(&price_text, x + card_w / 2.0, price_y, FONT_MEDIUM, price_color);

        // Buy button
        let btn_y = y + card_h - 50.0;
        let btn_x = x + 20.0;
        let btn_w = card_w - 40.0;
        let btn_h = 40.0;
        
        let btn_hovered = mouse.0 >= btn_x && mouse.0 <= btn_x + btn_w 
            && mouse.1 >= btn_y && mouse.1 <= btn_y + btn_h;
        
        let btn_color = if !can_afford {
            dark::TEXT_MUTED
        } else if btn_hovered {
            dark::BUTTON_HOVER
        } else {
            dark::BUTTON_BG
        };
        
        draw_rectangle(btn_x, btn_y, btn_w, btn_h, btn_color);
        draw_text_centered("Buy", x + card_w / 2.0, btn_y + 26.0, FONT_MEDIUM, dark::TEXT_PRIMARY);

        if can_afford && btn_hovered && is_mouse_button_pressed(MouseButton::Left) {
            return Some(UiAction::PurchaseKaiju(item.id.clone()));
        }
    }

    // Back button
    if draw_back_button(screen_w, screen_h) {
        return Some(UiAction::GoToLaboratory);
    }

    None
}

fn draw_back_button(screen_w: f32, screen_h: f32) -> bool {
    let btn_w = 150.0;
    let btn_h = 40.0;
    let btn_x = screen_w / 2.0 - btn_w / 2.0;
    let btn_y = screen_h - 60.0;
    let mouse = mouse_position();
    let hovered = mouse.0 >= btn_x && mouse.0 <= btn_x + btn_w && mouse.1 >= btn_y && mouse.1 <= btn_y + btn_h;
    
    let bg = if hovered { dark::BUTTON_HOVER } else { dark::BUTTON_BG };
    draw_rectangle(btn_x, btn_y, btn_w, btn_h, bg);
    draw_text_centered("Back", screen_w / 2.0, btn_y + 26.0, FONT_MEDIUM, dark::TEXT_PRIMARY);
    
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
