//! Player-specific data and progression.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Player-specific persistent data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerData {
    /// Unique player ID
    pub player_id: String,
    /// Display name
    pub display_name: String,
    /// Gold currency
    pub gold: i64,
    /// Premium currency
    pub premium_currency: i64,
    /// Player level
    pub player_level: u32,
    /// Total experience
    pub experience: u64,
    /// Reputation score
    pub reputation: i32,
    /// Lifetime statistics
    pub stats: PlayerStats,
    /// Player settings
    pub settings: PlayerSettings,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            player_id: Uuid::new_v4().to_string(),
            display_name: "Breeder".into(),
            gold: 1000,
            premium_currency: 0,
            player_level: 1,
            experience: 0,
            reputation: 0,
            stats: PlayerStats::default(),
            settings: PlayerSettings::default(),
        }
    }
}

impl PlayerData {
    /// Add gold
    pub fn add_gold(&mut self, amount: i64) {
        self.gold = (self.gold + amount).max(0);
    }

    /// Spend gold (returns false if insufficient)
    pub fn spend_gold(&mut self, amount: i64) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }

    /// Add experience and check for level up
    pub fn add_experience(&mut self, xp: u64) -> bool {
        self.experience += xp;
        let xp_needed = self.xp_for_next_level();

        if self.experience >= xp_needed {
            self.player_level += 1;
            true
        } else {
            false
        }
    }

    /// XP needed for next level
    pub fn xp_for_next_level(&self) -> u64 {
        100 * (self.player_level as u64).pow(2)
    }
}

/// Player lifetime statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    pub total_kaiju_bred: u32,
    pub total_tournaments_entered: u32,
    pub total_tournaments_won: u32,
    pub total_battles_fought: u32,
    pub total_battles_won: u32,
    pub kaiju_deaths: u32,
    pub highest_generation: u32,
}

/// Player settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub battle_speed: f32,
    pub auto_save_enabled: bool,
    pub show_hints: bool,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            battle_speed: 1.0,
            auto_save_enabled: true,
            show_hints: true,
            music_volume: 0.7,
            sfx_volume: 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_player() {
        let player = PlayerData::default();
        assert_eq!(player.gold, 1000);
        assert_eq!(player.player_level, 1);
    }

    #[test]
    fn test_gold_operations() {
        let mut player = PlayerData::default();

        player.add_gold(500);
        assert_eq!(player.gold, 1500);

        assert!(player.spend_gold(1000));
        assert_eq!(player.gold, 500);

        assert!(!player.spend_gold(1000)); // Not enough
        assert_eq!(player.gold, 500);
    }

    #[test]
    fn test_experience_level_up() {
        let mut player = PlayerData::default();
        assert_eq!(player.player_level, 1);

        // Level 1 needs 100 XP
        let leveled_up = player.add_experience(100);
        assert!(leveled_up);
        assert_eq!(player.player_level, 2);
    }
}
