//! Particle system for visual effects.

use macroquad::prelude::*;

use crate::data::random_range_f32;

/// A single particle
#[derive(Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub color: Color,
    pub active: bool,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, lifetime: f32, size: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            lifetime,
            max_lifetime: lifetime,
            size,
            color,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active {
            return;
        }

        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            self.active = false;
            return;
        }

        self.position += self.velocity * dt;
        self.velocity *= 0.98; // Drag
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }

        let alpha = (self.lifetime / self.max_lifetime).clamp(0.0, 1.0);
        let color = Color::new(self.color.r, self.color.g, self.color.b, alpha);
        let size = self.size * alpha;

        draw_circle(self.position.x, self.position.y, size, color);
    }

    pub fn is_alive(&self) -> bool {
        self.active && self.lifetime > 0.0
    }
}

/// Particle emitter configuration
#[derive(Clone)]
pub struct EmitterConfig {
    pub position: Vec2,
    pub emit_rate: f32,
    pub particle_lifetime: f32,
    pub velocity_min: Vec2,
    pub velocity_max: Vec2,
    pub size: f32,
    pub color: Color,
    pub count: u32,
}

impl Default for EmitterConfig {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            emit_rate: 10.0,
            particle_lifetime: 1.0,
            velocity_min: Vec2::new(-50.0, -50.0),
            velocity_max: Vec2::new(50.0, 50.0),
            size: 5.0,
            color: WHITE,
            count: 20,
        }
    }
}

/// Particle system with pooling
pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new(500)
    }
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    /// Spawn particles from a burst config
    pub fn spawn_burst(&mut self, config: &EmitterConfig) {
        for _ in 0..config.count {
            if self.particles.len() >= self.max_particles {
                // Recycle dead particles
                if let Some(dead) = self.particles.iter_mut().find(|p| !p.is_alive()) {
                    dead.position = config.position;
                    dead.velocity = Vec2::new(
                        random_range_f32(config.velocity_min.x, config.velocity_max.x),
                        random_range_f32(config.velocity_min.y, config.velocity_max.y),
                    );
                    dead.lifetime = config.particle_lifetime;
                    dead.max_lifetime = config.particle_lifetime;
                    dead.size = config.size;
                    dead.color = config.color;
                    dead.active = true;
                }
            } else {
                let velocity = Vec2::new(
                    random_range_f32(config.velocity_min.x, config.velocity_max.x),
                    random_range_f32(config.velocity_min.y, config.velocity_max.y),
                );

                self.particles.push(Particle::new(
                    config.position,
                    velocity,
                    config.particle_lifetime,
                    config.size,
                    config.color,
                ));
            }
        }
    }

    /// Update all particles
    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
    }

    /// Draw all active particles
    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }

    /// Get active particle count
    pub fn active_count(&self) -> usize {
        self.particles.iter().filter(|p| p.is_alive()).count()
    }

    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
    }
}

/// Preset particle effects
pub mod presets {
    use super::*;
    use crate::ui::colors::dark;

    /// Hit spark effect
    pub fn hit_spark(position: Vec2) -> EmitterConfig {
        EmitterConfig {
            position,
            emit_rate: 0.0,
            particle_lifetime: 0.3,
            velocity_min: Vec2::new(-100.0, -100.0),
            velocity_max: Vec2::new(100.0, 100.0),
            size: 4.0,
            color: WHITE,
            count: 15,
        }
    }

    /// Fire burst effect
    pub fn fire_burst(position: Vec2) -> EmitterConfig {
        EmitterConfig {
            position,
            emit_rate: 0.0,
            particle_lifetime: 0.5,
            velocity_min: Vec2::new(-60.0, -100.0),
            velocity_max: Vec2::new(60.0, -20.0),
            size: 8.0,
            color: Color::new(1.0, 0.5, 0.1, 1.0),
            count: 25,
        }
    }

    /// Ice shatter effect
    pub fn ice_shatter(position: Vec2) -> EmitterConfig {
        EmitterConfig {
            position,
            emit_rate: 0.0,
            particle_lifetime: 0.6,
            velocity_min: Vec2::new(-80.0, -80.0),
            velocity_max: Vec2::new(80.0, 80.0),
            size: 6.0,
            color: Color::new(0.6, 0.9, 1.0, 1.0),
            count: 20,
        }
    }

    /// Electric zap effect
    pub fn electric_zap(position: Vec2) -> EmitterConfig {
        EmitterConfig {
            position,
            emit_rate: 0.0,
            particle_lifetime: 0.25,
            velocity_min: Vec2::new(-120.0, -120.0),
            velocity_max: Vec2::new(120.0, 120.0),
            size: 3.0,
            color: Color::new(0.4, 0.7, 1.0, 1.0),
            count: 30,
        }
    }

    /// Healing aura effect
    pub fn healing_aura(position: Vec2) -> EmitterConfig {
        EmitterConfig {
            position,
            emit_rate: 0.0,
            particle_lifetime: 1.0,
            velocity_min: Vec2::new(-20.0, -40.0),
            velocity_max: Vec2::new(20.0, -80.0),
            size: 5.0,
            color: dark::POSITIVE,
            count: 15,
        }
    }
}
