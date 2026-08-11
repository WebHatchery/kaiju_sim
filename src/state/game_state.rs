//! Main game state - central authority for all mutable game data.

use crate::data::types::KaijuId;
use crate::data::{Kaiju, KaijuEvent, KaijuEventKind, KaijuStats, Trait};
use crate::state::battle_state::BattleResult;
use crate::state::player_data::PlayerData;
use serde::{Deserialize, Serialize};

/// Main game state - owns all mutable game data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    /// Save format version for migrations
    pub save_version: u32,
    /// Player data
    pub player: PlayerData,
    /// Kaiju roster
    pub roster: Vec<Kaiju>,
    /// Pending breeding session
    pub pending_breeding: Option<BreedingSession>,
    /// Unlocked facilities
    pub unlocked_facilities: Vec<String>,
    /// Research progress
    pub research_progress: ResearchProgress,
    /// Game time
    pub game_time: GameTime,
    /// Current selected kaiju (UI state, not persisted)
    #[serde(skip)]
    pub selected_kaiju: Option<KaijuId>,
    /// UI notifications (not persisted)
    #[serde(skip)]
    pub notifications: Vec<Notification>,
    /// Last fetched tournament status (UI state)
    #[serde(skip)]
    pub tournament_status: Option<crate::server_bridge::TournamentStatusDto>,
    /// Last local battle result for the results screen
    #[serde(skip)]
    pub last_battle: Option<LastBattleReport>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Create new game state with starter kaiju
    pub fn new() -> Self {
        let roster = vec![
            create_starter_kaiju("Volt", 0),
            create_starter_kaiju("Aqua", 1),
        ];

        Self {
            save_version: 1,
            player: PlayerData::default(),
            roster,
            pending_breeding: None,
            unlocked_facilities: vec!["laboratory_lv1".into()],
            research_progress: ResearchProgress::default(),
            game_time: GameTime::default(),
            selected_kaiju: None,
            notifications: Vec::new(),
            tournament_status: None,
            last_battle: None,
        }
    }

    /// Create a local MVP game with the chosen starter plus a second breeder.
    pub fn new_local_game(starter_choice: &str, traits: &[Trait]) -> Self {
        let companion_choice = match starter_choice {
            "Fire" => "Ice",
            "Ice" => "Electric",
            "Electric" => "Fire",
            _ => "Ice",
        };

        let mut state = Self {
            save_version: 1,
            player: PlayerData::default(),
            roster: vec![
                create_elemental_starter(starter_choice, traits, 1),
                create_elemental_starter(companion_choice, traits, 2),
            ],
            pending_breeding: None,
            unlocked_facilities: vec!["laboratory_lv1".into(), "training_ring".into()],
            research_progress: ResearchProgress::default(),
            game_time: GameTime::default(),
            selected_kaiju: None,
            notifications: Vec::new(),
            tournament_status: None,
            last_battle: None,
        };

        state.notify(
            "Facility online. Train, fight, and breed your first bloodline.".to_string(),
            NotificationType::Info,
        );
        state
    }

    /// Find kaiju by ID
    pub fn get_kaiju(&self, id: KaijuId) -> Option<&Kaiju> {
        self.roster.iter().find(|k| k.id == id)
    }

    /// Find mutable kaiju by ID
    pub fn get_kaiju_mut(&mut self, id: KaijuId) -> Option<&mut Kaiju> {
        self.roster.iter_mut().find(|k| k.id == id)
    }

    /// Get all living kaiju
    pub fn living_kaiju(&self) -> impl Iterator<Item = &Kaiju> {
        self.roster.iter().filter(|k| k.alive)
    }

    /// Get kaiju by generation
    pub fn kaiju_by_generation(&self, gen: u32) -> impl Iterator<Item = &Kaiju> {
        self.roster
            .iter()
            .filter(move |k| k.generation == gen && k.alive)
    }

    /// Add kaiju to roster
    pub fn add_kaiju(&mut self, mut kaiju: Kaiju) {
        let name = kaiju.name.clone();
        kaiju.record_event(KaijuEvent::new(
            KaijuEventKind::JoinedRoster,
            "Joined roster",
            "Added to the active breeding facility roster.",
        ));
        self.roster.push(kaiju);
        self.notify(
            format!("{} joined your roster!", name),
            NotificationType::Success,
        );
    }

    /// Mark kaiju as dead
    pub fn kill_kaiju(&mut self, kaiju_id: KaijuId) {
        if let Some(kaiju) = self.get_kaiju_mut(kaiju_id) {
            let name = kaiju.name.clone();
            kaiju.alive = false;
            kaiju.record_event(KaijuEvent::new(
                KaijuEventKind::Death,
                "Fell in competition",
                "Marked deceased in the local legacy record.",
            ));
            self.player.stats.kaiju_deaths += 1;
            self.notify(format!("{} has fallen.", name), NotificationType::Error);
        }
    }

    /// Add notification (max 10)
    pub fn notify(&mut self, message: String, notification_type: NotificationType) {
        self.notifications.push(Notification {
            message,
            notification_type,
            timestamp: self.game_time.total_ticks,
        });

        if self.notifications.len() > 10 {
            self.notifications.remove(0);
        }
    }

    /// Clear all notifications
    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    /// Update game time
    pub fn tick(&mut self) {
        if !self.game_time.paused {
            self.game_time.total_ticks += 1;
        }
    }

    /// Get roster count
    pub fn roster_count(&self) -> usize {
        self.roster.len()
    }

    /// Get living roster count
    pub fn living_count(&self) -> usize {
        self.roster.iter().filter(|k| k.alive).count()
    }

    /// Generate the next local token identity for off-chain MVP kaiju.
    pub fn next_token_id(&self) -> u64 {
        self.roster
            .iter()
            .map(|k| k.token_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }
}

/// Create a starter kaiju
fn create_starter_kaiju(name: &str, seed: u64) -> Kaiju {
    let base_hp = 200 + (seed * 20) as i32;
    let base_attack = 40 + (seed * 5) as i32;

    Kaiju {
        id: crate::data::new_kaiju_id(),
        token_id: seed + 1,
        name: name.to_string(),
        generation: 0,
        created_at: crate::data::now_timestamp(),
        original_breeder: "system".to_string(),
        parent_ids: None,
        visual_seed: seed,
        genome_hash: format!("starter_{}", seed),
        stats: KaijuStats::new(base_hp, base_attack, 30, 25, 100),
        traits: Vec::new(),
        hidden_traits: Vec::new(),
        experience: 0,
        alive: true,
        current_owner: "player".to_string(),
        image_uri: if seed == 0 {
            Some("assets/sprites/kaiju/kaiju_electric_elemental_1768091175509.png".to_string())
        } else {
            Some("assets/sprites/kaiju/kaiju_ice_elemental_1768091156648.png".to_string())
        },
        metadata_uri: String::new(),
        tournaments_won: 0,
        history: vec![
            KaijuEvent::new(
                KaijuEventKind::Created,
                "Starter kaiju registered",
                "Created as an original facility starter.",
            ),
            KaijuEvent::new(
                KaijuEventKind::JoinedRoster,
                "Joined roster",
                "Added to the active breeding facility roster.",
            ),
        ],
    }
}

fn create_elemental_starter(choice: &str, traits: &[Trait], token_id: u64) -> Kaiju {
    let (name, stats, trait_ids, image_uri, seed) = match choice {
        "Fire" => (
            "Ignis",
            KaijuStats::new(220, 58, 28, 34, 100),
            vec!["fire_core"],
            "assets/sprites/kaiju/kaiju_fire_elemental_1768091138860.png",
            11,
        ),
        "Electric" => (
            "Volt",
            KaijuStats::new(205, 45, 27, 54, 110),
            vec!["electric_breath", "swift_reflexes"],
            "assets/sprites/kaiju/kaiju_electric_elemental_1768091175509.png",
            13,
        ),
        _ => (
            "Glacies",
            KaijuStats::new(245, 42, 52, 24, 95),
            vec!["thick_armor"],
            "assets/sprites/kaiju/kaiju_ice_elemental_1768091156648.png",
            12,
        ),
    };

    let visible_traits = trait_ids
        .iter()
        .filter_map(|id| traits.iter().find(|trait_def| trait_def.id == *id).cloned())
        .collect();

    Kaiju {
        id: crate::data::new_kaiju_id(),
        token_id,
        name: name.to_string(),
        generation: 0,
        created_at: crate::data::now_timestamp(),
        original_breeder: "player".to_string(),
        parent_ids: None,
        visual_seed: seed,
        genome_hash: format!("starter_{}_{}", choice.to_lowercase(), seed),
        stats,
        traits: visible_traits,
        hidden_traits: Vec::new(),
        experience: 0,
        alive: true,
        current_owner: "player".to_string(),
        image_uri: Some(image_uri.to_string()),
        metadata_uri: String::new(),
        tournaments_won: 0,
        history: vec![
            KaijuEvent::new(
                KaijuEventKind::Created,
                "Starter chosen",
                format!("Selected as a {} starter for this facility.", choice),
            ),
            KaijuEvent::new(
                KaijuEventKind::JoinedRoster,
                "Joined roster",
                "Added to the active breeding facility roster.",
            ),
        ],
    }
}

/// Breeding session in progress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BreedingSession {
    pub parent_a_id: Option<KaijuId>,
    pub parent_b_id: Option<KaijuId>,
    pub cost: i64,
    pub started_at: u64,
}

/// Research facility progression
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResearchProgress {
    pub facility_level: u32,
    pub current_research: Option<ResearchProject>,
    pub completed_research: Vec<String>,
    pub research_points: i64,
}

/// Active research project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResearchProject {
    pub project_id: String,
    pub project_name: String,
    pub progress: f32,
    pub target_kaiju_id: Option<KaijuId>,
}

/// Game time tracking
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GameTime {
    pub total_ticks: u64,
    pub paused: bool,
}

/// UI notification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub message: String,
    pub notification_type: NotificationType,
    pub timestamp: u64,
}

/// Notification type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

/// Battle outcome cached for the post-fight results screen.
#[derive(Clone, Debug)]
pub struct LastBattleReport {
    pub player_kaiju_id: KaijuId,
    pub opponent: Kaiju,
    pub result: BattleResult,
    pub won: bool,
    pub gold_reward: i64,
    pub xp_reward: u32,
}

#[cfg(test)]
mod tests;
