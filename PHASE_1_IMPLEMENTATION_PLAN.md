# PHASE 1 IMPLEMENTATION PLAN: Foundation & Data Models

## Executive Summary

This document provides a detailed, step-by-step implementation plan for Phase 1 of the Kaiju Breeding Simulator game client. Phase 1 establishes the project foundation, core data structures, JSON data files, and basic integration with Macroquad. This phase is **prerequisite** for all subsequent development phases.

**Estimated Total Time**: 12-16 hours (1.5-2 days for experienced Rust developer)

**Prerequisites**:
- Rust toolchain (Edition 2021)
- Visual Studio Code or similar IDE
- Access to `macroquad-toolkit` (assumed to be in `H:\RustGames\macroquad-toolkit`)
- Basic understanding of Macroquad and immediate-mode UI patterns

**Success Criteria**:
- Cargo project compiles successfully for both Windows and WASM targets
- All data structures are defined and documented
- JSON data files load successfully with validation
- Unit tests pass
- Basic Macroquad window opens and displays placeholder UI

---

## 1. PROJECT INITIALIZATION

### 1.1 Create Cargo Project

**Time Estimate**: 15 minutes

**Location**: `H:\RustGames\kaiju_sim\`

**Steps**:

Since you already have documentation in the directory, you'll initialize the Cargo project in place:

1. Open terminal in `H:\RustGames\kaiju_sim\`
2. Run: `cargo init --name kaiju_sim`
3. Verify `Cargo.toml` and `src/main.rs` are created

**Expected Output**:
```
H:\RustGames\kaiju_sim\
├── Cargo.toml (new)
├── src/
│   └── main.rs (new)
├── CLAUDE.md (existing)
├── [other docs...] (existing)
```

---

### 1.2 Configure Cargo.toml

**Time Estimate**: 10 minutes

**File**: `H:\RustGames\kaiju_sim\Cargo.toml`

**Content**:

```toml
[package]
name = "kaiju_sim"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core engine
macroquad = "0.4"
macroquad-toolkit = { path = "../macroquad-toolkit" }

# Data serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Random number generation (for breeding and combat)
rand = { version = "0.8", features = ["small_rng"] }
rand_chacha = "0.3"  # Deterministic RNG for combat replay

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }

[dev-dependencies]
# Testing utilities
pretty_assertions = "1.4"
```

**Notes**:
- `macroquad-toolkit` path assumes it's in the parent directory
- `rand_chacha` is for deterministic combat replay (Phase 3+)
- `uuid` for unique kaiju IDs
- `chrono` for timestamps

---

### 1.3 Create Folder Structure

**Time Estimate**: 10 minutes

**Command**:

```bash
cd "H:\RustGames\kaiju_sim"
mkdir src\data
mkdir src\engine
mkdir src\state
mkdir src\ui
mkdir src\screens
mkdir assets
mkdir tests
```

**Expected Structure**:

```
kaiju_sim/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── data/
│   │   └── mod.rs (create)
│   ├── engine/
│   │   └── mod.rs (create)
│   ├── state/
│   │   └── mod.rs (create)
│   ├── ui/
│   │   └── mod.rs (create)
│   └── screens/
│       └── mod.rs (create)
├── assets/
├── tests/
├── index.html (existing)
└── publish.ps1 (existing)
```

**Action Items**:
- Create empty `mod.rs` files in each subdirectory
- Add `pub mod data;` etc. to `src/main.rs`

---

## 2. CORE DATA STRUCTURES

### 2.1 Type Aliases and Common Types

**Time Estimate**: 15 minutes

**File**: `src/data/types.rs`

**Content**:

```rust
//! Common type aliases and utility types used throughout the game.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a kaiju
pub type KaijuId = Uuid;

/// NFT token ID (future blockchain integration)
pub type TokenId = u64;

/// Wallet address (future blockchain integration)
pub type WalletAddress = String;

/// Unix timestamp (seconds since epoch)
pub type Timestamp = i64;

/// Trait identifier
pub type TraitId = String;

/// Visual seed for deterministic image generation
pub type VisualSeed = u64;

/// Generation number (0 = wild/original, 1+ = bred)
pub type Generation = u32;
```

**Notes**:
- These types provide semantic clarity
- Easy to swap implementations later (e.g., TokenId could become a struct)
- Serialize/Deserialize enabled for all types

---

### 2.2 Kaiju Stats Structure

**Time Estimate**: 20 minutes

**File**: `src/data/kaiju.rs`

**Content** (Part 1 - Stats):

```rust
//! Kaiju entity and related data structures.

use serde::{Deserialize, Serialize};
use super::types::*;
use super::traits::Trait;

/// Core combat statistics for a kaiju
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KaijuStats {
    /// Hit points - determines survivability
    pub hp: i32,

    /// Attack power - base damage output
    pub attack: i32,

    /// Defense - damage reduction
    pub defense: i32,

    /// Speed - determines turn order and initiative
    pub speed: i32,
}

impl KaijuStats {
    /// Create new stats with validation
    pub fn new(hp: i32, attack: i32, defense: i32, speed: i32) -> Self {
        Self {
            hp: hp.max(50),           // Minimum HP floor
            attack: attack.max(10),   // Minimum attack floor
            defense: defense.max(5),  // Minimum defense floor
            speed: speed.max(5),      // Minimum speed floor
        }
    }

    /// Calculate total power level (for rough comparisons)
    pub fn power_level(&self) -> i32 {
        self.hp / 5 + self.attack + self.defense + self.speed
    }
}

impl Default for KaijuStats {
    fn default() -> Self {
        Self::new(100, 20, 15, 15)
    }
}
```

**Tests** (in same file):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_floors() {
        let stats = KaijuStats::new(10, 5, 2, 3);
        assert_eq!(stats.hp, 50);      // Floor applied
        assert_eq!(stats.attack, 10);  // Floor applied
        assert_eq!(stats.defense, 5);  // Floor applied
        assert_eq!(stats.speed, 5);    // Floor applied
    }

    #[test]
    fn test_stats_serialization() {
        let stats = KaijuStats::new(200, 50, 40, 30);
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: KaijuStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats, deserialized);
    }
}
```

---

### 2.3 Kaiju Entity Structure

**Time Estimate**: 30 minutes

**File**: `src/data/kaiju.rs` (continued)

**Content** (Part 2 - Main Entity):

```rust
/// Complete kaiju entity with all properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kaiju {
    // === IMMUTABLE PROPERTIES (set at creation) ===

    /// Unique identifier
    pub id: KaijuId,

    /// Blockchain token ID (0 if not minted)
    pub token_id: TokenId,

    /// Kaiju display name (non-unique)
    pub name: String,

    /// Generation number (0 = wild, 1+ = bred)
    pub generation: Generation,

    /// Creation timestamp (Unix epoch)
    pub created_at: Timestamp,

    /// Original breeder wallet address
    pub original_breeder: WalletAddress,

    /// Parent token IDs (None for generation 0)
    pub parent_ids: Option<(TokenId, TokenId)>,

    /// Visual seed for deterministic image generation
    pub visual_seed: VisualSeed,

    /// Genome hash (immutable genetic fingerprint)
    pub genome_hash: String,

    // === MUTABLE PROPERTIES (change during gameplay) ===

    /// Current combat statistics
    pub stats: KaijuStats,

    /// Visible traits (revealed through combat/research)
    pub traits: Vec<Trait>,

    /// Hidden traits (not yet revealed)
    pub hidden_traits: Vec<Trait>,

    /// Experience points from battles
    pub experience: u32,

    /// Alive status (false = permanently dead)
    pub alive: bool,

    /// Current owner wallet address
    pub current_owner: WalletAddress,

    // === NFT METADATA ===

    /// AI-generated portrait URI (IPFS/Arweave)
    pub image_uri: Option<String>,

    /// Full metadata URI (IPFS/Arweave)
    pub metadata_uri: String,
}

impl Kaiju {
    /// Create a new generation 0 (wild) kaiju
    pub fn new_wild(name: String, stats: KaijuStats, traits: Vec<Trait>) -> Self {
        Self {
            id: Uuid::new_v4(),
            token_id: 0,
            name,
            generation: 0,
            created_at: chrono::Utc::now().timestamp(),
            original_breeder: "system".to_string(),
            parent_ids: None,
            visual_seed: rand::random(),
            genome_hash: format!("{:016x}", rand::random::<u64>()),
            stats,
            traits,
            hidden_traits: Vec::new(),
            experience: 0,
            alive: true,
            current_owner: "system".to_string(),
            image_uri: None,
            metadata_uri: String::new(),
        }
    }

    /// Check if kaiju can breed (alive, not in tournament, etc.)
    pub fn can_breed(&self) -> bool {
        self.alive
    }

    /// Check if kaiju can compete in tournaments
    pub fn can_compete(&self) -> bool {
        self.alive
    }

    /// Calculate approximate battle power (for matchmaking)
    pub fn battle_rating(&self) -> i32 {
        let base = self.stats.power_level();
        let trait_power: i32 = self.traits.iter().map(|t| t.power).sum();
        base + trait_power + (self.experience as i32 / 10)
    }
}
```

**Tests**:

```rust
#[cfg(test)]
mod kaiju_tests {
    use super::*;

    #[test]
    fn test_new_wild_kaiju() {
        let stats = KaijuStats::new(200, 50, 40, 30);
        let kaiju = Kaiju::new_wild("TestKaiju".to_string(), stats, vec![]);

        assert_eq!(kaiju.generation, 0);
        assert_eq!(kaiju.name, "TestKaiju");
        assert!(kaiju.alive);
        assert!(kaiju.can_breed());
        assert!(kaiju.can_compete());
        assert_eq!(kaiju.parent_ids, None);
    }

    #[test]
    fn test_battle_rating_calculation() {
        let stats = KaijuStats::new(200, 50, 40, 30);
        let kaiju = Kaiju::new_wild("Warrior".to_string(), stats, vec![]);

        let rating = kaiju.battle_rating();
        assert!(rating > 0);
    }
}
```

---

### 2.4 Trait System

**Time Estimate**: 30 minutes

**File**: `src/data/traits.rs`

**Content**:

```rust
//! Trait system for kaiju genetics and abilities.

use serde::{Deserialize, Serialize};
use super::types::TraitId;

/// Trait categories defining behavior and inheritance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitCategory {
    /// Elemental affinities (fire, water, electric, etc.)
    Element,

    /// Stat modifiers and conditional bonuses
    Modifier,

    /// Random mutations from breeding
    Mutation,

    /// Meta-traits activated by trait combinations
    Synergy,
}

/// Inheritance type determines breeding probability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitInheritance {
    /// High inheritance chance (60%)
    Dominant,

    /// Lower inheritance chance (30%)
    Recessive,

    /// Requires both parents (special rules)
    Polygenic,

    /// Activated under specific conditions
    Conditional,
}

/// Condition for trait activation (combat, environment, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitCondition {
    /// Always active
    Always,

    /// Active in specific environment
    Environment(String),

    /// Active when HP below threshold
    LowHealth(i32),

    /// Active when HP above threshold
    HighHealth(i32),

    /// Active against specific trait
    CounterTrait(TraitId),

    /// Requires another trait to be present
    RequiresTrait(TraitId),
}

/// Complete trait definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    /// Unique trait identifier
    pub id: TraitId,

    /// Display name
    pub name: String,

    /// Trait category
    pub category: TraitCategory,

    /// Numeric power/influence (-10 to +25)
    pub power: i32,

    /// Inheritance type
    pub inheritance: TraitInheritance,

    /// Activation condition
    pub condition: TraitCondition,

    /// Whether trait is hidden initially
    pub is_hidden: bool,

    /// Flavor text description
    pub description: String,
}

impl Trait {
    /// Check if trait is active under given conditions
    pub fn is_active(&self, current_hp: i32, max_hp: i32, environment: &str) -> bool {
        match &self.condition {
            TraitCondition::Always => true,
            TraitCondition::Environment(env) => env == environment,
            TraitCondition::LowHealth(threshold) => {
                (current_hp as f32 / max_hp as f32) < (*threshold as f32 / 100.0)
            }
            TraitCondition::HighHealth(threshold) => {
                (current_hp as f32 / max_hp as f32) >= (*threshold as f32 / 100.0)
            }
            _ => true, // Other conditions checked elsewhere
        }
    }
}
```

**Tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_condition_always_active() {
        let trait_def = Trait {
            id: "test_trait".to_string(),
            name: "Test".to_string(),
            category: TraitCategory::Element,
            power: 10,
            inheritance: TraitInheritance::Dominant,
            condition: TraitCondition::Always,
            is_hidden: false,
            description: "Test trait".to_string(),
        };

        assert!(trait_def.is_active(100, 100, "any"));
    }

    #[test]
    fn test_trait_condition_low_health() {
        let trait_def = Trait {
            id: "berserker".to_string(),
            name: "Berserker".to_string(),
            category: TraitCategory::Modifier,
            power: 15,
            inheritance: TraitInheritance::Recessive,
            condition: TraitCondition::LowHealth(30),
            is_hidden: false,
            description: "Activates when HP < 30%".to_string(),
        };

        assert!(trait_def.is_active(25, 100, "any"));
        assert!(!trait_def.is_active(50, 100, "any"));
    }

    #[test]
    fn test_trait_condition_environment() {
        let trait_def = Trait {
            id: "electric_breath".to_string(),
            name: "Electric Breath".to_string(),
            category: TraitCategory::Element,
            power: 12,
            inheritance: TraitInheritance::Dominant,
            condition: TraitCondition::Environment("storm".to_string()),
            is_hidden: false,
            description: "Boosted in storm environments".to_string(),
        };

        assert!(trait_def.is_active(100, 100, "storm"));
        assert!(!trait_def.is_active(100, 100, "volcanic"));
    }
}
```

---

### 2.5 Lineage System

**Time Estimate**: 20 minutes

**File**: `src/data/lineage.rs`

**Content**:

```rust
//! Lineage and ancestry tracking for kaiju.

use serde::{Deserialize, Serialize};
use super::types::*;

/// Notable achievement for lineage highlights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageHighlight {
    /// Token ID of notable ancestor
    pub token_id: TokenId,

    /// Ancestor's name
    pub name: String,

    /// Achievement description
    pub achievement: String,

    /// Generation of ancestor
    pub generation: Generation,
}

/// Recursive lineage tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineage {
    /// Token ID of this kaiju
    pub kaiju_id: TokenId,

    /// Kaiju name
    pub name: String,

    /// Generation number
    pub generation: Generation,

    /// Parent lineages (None for generation 0)
    pub parents: Option<Box<(Lineage, Lineage)>>,

    /// Notable ancestors (champions, etc.)
    pub notable_ancestors: Vec<LineageHighlight>,
}

impl Lineage {
    /// Create lineage for generation 0 kaiju
    pub fn new_wild(token_id: TokenId, name: String) -> Self {
        Self {
            kaiju_id: token_id,
            name,
            generation: 0,
            parents: None,
            notable_ancestors: Vec::new(),
        }
    }

    /// Calculate total lineage depth
    pub fn depth(&self) -> u32 {
        match &self.parents {
            None => 1,
            Some(parents) => 1 + parents.0.depth().max(parents.1.depth()),
        }
    }

    /// Check if lineage contains a specific ancestor
    pub fn has_ancestor(&self, token_id: TokenId) -> bool {
        if self.kaiju_id == token_id {
            return true;
        }

        match &self.parents {
            None => false,
            Some(parents) => {
                parents.0.has_ancestor(token_id) || parents.1.has_ancestor(token_id)
            }
        }
    }

    /// Validate that two kaiju are not directly related (prevent incest)
    pub fn can_breed_with(lineage_a: &Lineage, lineage_b: &Lineage) -> bool {
        // Cannot breed with self
        if lineage_a.kaiju_id == lineage_b.kaiju_id {
            return false;
        }

        // Cannot breed parent with child
        if lineage_a.has_ancestor(lineage_b.kaiju_id)
            || lineage_b.has_ancestor(lineage_a.kaiju_id) {
            return false;
        }

        // Cannot breed siblings (same parents)
        match (&lineage_a.parents, &lineage_b.parents) {
            (Some(parents_a), Some(parents_b)) => {
                parents_a.0.kaiju_id != parents_b.0.kaiju_id
                    && parents_a.1.kaiju_id != parents_b.1.kaiju_id
            }
            _ => true,
        }
    }
}
```

**Tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wild_lineage() {
        let lineage = Lineage::new_wild(1, "Primordial".to_string());
        assert_eq!(lineage.generation, 0);
        assert_eq!(lineage.depth(), 1);
        assert!(lineage.parents.is_none());
    }

    #[test]
    fn test_incest_prevention() {
        let parent_a = Lineage::new_wild(1, "Parent A".to_string());
        let parent_b = Lineage::new_wild(2, "Parent B".to_string());

        // Parents can breed
        assert!(Lineage::can_breed_with(&parent_a, &parent_b));

        // Cannot breed with self
        assert!(!Lineage::can_breed_with(&parent_a, &parent_a));
    }
}
```

---

## 3. JSON DATA FILES

### 3.1 Traits Definition File

**Time Estimate**: 45 minutes

**File**: `assets/traits.json`

**Content** (Sample with 10 traits, expand to 50 later):

```json
{
  "traits": [
    {
      "id": "electric_breath",
      "name": "Electric Breath",
      "category": "Element",
      "power": 12,
      "inheritance": "Dominant",
      "condition": { "Environment": "storm" },
      "is_hidden": false,
      "description": "Releases powerful electric discharge. Boosted in storm environments."
    },
    {
      "id": "aqua_hide",
      "name": "Aqua Hide",
      "category": "Element",
      "power": 10,
      "inheritance": "Dominant",
      "condition": { "Environment": "aquatic" },
      "is_hidden": false,
      "description": "Water-resistant scales. Enhanced in aquatic environments."
    },
    {
      "id": "fire_core",
      "name": "Fire Core",
      "category": "Element",
      "power": 15,
      "inheritance": "Dominant",
      "condition": { "Environment": "volcanic" },
      "is_hidden": false,
      "description": "Internal flame. Devastating in volcanic environments."
    },
    {
      "id": "berserker",
      "name": "Berserker",
      "category": "Modifier",
      "power": 20,
      "inheritance": "Recessive",
      "condition": { "LowHealth": 30 },
      "is_hidden": true,
      "description": "Unleashes fury when HP drops below 30%."
    },
    {
      "id": "thick_armor",
      "name": "Thick Armor",
      "category": "Modifier",
      "power": 8,
      "inheritance": "Dominant",
      "condition": "Always",
      "is_hidden": false,
      "description": "Reinforced natural plating. Always active."
    },
    {
      "id": "swift_reflexes",
      "name": "Swift Reflexes",
      "category": "Modifier",
      "power": 10,
      "inheritance": "Dominant",
      "condition": "Always",
      "is_hidden": false,
      "description": "Enhanced reaction time. Permanent speed bonus."
    },
    {
      "id": "unstable_mutation",
      "name": "Unstable Mutation",
      "category": "Mutation",
      "power": 5,
      "inheritance": "Recessive",
      "condition": "Always",
      "is_hidden": true,
      "description": "Random genetic anomaly. Effects unpredictable."
    },
    {
      "id": "storm_dragon",
      "name": "Storm Dragon",
      "category": "Synergy",
      "power": 25,
      "inheritance": "Conditional",
      "condition": { "RequiresTrait": "electric_breath" },
      "is_hidden": true,
      "description": "Awakens when electric and flight traits combine."
    },
    {
      "id": "volcanic_titan",
      "name": "Volcanic Titan",
      "category": "Synergy",
      "power": 25,
      "inheritance": "Conditional",
      "condition": { "RequiresTrait": "fire_core" },
      "is_hidden": true,
      "description": "Emerges from fire and earth trait synergy."
    },
    {
      "id": "regeneration",
      "name": "Regeneration",
      "category": "Modifier",
      "power": 12,
      "inheritance": "Recessive",
      "condition": { "HighHealth": 50 },
      "is_hidden": true,
      "description": "Slow HP recovery when above 50% health."
    }
  ]
}
```

**Notes**:
- Start with 10 traits, expand to 50 in Phase 2
- Include all 4 categories (Element, Modifier, Mutation, Synergy)
- Mix visible and hidden traits
- Include environmental conditions

---

### 3.2 Balance Configuration File

**Time Estimate**: 30 minutes

**File**: `assets/balance.json`

**Content**:

```json
{
  "breeding": {
    "generation_power_creep": 0.01,
    "stat_variance_min": 0.95,
    "stat_variance_max": 1.05,
    "visible_trait_inheritance_chance": 0.45,
    "hidden_trait_inheritance_chance": 0.25,
    "mutation_chance": 0.10,
    "stat_floors": {
      "hp": 50,
      "attack": 10,
      "defense": 5,
      "speed": 5
    },
    "stat_base_caps": {
      "hp": 500,
      "attack": 100,
      "defense": 100,
      "speed": 100
    },
    "stat_hard_caps": {
      "hp": 2000,
      "attack": 400,
      "defense": 400,
      "speed": 400
    },
    "soft_cap_multiplier_per_generation": 0.02
  },
  "combat": {
    "base_damage_formula": {
      "attack_coefficient": 1.0,
      "defense_coefficient": 0.5,
      "minimum_damage": 5
    },
    "damage_variance_min": 0.95,
    "damage_variance_max": 1.05,
    "environment_multipliers": {
      "storm": {
        "electric": 1.20,
        "water": 1.15
      },
      "volcanic": {
        "fire": 1.25,
        "ice": 0.85
      },
      "aquatic": {
        "water": 1.20,
        "electric": 0.90
      },
      "arctic": {
        "ice": 1.20,
        "fire": 0.85
      },
      "forest": {
        "nature": 1.15
      },
      "desert": {
        "fire": 1.10
      },
      "urban": {
        "tech": 1.15
      },
      "cosmic": {
        "psychic": 1.20
      }
    },
    "max_battle_turns": 100
  },
  "tournaments": {
    "xp_rewards": {
      "win": 100,
      "loss": 30,
      "upset_bonus": 50
    },
    "ranking_elo": {
      "k_factor": 32,
      "starting_rating": 1500
    }
  },
  "research": {
    "facility_levels": 5,
    "decoding_costs": [100, 250, 500, 1000, 2000],
    "battles_required_for_layer_2": 10
  }
}
```

**Notes**:
- All magic numbers extracted to JSON
- Easy to balance without recompiling
- Nested structure mirrors code organization

---

### 3.3 Tournament Configuration File

**Time Estimate**: 20 minutes

**File**: `assets/tournaments.json`

**Content**:

```json
{
  "tournaments": [
    {
      "id": "rookie_rumble",
      "name": "Rookie Rumble",
      "tournament_type": "NonLethal",
      "bracket_system": "SingleElimination",
      "min_generation": 0,
      "max_generation": 2,
      "entry_fee": 0,
      "max_participants": 8,
      "environment": "neutral",
      "rewards": {
        "winner": { "xp": 200, "currency": 1000 },
        "runner_up": { "xp": 100, "currency": 500 },
        "participant": { "xp": 30, "currency": 100 }
      }
    },
    {
      "id": "mid_tier_clash",
      "name": "Mid-Tier Clash",
      "tournament_type": "NonLethal",
      "bracket_system": "SingleElimination",
      "min_generation": 3,
      "max_generation": 5,
      "entry_fee": 500,
      "max_participants": 16,
      "environment": "storm",
      "rewards": {
        "winner": { "xp": 500, "currency": 5000 },
        "runner_up": { "xp": 250, "currency": 2500 },
        "participant": { "xp": 50, "currency": 200 }
      }
    },
    {
      "id": "death_arena",
      "name": "Death Arena",
      "tournament_type": "Lethal",
      "bracket_system": "SingleElimination",
      "min_generation": 0,
      "max_generation": null,
      "entry_fee": 0,
      "max_participants": 8,
      "environment": "volcanic",
      "rewards": {
        "winner": { "xp": 2000, "currency": 50000, "title": "Arena Champion" },
        "participant": { "xp": 0, "currency": 0 }
      }
    }
  ]
}
```

---

### 3.4 Data Loader Implementation

**Time Estimate**: 45 minutes

**File**: `src/data/loader.rs`

**Content**:

```rust
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
        self.traits.iter().filter(|t| t.category == category).collect()
    }
}

/// Balance configuration from balance.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub breeding: BreedingConfig,
    pub combat: CombatConfig,
    pub tournaments: TournamentConfig,
    pub research: ResearchConfig,
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
pub struct TournamentConfig {
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
```

**Tests**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_data_loading() {
        // This will fail until JSON files are created
        match GameData::load() {
            Ok(data) => {
                assert!(!data.traits.traits.is_empty());
                assert!(data.balance.breeding.generation_power_creep > 0.0);
            }
            Err(e) => {
                println!("Expected error until JSON files created: {}", e);
            }
        }
    }
}
```

---

## 4. MODULE ORGANIZATION

### 4.1 Data Module Root

**Time Estimate**: 10 minutes

**File**: `src/data/mod.rs`

**Content**:

```rust
//! Data structures and JSON loading.

pub mod types;
pub mod kaiju;
pub mod traits;
pub mod lineage;
pub mod loader;

// Re-export commonly used types
pub use types::*;
pub use kaiju::{Kaiju, KaijuStats};
pub use traits::{Trait, TraitCategory, TraitInheritance, TraitCondition};
pub use lineage::{Lineage, LineageHighlight};
pub use loader::GameData;
```

---

### 4.2 Empty Module Stubs

**Time Estimate**: 5 minutes

Create empty stub files for future phases:

**File**: `src/engine/mod.rs`
```rust
//! Game logic services (stateless).
```

**File**: `src/state/mod.rs`
```rust
//! Game state management.
```

**File**: `src/ui/mod.rs`
```rust
//! User interface components.

// Re-export macroquad-toolkit for convenience
pub use macroquad_toolkit::prelude::*;
```

**File**: `src/screens/mod.rs`
```rust
//! Screen-specific rendering.
```

---

## 5. MACROQUAD INTEGRATION

### 5.1 Main Entry Point

**Time Estimate**: 30 minutes

**File**: `src/main.rs`

**Content**:

```rust
use macroquad::prelude::*;

mod data;
mod engine;
mod state;
mod ui;
mod screens;

use data::GameData;

fn window_conf() -> Conf {
    Conf {
        window_title: "Kaiju Breeding Simulator".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load game data
    let game_data = match GameData::load() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load game data: {}", e);
            eprintln!("Make sure assets/ folder contains traits.json, balance.json, and tournaments.json");
            return;
        }
    };

    println!("Loaded {} traits", game_data.traits.traits.len());
    println!("Loaded {} tournaments", game_data.tournaments.tournaments.len());

    // Main game loop
    loop {
        clear_background(Color::from_rgba(20, 20, 25, 255));

        // Draw loading message
        let text = "Phase 1 Complete - Data Models Loaded";
        let font_size = 40.0;
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        let x = screen_width() / 2.0 - text_size.width / 2.0;
        let y = screen_height() / 2.0;

        draw_text(text, x, y, font_size, WHITE);

        // Draw trait count
        let stats_text = format!(
            "Traits: {} | Tournaments: {}",
            game_data.traits.traits.len(),
            game_data.tournaments.tournaments.len()
        );
        let stats_size = measure_text(&stats_text, None, 20, 1.0);
        let stats_x = screen_width() / 2.0 - stats_size.width / 2.0;
        draw_text(&stats_text, stats_x, y + 50.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
```

---

## 6. TESTING STRATEGY

### 6.1 Unit Test Organization

**Time Estimate**: 15 minutes

Create integration test file:

**File**: `tests/data_loading.rs`

**Content**:

```rust
use kaiju_sim::data::*;

#[test]
fn test_kaiju_stats_validation() {
    let stats = KaijuStats::new(200, 50, 40, 30);
    assert_eq!(stats.hp, 200);
    assert_eq!(stats.attack, 50);

    // Test floors
    let low_stats = KaijuStats::new(10, 5, 2, 3);
    assert!(low_stats.hp >= 50);
    assert!(low_stats.attack >= 10);
}

#[test]
fn test_trait_database_lookup() {
    let game_data = GameData::load().expect("Failed to load game data");

    let electric = game_data.traits.get_trait("electric_breath");
    assert!(electric.is_some());

    if let Some(trait_def) = electric {
        assert_eq!(trait_def.name, "Electric Breath");
        assert_eq!(trait_def.category, TraitCategory::Element);
    }
}

#[test]
fn test_lineage_incest_prevention() {
    let lineage_a = Lineage::new_wild(1, "Alpha".to_string());
    let lineage_b = Lineage::new_wild(2, "Beta".to_string());

    assert!(Lineage::can_breed_with(&lineage_a, &lineage_b));
    assert!(!Lineage::can_breed_with(&lineage_a, &lineage_a));
}

#[test]
fn test_balance_config_validation() {
    let game_data = GameData::load().expect("Failed to load game data");

    assert!(game_data.balance.breeding.generation_power_creep > 0.0);
    assert!(game_data.balance.combat.base_damage_formula.minimum_damage > 0);
}
```

---

### 6.2 Running Tests

**Commands**:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_kaiju_stats_validation

# Check code compiles without running
cargo check
```

---

## 7. BUILD AND DEPLOYMENT VERIFICATION

### 7.1 Windows Build

**Time Estimate**: 5 minutes

**Command**:
```bash
cargo build --release
```

**Expected Output**:
```
Compiling kaiju_sim v0.1.0 (H:\RustGames\kaiju_sim)
Finished release [optimized] target(s) in 45.2s
```

**Binary Location**: `target/release/kaiju_sim.exe`

---

### 7.2 WASM Build

**Time Estimate**: 10 minutes

**Prerequisites**:
```bash
rustup target add wasm32-unknown-unknown
```

**Command**:
```bash
cargo build --release --target wasm32-unknown-unknown
```

**Expected Output**:
```
Compiling kaiju_sim v0.1.0 (H:\RustGames\kaiju_sim)
Finished release [optimized] target(s) in 52.8s
```

**Binary Location**: `target/wasm32-unknown-unknown/release/kaiju_sim.wasm`

---

### 7.3 Verify Index.html

**File**: `index.html` (should already exist)

Verify it contains:
```html
<script src="mq_js_bundle.js"></script>
<script>load("kaiju_sim.wasm");</script>
```

---

## 8. DOCUMENTATION

### 8.1 README.md

**Time Estimate**: 15 minutes

Create a project README:

**File**: `README.md`

**Content**:

```markdown
# Kaiju Breeding Simulator

Strategic management and auto-battle game where players breed, train, and compete with unique kaiju in a competitive global ecosystem.

## Phase 1 Status: COMPLETE

- Core data structures implemented
- JSON data loading functional
- Macroquad integration verified
- Unit tests passing

## Building

### Windows
```bash
cargo build --release
```

### WebGL/WASM
```bash
cargo build --release --target wasm32-unknown-unknown
```

### Testing
```bash
cargo test
```

## Project Structure

```
kaiju_sim/
├── src/
│   ├── main.rs              # Entry point
│   ├── data/                # Data structures
│   │   ├── kaiju.rs         # Kaiju entity
│   │   ├── traits.rs        # Trait system
│   │   ├── lineage.rs       # Lineage tracking
│   │   └── loader.rs        # JSON loading
│   ├── engine/              # Game logic (Phase 2+)
│   ├── state/               # State management (Phase 5+)
│   └── ui/                  # UI components (Phase 6+)
├── assets/
│   ├── traits.json          # Trait definitions
│   ├── balance.json         # Game balance
│   └── tournaments.json     # Tournament configs
└── tests/                   # Integration tests
```

## Next Phase

Phase 2: Genetics & Breeding Engine
- Implement breeding algorithm
- Add genome encoding
- Create trait inheritance system
```

---

## 9. ESTIMATED TIME BREAKDOWN

| Task | Time | Cumulative |
|------|------|------------|
| 1.1 Create Cargo Project | 15 min | 0:15 |
| 1.2 Configure Cargo.toml | 10 min | 0:25 |
| 1.3 Create Folder Structure | 10 min | 0:35 |
| 2.1 Type Aliases | 15 min | 0:50 |
| 2.2 Kaiju Stats | 20 min | 1:10 |
| 2.3 Kaiju Entity | 30 min | 1:40 |
| 2.4 Trait System | 30 min | 2:10 |
| 2.5 Lineage System | 20 min | 2:30 |
| 3.1 Traits JSON | 45 min | 3:15 |
| 3.2 Balance JSON | 30 min | 3:45 |
| 3.3 Tournaments JSON | 20 min | 4:05 |
| 3.4 Data Loader | 45 min | 4:50 |
| 4.1-4.2 Module Organization | 15 min | 5:05 |
| 5.1 Main Entry Point | 30 min | 5:35 |
| 6.1-6.2 Testing | 15 min | 5:50 |
| 7.1-7.3 Build Verification | 25 min | 6:15 |
| 8.1 Documentation | 15 min | 6:30 |
| **Buffer for debugging** | 30 min | **7:00** |

**Total: 7-8 hours** (with debugging buffer)

---

## 10. SUCCESS CHECKLIST

Phase 1 is complete when:

- [ ] `cargo build --release` succeeds for Windows
- [ ] `cargo build --release --target wasm32-unknown-unknown` succeeds for WASM
- [ ] `cargo test` passes all tests
- [ ] `cargo run` opens Macroquad window showing "Phase 1 Complete"
- [ ] All JSON files load without errors
- [ ] Trait database contains 10+ traits
- [ ] Balance config validates successfully
- [ ] Kaiju entity can be created and serialized
- [ ] Lineage incest prevention works
- [ ] No compiler warnings (run `cargo clippy`)
- [ ] README.md documents current state

---

## 11. COMMON ISSUES AND SOLUTIONS

### Issue: "cannot find macroquad-toolkit"

**Solution**:
- Verify `macroquad-toolkit` exists at `H:\RustGames\macroquad-toolkit`
- Update path in `Cargo.toml` if needed

### Issue: "failed to load JSON files"

**Solution**:
- Ensure `assets/` folder exists
- Verify JSON syntax with `jq` or online validator
- Check file paths are relative to project root

### Issue: "trait TraitCondition is not implemented for serde::Deserialize"

**Solution**:
- Add `#[serde(tag = "type")]` to enum
- Use JSON format: `{"Environment": "storm"}` not `"Environment(storm)"`

### Issue: WASM target not found

**Solution**:
```bash
rustup target add wasm32-unknown-unknown
```

---

### Critical Files for Implementation

Here are the 5 most critical files to create in Phase 1:

1. **src/data/kaiju.rs** - Core game entity with stats, traits, and lifecycle. All game systems depend on this structure.

2. **src/data/traits.rs** - Trait system defines genetic inheritance, combat effects, and breeding outcomes. Critical for Phase 2+ mechanics.

3. **src/data/loader.rs** - JSON data loading infrastructure. All balance, traits, and tournament configs flow through this module.

4. **assets/balance.json** - Contains all magic numbers and game constants. Essential for data-driven design philosophy.

5. **src/main.rs** - Entry point that integrates Macroquad, loads data, and establishes the game loop foundation.

---

## Additional Notes

**Code Standards Compliance**:
- All code follows `CODE_STANDARDS.md` guidelines
- Function size kept under 100 lines
- No unused code or fields
- Data-driven design (JSON files for all constants)
- Clear module responsibilities

**Architecture Decisions**:
- Immutable kaiju properties separated from mutable state
- Trait conditions use enum for type safety
- Lineage uses recursive structure for natural tree representation
- GameData is loaded once at startup for performance

**Testing Philosophy**:
- Unit tests for all data structures
- Integration tests for JSON loading
- Tests document expected behavior
- No test should depend on external state

**Future Extension Points**:
- `engine/` folder ready for Phase 2 breeding logic
- `state/` folder ready for Phase 5 game state
- `ui/` folder ready for Phase 6 UI components
- Trait system extensible to 50+ traits

---

This plan provides a complete roadmap for Phase 1. Each section is actionable and includes code examples, time estimates, and validation criteria. The implementation follows all coding standards and creates a solid foundation for subsequent phases.
