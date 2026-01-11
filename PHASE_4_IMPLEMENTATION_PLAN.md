# PHASE 4 IMPLEMENTATION PLAN: TOURNAMENT SYSTEM

**Project**: Kaiju Breeding Simulator
**Phase**: 4 - Tournament System
**Status**: Design & Planning
**Dependencies**: Phase 1 (Data Models), Phase 2 (Genetics), Phase 3 (Combat Engine)

**Timeline Estimate**: 2-3 weeks
**Complexity**: High (integrates combat, breeding, death, and NFT systems)

---

## 1. Overview

Phase 4 implements the complete tournament system including:
- Multiple tournament types (Non-Lethal, Lethal, Generation-Restricted, Special Events)
- Three bracket systems (Single Elimination, Double Elimination, Swiss)
- Entry validation and seeding algorithms
- Match scheduling and execution
- Death mechanics for lethal tournaments
- Hall of Fame for fallen kaiju
- Reward distribution and Elo ranking
- Tournament state persistence

---

## 2. Architecture Overview

### 2.1 Module Structure

```
src/
├── data/
│   ├── tournament.rs          # Tournament data structures (NEW)
│   ├── rewards.rs             # Reward structures (NEW)
│   └── ranking.rs             # Elo ranking system (NEW)
├── engine/
│   ├── tournament_engine.rs   # Core tournament logic (NEW)
│   ├── bracket_generator.rs   # Bracket generation algorithms (NEW)
│   ├── seeding.rs             # Seeding and matchmaking (NEW)
│   ├── death_handler.rs       # Death mechanics and refunds (NEW)
│   └── combat.rs              # (Already exists from Phase 3)
├── state/
│   ├── tournament_state.rs    # Active tournament state (NEW)
│   └── hall_of_fame.rs        # Legacy records for dead kaiju (NEW)
└── screens/
    └── tournament_screen.rs   # Tournament UI (NEW)
```

### 2.2 Data Flow

```
Player selects tournament → Entry validation → Seeding
    ↓
Bracket generation → Match scheduling → Combat execution
    ↓
Result recording → Death processing (if lethal) → Reward distribution
    ↓
Ranking updates → Hall of Fame (if deaths) → Next round or finalize
```

---

## 3. Implementation Tasks

### 3.1 Tournament Configuration System

**File**: `H:\RustGames\kaiju_sim\src\data\tournament.rs`

**Core Data Structures**:

```rust
pub struct Tournament {
    pub id: TournamentId,
    pub name: String,
    pub tournament_type: TournamentType,
    pub bracket_system: BracketSystem,
    pub environment_mode: EnvMode,
    pub generation_restriction: Option<GenerationRestriction>,
    pub registration_window: RegistrationWindow,
    pub max_participants: u32,
    pub min_ranking_threshold: Option<i32>,
    pub participants: Vec<KaijuId>,
    pub bracket: Option<Bracket>,
    pub current_round: u32,
    pub status: TournamentStatus,
    pub rewards: RewardStructure,
}

pub enum TournamentType {
    NonLethalRanked,
    LethalWinnerTakesAll,
    GenerationRestricted(GenerationRestriction),
    SpecialEvent { name: String, modifiers: Vec<EventModifier> },
}

pub enum BracketSystem {
    SingleElimination,
    DoubleElimination,
    Swiss { rounds: u32 },
}

pub enum TournamentStatus {
    RegistrationOpen,
    RegistrationClosed,
    InProgress { current_round: u32 },
    Completed { winner_id: KaijuId },
    Cancelled,
}

pub struct Match {
    pub id: MatchId,
    pub tournament_id: TournamentId,
    pub round: u32,
    pub kaiju_a: KaijuId,
    pub kaiju_b: KaijuId,
    pub environment: Environment,
    pub result: Option<MatchResult>,
    pub battle_log: Option<BattleLog>,
    pub battle_seed: u64,
}

pub struct MatchResult {
    pub winner: KaijuId,
    pub loser: KaijuId,
    pub turns: u32,
    pub final_hp: (i32, i32),
    pub was_upset: bool,
    pub upset_probability: f32,
}
```

**Configuration Loading** (from JSON):

File: `H:\RustGames\kaiju_sim\assets\tournament_configs.json`

```json
{
  "tournaments": [
    {
      "id": "weekly_ranked",
      "name": "Weekly Ranked Battle",
      "type": "NonLethalRanked",
      "bracket": "SingleElimination",
      "max_participants": 8,
      "environment": "Random",
      "registration_hours": 2,
      "schedule": "Every 4 hours"
    },
    {
      "id": "lethal_crucible",
      "name": "Champion's Crucible",
      "type": "LethalWinnerTakesAll",
      "bracket": "SingleElimination",
      "max_participants": 8,
      "environment": "Neutral",
      "min_ranking": 1000,
      "registration_hours": 24,
      "schedule": "Weekly Friday 20:00 UTC"
    }
  ]
}
```

**Implementation Steps**:
1. Define all tournament enums and structs
2. Implement JSON deserialization for tournament configs
3. Create tournament factory function `create_tournament_from_config()`
4. Add tournament ID generation (UUID-based)
5. Implement tournament serialization for save/load

**Tests**:
- Load tournament configs from JSON
- Validate tournament structure constraints
- Serialize/deserialize tournament state

---

### 3.2 Entry Validation and Eligibility

**File**: `H:\RustGames\kaiju_sim\src\engine\tournament_engine.rs`

**Entry Validation System**:

```rust
pub struct EntryValidator;

impl EntryValidator {
    pub fn validate_entry(
        kaiju: &Kaiju,
        tournament: &Tournament,
        owner: &Player,
    ) -> Result<(), EntryError> {
        // 1. Check kaiju alive status
        if !kaiju.alive {
            return Err(EntryError::KaijuDead);
        }

        // 2. Check ownership
        if kaiju.owner != owner.id {
            return Err(EntryError::NotOwner);
        }

        // 3. Check generation restrictions
        if let Some(restriction) = &tournament.generation_restriction {
            if !self.meets_generation_requirement(kaiju, restriction) {
                return Err(EntryError::GenerationMismatch);
            }
        }

        // 4. Check ranking minimum (lethal only)
        if tournament.tournament_type == TournamentType::LethalWinnerTakesAll {
            if kaiju.ranking < tournament.min_ranking_threshold.unwrap_or(0) {
                return Err(EntryError::RankingTooLow);
            }
        }

        // 5. Check not already registered
        if self.is_registered_in_active_tournament(kaiju.id) {
            return Err(EntryError::AlreadyRegistered);
        }

        // 6. Check registration window
        if tournament.status != TournamentStatus::RegistrationOpen {
            return Err(EntryError::RegistrationClosed);
        }

        // 7. Check tournament not full
        if tournament.participants.len() >= tournament.max_participants as usize {
            return Err(EntryError::TournamentFull);
        }

        Ok(())
    }
}
```

**Entry Error Types**:

```rust
pub enum EntryError {
    KaijuDead,
    NotOwner,
    GenerationMismatch,
    RankingTooLow,
    AlreadyRegistered,
    RegistrationClosed,
    TournamentFull,
    InsufficientExperience,
}
```

**Lethal Tournament Confirmation Flow**:

```rust
pub struct LethalConfirmation {
    pub kaiju_id: KaijuId,
    pub tournament_id: TournamentId,
    pub confirmation_text: String,  // User must type kaiju name
    pub warnings: Vec<String>,
    pub offspring_count: u32,
    pub breeding_rights_issued: u32,
}

impl LethalConfirmation {
    pub fn create_for_kaiju(kaiju: &Kaiju, tournament: &Tournament) -> Self {
        let offspring_count = count_offspring(kaiju.id);
        let breeding_rights = count_unused_breeding_rights(kaiju.id);

        Self {
            kaiju_id: kaiju.id,
            tournament_id: tournament.id,
            confirmation_text: kaiju.name.clone(),
            warnings: vec![
                "This kaiju will die permanently if they lose".to_string(),
                format!("This kaiju has {} offspring", offspring_count),
                format!("{} unused breeding rights will be refunded", breeding_rights),
                "This action CANNOT be undone".to_string(),
            ],
            offspring_count,
            breeding_rights_issued: breeding_rights,
        }
    }
}
```

**Implementation Steps**:
1. Implement `EntryValidator` struct with all validation checks
2. Create `EntryError` enum with descriptive messages
3. Build lethal tournament confirmation system
4. Add registration tracking (prevent duplicates)
5. Implement registration window timing checks
6. Add database query for active tournament check

**Tests**:
- Valid entry passes all checks
- Dead kaiju rejected
- Generation mismatch rejected
- Ranking too low rejected
- Already registered rejected
- Full tournament rejected
- Lethal confirmation requires exact name match

---

### 3.3 Seeding Algorithm

**File**: `H:\RustGames\kaiju_sim\src\engine\seeding.rs`

**Seeding Formula**:

```rust
pub struct SeedCalculator;

impl SeedCalculator {
    pub fn calculate_seed_score(&self, kaiju: &Kaiju) -> f32 {
        let global_rank = kaiju.ranking as f32;
        let recent_winrate = self.calculate_recent_winrate(kaiju, 10);
        let tournament_wins = kaiju.tournament_victories as f32;
        let generation_factor = kaiju.generation as f32;

        (global_rank * 0.5)
            + (recent_winrate * 0.3 * 1000.0)  // Scale to comparable range
            + (tournament_wins * 0.15 * 50.0)
            + (generation_factor * 0.05 * 10.0)
    }

    fn calculate_recent_winrate(&self, kaiju: &Kaiju, last_n: u32) -> f32 {
        let recent_battles = get_recent_battles(kaiju.id, last_n);
        if recent_battles.is_empty() {
            return 0.5;  // Default to 50% if no history
        }

        let wins = recent_battles.iter().filter(|b| b.winner == kaiju.id).count();
        wins as f32 / recent_battles.len() as f32
    }

    pub fn seed_participants(&self, mut participants: Vec<Kaiju>) -> Vec<Kaiju> {
        participants.sort_by(|a, b| {
            let score_a = self.calculate_seed_score(a);
            let score_b = self.calculate_seed_score(b);
            score_b.partial_cmp(&score_a).unwrap()  // Descending order
        });
        participants
    }
}
```

**Bracket Pairing** (Single Elimination):

```rust
pub fn create_seeded_bracket_pairings(seeded: Vec<Kaiju>) -> Vec<(Kaiju, Kaiju)> {
    let mut pairings = Vec::new();
    let n = seeded.len();

    // Pair highest seed with lowest seed
    for i in 0..(n / 2) {
        pairings.push((seeded[i].clone(), seeded[n - 1 - i].clone()));
    }

    pairings
}
```

**Example Seeding** (8 participants):
```
Seed 1 vs Seed 8
Seed 4 vs Seed 5
Seed 2 vs Seed 7
Seed 3 vs Seed 6
```

**Implementation Steps**:
1. Implement seed score calculation function
2. Create recent winrate calculator
3. Build participant sorting by seed
4. Implement bracket pairing algorithm
5. Add unseeded (random) mode for practice tournaments
6. Create seed visualization for UI

**Tests**:
- Seed scores calculate correctly with known values
- Participants sort in descending seed order
- Bracket pairings follow standard seeding (1v8, 2v7, etc.)
- Edge case: Equal seed scores (use secondary tiebreaker)
- Edge case: Odd number of participants (bye round)

---

### 3.4 Bracket Generation

**File**: `H:\RustGames\kaiju_sim\src\engine\bracket_generator.rs`

**Bracket Data Structure**:

```rust
pub struct Bracket {
    pub bracket_type: BracketSystem,
    pub matches: Vec<Match>,
    pub rounds: Vec<Round>,
    pub advancement_map: HashMap<MatchId, MatchId>,  // Winner of X goes to Y
    pub participant_count: u32,
}

pub struct Round {
    pub number: u32,
    pub matches: Vec<MatchId>,
    pub status: RoundStatus,
    pub environment: Environment,
}

pub enum RoundStatus {
    Pending,
    InProgress,
    Completed,
}
```

#### 3.4.1 Single Elimination

```rust
pub struct SingleEliminationGenerator;

impl SingleEliminationGenerator {
    pub fn generate(&self, participants: Vec<Kaiju>, tournament: &Tournament) -> Bracket {
        let seeded = seed_participants(participants);
        let rounds_count = (seeded.len() as f32).log2().ceil() as u32;
        let mut matches = Vec::new();
        let mut rounds = Vec::new();

        // Round 1: Initial pairings
        let mut round_1_matches = Vec::new();
        for i in 0..(seeded.len() / 2) {
            let match_id = MatchId::new();
            let match_obj = Match {
                id: match_id,
                tournament_id: tournament.id,
                round: 1,
                kaiju_a: seeded[i].id,
                kaiju_b: seeded[seeded.len() - 1 - i].id,
                environment: select_environment(tournament, 1),
                result: None,
                battle_log: None,
                battle_seed: generate_battle_seed(),
            };
            matches.push(match_obj);
            round_1_matches.push(match_id);
        }

        rounds.push(Round {
            number: 1,
            matches: round_1_matches,
            status: RoundStatus::Pending,
            environment: select_environment(tournament, 1),
        });

        // Create advancement map (winners proceed to next round)
        let advancement_map = self.create_advancement_map(&matches, rounds_count);

        Bracket {
            bracket_type: BracketSystem::SingleElimination,
            matches,
            rounds,
            advancement_map,
            participant_count: seeded.len() as u32,
        }
    }
}
```

#### 3.4.2 Swiss System

```rust
pub struct SwissGenerator;

impl SwissGenerator {
    pub fn generate(&self, participants: Vec<Kaiju>, rounds: u32, tournament: &Tournament) -> Bracket {
        // Swiss system: All participants play fixed number of rounds
        // Pairing based on current record (wins/losses)

        let mut bracket = Bracket {
            bracket_type: BracketSystem::Swiss { rounds },
            matches: Vec::new(),
            rounds: Vec::new(),
            advancement_map: HashMap::new(),  // N/A for Swiss
            participant_count: participants.len() as u32,
        };

        // Round 1: Random pairing
        let round_1_matches = self.create_initial_pairings(participants, tournament);
        bracket.rounds.push(Round {
            number: 1,
            matches: round_1_matches.iter().map(|m| m.id).collect(),
            status: RoundStatus::Pending,
            environment: select_environment(tournament, 1),
        });
        bracket.matches.extend(round_1_matches);

        bracket
    }

    pub fn generate_next_round_pairings(
        &self,
        bracket: &Bracket,
        current_records: Vec<(KaijuId, WinLossRecord)>,
        round_number: u32,
    ) -> Vec<Match> {
        // Group by record (0-0, 1-0, 0-1, 1-1, etc.)
        let mut groups: HashMap<WinLossRecord, Vec<KaijuId>> = HashMap::new();
        for (kaiju_id, record) in current_records {
            groups.entry(record).or_insert_with(Vec::new).push(kaiju_id);
        }

        // Within each group, pair sequentially
        let mut matches = Vec::new();
        for (record, mut kaiju_list) in groups {
            // Sort by tiebreaker (opponent strength, damage dealt, etc.)
            kaiju_list.sort_by(|a, b| {
                self.compare_tiebreaker(a, b, bracket)
            });

            // Pair 1 vs 2, 3 vs 4, etc.
            for chunk in kaiju_list.chunks(2) {
                if chunk.len() == 2 {
                    matches.push(Match {
                        id: MatchId::new(),
                        tournament_id: bracket.tournament_id,
                        round: round_number,
                        kaiju_a: chunk[0],
                        kaiju_b: chunk[1],
                        environment: select_environment(tournament, round_number),
                        result: None,
                        battle_log: None,
                        battle_seed: generate_battle_seed(),
                    });
                } else {
                    // Bye round (odd number)
                    award_bye(chunk[0], round_number);
                }
            }
        }

        matches
    }
}
```

**Implementation Steps**:
1. Implement single elimination generator (priority 1)
2. Create advancement map builder
3. Implement Swiss system generator (priority 2)
4. Add bye round handling for odd participants
5. Implement double elimination (optional, priority 3)
6. Add bracket visualization helper functions

**Tests**:
- 8-participant single elimination creates correct bracket
- 16-participant bracket has correct number of rounds
- Advancement map correctly links matches
- Swiss system pairs by record correctly
- Bye rounds awarded properly for odd numbers
- Edge case: 2 participants (direct final)
- Edge case: 3 participants (one bye, then final)

---

### 3.5 Death System Implementation

**File**: `H:\RustGames\kaiju_sim\src\engine\death_handler.rs`

**Death Handler**:

```rust
pub struct DeathHandler {
    db_pool: PgPool,
}

impl DeathHandler {
    pub async fn process_lethal_loss(
        &self,
        kaiju_id: KaijuId,
        tournament_id: TournamentId,
    ) -> Result<(), DeathError> {
        // 1. Mark kaiju as dead in database
        self.mark_kaiju_dead(kaiju_id, tournament_id).await?;

        // 2. Refund unused breeding rights
        self.refund_breeding_rights(kaiju_id).await?;

        // 3. Create Hall of Fame entry
        self.create_hall_of_fame_entry(kaiju_id, tournament_id).await?;

        // 4. Notify owner and breeding rights holders
        self.send_death_notifications(kaiju_id).await?;

        // 5. Update blockchain if kaiju was minted
        if let Some(token_id) = self.get_blockchain_token_id(kaiju_id).await? {
            self.mark_dead_on_chain(token_id).await?;
        }

        Ok(())
    }

    async fn mark_kaiju_dead(
        &self,
        kaiju_id: KaijuId,
        tournament_id: TournamentId,
    ) -> Result<(), DeathError> {
        sqlx::query!(
            r#"
            UPDATE kaiju
            SET alive = FALSE,
                death_timestamp = NOW(),
                death_tournament_id = $1
            WHERE id = $2 AND alive = TRUE
            "#,
            tournament_id,
            kaiju_id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn refund_breeding_rights(&self, kaiju_id: KaijuId) -> Result<(), DeathError> {
        // Get all unused breeding rights for this kaiju
        let unused_rights = sqlx::query!(
            r#"
            SELECT id, owner_user_id, purchase_price
            FROM breeding_rights
            WHERE kaiju_id = $1 AND used = FALSE
            "#,
            kaiju_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        // Refund each holder
        for right in unused_rights {
            // Credit user account
            sqlx::query!(
                r#"
                UPDATE users
                SET currency_balance = currency_balance + $1
                WHERE id = $2
                "#,
                right.purchase_price,
                right.owner_user_id
            )
            .execute(&self.db_pool)
            .await?;

            // Mark right as refunded
            sqlx::query!(
                r#"
                UPDATE breeding_rights
                SET refunded = TRUE, refund_timestamp = NOW()
                WHERE id = $1
                "#,
                right.id
            )
            .execute(&self.db_pool)
            .await?;

            // Log refund transaction
            self.log_refund_transaction(right.id, right.owner_user_id, right.purchase_price).await?;
        }

        Ok(())
    }

    async fn create_hall_of_fame_entry(
        &self,
        kaiju_id: KaijuId,
        tournament_id: TournamentId,
    ) -> Result<(), DeathError> {
        // Load kaiju full data
        let kaiju = load_kaiju(kaiju_id).await?;

        // Gather legacy data
        let lifetime_record = get_lifetime_battle_record(kaiju_id).await?;
        let tournament_victories = get_tournament_victories(kaiju_id).await?;
        let offspring_count = count_offspring(kaiju_id).await?;
        let notable_descendants = find_notable_descendants(kaiju_id).await?;
        let achievements = get_kaiju_achievements(kaiju_id).await?;

        // Create legacy record
        let legacy = LegacyRecord {
            kaiju_id,
            name: kaiju.name,
            generation: kaiju.generation,
            final_stats: kaiju.stats,
            visible_traits: kaiju.visible_traits,
            hidden_traits: kaiju.hidden_traits,  // Revealed post-mortem
            lifetime_record,
            tournament_victories,
            offspring_count,
            notable_descendants,
            death_date: now(),
            death_context: format!("Defeated in {} tournament", get_tournament_name(tournament_id).await?),
            achievements,
        };

        // Insert into Hall of Fame table
        sqlx::query!(
            r#"
            INSERT INTO hall_of_fame (kaiju_id, legacy_data, created_at)
            VALUES ($1, $2, NOW())
            "#,
            kaiju_id,
            serde_json::to_value(&legacy)?
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
```

**Implementation Steps**:
1. Implement `DeathHandler` struct with database integration
2. Create `mark_kaiju_dead()` database function
3. Build breeding rights refund system
4. Implement Hall of Fame entry creation
5. Add notification system for owners and rights holders
6. Create blockchain death synchronization (async)
7. Build lethal confirmation UI components

**Tests**:
- Dead kaiju marked correctly in database
- Breeding rights refunded to correct holders
- Hall of Fame entry created with full data
- Hidden traits revealed post-mortem
- Blockchain death flag set (if minted)
- Cannot enter dead kaiju in tournaments
- Cannot breed with dead kaiju

---

### 3.6 Hall of Fame Implementation

**File**: `H:\RustGames\kaiju_sim\src\state\hall_of_fame.rs`

**Hall of Fame Data Structures**:

```rust
pub struct HallOfFame {
    pub entries: Vec<HallOfFameEntry>,
}

pub struct HallOfFameEntry {
    pub legacy: LegacyRecord,
    pub category: Vec<HallOfFameCategory>,
    pub ranking: u32,  // Position in Hall of Fame (by prestige)
}

pub enum HallOfFameCategory {
    LethalTournamentWinner,
    HighestRanking,
    MostTournamentWins,
    LongestWinStreak,
    MostOffspring,
    LegendaryBloodline,
    FirstOfGeneration,
}

pub struct LegacyRecord {
    pub kaiju_id: KaijuId,
    pub name: String,
    pub generation: u32,
    pub final_stats: KaijuStats,
    pub visible_traits: Vec<Trait>,
    pub hidden_traits: Vec<Trait>,  // Revealed!
    pub lifetime_record: MatchRecord,
    pub tournament_victories: Vec<TournamentId>,
    pub offspring_count: u32,
    pub notable_descendants: Vec<KaijuId>,
    pub death_date: u64,
    pub death_context: String,
    pub achievements: Vec<Achievement>,
}
```

**Implementation Steps**:
1. Create `hall_of_fame` database table
2. Implement `LegacyRecord` struct and serialization
3. Build Hall of Fame query system with filters
4. Create category classification logic
5. Implement search and sorting functions
6. Build lineage tree visualization
7. Create Hall of Fame UI screen

**Tests**:
- Legacy records saved correctly
- Hidden traits revealed in Hall of Fame
- Category filters work correctly
- Search finds kaiju by name
- Lineage tree includes dead ancestors
- Sorting by different criteria works

---

### 3.7 Reward Distribution System

**File**: `H:\RustGames\kaiju_sim\src\data\rewards.rs`

**Reward Structures**:

```rust
pub struct RewardStructure {
    pub tournament_type: TournamentType,
    pub placement_rewards: Vec<PlacementReward>,
    pub participation_reward: ParticipationReward,
}

pub struct PlacementReward {
    pub placement: u32,  // 1st, 2nd, 3rd, etc.
    pub xp: u32,
    pub ranking_points: i32,
    pub title: Option<String>,
    pub badge: Option<BadgeId>,
    pub currency: Option<u64>,
}
```

**XP Calculation**:

```rust
pub fn calculate_xp(
    result: &MatchResult,
    tournament_type: &TournamentType,
    round: u32,
) -> u32 {
    let base_xp = if result.winner == kaiju_id {
        100
    } else {
        30
    };

    let multiplier = match tournament_type {
        TournamentType::NonLethalRanked => 1.0,
        TournamentType::LethalWinnerTakesAll => 2.0,
        TournamentType::SpecialEvent { .. } => 1.5,
        _ => 1.0,
    };

    let round_bonus = match round {
        1 => 0,
        2 => 20,
        3 => 50,
        4 => 100,
        _ => 0,
    };

    let upset_bonus = if result.was_upset && result.winner == kaiju_id {
        50
    } else {
        0
    };

    ((base_xp as f32 * multiplier) as u32) + round_bonus + upset_bonus
}
```

**Elo Rating Update**:

```rust
pub fn update_elo_ranking(
    winner: &mut Kaiju,
    loser: &mut Kaiju,
    was_upset: bool,
) {
    const K_FACTOR: f32 = 32.0;

    let expected_winner = 1.0 / (1.0 + 10f32.powf((loser.ranking - winner.ranking) as f32 / 400.0));
    let expected_loser = 1.0 - expected_winner;

    let winner_gain = (K_FACTOR * (1.0 - expected_winner)) as i32;
    let loser_loss = (K_FACTOR * (0.0 - expected_loser)) as i32;

    winner.ranking += winner_gain;
    loser.ranking += loser_loss;

    // Floor at 0
    loser.ranking = loser.ranking.max(0);
}
```

**Implementation Steps**:
1. Create reward data structures
2. Implement XP calculation function
3. Build Elo ranking update logic
4. Create reward distributor
5. Implement title and badge system
6. Add currency reward distribution
7. Build lethal winner bonus system

**Tests**:
- XP calculated correctly for wins/losses
- Upset bonus applied correctly
- Round bonuses applied
- Elo ratings update correctly
- Ranking never goes negative
- Lethal winner receives all bonuses
- Titles and badges awarded correctly

---

## 4. Database Integration

### 4.1 Tournament Tables

**SQL Schema** (extends DATABASE_SCHEMA.md):

```sql
-- Tournaments
CREATE TABLE tournaments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    tournament_type TEXT NOT NULL,
    bracket_system TEXT NOT NULL,
    max_participants INT NOT NULL,
    min_ranking_threshold INT,
    generation_restriction JSONB,
    environment_mode TEXT NOT NULL,
    status TEXT NOT NULL,
    current_round INT NOT NULL DEFAULT 0,
    registration_opens_at TIMESTAMPTZ NOT NULL,
    registration_closes_at TIMESTAMPTZ NOT NULL,
    starts_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    winner_id UUID REFERENCES kaiju(id),
    bracket_data JSONB,
    reward_structure JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tournament Entries
CREATE TABLE tournament_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id),
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),
    owner_user_id UUID NOT NULL REFERENCES users(id),
    seed INT,
    placement INT,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed BOOLEAN NOT NULL DEFAULT FALSE,

    UNIQUE(tournament_id, kaiju_id)
);

-- Matches
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id),
    round INT NOT NULL,
    kaiju_a_id UUID NOT NULL REFERENCES kaiju(id),
    kaiju_b_id UUID NOT NULL REFERENCES kaiju(id),
    environment TEXT NOT NULL,
    battle_seed BIGINT NOT NULL,
    winner_id UUID REFERENCES kaiju(id),
    loser_id UUID REFERENCES kaiju(id),
    turns INT,
    final_hp_a INT,
    final_hp_b INT,
    was_upset BOOLEAN,
    upset_probability FLOAT,
    battle_log JSONB,
    executed_at TIMESTAMPTZ,

    CONSTRAINT valid_participants CHECK (kaiju_a_id != kaiju_b_id)
);

-- Hall of Fame
CREATE TABLE hall_of_fame (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),
    legacy_data JSONB NOT NULL,
    categories TEXT[] NOT NULL DEFAULT '{}',
    prestige_score INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## 5. Implementation Order

### Phase 4A: Core Tournament System (Week 1)
1. Tournament data structures (`data/tournament.rs`)
2. Entry validation (`engine/tournament_engine.rs`)
3. Seeding algorithm (`engine/seeding.rs`)
4. Single elimination bracket generator (`engine/bracket_generator.rs`)
5. Match execution integration with combat engine
6. Basic tournament state persistence

### Phase 4B: Rewards and Ranking (Week 1.5)
1. Reward structures (`data/rewards.rs`)
2. XP calculation
3. Elo ranking system (`data/ranking.rs`)
4. Reward distribution logic
5. Leaderboard queries

### Phase 4C: Death Mechanics (Week 2)
1. Death handler (`engine/death_handler.rs`)
2. Breeding rights refund system
3. Hall of Fame tables and queries (`state/hall_of_fame.rs`)
4. Legacy record creation
5. Lethal confirmation UI

### Phase 4D: Advanced Features (Week 2.5)
1. Swiss system bracket generator
2. Hall of Fame UI
3. Tournament history tracking
4. Spectator mode
5. Tournament scheduling system

### Phase 4E: Testing and Polish (Week 3)
1. Unit tests for all modules
2. Integration tests for full tournaments
3. Edge case testing
4. UI polish and error messages
5. Performance optimization

---

## 6. Critical Files for Implementation

### H:\RustGames\kaiju_sim\src\data\tournament.rs
**Reason**: Core tournament data structures (Tournament, Match, Bracket, MatchResult). This file defines the fundamental types for all tournament operations and must be implemented first as all other modules depend on these types.

### H:\RustGames\kaiju_sim\src\engine\tournament_engine.rs
**Reason**: Tournament execution logic including match scheduling, round progression, and integration with combat engine. This is the orchestrator that ties together bracket generation, combat simulation, and reward distribution. Contains the critical `execute_round()` and `execute_match()` functions.

### H:\RustGames\kaiju_sim\src\engine\bracket_generator.rs
**Reason**: Bracket generation algorithms for Single Elimination, Double Elimination, and Swiss systems. Implements seeding pairings and advancement logic. Critical for tournament structure creation and must handle edge cases like odd participants and bye rounds.

### H:\RustGames\kaiju_sim\src\engine\death_handler.rs
**Reason**: Death mechanics for lethal tournaments including permanent kaiju death, breeding rights refunds, and Hall of Fame entry creation. This is unique to the tournament system and handles the high-stakes consequences that define the game's "legacy is permanent" philosophy.

### H:\RustGames\kaiju_sim\src\state\hall_of_fame.rs
**Reason**: Hall of Fame implementation for preserving legacy records of fallen kaiju. Includes legacy record creation, query system, and lineage tree visualization. Essential for the "tombstone, not trash" design principle and provides long-term value to deceased kaiju.

---

## 7. Success Criteria

Phase 4 is considered complete when:

- [ ] Players can register kaiju for tournaments
- [ ] Single elimination tournaments execute correctly
- [ ] Swiss tournaments execute correctly
- [ ] XP and ranking rewards distributed
- [ ] Lethal tournaments kill losers
- [ ] Breeding rights refunded on death
- [ ] Hall of Fame displays fallen kaiju
- [ ] All unit tests pass (90%+ coverage)
- [ ] Integration tests pass for key scenarios
- [ ] Tournament UI functional (lobby, bracket, results)
- [ ] Performance acceptable (<1s per match execution)
- [ ] Database schema deployed and tested

---

**END OF PHASE 4 IMPLEMENTATION PLAN**
