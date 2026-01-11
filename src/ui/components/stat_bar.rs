//! StatBar component - horizontal stat visualization.

use macroquad::prelude::*;
use crate::ui::colors::{dark, StatType, stat_color};
use crate::ui::typography::*;

/// Stat bar with animation state
pub struct StatBarState {
    current: f32,
    target: f32,
    animation_time: f32,
}

impl StatBarState {
    pub fn new(value: i32) -> Self {
        Self {
            current: value as f32,
            target: value as f32,
            animation_time: 0.0,
        }
    }
    
    pub fn set_value(&mut self, value: i32) {
        if (self.target - value as f32).abs() > 0.01 {
            self.target = value as f32;
            self.animation_time = 0.0;
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        if (self.current - self.target).abs() > 0.01 {
            self.animation_time += dt;
            let progress = (self.animation_time / 0.3).min(1.0);
            self.current = lerp(self.current, self.target, progress);
        }
    }
    
    pub fn display_value(&self) -> f32 {
        self.current
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Draw a stat bar with label and value
pub fn draw_stat_bar(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    stat_type: StatType,
    current: i32,
    max: i32,
    label: &str,
) {
    let color = stat_color(stat_type);
    let fill_ratio = (current as f32 / max as f32).clamp(0.0, 1.0);
    
    // Background
    draw_rectangle(x, y, width, height, dark::PANEL);
    
    // Fill
    draw_rectangle(x, y, width * fill_ratio, height, color);
    
    // Border
    draw_rectangle_lines(x, y, width, height, 1.0, dark::BORDER);
    
    // Label (left)
    draw_text(label, x + 5.0, y + height - 5.0, FONT_SMALL, dark::TEXT_PRIMARY);
    
    // Value (right)
    let value_text = format!("{}/{}", current, max);
    draw_text_right(&value_text, x + width - 5.0, y + height - 5.0, FONT_SMALL, dark::TEXT_PRIMARY);
}

/// Draw compact stat bar (no label)
pub fn draw_stat_bar_compact(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    current: i32,
    max: i32,
    color: Color,
) {
    let fill_ratio = (current as f32 / max as f32).clamp(0.0, 1.0);
    
    draw_rectangle(x, y, width, height, dark::PANEL);
    draw_rectangle(x, y, width * fill_ratio, height, color);
    draw_rectangle_lines(x, y, width, height, 1.0, dark::BORDER);
}
