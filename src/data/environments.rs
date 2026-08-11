//! Environment definitions and multiplier logic for combat.
//!
//! There are 9 environments that modify trait effectiveness by 15-25%.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Combat environment types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Environment {
    #[default]
    Neutral,
    Storm,
    Volcanic,
    Aquatic,
    Tundra,
    Desert,
    Forest,
    Radiation,
    Void,
}

impl Environment {
    /// Get all environments
    pub fn all() -> Vec<Environment> {
        vec![
            Environment::Neutral,
            Environment::Storm,
            Environment::Volcanic,
            Environment::Aquatic,
            Environment::Tundra,
            Environment::Desert,
            Environment::Forest,
            Environment::Radiation,
            Environment::Void,
        ]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Environment::Neutral => "Neutral",
            Environment::Storm => "Storm",
            Environment::Volcanic => "Volcanic",
            Environment::Aquatic => "Aquatic",
            Environment::Tundra => "Tundra",
            Environment::Desert => "Desert",
            Environment::Forest => "Forest",
            Environment::Radiation => "Radiation",
            Environment::Void => "Void",
        }
    }

    /// Get trait multipliers for this environment
    pub fn get_trait_multipliers(&self) -> HashMap<String, f32> {
        let mut multipliers = HashMap::new();

        match self {
            Environment::Neutral => {
                // No modifiers
            }
            Environment::Storm => {
                multipliers.insert("Electric".to_string(), 1.15);
                multipliers.insert("Wind".to_string(), 1.15);
                multipliers.insert("Fire".to_string(), 0.90);
                multipliers.insert("Water".to_string(), 1.05);
            }
            Environment::Volcanic => {
                multipliers.insert("Fire".to_string(), 1.25);
                multipliers.insert("Earth".to_string(), 1.10);
                multipliers.insert("Ice".to_string(), 0.85);
                multipliers.insert("Water".to_string(), 0.90);
            }
            Environment::Aquatic => {
                multipliers.insert("Water".to_string(), 1.20);
                multipliers.insert("Ice".to_string(), 1.10);
                multipliers.insert("Fire".to_string(), 0.80);
                multipliers.insert("Electric".to_string(), 0.90);
            }
            Environment::Tundra => {
                multipliers.insert("Ice".to_string(), 1.20);
                multipliers.insert("Wind".to_string(), 1.05);
                multipliers.insert("Fire".to_string(), 0.85);
            }
            Environment::Desert => {
                multipliers.insert("Fire".to_string(), 1.15);
                multipliers.insert("Earth".to_string(), 1.10);
                multipliers.insert("Water".to_string(), 0.80);
                multipliers.insert("Ice".to_string(), 0.75);
            }
            Environment::Forest => {
                multipliers.insert("Earth".to_string(), 1.15);
                multipliers.insert("Toxic".to_string(), 1.20);
                multipliers.insert("Wind".to_string(), 1.10);
                multipliers.insert("Fire".to_string(), 1.05); // Can spread
            }
            Environment::Radiation => {
                multipliers.insert("Energy".to_string(), 1.25);
                multipliers.insert("Mutation".to_string(), 1.20);
                multipliers.insert("Toxic".to_string(), 1.10);
            }
            Environment::Void => {
                multipliers.insert("Energy".to_string(), 1.15);
                multipliers.insert("Psychic".to_string(), 1.20);
            }
        }

        multipliers
    }

    /// Get per-turn effects for this environment
    pub fn get_per_turn_effects(&self) -> Vec<PerTurnEffect> {
        match self {
            Environment::Volcanic => vec![PerTurnEffect::Damage { percent: 0.02 }],
            Environment::Radiation => vec![PerTurnEffect::StatusChance {
                effect: "Mutation".to_string(),
                chance: 0.05,
            }],
            Environment::Void => vec![PerTurnEffect::StatSwap { chance: 0.10 }],
            _ => vec![],
        }
    }

    /// Get multiplier for a specific trait name
    pub fn get_trait_multiplier(&self, trait_name: &str) -> f32 {
        self.get_trait_multipliers()
            .get(trait_name)
            .copied()
            .unwrap_or(1.0)
    }
}

/// Per-turn environmental effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerTurnEffect {
    /// Percentage HP damage per turn
    Damage { percent: f32 },
    /// Chance to apply a status effect
    StatusChance { effect: String, chance: f32 },
    /// Chance to swap attack and defense
    StatSwap { chance: f32 },
}

/// Environment definition loaded from JSON (for data-driven design)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentDefinition {
    pub name: String,
    pub trait_multipliers: HashMap<String, f32>,
    pub per_turn_effects: Vec<PerTurnEffect>,
}

/// Environment database
#[derive(Debug, Clone, Default)]
pub struct EnvironmentDatabase {
    pub environments: Vec<EnvironmentDefinition>,
}

impl EnvironmentDatabase {
    /// Get environment by name
    pub fn get(&self, name: &str) -> Option<&EnvironmentDefinition> {
        self.environments.iter().find(|e| e.name == name)
    }
}

#[cfg(test)]
mod tests;
