use macroquad::prelude::*;
use crate::state::GameState;
use crate::server_bridge;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use crate::ui::actions::UiAction;
use crate::ui::spacing::*;
use uuid::Uuid;

static mut LAST_FETCH: f64 = 0.0;
static mut FETCH_ERROR: Option<String> = None;

pub fn draw_tournament_lobby(state: &mut GameState) -> Option<UiAction> {
    let now = get_time();
    
    // Poll Server
    unsafe {
        if now - LAST_FETCH > 2.0 {
            match server_bridge::get_current_tournament() {
                Ok(info) => {
                    FETCH_ERROR = None;
                    state.tournament_status = Some(info);
                }
                Err(e) => {
                    FETCH_ERROR = Some(e);
                }
            }
            LAST_FETCH = now;
        }
    }
    
    clear_background(dark::BACKGROUND);
    
    // Header
    let sw = screen_width();
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("Tournament Lobby", 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);
    
    // Back Button
    let back_btn_w = 100.0;
    if draw_back_button(sw - back_btn_w - 20.0, 15.0) {
        return Some(UiAction::Back);
    }
    
    // Content
    let content_y = 80.0;
    
    // Error Display
    unsafe {
        if let Some(err) = &FETCH_ERROR {
            draw_text(&format!("Connection Error: {}", err), 20.0, content_y, FONT_MEDIUM, RED);
            return None;
        }
    }
    
    if let Some(info) = &state.tournament_status {
        // Status Info
        draw_text(&format!("State: {}", info.state), 20.0, content_y, FONT_MEDIUM, WHITE);
        draw_text(&format!("Round: {}", info.current_round), 200.0, content_y, FONT_MEDIUM, WHITE);
        draw_text(&format!("Participants: {}", info.participants_count), 400.0, content_y, FONT_MEDIUM, WHITE);
        
        let sub_y = content_y + 40.0;
        
        match info.state.as_str() {
            "Registration" => {
                draw_text(&format!("Next Round Starts: {}", info.start_time), 20.0, sub_y, FONT_NORMAL, YELLOW);
                
                // Enrollment Section
                let enroll_y = sub_y + 40.0;
                draw_text("Select Kaiju to Enroll:", 20.0, enroll_y, FONT_MEDIUM, WHITE);
                
                let mut valid_kaiju = state.roster.iter().filter(|k| k.alive).cloned().collect::<Vec<_>>();
                
                let mut k_y = enroll_y + 40.0;
                for k in valid_kaiju {
                    let btn_rect = Rect::new(20.0, k_y, 400.0, 50.0);
                    let mouse = mouse_position();
                    let hovered = btn_rect.contains(vec2(mouse.0, mouse.1));
                    
                    let color = if hovered { dark::BUTTON_HOVER } else { dark::PANEL };
                    draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, color);
                    
                    draw_text(&k.name, btn_rect.x + 10.0, btn_rect.y + 30.0, FONT_MEDIUM, WHITE);
                    draw_text(&format!("Stats: {}", k.stats.power_level()), btn_rect.x + 200.0, btn_rect.y + 30.0, FONT_SMALL, GRAY);
                    
                    // Enroll Button
                    let enroll_success = draw_centered_btn("Enroll", btn_rect.x + 300.0, btn_rect.y + 10.0, 80.0, 30.0);
                    if enroll_success {
                        if let Ok(msg) = server_bridge::enroll_in_tournament(Uuid::parse_str(&state.player.player_id).unwrap_or_default(), k.id) {
                            state.notify(msg, crate::state::game_state::NotificationType::Success);
                        } else {
                            state.notify("Enrollment Failed".to_string(), crate::state::game_state::NotificationType::Error);
                        }
                    }
                    
                    k_y += 60.0;
                }
            },
            "Running" | "Finished" => {
                // Determine layout calculation
                 // We want to visualize matches.
                 // Simple list for now.
                 let mut m_y = sub_y + 10.0;
                 // Group by round
                let current_r = info.current_round;
                draw_text(&format!("Current Brackets (Round {})", current_r), 20.0, m_y, FONT_MEDIUM, WHITE);
                m_y += 30.0;

                for m in &info.matches {
                    if m.round_number != current_r { continue; }
                    
                    draw_rectangle(20.0, m_y, 500.0, 40.0, dark::PANEL);
                    
                    let name_a = m.kaiju_a_name.as_deref().unwrap_or("Waiting...");
                    let name_b = m.kaiju_b_name.as_deref().unwrap_or("Waiting...");
                    
                    let winner = m.winner_id.as_deref();
                    
                    let color_a = if winner == m.kaiju_a_id.as_deref() && winner.is_some() { GREEN } else { WHITE };
                    let color_b = if winner == m.kaiju_b_id.as_deref() && winner.is_some() { GREEN } else { WHITE };
                    
                    draw_text(name_a, 30.0, m_y + 25.0, FONT_NORMAL, color_a);
                    draw_text("VS", 200.0, m_y + 25.0, FONT_SMALL, GRAY);
                    draw_text(name_b, 250.0, m_y + 25.0, FONT_NORMAL, color_b);
                    
                    m_y += 50.0;
                }
                
                if info.state == "Finished" {
                     draw_text("Tournament Complete!", 20.0, m_y + 20.0, FONT_LARGE, GOLD);
                }
            },
            _ => {}
        }
    } else {
        draw_text("Connecting to Tournament Network...", 20.0, content_y + 20.0, FONT_MEDIUM, GRAY);
    }
    
    
    None
}

fn draw_back_button(x: f32, y: f32) -> bool {
    let w = 100.0;
    let h = 30.0;
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    
    let bg = if hovered { dark::BUTTON_HOVER } else { dark::BUTTON_BG };
    draw_rectangle(x, y, w, h, bg);
    draw_text_centered("< Back", x + w / 2.0, y + 20.0, FONT_SMALL, dark::TEXT_PRIMARY);
    
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_centered_btn(text: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    let bg = if hovered { dark::ACCENT } else { dark::BUTTON_BG };
    draw_rectangle(x, y, w, h, bg);
    draw_text_centered(text, x + w/2.0, y + h/1.5, FONT_SMALL, WHITE);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
