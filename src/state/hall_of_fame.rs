//! Hall of Fame for fallen kaiju legacy records.
//!
//! Preserves the memory and achievements of kaiju who died in lethal tournaments.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::tournament::TournamentId;
use crate::data::traits::Trait;
use crate::data::types::KaijuId;
use crate::data::KaijuStats;

/// Hall of Fame entry for a fallen kaiju
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallOfFameEntry {
    /// Legacy record data
    pub legacy: LegacyRecord,
    /// Categories this kaiju earned
    pub categories: Vec<HallOfFameCategory>,
    /// Prestige ranking in Hall of Fame
    pub prestige_score: u32,
}

/// Complete legacy record of a fallen kaiju
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyRecord {
    /// Kaiju ID
    pub kaiju_id: KaijuId,
    /// Display name
    pub name: String,
    /// Generation number
    pub generation: u32,
    /// Final stats at time of death
    pub final_stats: KaijuStats,
    /// Visible traits (were always known)
    pub visible_traits: Vec<Trait>,
    /// Hidden traits (revealed post-mortem!)
    pub hidden_traits: Vec<Trait>,
    /// Lifetime battle record
    pub lifetime_record: MatchRecord,
    /// Tournament victories
    pub tournament_victories: Vec<TournamentId>,
    /// Number of offspring
    pub offspring_count: u32,
    /// Notable descendants (tournament winners, etc.)
    pub notable_descendants: Vec<KaijuId>,
    /// Death timestamp
    pub death_timestamp: i64,
    /// How they died
    pub death_context: String,
    /// Achievements earned
    pub achievements: Vec<Achievement>,
}

/// Battle record summary
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchRecord {
    pub total_matches: u32,
    pub wins: u32,
    pub losses: u32,
    pub tournament_matches: u32,
    pub lethal_matches_survived: u32,
}

impl MatchRecord {
    pub fn win_rate(&self) -> f32 {
        if self.total_matches == 0 {
            0.0
        } else {
            self.wins as f32 / self.total_matches as f32
        }
    }
}

/// Achievement for kaiju accomplishments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub earned_at: i64,
}

/// Categories for Hall of Fame classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HallOfFameCategory {
    /// Won at least one lethal tournament
    LethalTournamentWinner,
    /// Reached high ranking
    HighestRanking { peak_rank: i32 },
    /// Won many tournaments
    MostTournamentWins { count: u32 },
    /// Long win streak
    LongestWinStreak { streak: u32 },
    /// Many offspring
    ProlificBreeder { offspring: u32 },
    /// Has notable descendants
    LegendaryBloodline,
    /// First of their generation to achieve something
    Pioneer { achievement: String },
}

/// Hall of Fame database
#[derive(Debug, Clone, Default)]
pub struct HallOfFame {
    pub entries: Vec<HallOfFameEntry>,
}

impl HallOfFame {
    /// Create empty Hall of Fame
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new entry
    pub fn add_entry(&mut self, entry: HallOfFameEntry) {
        self.entries.push(entry);
        self.entries
            .sort_by_key(|entry| std::cmp::Reverse(entry.prestige_score));
    }

    /// Find entry by kaiju ID
    pub fn get(&self, kaiju_id: KaijuId) -> Option<&HallOfFameEntry> {
        self.entries.iter().find(|e| e.legacy.kaiju_id == kaiju_id)
    }

    /// Get entries by category
    pub fn get_by_category(&self, category: &HallOfFameCategory) -> Vec<&HallOfFameEntry> {
        self.entries
            .iter()
            .filter(|e| e.categories.contains(category))
            .collect()
    }

    /// Get top entries by prestige
    pub fn get_top(&self, count: usize) -> Vec<&HallOfFameEntry> {
        self.entries.iter().take(count).collect()
    }

    /// Search by name
    pub fn search_by_name(&self, query: &str) -> Vec<&HallOfFameEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.legacy.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get entries by generation
    pub fn get_by_generation(&self, generation: u32) -> Vec<&HallOfFameEntry> {
        self.entries
            .iter()
            .filter(|e| e.legacy.generation == generation)
            .collect()
    }
}

/// Calculate prestige score for Hall of Fame ranking
pub fn calculate_prestige_score(legacy: &LegacyRecord) -> u32 {
    let mut score = 0u32;

    // Battle record contribution
    score += legacy.lifetime_record.wins * 10;
    score += (legacy.lifetime_record.win_rate() * 100.0) as u32;

    // Tournament victories
    score += legacy.tournament_victories.len() as u32 * 100;

    // Lethal matches survived
    score += legacy.lifetime_record.lethal_matches_survived * 50;

    // Offspring contribution
    score += legacy.offspring_count * 20;

    // Notable descendants
    score += legacy.notable_descendants.len() as u32 * 30;

    // Achievements
    score += legacy.achievements.len() as u32 * 25;

    // Generation bonus (higher gens more prestigious)
    score += legacy.generation * 5;

    score
}

/// Create Hall of Fame entry from legacy record
pub fn create_hall_of_fame_entry(legacy: LegacyRecord) -> HallOfFameEntry {
    let mut categories = Vec::new();

    // Determine categories
    if !legacy.tournament_victories.is_empty() {
        categories.push(HallOfFameCategory::LethalTournamentWinner);
    }

    if legacy.offspring_count >= 10 {
        categories.push(HallOfFameCategory::ProlificBreeder {
            offspring: legacy.offspring_count,
        });
    }

    if !legacy.notable_descendants.is_empty() {
        categories.push(HallOfFameCategory::LegendaryBloodline);
    }

    let prestige_score = calculate_prestige_score(&legacy);

    HallOfFameEntry {
        legacy,
        categories,
        prestige_score,
    }
}

#[cfg(test)]
mod tests;
