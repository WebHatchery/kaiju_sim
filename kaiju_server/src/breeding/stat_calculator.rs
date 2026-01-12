//! Stat Calculator
//! Implements stat inheritance with constraints and generation scaling.

use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};
use super::{BreedingConfig, StatRoll, HybridElement, StatBonus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaijuStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub energy: i32,
}

impl KaijuStats {
    pub fn new(hp: i32, attack: i32, defense: i32, speed: i32, energy: i32) -> Self {
        Self { hp, attack, defense, speed, energy }
    }
    
    pub fn total(&self) -> i32 {
        self.hp + self.attack + self.defense + self.speed + self.energy
    }
    
    pub fn to_map(&self) -> HashMap<String, i32> {
        let mut map = HashMap::new();
        map.insert("hp".to_string(), self.hp);
        map.insert("attack".to_string(), self.attack);
        map.insert("defense".to_string(), self.defense);
        map.insert("speed".to_string(), self.speed);
        map.insert("energy".to_string(), self.energy);
        map
    }
    
    pub fn from_map(map: &HashMap<String, i32>) -> Self {
        Self {
            hp: *map.get("hp").unwrap_or(&100),
            attack: *map.get("attack").unwrap_or(&50),
            defense: *map.get("defense").unwrap_or(&50),
            speed: *map.get("speed").unwrap_or(&50),
            energy: *map.get("energy").unwrap_or(&50),
        }
    }
}

pub struct StatCalculator {
    config: BreedingConfig,
}

impl StatCalculator {
    pub fn new(config: BreedingConfig) -> Self {
        Self { config }
    }
    
    pub fn calculate_offspring_stats<R: Rng>(
        &self,
        parent_a: &KaijuStats,
        parent_b: &KaijuStats,
        generation: u32,
        hybrid: Option<&HybridElement>,
        stabilizer: bool,
        rng: &mut R,
    ) -> (KaijuStats, HashMap<String, StatRoll>) {
        let mut rolls = HashMap::new();
        let c = &self.config.stats;
        
        let hp = self.calc_stat("hp", parent_a.hp, parent_b.hp, generation, c.floors.hp, c.base_caps.hp, c.hard_ceilings.hp, stabilizer, rng, &mut rolls);
        let attack = self.calc_stat("attack", parent_a.attack, parent_b.attack, generation, c.floors.attack, c.base_caps.attack, c.hard_ceilings.attack, stabilizer, rng, &mut rolls);
        let defense = self.calc_stat("defense", parent_a.defense, parent_b.defense, generation, c.floors.defense, c.base_caps.defense, c.hard_ceilings.defense, stabilizer, rng, &mut rolls);
        let speed = self.calc_stat("speed", parent_a.speed, parent_b.speed, generation, c.floors.speed, c.base_caps.speed, c.hard_ceilings.speed, stabilizer, rng, &mut rolls);
        let energy = self.calc_stat("energy", parent_a.energy, parent_b.energy, generation, c.floors.energy, c.base_caps.energy, c.hard_ceilings.energy, stabilizer, rng, &mut rolls);
        
        let mut stats = KaijuStats { hp, attack, defense, speed, energy };
        
        if let Some(h) = hybrid {
            self.apply_hybrid(&mut stats, h, &mut rolls);
        }
        
        (stats, rolls)
    }
    
    fn calc_stat<R: Rng>(&self, name: &str, a: i32, b: i32, gen: u32, floor: i32, base_cap: i32, hard_cap: i32, stabilizer: bool, rng: &mut R, rolls: &mut HashMap<String, StatRoll>) -> i32 {
        let base = (a + b) as f32 / 2.0;
        let g = gen as f32;
        let dim = 1.0 / (1.0 + (g / 20.0));
        let gen_mult = (1.0 + g * self.config.stats.generation_power_creep) * dim;
        let variance = if stabilizer { rng.gen_range(1.0..=self.config.stats.variance_max) } else { rng.gen_range(self.config.stats.variance_min..=self.config.stats.variance_max) };
        let raw = base * gen_mult * variance;
        let mut caps = Vec::new();
        let floored = if raw < floor as f32 { caps.push(format!("floor:{}", floor)); floor as f32 } else { raw };
        let soft_cap = (base_cap as f32 * (1.0 + gen as f32 * self.config.stats.soft_ceiling_scaling)) as i32;
        let soft_capped = if floored > soft_cap as f32 { caps.push(format!("soft:{}", soft_cap)); soft_cap as f32 } else { floored };
        let final_val = if soft_capped > hard_cap as f32 { caps.push(format!("hard:{}", hard_cap)); hard_cap } else { soft_capped as i32 };
        rolls.insert(name.to_string(), StatRoll { roll: variance, formula: format!("({} + {}) / 2 * {:.3} * {:.3}", a, b, gen_mult, variance), base_value: base, gen_multiplier: gen_mult, variance, result: final_val, caps_applied: caps });
        final_val
    }
    
    fn apply_hybrid(&self, stats: &mut KaijuStats, h: &HybridElement, rolls: &mut HashMap<String, StatRoll>) {
        match &h.primary_stat_bonus {
            StatBonus::Speed(b) => { stats.speed += b; if let Some(r) = rolls.get_mut("speed") { r.result = stats.speed; } }
            StatBonus::Attack(b) => { stats.attack += b; if let Some(r) = rolls.get_mut("attack") { r.result = stats.attack; } }
            StatBonus::Defense(b) => { stats.defense += b; if let Some(r) = rolls.get_mut("defense") { r.result = stats.defense; } }
            StatBonus::Hp(b) => { stats.hp += b; if let Some(r) = rolls.get_mut("hp") { r.result = stats.hp; } }
            StatBonus::Evasion(_) => {}
            StatBonus::Multiple(bonuses) => { for (n, b) in bonuses { match n.as_str() { "speed" => { stats.speed += b; } "attack" => { stats.attack += b; } "defense" => { stats.defense += b; } "hp" => { stats.hp += b; } _ => {} } } }
        }
    }
    
    pub fn calculate_gestation_hours(&self, gen: u32, power: i32) -> f32 {
        let base = self.config.timing.gestation_min_hours;
        let max = self.config.timing.gestation_max_hours;
        let f = ((gen as f32 / 10.0).min(1.0) + (power as f32 / 100.0).min(1.0)) / 2.0;
        base + (max - base) * f
    }
    
    pub fn calculate_maturation_hours(&self, gen: u32) -> f32 {
        let base = self.config.timing.maturation_min_hours;
        let max = self.config.timing.maturation_max_hours;
        base + (max - base) * (gen as f32 / 15.0).min(1.0)
    }
}
