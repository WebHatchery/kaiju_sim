use sqlx::{MySqlPool, Row};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::tournament::simulator;
use crate::breeding_service::KaijuStats;
use rand::seq::SliceRandom;

pub async fn ensure_active_tournament(pool: &MySqlPool) -> Result<String, anyhow::Error> {
    // Check for active tournament
    let current = sqlx::query("SELECT id FROM tournaments WHERE state != 'Finished' LIMIT 1")
        .fetch_optional(pool)
        .await?;

    if let Some(row) = current {
        let id: String = row.get("id");
        return Ok(id);
    }

    // Create new one
    let id = Uuid::new_v4().to_string();
    let start_time = Utc::now() + chrono::Duration::minutes(1); 
    
    sqlx::query("INSERT INTO tournaments (id, start_time, state) VALUES (?, ?, 'Registration')")
        .bind(&id)
        .bind(start_time)
        .execute(pool)
        .await?;
        
    tracing::info!("Created new tournament {} scheduled for {}", id, start_time);
    Ok(id)
}

pub struct TournamentManager {
    pool: MySqlPool,
}

impl TournamentManager {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn run(self: Arc<Self>) {
        tracing::info!("Tournament Scheduler Started");
        loop {
            if let Err(e) = self.process_tick().await {
                tracing::error!("Tournament Scheduler Error: {:?}", e);
            }
            sleep(Duration::from_secs(10)).await;
        }
    }

    async fn process_tick(&self) -> Result<(), anyhow::Error> {
        // Get current active tournament (don't create one if missing)
        let current = sqlx::query("SELECT id, start_time, state, current_round FROM tournaments WHERE state != 'Finished' ORDER BY created_at DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = current {
            let id: String = row.get("id");
            let state: String = row.get("state");
            let start_time: chrono::DateTime<Utc> = row.get("start_time");
            let round: i32 = row.get("current_round");
            
            match state.as_str() {
                "Registration" => {
                    let now = Utc::now();
                    if now >= start_time {
                        // Check for real players
                        let real_players: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tournament_participants WHERE tournament_id = ? AND is_bot = FALSE")
                            .bind(&id)
                            .fetch_one(&self.pool)
                            .await?;

                        if real_players == 0 {
                            tracing::info!("No real players in tournament {}, extending registration", id);
                            let new_time = now + chrono::Duration::minutes(1);
                            sqlx::query("UPDATE tournaments SET start_time = ? WHERE id = ?")
                                .bind(new_time)
                                .bind(&id)
                                .execute(&self.pool)
                                .await?;
                        } else {
                            self.start_tournament(&id).await?;
                        }
                    }
                },
                "Running" => {
                    self.process_round(&id, round).await?;
                },
                _ => {}
            }
        }

        Ok(())
    }


    async fn start_tournament(&self, tournament_id: &str) -> Result<(), anyhow::Error> {
        tracing::info!("Starting tournament {}", tournament_id);
        
        // 1. Fetch participants
        let participants: Vec<(String, bool)> = sqlx::query_as::<_, (String, bool)>("SELECT kaiju_id, is_bot FROM tournament_participants WHERE tournament_id = ?")
            .bind(tournament_id)
            .fetch_all(&self.pool)
            .await?;

        let mut participant_ids: Vec<String> = participants.into_iter().map(|p| p.0).collect();

        // 2. Pad to power of 2 (min 2)
        let count = participant_ids.len();
        let target = if count < 2 { 2 } else { count.next_power_of_two() };
        
        if count < target {
            tracing::info!("Padding tournament with {} bots", target - count);
            self.add_bots(tournament_id, target - count, &mut participant_ids).await?;
        }
        
        // 3. Shuffle
        participant_ids.shuffle(&mut rand::thread_rng());
        
        // 4. Create Round 1 Matches
        for (i, chunk) in participant_ids.chunks(2).enumerate() {
            if chunk.len() == 2 {
                sqlx::query("INSERT INTO tournament_matches (tournament_id, round_number, match_index, kaiju_a_id, kaiju_b_id) VALUES (?, 1, ?, ?, ?)")
                    .bind(tournament_id)
                    .bind(i as i32)
                    .bind(&chunk[0])
                    .bind(&chunk[1])
                    .execute(&self.pool)
                    .await?;
            } else {
                // Bye? logic for power of 2 ensures chunks(2) is mostly 2. 
                // But next_power_of_two handles it.
            }
        }
        
        // 5. Update State
        sqlx::query("UPDATE tournaments SET state = 'Running', current_round = 1 WHERE id = ?")
            .bind(tournament_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    
    async fn add_bots(&self, tournament_id: &str, count: usize, list: &mut Vec<String>) -> Result<(), anyhow::Error> {
        // Need a system user? For now assume NULL owner is allowed or we use a hack.
        // Actually kaiju.owner_user_id is NOT NULL foreign key. 
        // We need a "Bot User".
        
        // Find or create bot user
        let bot_user_id = "00000000-0000-0000-0000-000000000000"; // Fixed UUID for System
        
        // Ensure user exists (ignore error)
        let _ = sqlx::query("INSERT IGNORE INTO users (id, username, password_hash, email, gold) VALUES (?, 'SystemBot', '', 'bot@system', 0)")
            .bind(bot_user_id)
            .execute(&self.pool)
            .await;
            
        for i in 0..count {
            let bot_id = Uuid::new_v4().to_string();
            let name = format!("Bot-{}", i + 1);
            
            // Insert Bot Kaiju
            // Minimal stats
            let stats = r#"{"hp": 500, "attack": 50, "defense": 50, "speed": 50, "energy": 100}"#;
            
            sqlx::query(r#"
                INSERT INTO kaiju (id, name, generation, owner_user_id, custody_state, genome_hash, genome_data, visual_seed, base_stats, current_stats, visible_traits, state_hash, image_url)
                VALUES (?, ?, 0, ?, 'server', 'bot', ?, '0', ?, ?, '[]', 'bot', 'http://localhost:3000/assets/sprites/kaiju/kaiju_bipedal_neutral_1768091093175.png')
            "#)
            .bind(&bot_id)
            .bind(&name)
            .bind(bot_user_id)
            .bind(&vec![0u8]) // genome_data dummy
            .bind(stats)
            .bind(stats)
            .execute(&self.pool)
            .await?;
            
            // Add to participants
            sqlx::query("INSERT INTO tournament_participants (tournament_id, kaiju_id, user_id, is_bot) VALUES (?, ?, ?, TRUE)")
                .bind(tournament_id)
                .bind(&bot_id)
                .bind(bot_user_id)
                .execute(&self.pool)
                .await?;
                
            list.push(bot_id);
        }
        
        Ok(())
    }

    async fn process_round(&self, tournament_id: &str, round: i32) -> Result<(), anyhow::Error> {
        // Find unresolved matches
        let matches = sqlx::query("SELECT id, kaiju_a_id, kaiju_b_id FROM tournament_matches WHERE tournament_id = ? AND round_number = ? AND winner_id IS NULL")
            .bind(tournament_id)
            .bind(round)
            .fetch_all(&self.pool)
            .await?;

        if matches.is_empty() {
            // Round Complete?
            // Check if there are ANY matches for this round.
             let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tournament_matches WHERE tournament_id = ? AND round_number = ?")
                .bind(tournament_id)
                .bind(round)
                .fetch_one(&self.pool)
                .await?;
                
             if count > 0 {
                 // All matches resolved. Advance round.
                 self.advance_round(tournament_id, round).await?;
             }
             return Ok(());
        }

        // Simulate
        for m in matches {
            let match_id: String = m.get("id");
            let a_id: String = m.get("kaiju_a_id");
            let b_id: String = m.get("kaiju_b_id");
            
            // Fetch stats
            let a_stats: KaijuStats = self.fetch_stats(&a_id).await?;
            let b_stats: KaijuStats = self.fetch_stats(&b_id).await?;
            let a_name: String = sqlx::query_scalar("SELECT name FROM kaiju WHERE id = ?").bind(&a_id).fetch_one(&self.pool).await?;
            let b_name: String = sqlx::query_scalar("SELECT name FROM kaiju WHERE id = ?").bind(&b_id).fetch_one(&self.pool).await?;
            
            let result = simulator::simulate_battle(&a_name, &a_stats, &b_name, &b_stats);
            
            let winner_id = if result.winner_is_a { &a_id } else { &b_id };
            tracing::info!("Match {}: {} vs {} -> Winner {}", match_id, a_name, b_name, winner_id);
            
            sqlx::query("UPDATE tournament_matches SET winner_id = ? WHERE id = ?")
                .bind(winner_id)
                .bind(&match_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    async fn fetch_stats(&self, kaiju_id: &str) -> Result<KaijuStats, anyhow::Error> {
        let json: sqlx::types::Json<KaijuStats> = sqlx::query_scalar("SELECT base_stats FROM kaiju WHERE id = ?")
            .bind(kaiju_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(json.0)
    }

    async fn advance_round(&self, tournament_id: &str, current_round: i32) -> Result<(), anyhow::Error> {
        // Fetch winners of current round matching by index
        // Match 0 winner vs Match 1 winner -> Next Match 0
        
        let winners: Vec<String> = sqlx::query_scalar("SELECT winner_id FROM tournament_matches WHERE tournament_id = ? AND round_number = ? ORDER BY match_index")
            .bind(tournament_id)
            .bind(current_round)
            .fetch_all(&self.pool)
            .await?;
            
        if winners.len() == 1 {
            // Tournament Finished
            let winner = &winners[0];
            tracing::info!("Tournament {} Finished! Winner: {}", tournament_id, winner);
            
            sqlx::query("UPDATE tournaments SET state = 'Finished', winner_kaiju_id = ? WHERE id = ?")
                .bind(winner)
                .bind(tournament_id)
                .execute(&self.pool)
                .await?;
                
            // Increment win count
            sqlx::query("UPDATE kaiju SET tournaments_won = tournaments_won + 1 WHERE id = ?")
                .bind(winner)
                .execute(&self.pool)
                .await?;
                
        } else {
            // Generate next round
            let next_round = current_round + 1;
            for (i, chunk) in winners.chunks(2).enumerate() {
                if chunk.len() == 2 {
                    sqlx::query("INSERT INTO tournament_matches (tournament_id, round_number, match_index, kaiju_a_id, kaiju_b_id) VALUES (?, ?, ?, ?, ?)")
                        .bind(tournament_id)
                        .bind(next_round)
                        .bind(i as i32)
                        .bind(&chunk[0])
                        .bind(&chunk[1])
                        .execute(&self.pool)
                        .await?;
                }
            }
            
            sqlx::query("UPDATE tournaments SET current_round = ? WHERE id = ?")
                .bind(next_round)
                .bind(tournament_id)
                .execute(&self.pool)
                .await?;
        }
        
        Ok(())
    }
}
