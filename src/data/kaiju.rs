//! Kaiju entity and related data structures.

use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    /// Number of tournaments won
    #[serde(default)]
    pub tournaments_won: i32,
}

impl Kaiju {
    /// Create a new generation 0 (wild) kaiju
    pub fn new_wild(name: String, stats: KaijuStats, traits: Vec<Trait>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: Uuid::new_v4(),
            token_id: 0,
            name,
            generation: 0,
            created_at: Utc::now().timestamp(),
            original_breeder: "system".to_string(),
            parent_ids: None,
            visual_seed: rng.gen(),
            genome_hash: format!("{:016x}", rng.gen::<u64>()),
            stats,
            traits,
            hidden_traits: Vec::new(),
            experience: 0,
            alive: true,
            current_owner: "system".to_string(),
            image_uri: None,
            metadata_uri: String::new(),
            tournaments_won: 0,
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
    }

    #[test]
    fn test_battle_rating_calculation() {
        let stats = KaijuStats::new(200, 50, 40, 30, 100);
        let kaiju = Kaiju::new_wild("Warrior".to_string(), stats, vec![]);

        let rating = kaiju.battle_rating();
        assert!(rating > 0);
    }
}
