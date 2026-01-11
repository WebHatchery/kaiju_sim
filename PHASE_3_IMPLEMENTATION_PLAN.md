# PHASE 3 IMPLEMENTATION PLAN: Combat Engine

## Executive Summary

This document provides a complete implementation roadmap for Phase 3 (Combat Engine) of the Kaiju Breeding Simulator. The combat engine is a **deterministic auto-battle simulation system** with narrow randomness (±5%), supporting environmental effects, trait interactions, and full battle replay capability.

**Implementation Time Estimate**: 2 weeks
**Prerequisites**: Phase 1 (Data Models) and Phase 2 (Genetics & Breeding Engine) must be completed
**Performance Target**: <100ms per battle simulation
**Testing Coverage Target**: 90%+ for core damage calculations

---

## 1. Architecture Overview

### 1.1 Design Principles

The combat system follows these core principles from the project design documents:

- **Predictability with Probabilistic Elements**: Outcomes are 90%+ predictable with full knowledge
- **Narrow Randomness**: ±5% variance prevents excessive upsets while allowing minor surprises
- **Deterministic Replay**: Every battle can be replayed identically using a stored seed
- **Information Asymmetry**: Hidden traits create strategic depth
- **Environmental Interaction**: 9 environments modify trait effectiveness by 15-25%
- **Stateless Services**: Combat engine receives state and returns results without mutation

### 1.2 Module Responsibilities

Following CODE_STANDARDS.md structure:

**`src/engine/combat.rs`** (Primary):
- Damage calculation (7-step formula)
- Battle simulation loop
- Turn order determination
- Environment effect application
- Battle result generation

**`src/data/environments.rs`**:
- Environment definitions and multipliers
- Trait-to-environment matching logic
- Per-turn environmental effects

**`src/state/battle_state.rs`**:
- BattleState struct (HP tracking, turn count, logs)
- Battle log entry structure
- Replay data storage

**`assets/balance.json`**:
- Damage formula constants
- Environment multipliers
- Combat configuration

---

## 2. Damage Calculation System (7-Step Formula)

### 2.1 Implementation Priority: CRITICAL

This is the foundation of all combat resolution.

### 2.2 Formula Breakdown

```rust
// File: src/engine/combat.rs

pub fn calculate_damage(
    attacker: &Kaiju,
    defender: &Kaiju,
    environment: &Environment,
    rng: &mut ChaCha8Rng,
    config: &CombatConfig,
) -> i32 {
    // Step 1: Base damage = attack - (defense * 0.5)
    let base = (attacker.stats.attack as f32) - (defender.stats.defense as f32 * config.defense_scaling);

    // Step 2: Apply minimum floor (default: 5)
    let base = base.max(config.minimum_damage as f32);

    // Step 3: Add trait power bonuses (element traits only)
    let trait_bonus = calculate_trait_bonuses(attacker, defender, environment);

    // Step 4: Apply environment multiplier (1.0 baseline, 0.85-1.25 range)
    let env_multiplier = calculate_environment_multiplier(attacker, environment);

    // Step 5: Apply synergy bonuses (future expansion)
    let synergy_multiplier = 1.0; // TODO: Phase 2 integration

    // Step 6: Apply narrow randomness (±5%)
    let variance = rng.gen_range(config.variance_min..=config.variance_max);

    // Step 7: Calculate final damage
    let final_damage = ((base + trait_bonus as f32) * env_multiplier * synergy_multiplier * variance) as i32;

    final_damage.max(1) // Absolute minimum: 1 damage
}
```

### 2.3 Sub-Functions

**Trait Bonus Calculation**:
```rust
fn calculate_trait_bonuses(
    attacker: &Kaiju,
    defender: &Kaiju,
    environment: &Environment,
) -> i32 {
    let mut bonus = 0;

    // Sum element trait powers
    for trait in &attacker.traits {
        if trait.category == TraitCategory::Element {
            // Check if condition is met (e.g., HP threshold, turn number)
            if trait.condition_met(attacker, defender, environment) {
                bonus += trait.power;
            }
        }
    }

    // Apply modifier trait percentages (future)
    for trait in &attacker.traits {
        if trait.category == TraitCategory::Modifier {
            // Modifier traits apply multiplicatively in Step 4
        }
    }

    bonus
}
```

**Environment Multiplier Calculation**:
```rust
fn calculate_environment_multiplier(
    kaiju: &Kaiju,
    environment: &Environment,
) -> f32 {
    let mut multiplier = 1.0;

    for trait in &kaiju.traits {
        if let Some(bonus) = environment.get_trait_multiplier(&trait.name) {
            multiplier += bonus;
        }
    }

    multiplier.clamp(0.5, 2.0) // Safety bounds
}
```

### 2.4 Configuration Structure

```rust
// File: src/data/combat_config.rs

#[derive(Debug, Clone, Deserialize)]
pub struct CombatConfig {
    pub defense_scaling: f32,        // Default: 0.5
    pub minimum_damage: i32,          // Default: 5
    pub variance_min: f32,            // Default: 0.95
    pub variance_max: f32,            // Default: 1.05
    pub max_turns: u32,               // Default: 1000
}
```

### 2.5 Unit Tests Required

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_minimum_damage_floor() {
        // Low attack vs high defense should yield minimum 5 damage
    }

    #[test]
    fn test_variance_bounds() {
        // Run 1000 calculations, verify all results within ±5%
    }

    #[test]
    fn test_defense_scaling() {
        // Verify defense reduces damage by exactly 50% effectiveness
    }

    #[test]
    fn test_trait_power_addition() {
        // Electric Breath (8) + Flame Core (12) = +20 damage
    }

    #[test]
    fn test_environment_multiplier() {
        // Electric trait in Storm = 1.15x multiplier
    }
}
```

---

## 3. Environment System Implementation

### 3.1 Environment Definitions

```rust
// File: src/data/environments.rs

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Environment {
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

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentDefinition {
    pub name: String,
    pub trait_multipliers: HashMap<String, f32>, // "Electric" -> 1.15
    pub per_turn_effects: Vec<PerTurnEffect>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum PerTurnEffect {
    Damage { percent: f32 },        // Volcanic: 2% HP burn
    StatusChance { effect: String, chance: f32 }, // Radiation: 5% mutation
    StatSwap { chance: f32 },       // Void: 10% attack↔defense swap
}
```

### 3.2 Environment Multiplier Table

From COMBAT_SYSTEM_SPEC.md Section 3.2:

| Environment | Affected Traits | Multiplier |
|-------------|----------------|------------|
| Storm | Electric, Wind | +15% |
| Storm | Fire | -10% |
| Volcanic | Fire | +25% |
| Volcanic | Ice/Frost | -15% |
| Aquatic | Water | +20% |
| Aquatic | Fire | -20% |
| Tundra | Ice | +20% |
| Desert | Fire, Earth | +15%, +10% |
| Forest | Earth, Toxic, Wind | +15%, +20%, +10% |
| Radiation | Energy, Mutation | +25%, +20% |
| Void | Energy (random) | ±15% per turn |

### 3.3 JSON Configuration

```json
// File: assets/environments.json
{
  "environments": [
    {
      "name": "Storm",
      "trait_multipliers": {
        "Electric": 1.15,
        "Wind": 1.15,
        "Fire": 0.90,
        "Water": 1.05
      },
      "per_turn_effects": []
    },
    {
      "name": "Volcanic",
      "trait_multipliers": {
        "Fire": 1.25,
        "Earth": 1.10,
        "Ice": 0.85,
        "Water": 0.90
      },
      "per_turn_effects": [
        { "Damage": { "percent": 0.02 } }
      ]
    }
  ]
}
```

### 3.4 Per-Turn Effects

```rust
impl PerTurnEffect {
    pub fn apply(&self, state: &mut BattleState, rng: &mut ChaCha8Rng) {
        match self {
            PerTurnEffect::Damage { percent } => {
                let damage_a = (state.hp_a as f32 * percent) as i32;
                let damage_b = (state.hp_b as f32 * percent) as i32;
                state.hp_a -= damage_a;
                state.hp_b -= damage_b;
                state.log_special_effect(format!("Volcanic burn: -{} HP each", damage_a));
            },
            PerTurnEffect::StatusChance { effect, chance } => {
                if rng.gen_bool(*chance as f64) {
                    state.log_special_effect(format!("{} triggered!", effect));
                }
            },
            PerTurnEffect::StatSwap { chance } => {
                if rng.gen_bool(*chance as f64) {
                    // Swap attack and defense stats (temporary, one turn)
                    state.log_special_effect("Void effect: stats swapped!".to_string());
                }
            },
        }
    }
}
```

---

## 4. Battle Flow and Turn Order

### 4.1 Turn Order Determination

```rust
// File: src/engine/combat.rs

pub fn determine_turn_order<'a>(
    kaiju_a: &'a Kaiju,
    kaiju_b: &'a Kaiju,
    rng: &mut ChaCha8Rng,
) -> (bool, &'a Kaiju, &'a Kaiju) {
    let speed_a = calculate_effective_speed(kaiju_a);
    let speed_b = calculate_effective_speed(kaiju_b);

    if speed_a > speed_b {
        (true, kaiju_a, kaiju_b) // A attacks first
    } else if speed_b > speed_a {
        (false, kaiju_b, kaiju_a) // B attacks first
    } else {
        // Tie-breaker: higher attack goes first
        if kaiju_a.stats.attack >= kaiju_b.stats.attack {
            (true, kaiju_a, kaiju_b)
        } else if rng.gen_bool(0.5) {
            (true, kaiju_a, kaiju_b)
        } else {
            (false, kaiju_b, kaiju_a)
        }
    }
}

fn calculate_effective_speed(kaiju: &Kaiju) -> i32 {
    let mut speed = kaiju.stats.speed;

    // Apply speed modifier traits
    for trait in &kaiju.traits {
        match trait.name.as_str() {
            "Swift" => speed = (speed as f32 * 1.10) as i32,
            "Heavy" => speed = (speed as f32 * 0.95) as i32,
            _ => {}
        }
    }

    speed
}
```

### 4.2 Battle Simulation Loop

```rust
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
    let mut state = BattleState {
        kaiju_a: kaiju_a.clone(),
        kaiju_b: kaiju_b.clone(),
        hp_a: kaiju_a.stats.hp,
        hp_b: kaiju_b.stats.hp,
        environment: environment.clone(),
        turn_count: 0,
        battle_log: Vec::new(),
        seed,
    };

    // Determine who attacks first
    let (a_first, mut attacker, mut defender) = determine_turn_order(&kaiju_a, &kaiju_b, &mut rng);
    let mut attacker_is_a = a_first;

    // Main battle loop
    while state.hp_a > 0 && state.hp_b > 0 && state.turn_count < config.max_turns {
        state.turn_count += 1;

        // Calculate and apply damage
        let damage = calculate_damage(attacker, defender, &state.environment, &mut rng, config);

        if attacker_is_a {
            state.hp_b -= damage;
        } else {
            state.hp_a -= damage;
        }

        // Log turn
        state.battle_log.push(BattleLogEntry {
            turn: state.turn_count,
            attacker_name: attacker.name.clone(),
            defender_name: defender.name.clone(),
            damage,
            hp_remaining: if attacker_is_a { state.hp_b } else { state.hp_a },
            special_effects: Vec::new(),
        });

        // Apply per-turn environmental effects
        for effect in &environment.per_turn_effects {
            effect.apply(&mut state, &mut rng);
        }

        // Swap attacker/defender
        std::mem::swap(&mut attacker, &mut defender);
        attacker_is_a = !attacker_is_a;
    }

    // Determine winner
    create_battle_result(state)
}
```

### 4.3 Battle State Structure

```rust
// File: src/state/battle_state.rs

pub struct BattleState {
    pub kaiju_a: Kaiju,
    pub kaiju_b: Kaiju,
    pub hp_a: i32,
    pub hp_b: i32,
    pub environment: Environment,
    pub turn_count: u32,
    pub battle_log: Vec<BattleLogEntry>,
    pub seed: u64,
}

pub struct BattleLogEntry {
    pub turn: u32,
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub hp_remaining: i32,
    pub special_effects: Vec<String>,
}

pub struct BattleResult {
    pub winner: String,
    pub loser: String,
    pub hp_remaining: i32,
    pub turns_elapsed: u32,
    pub battle_log: Vec<BattleLogEntry>,
    pub seed: u64,
    pub insights: Vec<String>,      // Performance analysis
    pub suggestions: Vec<String>,   // Improvement recommendations
}
```

---

## 5. Deterministic RNG Implementation

### 5.1 ChaCha8Rng Integration

```rust
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct BattleSimulator {
    config: CombatConfig,
}

impl BattleSimulator {
    pub fn new(config: CombatConfig) -> Self {
        Self { config }
    }

    pub fn simulate(&self, kaiju_a: Kaiju, kaiju_b: Kaiju, environment: Environment, seed: u64) -> BattleResult {
        execute_battle(kaiju_a, kaiju_b, environment, seed, &self.config)
    }

    pub fn replay(&self, battle_result: &BattleResult, kaiju_a: Kaiju, kaiju_b: Kaiju, environment: Environment) -> BattleResult {
        // Use stored seed to replay exact battle
        execute_battle(kaiju_a, kaiju_b, environment, battle_result.seed, &self.config)
    }
}
```

### 5.2 Seed Generation

```rust
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_battle_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
```

### 5.3 Variance Range Enforcement

```rust
#[test]
fn test_variance_distribution() {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut results = Vec::new();

    for _ in 0..10000 {
        let variance = rng.gen_range(0.95..=1.05);
        results.push(variance);
    }

    // Verify all results within bounds
    assert!(results.iter().all(|&v| v >= 0.95 && v <= 1.05));

    // Verify uniform distribution (statistical test)
    let mean: f32 = results.iter().sum::<f32>() / results.len() as f32;
    assert!((mean - 1.0).abs() < 0.01); // Mean should be ~1.0
}
```

---

## 6. Battle Log Generation

### 6.1 Log Entry Structure

```rust
impl BattleState {
    pub fn log_turn(&mut self, entry: BattleLogEntry) {
        self.battle_log.push(entry);
    }

    pub fn log_special_effect(&mut self, effect: String) {
        if let Some(last_entry) = self.battle_log.last_mut() {
            last_entry.special_effects.push(effect);
        }
    }
}
```

### 6.2 Human-Readable Output

```rust
impl BattleResult {
    pub fn format_log(&self) -> String {
        let mut output = String::new();

        for entry in &self.battle_log {
            output.push_str(&format!(
                "Turn {}: {} hits {} for {} damage ({} HP remaining)\n",
                entry.turn,
                entry.attacker_name,
                entry.defender_name,
                entry.damage,
                entry.hp_remaining
            ));

            for effect in &entry.special_effects {
                output.push_str(&format!("    [{}]\n", effect));
            }
        }

        output.push_str(&format!(
            "\nVICTORY: {} wins with {} HP remaining!\n",
            self.winner,
            self.hp_remaining
        ));

        output
    }
}
```

### 6.3 JSON Export

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BattleLogExport {
    pub battle_id: String,
    pub timestamp: u64,
    pub seed: u64,
    pub environment: String,
    pub participants: [String; 2],
    pub turns: Vec<BattleLogEntry>,
    pub result: BattleSummary,
}

impl BattleResult {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let export = BattleLogExport {
            battle_id: format!("battle_{}", self.seed),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            seed: self.seed,
            environment: "Storm".to_string(), // TODO: Store environment in BattleResult
            participants: [self.winner.clone(), self.loser.clone()],
            turns: self.battle_log.clone(),
            result: BattleSummary {
                winner: self.winner.clone(),
                hp_remaining: self.hp_remaining,
                turns_elapsed: self.turns_elapsed,
            },
        };

        serde_json::to_string_pretty(&export)
    }
}
```

### 6.4 Partial Trait Revelation

```rust
pub fn generate_insights(battle_result: &BattleResult, kaiju_a: &Kaiju, kaiju_b: &Kaiju) -> Vec<String> {
    let mut insights = Vec::new();

    // Analyze trait performance without revealing exact values
    if battle_result.winner == kaiju_a.name {
        if kaiju_a.has_trait_category(TraitCategory::Element) {
            insights.push("Winner's elemental traits performed effectively".to_string());
        }

        if kaiju_a.stats.speed > kaiju_b.stats.speed {
            insights.push("Speed advantage allowed winner to control tempo".to_string());
        }
    }

    // Environmental analysis
    insights.push("Consider environment matchups for future battles".to_string());

    insights
}
```

---

## 7. Replay System Implementation

### 7.1 Replay Function

```rust
pub fn replay_battle(
    battle_result: &BattleResult,
    kaiju_a: &Kaiju,
    kaiju_b: &Kaiju,
    environment: &Environment,
    config: &CombatConfig,
) -> BattleResult {
    execute_battle(
        kaiju_a.clone(),
        kaiju_b.clone(),
        environment.clone(),
        battle_result.seed,
        config,
    )
}

#[test]
fn test_replay_determinism() {
    let kaiju_a = create_test_kaiju("Flossy");
    let kaiju_b = create_test_kaiju("Reefmaw");
    let env = Environment::Storm;
    let config = CombatConfig::default();
    let seed = 12345;

    // Simulate battle
    let result1 = execute_battle(kaiju_a.clone(), kaiju_b.clone(), env.clone(), seed, &config);

    // Replay with same seed
    let result2 = replay_battle(&result1, &kaiju_a, &kaiju_b, &env, &config);

    // Verify identical outcomes
    assert_eq!(result1.winner, result2.winner);
    assert_eq!(result1.hp_remaining, result2.hp_remaining);
    assert_eq!(result1.turns_elapsed, result2.turns_elapsed);
    assert_eq!(result1.battle_log.len(), result2.battle_log.len());
}
```

### 7.2 Public Verification API

```rust
pub struct BattleVerifier {
    config: CombatConfig,
}

impl BattleVerifier {
    pub fn verify(&self, battle_result: &BattleResult, kaiju_a: &Kaiju, kaiju_b: &Kaiju, environment: &Environment) -> bool {
        let replayed = replay_battle(battle_result, kaiju_a, kaiju_b, environment, &self.config);

        replayed.winner == battle_result.winner &&
        replayed.hp_remaining == battle_result.hp_remaining &&
        replayed.turns_elapsed == battle_result.turns_elapsed
    }
}
```

---

## 8. Unit Tests for Damage Formulas

### 8.1 Test Suite Structure

```rust
#[cfg(test)]
mod combat_tests {
    use super::*;

    fn create_test_kaiju(name: &str, hp: i32, atk: i32, def: i32, spd: i32) -> Kaiju {
        Kaiju {
            name: name.to_string(),
            generation: 1,
            stats: KaijuStats { hp, attack: atk, defense: def, speed: spd },
            traits: Vec::new(),
            hidden_traits: Vec::new(),
            experience: 0,
            alive: true,
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
        assert!(damage >= 33 && damage <= 37);
    }

    #[test]
    fn test_minimum_damage_floor() {
        // Very weak attacker vs very strong defender
        let attacker = create_test_kaiju("A", 100, 10, 10, 10);
        let defender = create_test_kaiju("B", 100, 10, 100, 10);
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        let damage = calculate_damage(&attacker, &defender, &Environment::Neutral, &mut rng, &config);

        // Should be minimum 5 damage (configured minimum)
        assert!(damage >= 5);
    }

    #[test]
    fn test_trait_power_bonus() {
        let mut attacker = create_test_kaiju("A", 100, 50, 20, 30);
        attacker.traits.push(Trait {
            name: "Electric Breath".to_string(),
            category: TraitCategory::Element,
            power: 8,
            condition: None,
        });

        let defender = create_test_kaiju("B", 100, 40, 30, 25);
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        let damage = calculate_damage(&attacker, &defender, &Environment::Neutral, &mut rng, &config);

        // Expected: (50 - 15 + 8) * 1.0 * ~1.0 = ~43
        assert!(damage >= 41 && damage <= 45);
    }

    #[test]
    fn test_environment_multiplier() {
        let mut attacker = create_test_kaiju("A", 100, 50, 20, 30);
        attacker.traits.push(Trait {
            name: "Electric Breath".to_string(),
            category: TraitCategory::Element,
            power: 8,
            condition: None,
        });

        let defender = create_test_kaiju("B", 100, 40, 30, 25);
        let config = CombatConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        // Storm environment: Electric gets +15%
        let env = Environment::Storm;
        let damage = calculate_damage(&attacker, &defender, &env, &mut rng, &config);

        // Expected: (50 - 15 + 8) * 1.15 * ~1.0 = ~49
        assert!(damage >= 47 && damage <= 52);
    }

    #[test]
    fn test_defense_scaling() {
        let attacker = create_test_kaiju("A", 100, 100, 20, 30);
        let defender1 = create_test_kaiju("B1", 100, 40, 0, 25);
        let defender2 = create_test_kaiju("B2", 100, 40, 40, 25);
        let config = CombatConfig::default();

        let mut rng1 = ChaCha8Rng::seed_from_u64(0);
        let mut rng2 = ChaCha8Rng::seed_from_u64(0);

        let damage1 = calculate_damage(&attacker, &defender1, &Environment::Neutral, &mut rng1, &config);
        let damage2 = calculate_damage(&attacker, &defender2, &Environment::Neutral, &mut rng2, &config);

        // Defense should reduce damage by 40 * 0.5 = 20
        assert!((damage1 - damage2 - 20).abs() <= 2); // Allow ±2 for variance
    }
}
```

### 8.2 Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn damage_never_exceeds_reasonable_bounds(
            atk in 10i32..200,
            def in 5i32..100,
            trait_power in 0i32..15,
        ) {
            let mut attacker = create_test_kaiju("A", 100, atk, 20, 30);
            if trait_power > 0 {
                attacker.traits.push(Trait {
                    name: "Test".to_string(),
                    category: TraitCategory::Element,
                    power: trait_power,
                    condition: None,
                });
            }

            let defender = create_test_kaiju("B", 100, 40, def, 25);
            let config = CombatConfig::default();
            let mut rng = ChaCha8Rng::seed_from_u64(0);

            let damage = calculate_damage(&attacker, &defender, &Environment::Neutral, &mut rng, &config);

            // Damage should be positive and below a reasonable max
            prop_assert!(damage > 0);
            prop_assert!(damage < 1000);
        }
    }
}
```

---

## 9. Integration Tests for Full Battles

### 9.1 Full Battle Simulation

```rust
#[test]
fn test_full_battle_simulation() {
    let kaiju_a = Kaiju {
        name: "Flossy".to_string(),
        generation: 4,
        stats: KaijuStats { hp: 300, attack: 60, defense: 40, speed: 30 },
        traits: vec![Trait {
            name: "Electric Breath".to_string(),
            category: TraitCategory::Element,
            power: 8,
            condition: None,
        }],
        hidden_traits: Vec::new(),
        experience: 0,
        alive: true,
    };

    let kaiju_b = Kaiju {
        name: "Reefmaw".to_string(),
        generation: 4,
        stats: KaijuStats { hp: 320, attack: 55, defense: 45, speed: 25 },
        traits: vec![Trait {
            name: "Aqua Hide".to_string(),
            category: TraitCategory::Element,
            power: 6,
            condition: None,
        }],
        hidden_traits: Vec::new(),
        experience: 0,
        alive: true,
    };

    let config = CombatConfig::default();
    let seed = 1234567890;
    let result = execute_battle(kaiju_a, kaiju_b, Environment::Storm, seed, &config);

    // Verify battle completed
    assert!(!result.winner.is_empty());
    assert!(!result.loser.is_empty());
    assert!(result.hp_remaining >= 0);
    assert!(result.turns_elapsed > 0);
    assert!(result.turns_elapsed < 100); // Reasonable turn count
    assert!(!result.battle_log.is_empty());
}

#[test]
fn test_battle_ends_when_hp_reaches_zero() {
    let weak = create_test_kaiju("Weak", 10, 5, 5, 10);
    let strong = create_test_kaiju("Strong", 500, 100, 50, 50);

    let config = CombatConfig::default();
    let result = execute_battle(weak, strong, Environment::Neutral, 0, &config);

    // Weak should lose quickly
    assert_eq!(result.loser, "Weak");
    assert!(result.turns_elapsed < 10);
}

#[test]
fn test_speed_determines_first_attacker() {
    let fast = create_test_kaiju("Fast", 100, 50, 30, 100);
    let slow = create_test_kaiju("Slow", 100, 50, 30, 10);

    let config = CombatConfig::default();
    let result = execute_battle(fast.clone(), slow.clone(), Environment::Neutral, 0, &config);

    // Fast should attack first (check battle log)
    assert_eq!(result.battle_log[0].attacker_name, "Fast");
}

#[test]
fn test_timeout_protection() {
    // Create two kaiju that deal minimum damage to each other
    let tank_a = create_test_kaiju("A", 10000, 10, 100, 10);
    let tank_b = create_test_kaiju("B", 10000, 10, 100, 10);

    let config = CombatConfig {
        max_turns: 100,
        ..Default::default()
    };

    let result = execute_battle(tank_a, tank_b, Environment::Neutral, 0, &config);

    // Battle should timeout at max_turns
    assert!(result.turns_elapsed <= 100);
}
```

### 9.2 Environment Effect Tests

```rust
#[test]
fn test_volcanic_burn_damage() {
    let kaiju_a = create_test_kaiju("A", 1000, 50, 30, 30);
    let kaiju_b = create_test_kaiju("B", 1000, 50, 30, 30);

    let mut env = Environment::Volcanic;
    // Assume volcanic environment has 2% HP burn per turn

    let config = CombatConfig::default();
    let result = execute_battle(kaiju_a, kaiju_b, env, 0, &config);

    // Check battle log for burn damage entries
    let burn_effects: Vec<_> = result.battle_log.iter()
        .flat_map(|entry| &entry.special_effects)
        .filter(|effect| effect.contains("burn"))
        .collect();

    assert!(!burn_effects.is_empty(), "Volcanic burn should appear in log");
}

#[test]
fn test_environment_trait_synergy() {
    // Electric trait in Storm should deal more damage than in Neutral
    let mut electric_kaiju = create_test_kaiju("Electric", 300, 60, 40, 30);
    electric_kaiju.traits.push(Trait {
        name: "Electric Breath".to_string(),
        category: TraitCategory::Element,
        power: 8,
        condition: None,
    });

    let defender = create_test_kaiju("Defender", 500, 40, 40, 25);

    let config = CombatConfig::default();

    // Battle in Neutral
    let result_neutral = execute_battle(
        electric_kaiju.clone(),
        defender.clone(),
        Environment::Neutral,
        12345,
        &config
    );

    // Battle in Storm
    let result_storm = execute_battle(
        electric_kaiju.clone(),
        defender.clone(),
        Environment::Storm,
        12345,
        &config
    );

    // Storm should result in faster victory (fewer turns)
    assert!(result_storm.turns_elapsed <= result_neutral.turns_elapsed);
}
```

---

## 10. Performance Targets and Optimization

### 10.1 Performance Target: <100ms per battle

```rust
#[test]
fn test_battle_performance() {
    use std::time::Instant;

    let kaiju_a = create_test_kaiju("A", 300, 60, 40, 30);
    let kaiju_b = create_test_kaiju("B", 320, 55, 45, 25);

    let config = CombatConfig::default();

    let start = Instant::now();
    let _result = execute_battle(kaiju_a, kaiju_b, Environment::Neutral, 0, &config);
    let duration = start.elapsed();

    println!("Battle took: {:?}", duration);
    assert!(duration.as_millis() < 100, "Battle took too long: {:?}", duration);
}

#[test]
fn test_1000_battle_throughput() {
    let kaiju_a = create_test_kaiju("A", 300, 60, 40, 30);
    let kaiju_b = create_test_kaiju("B", 320, 55, 45, 25);

    let config = CombatConfig::default();

    let start = Instant::now();
    for i in 0..1000 {
        let _result = execute_battle(
            kaiju_a.clone(),
            kaiju_b.clone(),
            Environment::Neutral,
            i,
            &config
        );
    }
    let duration = start.elapsed();

    let avg_ms = duration.as_millis() / 1000;
    println!("Average battle time: {}ms", avg_ms);
    assert!(avg_ms < 100, "Average battle too slow: {}ms", avg_ms);
}
```

### 10.2 Optimization Strategies

**Memory Allocation**:
- Pre-allocate battle log Vec with estimated capacity
- Reuse RNG instances where possible
- Avoid unnecessary cloning in hot paths

**Computation**:
- Cache effective speed calculations
- Cache environment multipliers per kaiju
- Use integer math where possible (avoid f32 conversions)

**Profiling**:
```rust
// Use cargo-flamegraph for profiling
// cargo install flamegraph
// cargo flamegraph --test combat_tests -- test_full_battle_simulation
```

---

## 11. Critical Files to Create

Based on this implementation plan, here are the critical files needed:

### 1. **H:\RustGames\kaiju_sim\src\engine\combat.rs**
**Purpose**: Core combat simulation engine
**Critical Functions**:
- `calculate_damage()` - 7-step damage formula
- `execute_battle()` - Main battle loop
- `determine_turn_order()` - Speed-based initiative
- `calculate_trait_bonuses()` - Element trait power summation
- `calculate_environment_multiplier()` - Environment synergy calculation

**Lines of Code Estimate**: 400-500 lines

---

### 2. **H:\RustGames\kaiju_sim\src\data\environments.rs**
**Purpose**: Environment definitions and multiplier logic
**Critical Components**:
- `Environment` enum (9 environments)
- `EnvironmentDefinition` struct
- `PerTurnEffect` enum and implementation
- JSON loading for environment configurations
- Trait-to-environment matching logic

**Lines of Code Estimate**: 200-300 lines

---

### 3. **H:\RustGames\kaiju_sim\src\state\battle_state.rs**
**Purpose**: Battle state tracking and log management
**Critical Components**:
- `BattleState` struct (HP tracking, turn count, logs)
- `BattleLogEntry` struct
- `BattleResult` struct
- Log formatting functions (human-readable and JSON)
- Battle insights generation

**Lines of Code Estimate**: 250-350 lines

---

### 4. **H:\RustGames\kaiju_sim\assets\balance.json**
**Purpose**: Combat configuration and balance values
**Critical Data**:
- Damage formula constants (defense_scaling: 0.5, minimum_damage: 5)
- Variance range (min: 0.95, max: 1.05)
- Max turns limit (1000)
- Environment multipliers for all 9 environments

**Lines of Code Estimate**: 100-150 lines (JSON)

---

### 5. **H:\RustGames\kaiju_sim\src\data\combat_config.rs**
**Purpose**: Combat configuration structures
**Critical Components**:
- `CombatConfig` struct
- Default implementations
- JSON deserialization for balance.json
- Validation logic for configuration values

**Lines of Code Estimate**: 100-150 lines

---

## 12. Implementation Sequence

### Week 1: Core Combat Engine

**Day 1-2: Damage Calculation**
- Implement `calculate_damage()` with 7-step formula
- Create `CombatConfig` struct and loading
- Write unit tests for each step
- Achieve 100% test coverage on damage calculation

**Day 3-4: Battle Simulation Loop**
- Implement `execute_battle()` main loop
- Create `BattleState` tracking
- Implement turn order determination
- Add battle log generation
- Write integration tests for full battles

**Day 5: Deterministic RNG**
- Integrate ChaCha8Rng
- Implement seed storage and replay
- Test replay determinism
- Verify variance bounds (±5%)

### Week 2: Environments and Polish

**Day 6-7: Environment System**
- Create `Environment` enum and definitions
- Implement environment multiplier logic
- Add per-turn effects (volcanic burn, radiation)
- Load environment data from JSON
- Write environment-specific tests

**Day 8: Battle Logging and Reporting**
- Implement battle log formatting (human-readable)
- Add JSON export functionality
- Create battle insights generation
- Add partial trait revelation hints

**Day 9: Performance Optimization**
- Profile battle simulation
- Optimize hot paths (<100ms target)
- Test throughput (1000 battles)
- Verify memory efficiency

**Day 10: Integration and Documentation**
- Integrate with Phase 2 (trait system)
- Write API documentation
- Create example battles
- Final testing and bug fixes

---

## 13. Testing Strategy Summary

### Unit Test Coverage (Target: 95%)

**Damage Calculation** (15 tests):
- Base damage formula
- Minimum damage floor
- Defense scaling (50% effectiveness)
- Trait power bonuses
- Environment multipliers
- Variance bounds (±5%)
- Edge cases (zero attack, max defense)

**Turn Order** (8 tests):
- Speed-based initiative
- Tie-breaking (attack, then random)
- Speed modifier traits
- Equal stats handling

**Environment Effects** (12 tests):
- All 9 environment multipliers
- Per-turn effects (burn, mutation chance)
- Trait-environment synergy
- Multiple trait combinations

### Integration Tests (Target: 20 tests)

**Full Battle Simulation** (10 tests):
- Standard battle (balanced kaiju)
- One-sided battle (strong vs weak)
- Timeout scenario (defensive builds)
- Speed advantage scenarios
- Environment-specific battles

**Replay System** (5 tests):
- Deterministic replay verification
- Multiple replays produce same result
- Seed variation produces different results
- Replay with different environments
- Replay with modified kaiju (should differ)

**Performance Tests** (5 tests):
- Single battle <100ms
- 1000 battles throughput
- Memory usage monitoring
- Log generation overhead
- JSON serialization performance

### Manual Testing Checklist

- [ ] Battle between two Gen-0 kaiju
- [ ] Battle with extreme stat differences
- [ ] All 9 environments tested
- [ ] Trait activation in combat
- [ ] Hidden trait revelation (future)
- [ ] Battle log readability
- [ ] JSON export integrity
- [ ] Replay system verification
- [ ] Performance profiling (flamegraph)
- [ ] Memory leak detection (valgrind/heaptrack)

---

## 14. Dependencies and Cargo.toml

```toml
[dependencies]
macroquad = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
rand_chacha = "0.3"

[dev-dependencies]
proptest = "1.0"  # Property-based testing
criterion = "0.5"  # Benchmarking
```

---

## 15. Risks and Mitigation

### Risk 1: Performance Bottlenecks
**Impact**: Battles take >100ms
**Mitigation**:
- Profile early with cargo-flamegraph
- Optimize trait lookup (use HashMap)
- Cache environment multipliers
- Use integer math where possible

### Risk 2: Determinism Failures
**Impact**: Replays produce different results
**Mitigation**:
- Use ChaCha8Rng (cryptographically secure)
- Store all random seeds
- Test replay on every build
- Add determinism assertions to all tests

### Risk 3: Balance Issues
**Impact**: Certain traits/environments dominate
**Mitigation**:
- Data-driven design (easy tuning in JSON)
- Statistical analysis of 10,000+ simulated battles
- Track win rates by trait/environment
- Regular balance passes

### Risk 4: Integration with Phase 2
**Impact**: Trait system incompatibility
**Mitigation**:
- Define trait interfaces early
- Mock trait system for Phase 3 development
- Write integration tests before Phase 2 completion
- Regular cross-phase communication

---

## 16. Success Criteria

Phase 3 is complete when:

- [ ] All damage formula tests pass (95%+ coverage)
- [ ] Full battle simulation works end-to-end
- [ ] Replay system produces identical results
- [ ] All 9 environments implemented and tested
- [ ] Performance target met (<100ms per battle)
- [ ] Battle logs are human-readable and JSON-exportable
- [ ] Integration tests pass with Phase 2 trait system
- [ ] Documentation complete (API docs + examples)
- [ ] No clippy warnings
- [ ] Code review approved by project lead

---

## Conclusion

This implementation plan provides a complete roadmap for Phase 3 (Combat Engine) with:

- **7-step damage formula** with unit tests
- **9 environments** with trait synergies
- **Deterministic replay** using ChaCha8Rng
- **Battle logging** with JSON export
- **Performance target** <100ms per battle
- **95%+ test coverage** for core systems

Following this plan will result in a robust, performant, and fully tested combat engine that integrates seamlessly with Phase 2 (Genetics) and prepares for Phase 4 (Tournaments).
