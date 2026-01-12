//! Mutation System
//! Handles mutation triggering, types, and effects.

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{BreedingConfig, MutationRoll, MutationType as LogMutationType, MutationModifier, TraitDefinition, TraitCategory, InheritanceType};

/// Mutation type weights for rolling
const MUTATION_TYPE_WEIGHTS: [(MutationKind, f32); 4] = [
    (MutationKind::StatBoost, 0.40),
    (MutationKind::NewTrait, 0.30),
    (MutationKind::TraitPowerBoost, 0.20),
    (MutationKind::HiddenUnlock, 0.10),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationKind {
    /// +5% to +15% to one random stat
    StatBoost,
    /// Add a new random mutation trait
    NewTrait,
    /// Increase power of existing trait by 1-3
    TraitPowerBoost,
    /// Reveal/enhance a hidden trait
    HiddenUnlock,
}

/// Result of a mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationResult {
    pub occurred: bool,
    pub kind: Option<MutationKind>,
    pub affected_target: Option<String>,
    pub effect_value: Option<f32>,
    pub new_trait: Option<TraitDefinition>,
}

impl Default for MutationResult {
    fn default() -> Self {
        Self {
            occurred: false,
            kind: None,
            affected_target: None,
            effect_value: None,
            new_trait: None,
        }
    }
}

/// Possible mutation traits that can be generated
pub fn get_mutation_trait_pool() -> Vec<TraitDefinition> {
    vec![
        TraitDefinition {
            id: "MU01".to_string(),
            name: "Unstable Mutation".to_string(),
            description: "+5 to a random stat".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 5,
            is_hidden: true,
            visual_keywords: vec!["chaotic patterns".to_string(), "unstable form".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU02".to_string(),
            name: "Extra Limb".to_string(),
            description: "+8 attack, -3 speed".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 8,
            is_hidden: true,
            visual_keywords: vec!["additional appendage".to_string(), "asymmetric limbs".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU03".to_string(),
            name: "Hardened Carapace".to_string(),
            description: "+10 defense, -5 speed".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 10,
            is_hidden: true,
            visual_keywords: vec!["thick shell".to_string(), "armored plates".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU04".to_string(),
            name: "Bioluminescence".to_string(),
            description: "+3 damage in darkness".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 3,
            is_hidden: true,
            visual_keywords: vec!["glowing marks".to_string(), "luminescent patterns".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU05".to_string(),
            name: "Overcharged Cells".to_string(),
            description: "+12 attack, -10 HP per turn".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 12,
            is_hidden: true,
            visual_keywords: vec!["crackling energy".to_string(), "overloaded aura".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU06".to_string(),
            name: "Fragile Frame".to_string(),
            description: "-8 defense (negative mutation)".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: -8,
            is_hidden: true,
            visual_keywords: vec!["thin structure".to_string(), "fragile appearance".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU07".to_string(),
            name: "Rapid Growth".to_string(),
            description: "+15% XP gain".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 15,
            is_hidden: true,
            visual_keywords: vec!["oversized features".to_string(), "accelerated development".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU08".to_string(),
            name: "Parasitic Spores".to_string(),
            description: "Drain 3 HP/turn from enemy".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 7,
            is_hidden: true,
            visual_keywords: vec!["fungal growth".to_string(), "spore clouds".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU09".to_string(),
            name: "Thermal Vision".to_string(),
            description: "+6 attack in heat environments".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 6,
            is_hidden: true,
            visual_keywords: vec!["heat-sensing organs".to_string(), "infrared eyes".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU10".to_string(),
            name: "Echo Location".to_string(),
            description: "+5 speed in darkness".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 5,
            is_hidden: true,
            visual_keywords: vec!["sensor organs".to_string(), "large ears".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU11".to_string(),
            name: "Neon Spines".to_string(),
            description: "Counter 10% damage when hit".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 8,
            is_hidden: true,
            visual_keywords: vec!["glowing spines".to_string(), "neon protrusions".to_string()],
            condition: None,
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "MU12".to_string(),
            name: "Adaptive Pigment".to_string(),
            description: "+5% evasion, changes color in combat".to_string(),
            category: TraitCategory::Mutation,
            inheritance_type: InheritanceType::Recessive,
            power: 6,
            is_hidden: true,
            visual_keywords: vec!["color-shifting skin".to_string(), "chromatic scales".to_string()],
            condition: None,
            polygenic_points: 0,
        },
    ]
}

/// Mutation processor
pub struct MutationProcessor {
    config: BreedingConfig,
}

impl MutationProcessor {
    pub fn new(config: BreedingConfig) -> Self {
        Self { config }
    }
    
    /// Calculate mutation chance based on various factors
    pub fn calculate_mutation_chance(
        &self,
        offspring_generation: u32,
        total_parent_traits: usize,
        mutation_catalyst_used: bool,
    ) -> (f32, Vec<MutationModifier>) {
        let mut chance = self.config.mutation.base_chance;
        let mut modifiers = Vec::new();
        
        // Generation modifier
        let gen_mod = offspring_generation as f32 * self.config.mutation.generation_modifier;
        if gen_mod > 0.0 {
            modifiers.push(MutationModifier {
                source: format!("Generation {} bonus", offspring_generation),
                value: gen_mod,
            });
            chance += gen_mod;
        }
        
        // High trait count modifier
        if total_parent_traits > self.config.mutation.high_trait_threshold {
            modifiers.push(MutationModifier {
                source: format!("High trait count ({} > {})", total_parent_traits, self.config.mutation.high_trait_threshold),
                value: self.config.mutation.high_trait_bonus,
            });
            chance += self.config.mutation.high_trait_bonus;
        }
        
        // Mutation Catalyst item
        if mutation_catalyst_used {
            modifiers.push(MutationModifier {
                source: "Mutation Catalyst".to_string(),
                value: 0.05,
            });
            chance += 0.05;
        }
        
        // Cap at max chance
        chance = chance.min(self.config.mutation.max_chance);
        
        (chance, modifiers)
    }
    
    /// Process mutation roll
    pub fn process_mutation<R: Rng>(
        &self,
        offspring_generation: u32,
        total_parent_traits: usize,
        mutation_catalyst_used: bool,
        existing_visible_traits: &mut Vec<TraitDefinition>,
        existing_hidden_traits: &mut Vec<TraitDefinition>,
        stats: &mut std::collections::HashMap<String, i32>,
        rng: &mut R,
    ) -> (MutationResult, MutationRoll) {
        let (final_chance, modifiers) = self.calculate_mutation_chance(
            offspring_generation,
            total_parent_traits,
            mutation_catalyst_used,
        );
        
        let roll: f32 = rng.gen();
        let triggered = roll < final_chance;
        
        let mut result = MutationResult::default();
        let mut log_type = None;
        let mut log_result = None;
        
        if triggered {
            result.occurred = true;
            
            // Roll for mutation type
            let type_roll: f32 = rng.gen();
            let mut cumulative = 0.0;
            let mut mutation_kind = MutationKind::StatBoost;
            
            for (kind, weight) in MUTATION_TYPE_WEIGHTS.iter() {
                cumulative += weight;
                if type_roll < cumulative {
                    mutation_kind = *kind;
                    break;
                }
            }
            
            result.kind = Some(mutation_kind);
            
            match mutation_kind {
                MutationKind::StatBoost => {
                    // Boost random stat by 5-15%
                    let stat_names = ["hp", "attack", "defense", "speed", "energy"];
                    let target_stat = stat_names[rng.gen_range(0..stat_names.len())].to_string();
                    let boost = rng.gen_range(self.config.mutation.stat_boost_min..=self.config.mutation.stat_boost_max);
                    
                    if let Some(stat_val) = stats.get_mut(&target_stat) {
                        let new_val = (*stat_val as f32 * boost) as i32;
                        *stat_val = new_val;
                    }
                    
                    result.affected_target = Some(target_stat.clone());
                    result.effect_value = Some(boost);
                    log_type = Some(LogMutationType::StatBoost);
                    log_result = Some(format!("{} x{:.2}", target_stat, boost));
                }
                MutationKind::NewTrait => {
                    // Add random mutation trait
                    let pool = get_mutation_trait_pool();
                    let existing_ids: std::collections::HashSet<_> = existing_visible_traits.iter()
                        .chain(existing_hidden_traits.iter())
                        .map(|t| t.id.clone())
                        .collect();
                    
                    let available: Vec<_> = pool.into_iter()
                        .filter(|t| !existing_ids.contains(&t.id))
                        .collect();
                    
                    if !available.is_empty() {
                        let new_trait = available[rng.gen_range(0..available.len())].clone();
                        result.affected_target = Some(new_trait.name.clone());
                        result.new_trait = Some(new_trait.clone());
                        existing_hidden_traits.push(new_trait.clone());
                        log_type = Some(LogMutationType::NewTrait);
                        log_result = Some(new_trait.name);
                    }
                }
                MutationKind::TraitPowerBoost => {
                    // Boost existing trait power by 1-3
                    let all_traits: Vec<_> = existing_visible_traits.iter_mut()
                        .chain(existing_hidden_traits.iter_mut())
                        .collect();
                    
                    if !all_traits.is_empty() {
                        let idx = rng.gen_range(0..all_traits.len());
                        let boost = rng.gen_range(1..=3);
                        
                        // Need to re-collect mutably
                        let combined_len = existing_visible_traits.len() + existing_hidden_traits.len();
                        if combined_len > 0 {
                            let target_idx = rng.gen_range(0..combined_len);
                            if target_idx < existing_visible_traits.len() {
                                let t = &mut existing_visible_traits[target_idx];
                                t.power += boost;
                                result.affected_target = Some(t.name.clone());
                                result.effect_value = Some(boost as f32);
                                log_result = Some(format!("{} +{} power", t.name, boost));
                            } else {
                                let hidden_idx = target_idx - existing_visible_traits.len();
                                if hidden_idx < existing_hidden_traits.len() {
                                    let t = &mut existing_hidden_traits[hidden_idx];
                                    t.power += boost;
                                    result.affected_target = Some(t.name.clone());
                                    result.effect_value = Some(boost as f32);
                                    log_result = Some(format!("{} +{} power", t.name, boost));
                                }
                            }
                        }
                        log_type = Some(LogMutationType::TraitPowerBoost);
                    }
                }
                MutationKind::HiddenUnlock => {
                    // Reveal a hidden trait or boost its power by 50%
                    if !existing_hidden_traits.is_empty() {
                        let idx = rng.gen_range(0..existing_hidden_traits.len());
                        let t = &mut existing_hidden_traits[idx];
                        
                        // 50% chance to reveal, 50% chance to boost
                        if rng.gen_bool(0.5) {
                            t.is_hidden = false;
                            let revealed = t.clone();
                            result.affected_target = Some(revealed.name.clone());
                            log_result = Some(format!("{} REVEALED", revealed.name));
                            // Move to visible
                            existing_visible_traits.push(existing_hidden_traits.remove(idx));
                        } else {
                            let boost = (t.power as f32 * 0.5) as i32;
                            t.power += boost;
                            result.affected_target = Some(t.name.clone());
                            result.effect_value = Some(0.5);
                            log_result = Some(format!("{} +50% power", t.name));
                        }
                        log_type = Some(LogMutationType::HiddenUnlock);
                    }
                }
            }
        }
        
        let mutation_log = MutationRoll {
            base_chance: self.config.mutation.base_chance,
            modifiers,
            final_chance,
            roll,
            triggered,
            mutation_type: log_type,
            mutation_result: log_result,
        };
        
        (result, mutation_log)
    }
}

// Serialization support for MutationKind
impl Serialize for MutationKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MutationKind::StatBoost => serializer.serialize_str("stat_boost"),
            MutationKind::NewTrait => serializer.serialize_str("new_trait"),
            MutationKind::TraitPowerBoost => serializer.serialize_str("trait_power_boost"),
            MutationKind::HiddenUnlock => serializer.serialize_str("hidden_unlock"),
        }
    }
}

impl<'de> Deserialize<'de> for MutationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "stat_boost" => Ok(MutationKind::StatBoost),
            "new_trait" => Ok(MutationKind::NewTrait),
            "trait_power_boost" => Ok(MutationKind::TraitPowerBoost),
            "hidden_unlock" => Ok(MutationKind::HiddenUnlock),
            _ => Err(serde::de::Error::custom(format!("unknown mutation kind: {}", s))),
        }
    }
}
