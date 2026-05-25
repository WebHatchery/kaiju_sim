//! Battle state structures for combat tracking.
//!
//! Tracks HP, turn count, battle logs, and results.

use serde::{Deserialize, Serialize};

use crate::data::environments::Environment;
use crate::data::Kaiju;

/// Current state of a battle in progress
#[derive(Debug, Clone)]
pub struct BattleState {
    /// First participant
    pub kaiju_a: Kaiju,
    /// Second participant
    pub kaiju_b: Kaiju,
    /// Current HP for kaiju A
    pub hp_a: i32,
    /// Current HP for kaiju B
    pub hp_b: i32,
    /// Battle environment
    pub environment: Environment,
    /// Current turn number
    pub turn_count: u32,
    /// Battle log entries
    pub battle_log: Vec<BattleLogEntry>,
    /// Random seed for deterministic replay
    pub seed: u64,
}

impl BattleState {
    /// Create a new battle state
    pub fn new(kaiju_a: Kaiju, kaiju_b: Kaiju, environment: Environment, seed: u64) -> Self {
        let hp_a = kaiju_a.stats.hp;
        let hp_b = kaiju_b.stats.hp;

        Self {
            kaiju_a,
            kaiju_b,
            hp_a,
            hp_b,
            environment,
            turn_count: 0,
            battle_log: Vec::with_capacity(50), // Pre-allocate for performance
            seed,
        }
    }

    /// Log a turn
    pub fn log_turn(&mut self, entry: BattleLogEntry) {
        self.battle_log.push(entry);
    }

    /// Log a special effect to the last turn
    pub fn log_special_effect(&mut self, effect: String) {
        if let Some(last_entry) = self.battle_log.last_mut() {
            last_entry.special_effects.push(effect);
        }
    }

    /// Check if battle is over
    pub fn is_over(&self) -> bool {
        self.hp_a <= 0 || self.hp_b <= 0
    }

    /// Get the winner name (None if battle not over)
    pub fn winner_name(&self) -> Option<&str> {
        if self.hp_a <= 0 {
            Some(&self.kaiju_b.name)
        } else if self.hp_b <= 0 {
            Some(&self.kaiju_a.name)
        } else {
            None
        }
    }

    /// Get the loser name (None if battle not over)
    pub fn loser_name(&self) -> Option<&str> {
        if self.hp_a <= 0 {
            Some(&self.kaiju_a.name)
        } else if self.hp_b <= 0 {
            Some(&self.kaiju_b.name)
        } else {
            None
        }
    }

    /// Get remaining HP of winner
    pub fn winner_hp(&self) -> i32 {
        if self.hp_a <= 0 {
            self.hp_b
        } else {
            self.hp_a
        }
    }
}

/// Single entry in the battle log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleLogEntry {
    /// Turn number
    pub turn: u32,
    /// Name of attacker
    pub attacker_name: String,
    /// Name of defender
    pub defender_name: String,
    /// Damage dealt
    pub damage: i32,
    /// Defender HP remaining after this attack
    pub hp_remaining: i32,
    /// Any special effects that occurred
    pub special_effects: Vec<String>,
}

/// Final result of a battle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResult {
    /// Winner's name
    pub winner: String,
    /// Loser's name
    pub loser: String,
    /// Winner's remaining HP
    pub hp_remaining: i32,
    /// Total turns elapsed
    pub turns_elapsed: u32,
    /// Complete battle log
    pub battle_log: Vec<BattleLogEntry>,
    /// Random seed (for replay)
    pub seed: u64,
    /// Environment the battle took place in
    pub environment: String,
    /// Battle insights for players
    pub insights: Vec<String>,
    /// Improvement suggestions
    pub suggestions: Vec<String>,
}

impl BattleResult {
    /// Create from battle state
    pub fn from_state(state: BattleState) -> Self {
        let (winner, loser, hp_remaining) = if state.hp_a <= 0 {
            (
                state.kaiju_b.name.clone(),
                state.kaiju_a.name.clone(),
                state.hp_b,
            )
        } else if state.hp_b <= 0 {
            (
                state.kaiju_a.name.clone(),
                state.kaiju_b.name.clone(),
                state.hp_a,
            )
        } else {
            // Timeout - winner is whoever has more HP
            if state.hp_a >= state.hp_b {
                (
                    state.kaiju_a.name.clone(),
                    state.kaiju_b.name.clone(),
                    state.hp_a,
                )
            } else {
                (
                    state.kaiju_b.name.clone(),
                    state.kaiju_a.name.clone(),
                    state.hp_b,
                )
            }
        };

        Self {
            winner,
            loser,
            hp_remaining,
            turns_elapsed: state.turn_count,
            battle_log: state.battle_log,
            seed: state.seed,
            environment: state.environment.name().to_string(),
            insights: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Format as human-readable log
    pub fn format_log(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "=== BATTLE: {} vs {} (Environment: {}) ===\n\n",
            self.battle_log
                .first()
                .map(|e| e.attacker_name.as_str())
                .unwrap_or("Unknown"),
            self.battle_log
                .first()
                .map(|e| e.defender_name.as_str())
                .unwrap_or("Unknown"),
            self.environment
        ));

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
            "\n=== VICTORY: {} wins with {} HP remaining! ({} turns) ===\n",
            self.winner, self.hp_remaining, self.turns_elapsed
        ));

        output
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Summary for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleSummary {
    pub winner: String,
    pub hp_remaining: i32,
    pub turns_elapsed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{new_kaiju_id, now_timestamp, KaijuStats};

    fn create_test_kaiju(name: &str, hp: i32) -> Kaiju {
        Kaiju {
            id: new_kaiju_id(),
            token_id: 0,
            name: name.to_string(),
            generation: 1,
            created_at: now_timestamp(),
            original_breeder: "test".to_string(),
            parent_ids: None,
            visual_seed: 0,
            genome_hash: "test".to_string(),
            stats: KaijuStats::new(hp, 50, 30, 25, 100),
            traits: vec![],
            hidden_traits: vec![],
            experience: 0,
            alive: true,
            current_owner: "test".to_string(),
            image_uri: None,
            metadata_uri: String::new(),
            tournaments_won: 0,
            history: Vec::new(),
        }
    }

    #[test]
    fn test_battle_state_creation() {
        let a = create_test_kaiju("Alpha", 300);
        let b = create_test_kaiju("Beta", 320);

        let state = BattleState::new(a, b, Environment::Neutral, 12345);

        assert_eq!(state.hp_a, 300);
        assert_eq!(state.hp_b, 320);
        assert_eq!(state.turn_count, 0);
        assert!(!state.is_over());
    }

    #[test]
    fn test_battle_result_formatting() {
        let a = create_test_kaiju("Winner", 100);
        let b = create_test_kaiju("Loser", 0);

        let mut state = BattleState::new(a, b, Environment::Storm, 12345);
        state.hp_b = 0;
        state.battle_log.push(BattleLogEntry {
            turn: 1,
            attacker_name: "Winner".to_string(),
            defender_name: "Loser".to_string(),
            damage: 50,
            hp_remaining: 0,
            special_effects: vec![],
        });

        let result = BattleResult::from_state(state);
        let log = result.format_log();

        assert!(log.contains("VICTORY: Winner wins"));
    }
}
