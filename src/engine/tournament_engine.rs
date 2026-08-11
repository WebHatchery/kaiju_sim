//! Tournament execution engine.
//!
//! Orchestrates bracket generation, match execution, and reward distribution.

use uuid::Uuid;

use crate::data::environments::Environment;
use crate::data::ranking::{calculate_upset, update_elo_ratings, EloRating};
use crate::data::tournament::{
    Bracket, Match, MatchResult, Round, RoundStatus, Tournament, TournamentError, TournamentStatus,
};
use crate::data::Kaiju;
use crate::engine::bracket_generator::{
    select_environment, BracketGenerator, SingleEliminationGenerator, SwissGenerator,
};
use crate::engine::combat::{execute_battle, CombatConfig};
use crate::state::BattleResult;

/// Tournament executor
pub struct TournamentEngine {
    combat_config: CombatConfig,
}

impl TournamentEngine {
    /// Create a new tournament engine
    pub fn new(combat_config: CombatConfig) -> Self {
        Self { combat_config }
    }

    /// Start a tournament (close registration and generate bracket)
    pub fn start_tournament(&self, tournament: &mut Tournament) -> Result<(), TournamentError> {
        if tournament.participants.len() < 2 {
            return Err(TournamentError::InsufficientParticipants);
        }

        // Close registration
        tournament.status = TournamentStatus::RegistrationClosed;

        // Select environment for first round
        let environment = select_environment(&tournament.environment_mode, 1);

        // Generate bracket based on type
        let bracket = match &tournament.bracket_system {
            crate::data::tournament::BracketSystem::SingleElimination => SingleEliminationGenerator
                .generate(tournament.id, tournament.participants.clone(), environment)
                .map_err(|_| TournamentError::InsufficientParticipants)?,
            crate::data::tournament::BracketSystem::Swiss { rounds } => SwissGenerator {
                total_rounds: *rounds,
            }
            .generate(tournament.id, tournament.participants.clone(), environment)
            .map_err(|_| TournamentError::InsufficientParticipants)?,
            crate::data::tournament::BracketSystem::DoubleElimination => {
                // For now, fall back to single elimination
                SingleEliminationGenerator
                    .generate(tournament.id, tournament.participants.clone(), environment)
                    .map_err(|_| TournamentError::InsufficientParticipants)?
            }
        };

        tournament.bracket = Some(bracket);
        tournament.current_round = 1;
        tournament.status = TournamentStatus::InProgress { current_round: 1 };

        Ok(())
    }

    /// Execute a single match
    pub fn execute_match(
        &self,
        match_obj: &mut Match,
        kaiju_a: &Kaiju,
        kaiju_b: &Kaiju,
        rating_a: &mut EloRating,
        rating_b: &mut EloRating,
    ) -> BattleResult {
        // Execute combat
        let battle_result = execute_battle(
            kaiju_a.clone(),
            kaiju_b.clone(),
            match_obj.environment.clone(),
            match_obj.battle_seed,
            &self.combat_config,
        );

        // Determine winner/loser
        let winner_is_a = battle_result.winner == kaiju_a.name;
        let (winner_id, loser_id) = if winner_is_a {
            (kaiju_a.id, kaiju_b.id)
        } else {
            (kaiju_b.id, kaiju_a.id)
        };

        // Calculate upset
        let (was_upset, upset_probability) = if winner_is_a {
            calculate_upset(rating_a.rating, rating_b.rating)
        } else {
            calculate_upset(rating_b.rating, rating_a.rating)
        };

        // Record match result
        match_obj.result = Some(MatchResult {
            winner: winner_id,
            loser: loser_id,
            turns: battle_result.turns_elapsed,
            final_hp: (
                if winner_is_a {
                    battle_result.hp_remaining
                } else {
                    0
                },
                if winner_is_a {
                    0
                } else {
                    battle_result.hp_remaining
                },
            ),
            was_upset,
            upset_probability,
        });

        // Update Elo ratings
        if winner_is_a {
            update_elo_ratings(rating_a, rating_b);
        } else {
            update_elo_ratings(rating_b, rating_a);
        }

        battle_result
    }

    /// Check if a round is complete
    pub fn is_round_complete(&self, bracket: &Bracket, round: u32) -> bool {
        let round_data = bracket.rounds.iter().find(|r| r.number == round);

        if let Some(round_data) = round_data {
            round_data.match_ids.iter().all(|match_id| {
                bracket
                    .matches
                    .iter()
                    .find(|m| &m.id == match_id)
                    .map(|m| m.is_complete())
                    .unwrap_or(false)
            })
        } else {
            false
        }
    }

    /// Advance to next round (for single elimination)
    pub fn advance_round(
        &self,
        tournament: &mut Tournament,
    ) -> Result<Vec<Match>, TournamentError> {
        let bracket = tournament
            .bracket
            .as_mut()
            .ok_or(TournamentError::RoundNotComplete)?;

        if !self.is_round_complete(bracket, tournament.current_round) {
            return Err(TournamentError::RoundNotComplete);
        }

        // Get winners from current round
        let current_round = tournament.current_round;
        let winners: Vec<Uuid> = bracket
            .matches
            .iter()
            .filter(|m| m.round == current_round)
            .filter_map(|m| m.winner())
            .collect();

        // Check if tournament is over
        if winners.len() <= 1 {
            if let Some(winner_id) = winners.first() {
                tournament.status = TournamentStatus::Completed {
                    winner_id: *winner_id,
                };
            }
            return Ok(Vec::new());
        }

        // Create next round matches
        let next_round = current_round + 1;
        let environment = select_environment(&tournament.environment_mode, next_round);

        let mut new_matches = Vec::new();
        for chunk in winners.chunks(2) {
            if chunk.len() == 2 {
                let match_obj = Match::new(
                    tournament.id,
                    next_round,
                    chunk[0],
                    chunk[1],
                    environment.clone(),
                );
                new_matches.push(match_obj);
            }
        }

        // Update bracket
        let match_ids: Vec<_> = new_matches.iter().map(|m| m.id).collect();
        if let Some(round) = bracket.rounds.iter_mut().find(|r| r.number == next_round) {
            round.match_ids = match_ids;
            round.environment = environment;
        }
        bracket.matches.extend(new_matches.clone());

        // Mark current round complete, advance
        if let Some(round) = bracket
            .rounds
            .iter_mut()
            .find(|r| r.number == current_round)
        {
            round.status = RoundStatus::Completed;
        }

        tournament.current_round = next_round;
        tournament.status = TournamentStatus::InProgress {
            current_round: next_round,
        };

        Ok(new_matches)
    }

    /// Get tournament standings
    pub fn get_standings(&self, tournament: &Tournament) -> Vec<TournamentStanding> {
        let mut standings = Vec::new();

        if let Some(bracket) = &tournament.bracket {
            // Track wins/losses per participant
            let mut records: std::collections::HashMap<Uuid, (u32, u32)> =
                std::collections::HashMap::new();

            for match_obj in &bracket.matches {
                if let Some(result) = &match_obj.result {
                    let entry = records.entry(result.winner).or_insert((0, 0));
                    entry.0 += 1;

                    let entry = records.entry(result.loser).or_insert((0, 0));
                    entry.1 += 1;
                }
            }

            // Convert to standings
            for (kaiju_id, (wins, losses)) in records {
                standings.push(TournamentStanding {
                    kaiju_id,
                    wins,
                    losses,
                    eliminated: losses > 0
                        && matches!(
                            tournament.bracket_system,
                            crate::data::tournament::BracketSystem::SingleElimination
                        ),
                });
            }

            // Sort by wins (descending), then losses (ascending)
            standings.sort_by(|a, b| b.wins.cmp(&a.wins).then(a.losses.cmp(&b.losses)));
        }

        standings
    }
}

impl Default for TournamentEngine {
    fn default() -> Self {
        Self::new(CombatConfig::default())
    }
}

/// Tournament standing entry
#[derive(Debug, Clone)]
pub struct TournamentStanding {
    pub kaiju_id: Uuid,
    pub wins: u32,
    pub losses: u32,
    pub eliminated: bool,
}

/// XP reward calculation
pub fn calculate_xp_reward(
    won: bool,
    tournament_type: &crate::data::tournament::TournamentType,
    round: u32,
    was_upset: bool,
) -> u32 {
    let base_xp = if won { 100 } else { 30 };

    let multiplier = match tournament_type {
        crate::data::tournament::TournamentType::NonLethalRanked => 1.0,
        crate::data::tournament::TournamentType::LethalWinnerTakesAll => 2.0,
        crate::data::tournament::TournamentType::SpecialEvent { xp_multiplier, .. } => {
            *xp_multiplier
        }
        _ => 1.0,
    };

    let round_bonus = match round {
        1 => 0,
        2 => 20,
        3 => 50,
        _ => 100,
    };

    let upset_bonus = if was_upset && won { 50 } else { 0 };

    ((base_xp as f32 * multiplier) as u32) + round_bonus + upset_bonus
}

#[cfg(test)]
mod tests;
