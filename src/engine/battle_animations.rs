//! Battle animation system.

use macroquad::prelude::*;

/// Animation state for battle turns
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    Idle,
    WindUp { progress: f32 },
    Strike { progress: f32 },
    Impact { progress: f32 },
    Recoil { progress: f32 },
    Return { progress: f32 },
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
            attack_offset: Vec2::ZERO,
        }
    }

    /// Start attack animation
    pub fn start_attack(&mut self) {
        self.state = AnimationState::WindUp { progress: 0.0 };
        self.attack_offset = self.defender_base - self.attacker_base;
    }

    /// Update animation state
    pub fn update(&mut self, dt: f32) {
        let scaled_dt = dt * self.time_scale;

        match self.state.clone() {
            AnimationState::Idle => {}

            AnimationState::WindUp { progress } => {
                let new_progress = progress + scaled_dt / 0.3;
                if new_progress >= 1.0 {
                    self.state = AnimationState::Strike { progress: 0.0 };
                } else {
                    self.state = AnimationState::WindUp {
                        progress: new_progress,
                    };
                    // Pull back slightly
                    self.attacker_pos =
                        self.attacker_base - self.attack_offset.normalize() * 20.0 * new_progress;
                }
            }

            AnimationState::Strike { progress } => {
                let new_progress = progress + scaled_dt / 0.2;
                if new_progress >= 1.0 {
                    self.state = AnimationState::Impact { progress: 0.0 };
                } else {
                    self.state = AnimationState::Strike {
                        progress: new_progress,
                    };
                    // Lunge forward
                    let t = ease_out_quad(new_progress);
                    self.attacker_pos = self.attacker_base + self.attack_offset * 0.6 * t;
                }
            }

            AnimationState::Impact { progress } => {
                let new_progress = progress + scaled_dt / 0.1;
                if new_progress >= 1.0 {
                    self.state = AnimationState::Recoil { progress: 0.0 };
                } else {
                    self.state = AnimationState::Impact {
                        progress: new_progress,
                    };
                    // Screen shake
                }
            }

            AnimationState::Recoil { progress } => {
                let new_progress = progress + scaled_dt / 0.3;
                if new_progress >= 1.0 {
                    self.state = AnimationState::Return { progress: 0.0 };
                } else {
                    self.state = AnimationState::Recoil {
                        progress: new_progress,
                    };
                    // Defender recoils
                    let t = ease_out_quad(new_progress);
                    self.defender_pos = self.defender_base + Vec2::new(30.0 * (1.0 - t), 0.0);
                }
            }

            AnimationState::Return { progress } => {
                let new_progress = progress + scaled_dt / 0.4;
                if new_progress >= 1.0 {
                    self.state = AnimationState::Idle;
                    self.attacker_pos = self.attacker_base;
                    self.defender_pos = self.defender_base;
                } else {
                    self.state = AnimationState::Return {
                        progress: new_progress,
                    };
                    // Return to base
                    let t = ease_out_quad(new_progress);
                    self.attacker_pos = self.attacker_base + self.attack_offset * 0.6 * (1.0 - t);
                }
            }

            AnimationState::Victory | AnimationState::Defeat => {}
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

    /// Get screen shake offset
    pub fn get_shake_offset(&self) -> Vec2 {
        if let AnimationState::Impact { progress } = self.state {
            let intensity = 10.0 * (1.0 - progress);
            Vec2::new(
                (::rand::random::<f32>() - 0.5) * 2.0 * intensity,
                (::rand::random::<f32>() - 0.5) * 2.0 * intensity,
            )
        } else {
            Vec2::ZERO
        }
    }
}

/// Ease out quadratic function
fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Floating damage number
pub struct DamageNumber {
    pub value: i32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color,
    pub is_critical: bool,
}

impl DamageNumber {
    pub fn new(value: i32, position: Vec2, is_critical: bool, is_healing: bool) -> Self {
        let color = if is_healing {
            Color::new(0.3, 0.9, 0.3, 1.0)
        } else if is_critical {
            Color::new(1.0, 0.9, 0.2, 1.0)
        } else {
            WHITE
        };

        Self {
            value,
            position,
            velocity: Vec2::new(0.0, -60.0),
            lifetime: 1.5,
            max_lifetime: 1.5,
            color,
            is_critical,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
        self.position += self.velocity * dt;
        self.velocity *= 0.95;
    }

    pub fn draw(&self) {
        if self.lifetime <= 0.0 {
            return;
        }

        let alpha = (self.lifetime / self.max_lifetime).clamp(0.0, 1.0);
        let color = Color::new(self.color.r, self.color.g, self.color.b, alpha);

        let font_size = if self.is_critical { 32.0 } else { 24.0 };
        let text = self.value.to_string();

        // Shadow
        draw_text(
            &text,
            self.position.x + 2.0,
            self.position.y + 2.0,
            font_size,
            Color::new(0.0, 0.0, 0.0, alpha * 0.5),
        );
        draw_text(&text, self.position.x, self.position.y, font_size, color);
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

/// Damage number manager
pub struct DamageNumberManager {
    numbers: Vec<DamageNumber>,
}

impl Default for DamageNumberManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DamageNumberManager {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }

    pub fn spawn(&mut self, value: i32, position: Vec2, is_critical: bool, is_healing: bool) {
        self.numbers
            .push(DamageNumber::new(value, position, is_critical, is_healing));
    }

    pub fn update(&mut self, dt: f32) {
        for num in &mut self.numbers {
            num.update(dt);
        }
        self.numbers.retain(|n| n.is_alive());
    }

    pub fn draw(&self) {
        for num in &self.numbers {
            num.draw();
        }
    }
}
