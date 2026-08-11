//! Synergy detection system for trait combinations.
//!
//! Synergies are meta-traits that activate when specific trait combinations are present.

use serde::{Deserialize, Serialize};

use crate::data::traits::{TraitCategory, TraitCondition, TraitInheritance};
use crate::data::Trait;

/// Synergy definition loaded from traits.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyDefinition {
    /// Synergy ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Required trait IDs
    pub required_traits: Vec<String>,
    /// Power bonus
    pub power: i32,
    /// Effect description
    pub effect: String,
}

/// Database of synergy definitions
#[derive(Debug, Clone, Default)]
pub struct SynergyDatabase {
    pub synergies: Vec<SynergyDefinition>,
}

impl SynergyDatabase {
    /// Create from synergy definitions
    pub fn new(synergies: Vec<SynergyDefinition>) -> Self {
        Self { synergies }
    }

    /// Check which synergies are active for a kaiju's traits
    pub fn check_synergies(&self, trait_ids: &[String]) -> Vec<Trait> {
        let mut active_synergies = Vec::new();

        for synergy_def in &self.synergies {
            let has_all_required = synergy_def
                .required_traits
                .iter()
                .all(|req| trait_ids.contains(req));

            if has_all_required {
                active_synergies.push(synergy_def.to_trait());
            }
        }

        active_synergies
    }

    /// Get synergy by ID
    pub fn get(&self, id: &str) -> Option<&SynergyDefinition> {
        self.synergies.iter().find(|s| s.id == id)
    }

    /// Check if a specific synergy is active
    pub fn is_synergy_active(&self, synergy_id: &str, trait_ids: &[String]) -> bool {
        if let Some(synergy) = self.get(synergy_id) {
            synergy
                .required_traits
                .iter()
                .all(|req| trait_ids.contains(req))
        } else {
            false
        }
    }
}

impl SynergyDefinition {
    /// Convert synergy definition to a trait
    pub fn to_trait(&self) -> Trait {
        Trait {
            id: self.id.clone(),
            name: self.name.clone(),
            category: TraitCategory::Synergy,
            power: self.power,
            inheritance: TraitInheritance::Conditional,
            condition: TraitCondition::Simple("Always".to_string()),
            is_hidden: true,
            description: self.effect.clone(),
        }
    }
}

/// Trait incompatibility matrix
#[derive(Debug, Clone, Default)]
pub struct IncompatibilityMatrix {
    /// Pairs of incompatible trait IDs
    pub pairs: Vec<(String, String)>,
}

impl IncompatibilityMatrix {
    /// Create from pairs
    pub fn new(pairs: Vec<(String, String)>) -> Self {
        Self { pairs }
    }

    /// Check if two traits are incompatible
    pub fn are_incompatible(&self, trait_a: &str, trait_b: &str) -> bool {
        self.pairs
            .iter()
            .any(|(a, b)| (a == trait_a && b == trait_b) || (a == trait_b && b == trait_a))
    }

    /// Filter out incompatible traits from a list
    pub fn filter_compatible(&self, traits: &[Trait]) -> Vec<Trait> {
        let mut result = Vec::new();

        for trait_def in traits {
            let is_compatible = !result
                .iter()
                .any(|t: &Trait| self.are_incompatible(&t.id, &trait_def.id));

            if is_compatible {
                result.push(trait_def.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests;
