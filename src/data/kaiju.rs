//! Kaiju entity and related data structures.

use serde::{Deserialize, Serialize};

use super::traits::Trait;
use super::types::*;

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

    /// Energy - resource for special moves
    pub energy: i32,
}

impl KaijuStats {
    /// Create new stats with validation
    pub fn new(hp: i32, attack: i32, defense: i32, speed: i32, energy: i32) -> Self {
        Self {
            hp: hp.max(50),          // Minimum HP floor
            attack: attack.max(10),  // Minimum attack floor
            defense: defense.max(5), // Minimum defense floor
            speed: speed.max(5),     // Minimum speed floor
            energy: energy.max(50),  // Minimum energy floor
        }
    }

    /// Calculate total power level (for rough comparisons)
    pub fn power_level(&self) -> i32 {
        self.hp / 5 + self.attack + self.defense + self.speed + self.energy / 2
    }
}

impl Default for KaijuStats {
    fn default() -> Self {
        Self::new(100, 20, 15, 15, 100)
    }
}

/// Complete kaiju entity with all properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kaiju {
    // === IMMUTABLE PROPERTIES (set at creation) ===
    /// Unique identifier
    pub id: KaijuId,

    /// Persistent local legacy ID used for lineage and history references.
    pub token_id: TokenId,

    /// Kaiju display name (non-unique)
    pub name: String,

    /// Generation number (0 = wild, 1+ = bred)
    pub generation: Generation,

    /// Creation timestamp (Unix epoch)
    pub created_at: Timestamp,

    /// Original breeder or system source.
    pub original_breeder: WalletAddress,

    /// Parent legacy IDs (None for generation 0)
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

    /// Current keeper. Multiplayer ownership can map onto this later.
    pub current_owner: WalletAddress,

    // === PROFILE METADATA ===
    /// Portrait URI or local sprite path.
    pub image_uri: Option<String>,

    /// Optional external profile URI.
    pub metadata_uri: String,

    /// Number of tournaments won
    #[serde(default)]
    pub tournaments_won: i32,

    /// Documented history of creation, training, battles, breeding, ownership, and future events.
    #[serde(default)]
    pub history: Vec<KaijuEvent>,
}

/// Historical event category for a kaiju legacy record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KaijuEventKind {
    Created,
    JoinedRoster,
    Training,
    Battle,
    Breeding,
    Offspring,
    Research,
    Transfer,
    Death,
}

impl KaijuEventKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Created => "Created",
            Self::JoinedRoster => "Joined Roster",
            Self::Training => "Training",
            Self::Battle => "Battle",
            Self::Breeding => "Breeding",
            Self::Offspring => "Offspring",
            Self::Research => "Research",
            Self::Transfer => "Transfer",
            Self::Death => "Death",
        }
    }
}

/// A durable record of something meaningful in a kaiju's life.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KaijuEvent {
    pub timestamp: Timestamp,
    pub kind: KaijuEventKind,
    pub title: String,
    pub details: String,
    pub related_kaiju_ids: Vec<KaijuId>,
    pub battle_seed: Option<u64>,
}

impl KaijuEvent {
    pub fn new(kind: KaijuEventKind, title: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            timestamp: now_timestamp(),
            kind,
            title: title.into(),
            details: details.into(),
            related_kaiju_ids: Vec::new(),
            battle_seed: None,
        }
    }

    pub fn with_related(mut self, related_kaiju_ids: Vec<KaijuId>) -> Self {
        self.related_kaiju_ids = related_kaiju_ids;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.battle_seed = Some(seed);
        self
    }
}

impl Kaiju {
    /// Create a new generation 0 (wild) kaiju
    pub fn new_wild(name: String, stats: KaijuStats, traits: Vec<Trait>) -> Self {
        Self {
            id: new_kaiju_id(),
            token_id: 0,
            name,
            generation: 0,
            created_at: now_timestamp(),
            original_breeder: "system".to_string(),
            parent_ids: None,
            visual_seed: random_u64(),
            genome_hash: format!("{:016x}", random_u64()),
            stats,
            traits,
            hidden_traits: Vec::new(),
            experience: 0,
            alive: true,
            current_owner: "system".to_string(),
            image_uri: None,
            metadata_uri: String::new(),
            tournaments_won: 0,
            history: vec![KaijuEvent::new(
                KaijuEventKind::Created,
                "Wild kaiju discovered",
                "Entered the local legacy registry as a generation 0 kaiju.",
            )],
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

    /// Append a durable history entry.
    pub fn record_event(&mut self, event: KaijuEvent) {
        self.history.push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_floors() {
        let stats = KaijuStats::new(10, 5, 2, 3, 10);
        assert_eq!(stats.hp, 50); // Floor applied
        assert_eq!(stats.attack, 10); // Floor applied
        assert_eq!(stats.defense, 5); // Floor applied
        assert_eq!(stats.speed, 5); // Floor applied
        assert_eq!(stats.energy, 50); // Floor applied
    }

    #[test]
    fn test_stats_serialization() {
        let stats = KaijuStats::new(200, 50, 40, 30, 100);
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: KaijuStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats, deserialized);
    }

    #[test]
    fn test_new_wild_kaiju() {
        let stats = KaijuStats::new(200, 50, 40, 30, 100);
        let kaiju = Kaiju::new_wild("TestKaiju".to_string(), stats, vec![]);

        assert_eq!(kaiju.generation, 0);
        assert_eq!(kaiju.name, "TestKaiju");
        assert!(kaiju.alive);
        assert!(kaiju.can_breed());
        assert!(kaiju.can_compete());
        assert_eq!(kaiju.parent_ids, None);
        assert_eq!(kaiju.history.len(), 1);
        assert_eq!(kaiju.history[0].kind, KaijuEventKind::Created);
    }

    #[test]
    fn test_battle_rating_calculation() {
        let stats = KaijuStats::new(200, 50, 40, 30, 100);
        let kaiju = Kaiju::new_wild("Warrior".to_string(), stats, vec![]);

        let rating = kaiju.battle_rating();
        assert!(rating > 0);
    }

    #[test]
    fn test_missing_history_defaults_when_loading_old_save() {
        let stats = KaijuStats::new(200, 50, 40, 30, 100);
        let kaiju = Kaiju::new_wild("Legacy".to_string(), stats, vec![]);
        let mut value = serde_json::to_value(&kaiju).unwrap();
        value.as_object_mut().unwrap().remove("history");

        let loaded: Kaiju = serde_json::from_value(value).unwrap();
        assert!(loaded.history.is_empty());
    }
}
