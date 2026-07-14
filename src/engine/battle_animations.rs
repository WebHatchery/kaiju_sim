//! Battle animation system.

use macroquad::prelude::*;
use macroquad_toolkit::fx::{FloatingText, FloatingTextLayer, ScreenShake};
use macroquad_toolkit::math::ease_out_quad;
use macroquad_toolkit::timing::Timeline;

/// A phase within a single attack animation, driven by a [`Timeline`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPhase {
    WindUp,
    Strike,
    Impact,
    Recoil,
    Return,
}

/// Animation state for battle turns
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Attacking(AnimationPhase),
    Victory,
    Defeat,
}

impl AnimationState {
    pub fn is_complete(&self) -> bool {
        matches!(
            self,
            AnimationState::Idle | AnimationState::Victory | AnimationState::Defeat
        )
    }
}

/// Battle animator for turn-by-turn combat
pub struct BattleAnimator {
    pub state: AnimationState,
    pub attacker_pos: Vec2,
    pub defender_pos: Vec2,
    pub attacker_base: Vec2,
    pub defender_base: Vec2,
    pub time_scale: f32,
    /// Trauma-based screen shake, triggered on impact. Read via `shake.offset()`.
    pub shake: ScreenShake,
    timeline: Timeline<AnimationPhase>,
    attack_offset: Vec2,
}

impl Default for BattleAnimator {
    fn default() -> Self {
        Self::new(Vec2::new(150.0, 300.0), Vec2::new(650.0, 300.0))
    }
}

impl BattleAnimator {
    pub fn new(attacker_base: Vec2, defender_base: Vec2) -> Self {
        Self {
            state: AnimationState::Idle,
            attacker_pos: attacker_base,
            defender_pos: defender_base,
            attacker_base,
            defender_base,
            time_scale: 1.0,
            shake: ScreenShake::new(10.0),
            timeline: Timeline::new(Vec::new()),
            attack_offset: Vec2::ZERO,
        }
    }

    /// Start attack animation
    pub fn start_attack(&mut self) {
        self.timeline = Timeline::new(vec![
            (AnimationPhase::WindUp, 0.3),
            (AnimationPhase::Strike, 0.2),
            (AnimationPhase::Impact, 0.1),
            (AnimationPhase::Recoil, 0.3),
            (AnimationPhase::Return, 0.4),
        ]);
        self.state = AnimationState::Attacking(AnimationPhase::WindUp);
        self.attack_offset = self.defender_base - self.attacker_base;
    }

    /// Update animation state
    pub fn update(&mut self, dt: f32) {
        self.shake.update(dt);

        let AnimationState::Attacking(previous_phase) = self.state else {
            return;
        };

        self.timeline.advance(dt * self.time_scale);
        let Some((phase, progress)) = self.timeline.current() else {
            self.state = AnimationState::Idle;
            self.attacker_pos = self.attacker_base;
            self.defender_pos = self.defender_base;
            return;
        };
        let phase = *phase;

        if phase == AnimationPhase::Impact && previous_phase != AnimationPhase::Impact {
            // Short, sharp shake on the impact frame.
            self.shake.shake(1.0, 0.15);
        }

        self.state = AnimationState::Attacking(phase);
        match phase {
            AnimationPhase::WindUp => {
                // Pull back slightly
                self.attacker_pos =
                    self.attacker_base - self.attack_offset.normalize() * 20.0 * progress;
            }
            AnimationPhase::Strike => {
                // Lunge forward
                let t = ease_out_quad(progress);
                self.attacker_pos = self.attacker_base + self.attack_offset * 0.6 * t;
            }
            AnimationPhase::Impact => {}
            AnimationPhase::Recoil => {
                // Defender recoils
                let t = ease_out_quad(progress);
                self.defender_pos = self.defender_base + Vec2::new(30.0 * (1.0 - t), 0.0);
            }
            AnimationPhase::Return => {
                // Return to base
                let t = ease_out_quad(progress);
                self.attacker_pos = self.attacker_base + self.attack_offset * 0.6 * (1.0 - t);
            }
        }
    }

    /// Set victory state
    pub fn set_victory(&mut self, attacker_won: bool) {
        if attacker_won {
            self.state = AnimationState::Victory;
        } else {
            self.state = AnimationState::Defeat;
        }
    }
}

/// Color and font size to use for a damage number, based on its kind.
fn damage_number_style(is_critical: bool, is_healing: bool) -> (Color, f32) {
    let color = if is_healing {
        Color::new(0.3, 0.9, 0.3, 1.0)
    } else if is_critical {
        Color::new(1.0, 0.9, 0.2, 1.0)
    } else {
        WHITE
    };
    let font_size = if is_critical { 32.0 } else { 24.0 };
    (color, font_size)
}

/// Manages rising, fading damage/heal numbers for combat feedback.
pub struct DamageNumberManager {
    layer: FloatingTextLayer,
}

impl Default for DamageNumberManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DamageNumberManager {
    pub fn new() -> Self {
        let mut layer = FloatingTextLayer::new();
        layer.default_lifetime = 1.5;
        layer.default_rise_speed = 60.0;
        layer.drag = 0.95;
        Self { layer }
    }

    pub fn spawn(&mut self, value: i32, position: Vec2, is_critical: bool, is_healing: bool) {
        let (color, font_size) = damage_number_style(is_critical, is_healing);
        self.layer.push(FloatingText::new(
            value.to_string(),
            position,
            color,
            font_size,
            self.layer.default_lifetime,
            self.layer.default_rise_speed,
        ));
    }

    pub fn update(&mut self, dt: f32) {
        self.layer.update(dt);
    }

    pub fn draw(&self) {
        self.layer.draw();
    }
}
