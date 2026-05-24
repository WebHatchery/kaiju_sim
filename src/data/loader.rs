//! JSON data loading and validation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::traits::Trait;

/// Complete game data loaded from JSON files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub traits: TraitDatabase,
    pub balance: BalanceConfig,
    pub tournaments: TournamentDatabase,
}

/// Trait database loaded from traits.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDatabase {
    pub traits: Vec<Trait>,
}

impl TraitDatabase {
    /// Get trait by ID
    pub fn get_trait(&self, id: &str) -> Option<&Trait> {
        self.traits.iter().find(|t| t.id == id)
    }

    /// Get all traits by category
    pub fn get_by_category(&self, category: super::traits::TraitCategory) -> Vec<&Trait> {
        self.traits
            .iter()
            .filter(|t| t.category == category)
            .collect()
    }
}

/// Balance configuration from balance.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub breeding: BreedingConfig,
    pub combat: CombatConfig,
    pub tournaments: TournamentRewardsConfig,
    pub research: ResearchConfig,
    pub mvp: MvpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingConfig {
    pub generation_power_creep: f32,
    pub stat_variance_min: f32,
    pub stat_variance_max: f32,
    pub visible_trait_inheritance_chance: f32,
    pub hidden_trait_inheritance_chance: f32,
    pub mutation_chance: f32,
    pub stat_floors: HashMap<String, i32>,
    pub stat_base_caps: HashMap<String, i32>,
    pub stat_hard_caps: HashMap<String, i32>,
    pub soft_cap_multiplier_per_generation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    pub base_damage_formula: DamageFormulaConfig,
    pub damage_variance_min: f32,
    pub damage_variance_max: f32,
    pub environment_multipliers: HashMap<String, HashMap<String, f32>>,
    pub max_battle_turns: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageFormulaConfig {
    pub attack_coefficient: f32,
    pub defense_coefficient: f32,
    pub minimum_damage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentRewardsConfig {
    pub xp_rewards: HashMap<String, u32>,
    pub ranking_elo: EloConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EloConfig {
    pub k_factor: i32,
    pub starting_rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchConfig {
    pub facility_levels: u32,
    pub decoding_costs: Vec<u32>,
    pub battles_required_for_layer_2: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvpConfig {
    pub training: TrainingMvpConfig,
    pub battle: BattleMvpConfig,
    pub breeding: BreedingMvpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMvpConfig {
    pub cost: i64,
    pub xp_gain: u32,
    pub stat_gain_min: i32,
    pub stat_gain_max: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleMvpConfig {
    pub entry_fee: i64,
    pub win_gold: i64,
    pub loss_gold: i64,
    pub win_xp: u32,
    pub loss_xp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingMvpConfig {
    pub cost: i64,
}

/// Tournament database from tournaments.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentDatabase {
    pub tournaments: Vec<TournamentDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentDefinition {
    pub id: String,
    pub name: String,
    pub tournament_type: String,
    pub bracket_system: String,
    pub min_generation: u32,
    pub max_generation: Option<u32>,
    pub entry_fee: u32,
    pub max_participants: u32,
    pub environment: String,
    pub rewards: HashMap<String, serde_json::Value>,
}

impl GameData {
    /// Load all game data from assets folder
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let traits_json = std::fs::read_to_string("assets/traits.json")?;
        let balance_json = std::fs::read_to_string("assets/balance.json")?;
        let tournaments_json = std::fs::read_to_string("assets/tournaments.json")?;

        let traits: TraitDatabase = serde_json::from_str(&traits_json)?;
        let balance: BalanceConfig = serde_json::from_str(&balance_json)?;
        let tournaments: TournamentDatabase = serde_json::from_str(&tournaments_json)?;

        // Validate data
        Self::validate_traits(&traits)?;
        Self::validate_balance(&balance)?;

        Ok(Self {
            traits,
            balance,
            tournaments,
        })
    }

    fn validate_traits(traits: &TraitDatabase) -> Result<(), Box<dyn std::error::Error>> {
        if traits.traits.is_empty() {
            return Err("No traits loaded".into());
        }

        // Check for duplicate IDs
        let mut seen_ids = std::collections::HashSet::new();
        for trait_def in &traits.traits {
            if !seen_ids.insert(&trait_def.id) {
                return Err(format!("Duplicate trait ID: {}", trait_def.id).into());
            }
        }

        Ok(())
    }

    fn validate_balance(balance: &BalanceConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Validate variance ranges
        if balance.breeding.stat_variance_min >= balance.breeding.stat_variance_max {
            return Err("Invalid breeding variance range".into());
        }

        if balance.combat.damage_variance_min >= balance.combat.damage_variance_max {
            return Err("Invalid combat variance range".into());
        }

        Ok(())
    }
}
