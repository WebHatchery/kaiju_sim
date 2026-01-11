//! Main game state - central authority for all mutable game data.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::{Kaiju, KaijuStats, Trait};
use crate::data::types::KaijuId;
use crate::state::player_data::PlayerData;

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
        }
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
        self.roster.iter().filter(move |k| k.generation == gen && k.alive)
    }

    /// Add kaiju to roster
    pub fn add_kaiju(&mut self, kaiju: Kaiju) {
        let name = kaiju.name.clone();
        self.roster.push(kaiju);
        self.notify(format!("{} joined your roster!", name), NotificationType::Success);
    }

    /// Mark kaiju as dead
    pub fn kill_kaiju(&mut self, kaiju_id: KaijuId) {
        if let Some(kaiju) = self.get_kaiju_mut(kaiju_id) {
            let name = kaiju.name.clone();
            kaiju.alive = false;
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
}

/// Create a starter kaiju
fn create_starter_kaiju(name: &str, seed: u64) -> Kaiju {
    let base_hp = 200 + (seed * 20) as i32;
    let base_attack = 40 + (seed * 5) as i32;
    
    Kaiju {
        id: Uuid::new_v4(),
        token_id: 0,
        name: name.to_string(),
        generation: 0,
        created_at: chrono::Utc::now().timestamp(),
        original_breeder: "system".to_string(),
        parent_ids: None,
        visual_seed: seed,
        genome_hash: format!("starter_{}", seed),
        stats: KaijuStats::new(base_hp, base_attack, 30, 25),
        traits: Vec::new(),
        hidden_traits: Vec::new(),
        experience: 0,
        alive: true,
        current_owner: "player".to_string(),
        image_uri: None,
        metadata_uri: String::new(),
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameTime {
    pub total_ticks: u64,
    pub paused: bool,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            total_ticks: 0,
            paused: false,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_state() {
        let state = GameState::new();
        assert_eq!(state.roster.len(), 2);
        assert_eq!(state.player.gold, 1000);
    }

    #[test]
    fn test_get_kaiju() {
        let state = GameState::new();
        let first_id = state.roster[0].id;
        
        assert!(state.get_kaiju(first_id).is_some());
        assert!(state.get_kaiju(Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_living_kaiju() {
        let mut state = GameState::new();
        assert_eq!(state.living_count(), 2);

        let first_id = state.roster[0].id;
        state.kill_kaiju(first_id);
        assert_eq!(state.living_count(), 1);
    }

    #[test]
    fn test_notifications() {
        let mut state = GameState::new();
        state.notify("Test".to_string(), NotificationType::Info);
        assert_eq!(state.notifications.len(), 1);

        // Add 15 more (should cap at 10)
        for i in 0..15 {
            state.notify(format!("Msg {}", i), NotificationType::Info);
        }
        assert_eq!(state.notifications.len(), 10);
    }
}
