//! Core combat simulation engine.
//!
//! Implements deterministic auto-battle with 7-step damage formula,
//! environment effects, trait bonuses, and full replay capability.

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::data::environments::{Environment, PerTurnEffect};
use crate::data::traits::TraitCategory;
use crate::data::Kaiju;
use crate::state::battle_state::{BattleLogEntry, BattleResult, BattleState};

/// Combat configuration (loaded from balance.json)
#[derive(Debug, Clone)]
pub struct CombatConfig {
    /// Defense scaling factor (default: 0.5)
    pub defense_scaling: f32,
    /// Minimum damage floor (default: 5)
    pub minimum_damage: i32,
    /// Variance minimum (default: 0.95)
    pub variance_min: f32,
    /// Variance maximum (default: 1.05)
    pub variance_max: f32,
    /// Maximum turns before timeout (default: 100)
    pub max_turns: u32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            defense_scaling: 0.5,
            minimum_damage: 5,
            variance_min: 0.95,
            variance_max: 1.05,
            max_turns: 100,
        }
    }
}

/// Battle simulator with configurable parameters
#[derive(Debug, Clone)]
pub struct BattleSimulator {
    config: CombatConfig,
}

impl BattleSimulator {
    /// Create a new simulator with the given config
    pub fn new(config: CombatConfig) -> Self {
        Self { config }
    }

    /// Simulate a battle between two kaiju
    pub fn simulate(
        &self,
        kaiju_a: Kaiju,
        kaiju_b: Kaiju,
        environment: Environment,
        seed: u64,
    ) -> BattleResult {
        execute_battle(kaiju_a, kaiju_b, environment, seed, &self.config)
    }

    /// Replay a battle using the stored seed
    pub fn replay(
        &self,
        battle_result: &BattleResult,
        kaiju_a: Kaiju,
        kaiju_b: Kaiju,
        environment: Environment,
    ) -> BattleResult {
        execute_battle(kaiju_a, kaiju_b, environment, battle_result.seed, &self.config)
    }

    /// Verify a battle result is valid
    pub fn verify(
        &self,
        battle_result: &BattleResult,
        kaiju_a: &Kaiju,
        kaiju_b: &Kaiju,
        environment: &Environment,
    ) -> bool {
        let replayed = execute_battle(
            kaiju_a.clone(),
            kaiju_b.clone(),
            environment.clone(),
            battle_result.seed,
            &self.config,
        );

        replayed.winner == battle_result.winner
            && replayed.hp_remaining == battle_result.hp_remaining
            && replayed.turns_elapsed == battle_result.turns_elapsed
    }
}

impl Default for BattleSimulator {
    fn default() -> Self {
        Self::new(CombatConfig::default())
    }
}

/// Execute a complete battle between two kaiju
pub fn execute_battle(
    kaiju_a: Kaiju,
    kaiju_b: Kaiju,
    environment: Environment,
    seed: u64,
    config: &CombatConfig,
) -> BattleResult {
    // Initialize deterministic RNG
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    // Initialize battle state
    let mut state = BattleState::new(kaiju_a.clone(), kaiju_b.clone(), environment.clone(), seed);

    // Determine who attacks first
    let a_first = determine_first_attacker(&kaiju_a, &kaiju_b, &mut rng);

    // Main battle loop
    while !state.is_over() && state.turn_count < config.max_turns {
        state.turn_count += 1;

        // Determine attacker and defender for this turn
        let (attacker, defender, is_a_attacking) = if (state.turn_count % 2 == 1) == a_first {
            (&state.kaiju_a, &state.kaiju_b, true)
        } else {
            (&state.kaiju_b, &state.kaiju_a, false)
        };

        // Calculate damage
        let damage = calculate_damage(attacker, defender, &state.environment, &mut rng, config);

        // Apply damage
        if is_a_attacking {
            state.hp_b -= damage;
        } else {
            state.hp_a -= damage;
        }

        // Log turn
        state.log_turn(BattleLogEntry {
            turn: state.turn_count,
            attacker_name: attacker.name.clone(),
            defender_name: defender.name.clone(),
            damage,
            hp_remaining: if is_a_attacking {
                state.hp_b.max(0)
            } else {
                state.hp_a.max(0)
            },
            special_effects: Vec::new(),
        });

        // Apply per-turn environmental effects
        for effect in environment.get_per_turn_effects() {
            apply_per_turn_effect(&mut state, &effect, &mut rng);
        }

        // Check for battle end
        if state.is_over() {
            break;
        }
    }

    // Generate result with insights
    let mut result = BattleResult::from_state(state);
    result.insights = generate_insights(&result, &kaiju_a, &kaiju_b);
    result
}

/// Determine which kaiju attacks first based on speed
fn determine_first_attacker(kaiju_a: &Kaiju, kaiju_b: &Kaiju, rng: &mut ChaCha8Rng) -> bool {
    let speed_a = calculate_effective_speed(kaiju_a);
    let speed_b = calculate_effective_speed(kaiju_b);

    if speed_a > speed_b {
        true // A goes first
    } else if speed_b > speed_a {
        false // B goes first
    } else {
        // Tie-breaker: higher attack, then random
        if kaiju_a.stats.attack > kaiju_b.stats.attack {
            true
        } else if kaiju_b.stats.attack > kaiju_a.stats.attack {
            false
        } else {
            rng.gen_bool(0.5)
        }
    }
}

/// Calculate effective speed including trait modifiers
fn calculate_effective_speed(kaiju: &Kaiju) -> i32 {
    let mut speed = kaiju.stats.speed;

    for trait_def in &kaiju.traits {
        // Apply speed modifier traits
        if trait_def.name.contains("Swift") {
            speed = (speed as f32 * 1.10) as i32;
        }
        if trait_def.name.contains("Heavy") {
            speed = (speed as f32 * 0.95) as i32;
        }
    }

    speed
}

/// Calculate damage using the 7-step formula
pub fn calculate_damage(
    attacker: &Kaiju,
    defender: &Kaiju,
    environment: &Environment,
    rng: &mut ChaCha8Rng,
    config: &CombatConfig,
) -> i32 {
    // Step 1: Base damage = attack - (defense * 0.5)
    let base = attacker.stats.attack as f32 - (defender.stats.defense as f32 * config.defense_scaling);

    // Step 2: Apply minimum floor
    let base = base.max(config.minimum_damage as f32);

    // Step 3: Add trait power bonuses (element traits only)
    let trait_bonus = calculate_trait_bonuses(attacker, defender, environment);

    // Step 4: Apply environment multiplier
    let env_multiplier = calculate_environment_multiplier(attacker, environment);

    // Step 5: Apply synergy bonuses (future expansion)
    let synergy_multiplier = 1.0;

    // Step 6: Apply narrow randomness (±5%)
    let variance = rng.gen_range(config.variance_min..=config.variance_max);

    // Step 7: Calculate final damage
    let final_damage = ((base + trait_bonus as f32) * env_multiplier * synergy_multiplier * variance) as i32;

    final_damage.max(1) // Absolute minimum: 1 damage
}

/// Calculate trait power bonuses
fn calculate_trait_bonuses(attacker: &Kaiju, _defender: &Kaiju, _environment: &Environment) -> i32 {
    let mut bonus = 0;

    for trait_def in &attacker.traits {
        if trait_def.category == TraitCategory::Element {
            // Check if condition is met
            if !trait_def.is_hidden {
                bonus += trait_def.power;
            }
        }
    }

    bonus
}

/// Calculate environment multiplier for a kaiju
fn calculate_environment_multiplier(kaiju: &Kaiju, environment: &Environment) -> f32 {
    let mut multiplier = 1.0;

    for trait_def in &kaiju.traits {
        // Check trait name against environment bonuses
        let trait_type = extract_element_type(&trait_def.name);
        if let Some(type_name) = trait_type {
            let bonus = environment.get_trait_multiplier(&type_name);
            if bonus != 1.0 {
                multiplier += bonus - 1.0; // Additive bonuses
            }
        }
    }

    multiplier.clamp(0.5, 2.0) // Safety bounds
}

/// Extract element type from trait name
fn extract_element_type(trait_name: &str) -> Option<String> {
    let elements = [
        "Electric", "Fire", "Water", "Ice", "Wind", "Earth", "Toxic", "Energy", "Psychic",
    ];

    for element in elements {
        if trait_name.contains(element) {
            return Some(element.to_string());
        }
    }

    None
}

/// Apply per-turn environmental effects
fn apply_per_turn_effect(state: &mut BattleState, effect: &PerTurnEffect, rng: &mut ChaCha8Rng) {
    match effect {
        PerTurnEffect::Damage { percent } => {
            let damage_a = (state.hp_a as f32 * percent).ceil() as i32;
            let damage_b = (state.hp_b as f32 * percent).ceil() as i32;
            state.hp_a -= damage_a;
            state.hp_b -= damage_b;
            state.log_special_effect(format!(
                "Environmental damage: {} takes {} damage, {} takes {} damage",
                state.kaiju_a.name, damage_a, state.kaiju_b.name, damage_b
            ));
        }
        PerTurnEffect::StatusChance { effect, chance } => {
            if rng.gen::<f32>() < *chance {
                state.log_special_effect(format!("{} triggered!", effect));
            }
        }
        PerTurnEffect::StatSwap { chance } => {
            if rng.gen::<f32>() < *chance {
                state.log_special_effect("Void effect: stats temporarily unstable!".to_string());
            }
        }
    }
}

/// Generate battle insights
fn generate_insights(result: &BattleResult, kaiju_a: &Kaiju, kaiju_b: &Kaiju) -> Vec<String> {
    let mut insights = Vec::new();

    // Speed analysis
    if kaiju_a.stats.speed > kaiju_b.stats.speed && result.winner == kaiju_a.name {
        insights.push("Speed advantage allowed the winner to control tempo".to_string());
    }

    // HP efficiency
    let total_hp = if result.winner == kaiju_a.name {
        kaiju_a.stats.hp
    } else {
        kaiju_b.stats.hp
    };
    let efficiency = (result.hp_remaining as f32 / total_hp as f32 * 100.0) as i32;
    insights.push(format!("Winner retained {}% of HP", efficiency));

    // Environment tip
    if result.environment != "Neutral" {
        insights.push(format!(
            "The {} environment may have influenced trait effectiveness",
            result.environment
        ));
    }

    insights
}

/// Generate a random battle seed
pub fn generate_battle_seed() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::traits::{Trait, TraitCondition, TraitInheritance};
    use crate::data::KaijuStats;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_kaiju(name: &str, hp: i32, atk: i32, def: i32, spd: i32) -> Kaiju {
        Kaiju {
            id: Uuid::new_v4(),
            token_id: 0,
            name: name.to_string(),
            generation: 1,
            created_at: Utc::now().timestamp(),
            original_breeder: "test".to_string(),
            parent_ids: None,
            visual_seed: 0,
            genome_hash: "test".to_string(),
            stats: KaijuStats::new(hp, atk, def, spd),
            traits: vec![],
            hidden_traits: vec![],
            experience: 0,
            alive: true,
            current_owner: "test".to_string(),
            image_uri: None,
            metadata_uri: String::new(),
        }
    }

    #[test]
    fn test_base_damage_calculation() {
        let attacker = create_test_kaiju("A", 100, 50, 20, 30);
        let defender = create_test_kaiju("B", 100, 40, 30, 25);
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        // Expected: 50 - (30 * 0.5) = 50 - 15 = 35
        let damage = calculate_damage(&attacker, &defender, &Environment::Neutral, &mut rng, &config);

        // With ±5% variance, should be 33-37
        assert!(damage >= 33 && damage <= 37, "Damage was {}", damage);
    }

    #[test]
    fn test_minimum_damage_floor() {
        let attacker = create_test_kaiju("A", 100, 10, 10, 10);
        let defender = create_test_kaiju("B", 100, 10, 100, 10);
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        let damage = calculate_damage(&attacker, &defender, &Environment::Neutral, &mut rng, &config);

        // Should apply minimum floor and then variance
        assert!(damage >= 4, "Damage was {}", damage);
    }

    #[test]
    fn test_trait_power_bonus() {
        let mut attacker = create_test_kaiju("A", 100, 50, 20, 30);
        attacker.traits.push(Trait {
            id: "electric_breath".to_string(),
            name: "Electric Breath".to_string(),
            category: TraitCategory::Element,
            power: 8,
            inheritance: TraitInheritance::Dominant,
            condition: TraitCondition::Always,
            is_hidden: false,
            description: "Test trait".to_string(),
        });

        let defender = create_test_kaiju("B", 100, 40, 30, 25);
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        let damage = calculate_damage(&attacker, &defender, &Environment::Neutral, &mut rng, &config);

        // Expected: (50 - 15 + 8) * 1.0 * ~1.0 = ~43
        assert!(damage >= 40 && damage <= 46, "Damage was {}", damage);
    }

    #[test]
    fn test_full_battle_simulation() {
        let kaiju_a = create_test_kaiju("Flossy", 300, 60, 40, 30);
        let kaiju_b = create_test_kaiju("Reefmaw", 320, 55, 45, 25);

        let config = CombatConfig::default();
        let result = execute_battle(kaiju_a, kaiju_b, Environment::Neutral, 12345, &config);

        assert!(!result.winner.is_empty());
        assert!(!result.loser.is_empty());
        assert!(result.hp_remaining >= 0);
        assert!(result.turns_elapsed > 0);
        assert!(!result.battle_log.is_empty());
    }

    #[test]
    fn test_replay_determinism() {
        let kaiju_a = create_test_kaiju("Alpha", 300, 60, 40, 30);
        let kaiju_b = create_test_kaiju("Beta", 320, 55, 45, 25);
        let config = CombatConfig::default();

        let result1 = execute_battle(
            kaiju_a.clone(),
            kaiju_b.clone(),
            Environment::Storm,
            12345,
            &config,
        );

        let result2 = execute_battle(
            kaiju_a.clone(),
            kaiju_b.clone(),
            Environment::Storm,
            12345,
            &config,
        );

        assert_eq!(result1.winner, result2.winner);
        assert_eq!(result1.hp_remaining, result2.hp_remaining);
        assert_eq!(result1.turns_elapsed, result2.turns_elapsed);
        assert_eq!(result1.battle_log.len(), result2.battle_log.len());
    }

    #[test]
    fn test_speed_determines_first_attacker() {
        let fast = create_test_kaiju("Fast", 100, 50, 30, 100);
        let slow = create_test_kaiju("Slow", 100, 50, 30, 10);

        let config = CombatConfig::default();
        let result = execute_battle(fast, slow, Environment::Neutral, 0, &config);

        assert_eq!(result.battle_log[0].attacker_name, "Fast");
    }

    #[test]
    fn test_variance_distribution() {
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        let mut results = Vec::new();
        for _ in 0..1000 {
            let variance = rng.gen_range(config.variance_min..=config.variance_max);
            results.push(variance);
        }

        // All within bounds
        assert!(results.iter().all(|&v| v >= 0.95 && v <= 1.05));

        // Mean should be close to 1.0
        let mean: f32 = results.iter().sum::<f32>() / results.len() as f32;
        assert!((mean - 1.0).abs() < 0.02);
    }
}
