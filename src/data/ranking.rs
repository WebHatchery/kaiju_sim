//! Elo ranking system for kaiju competitive play.

use serde::{Deserialize, Serialize};

/// Starting Elo rating for new kaiju
pub const STARTING_ELO: i32 = 1500;

/// K-factor for rating changes
pub const K_FACTOR: f32 = 32.0;

/// Elo ranking for a kaiju
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EloRating {
    /// Current rating
    pub rating: i32,
    /// Total matches played
    pub matches_played: u32,
    /// Total wins
    pub wins: u32,
    /// Total losses
    pub losses: u32,
    /// Highest rating achieved
    pub peak_rating: i32,
}

impl Default for EloRating {
    fn default() -> Self {
        Self {
            rating: STARTING_ELO,
            matches_played: 0,
            wins: 0,
            losses: 0,
            peak_rating: STARTING_ELO,
        }
    }
}

impl EloRating {
    /// Create new rating with starting value
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate win probability against opponent
    pub fn expected_score(&self, opponent_rating: i32) -> f32 {
        1.0 / (1.0 + 10f32.powf((opponent_rating - self.rating) as f32 / 400.0))
    }

    /// Update rating after a win
    pub fn record_win(&mut self, opponent_rating: i32) {
        let expected = self.expected_score(opponent_rating);
        let change = (K_FACTOR * (1.0 - expected)) as i32;

        self.rating += change;
        self.matches_played += 1;
        self.wins += 1;

        if self.rating > self.peak_rating {
            self.peak_rating = self.rating;
        }
    }

    /// Update rating after a loss
    pub fn record_loss(&mut self, opponent_rating: i32) {
        let expected = self.expected_score(opponent_rating);
        let change = (K_FACTOR * (0.0 - expected)) as i32;

        self.rating += change;
        self.rating = self.rating.max(0); // Floor at 0
        self.matches_played += 1;
        self.losses += 1;
    }

    /// Get win rate percentage
    pub fn win_rate(&self) -> f32 {
        if self.matches_played == 0 {
            0.0
        } else {
            self.wins as f32 / self.matches_played as f32
        }
    }
}

/// Update Elo ratings for both participants after a match
pub fn update_elo_ratings(winner: &mut EloRating, loser: &mut EloRating) {
    let winner_rating = winner.rating;
    let loser_rating = loser.rating;

    winner.record_win(loser_rating);
    loser.record_loss(winner_rating);
}

/// Calculate if a match result was an upset
pub fn calculate_upset(winner_rating: i32, loser_rating: i32) -> (bool, f32) {
    // Calculate probability of the actual winner winning
    let winner_expected = 1.0 / (1.0 + 10f32.powf((loser_rating - winner_rating) as f32 / 400.0));

    // Upset if winner was expected to lose
    let is_upset = winner_expected < 0.5;

    // Upset probability is how unlikely the result was
    let upset_probability = 1.0 - winner_expected;

    (is_upset, upset_probability)
}

/// Seed score for tournament seeding
#[derive(Debug, Clone)]
pub struct SeedScore {
    pub kaiju_id: uuid::Uuid,
    pub score: f32,
    pub ranking: i32,
    pub recent_win_rate: f32,
}

/// Calculate seed score for tournament seeding
pub fn calculate_seed_score(
    ranking: i32,
    recent_win_rate: f32,
    tournament_wins: u32,
    generation: u32,
) -> f32 {
    (ranking as f32 * 0.5)
        + (recent_win_rate * 0.3 * 1000.0)
        + (tournament_wins as f32 * 0.15 * 50.0)
        + (generation as f32 * 0.05 * 10.0)
}

#[cfg(test)]
mod tests;
