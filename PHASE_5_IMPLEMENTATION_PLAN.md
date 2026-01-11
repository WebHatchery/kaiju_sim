# PHASE 5 IMPLEMENTATION PLAN: State Management

## Document Overview

**Phase**: 5 - State Management
**Purpose**: Implement game state, persistence, and progression tracking for Kaiju Breeding Simulator
**Prerequisites**: Phase 1 (Foundation & Data Models) must be complete
**Estimated Effort**: 1 week
**Status**: Planning Complete, Implementation Pending

---

## 1. Introduction

### 1.1 Phase Goals

Phase 5 establishes the core state management infrastructure for Kaiju Breeding Simulator. This includes:

- **GameState structure** - Central authority for all game state
- **Phase transition system** - Explicit state machine for game screens
- **Persistence layer** - Save/load functionality for local gameplay
- **Player progression** - Player-specific data management
- **Roster management** - Kaiju collection tracking
- **Tournament state** - Active tournament tracking
- **Validation system** - State integrity checks
- **Migration strategy** - Forward compatibility for save format changes

### 1.2 Design Philosophy

Following the project's architectural principles:

- **Single Source of Truth**: GameState owns all mutable state
- **Explicit Transitions**: No implicit state changes
- **Stateless Services**: Engine modules receive state, return results
- **UI is Dumb**: UI reads state, returns action intents
- **Data-Driven**: Configuration loaded from JSON, not hardcoded

### 1.3 Dual-Mode Architecture

This phase supports **two deployment modes**:

1. **Local-Only Mode** (Phase 1-8): All state in local JSON files, no server
2. **Server-Custodial Mode** (Phase 0+): State synchronized with MySQL database

The implementation starts with local-only mode and provides hooks for server integration.

---

## 2. GameState Structure Design

### 2.1 Core GameState Struct

**File**: `src/state/game_state.rs`

```rust
use serde::{Deserialize, Serialize};
use crate::data::{Kaiju, KaijuId, TournamentId, Tournament};

/// Main game state - owns all mutable game data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    // Versioning
    pub save_version: u32,

    // Player data
    pub player: PlayerData,

    // Kaiju roster
    pub roster: Vec<Kaiju>,

    // Active sessions
    pub pending_breeding: Option<BreedingSession>,
    pub active_tournament: Option<TournamentState>,

    // Progression
    pub unlocked_facilities: Vec<String>,
    pub research_progress: ResearchProgress,

    // Game time (for tournament scheduling, breeding cooldowns)
    pub game_time: GameTime,

    // UI state (not persisted)
    #[serde(skip)]
    pub selected_kaiju: Option<KaijuId>,
    #[serde(skip)]
    pub ui_notifications: Vec<Notification>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Create new game state with starter kaiju
    pub fn new() -> Self {
        let mut roster = Vec::new();

        // Add 2 starter gen-0 kaiju
        roster.push(Kaiju::generate_starter("Volt", 0));
        roster.push(Kaiju::generate_starter("Aqua", 1));

        Self {
            save_version: 1,
            player: PlayerData::default(),
            roster,
            pending_breeding: None,
            active_tournament: None,
            unlocked_facilities: vec!["laboratory_lv1".into()],
            research_progress: ResearchProgress::default(),
            game_time: GameTime::default(),
            selected_kaiju: None,
            ui_notifications: Vec::new(),
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

    /// Add notification (max 10)
    pub fn notify(&mut self, message: String, notification_type: NotificationType) {
        self.ui_notifications.push(Notification {
            message,
            notification_type,
            timestamp: self.game_time.total_ticks,
        });

        if self.ui_notifications.len() > 10 {
            self.ui_notifications.remove(0);
        }
    }
}
```

### 2.2 PlayerData Structure

**File**: `src/state/player_data.rs`

```rust
use serde::{Deserialize, Serialize};

/// Player-specific persistent data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub player_id: String,
    pub display_name: String,

    // Currency
    pub gold: i64,
    pub premium_currency: i64,

    // Progression
    pub player_level: u32,
    pub experience: u64,
    pub reputation: i32,

    // Statistics
    pub stats: PlayerStats,

    // Settings
    pub settings: PlayerSettings,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            player_id: uuid::Uuid::new_v4().to_string(),
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

/// Player statistics (lifetime tracking)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    pub total_kaiju_bred: u32,
    pub total_tournaments_entered: u32,
    pub total_tournaments_won: u32,
    pub total_battles_fought: u32,
    pub total_battles_won: u32,
    pub kaiju_deaths: u32,
    pub highest_generation: u32,
    pub legendary_bloodlines: Vec<String>,
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
```

### 2.3 Supporting Structures

**File**: `src/state/game_state.rs` (continued)

```rust
/// Breeding session in progress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BreedingSession {
    pub parent_a_id: Option<KaijuId>,
    pub parent_b_id: Option<KaijuId>,
    pub offspring_preview: Option<KaijuPreview>,
    pub cost: i64,
    pub started_at: u64,
}

/// Preview of potential offspring
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KaijuPreview {
    pub estimated_stats: StatRange,
    pub possible_traits: Vec<String>,
    pub generation: u32,
}

/// Stat range for previews
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatRange {
    pub hp: (i32, i32),
    pub attack: (i32, i32),
    pub defense: (i32, i32),
    pub speed: (i32, i32),
}

/// Active tournament state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TournamentState {
    pub tournament_id: TournamentId,
    pub tournament_config: Tournament,
    pub entered_kaiju_id: KaijuId,
    pub current_round: u32,
    pub bracket: TournamentBracket,
    pub status: TournamentStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TournamentStatus {
    Registration,
    InProgress,
    Completed,
    Failed,
}

/// Research facility progression
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResearchProgress {
    pub facility_level: u32,
    pub current_research: Option<ResearchProject>,
    pub completed_research: Vec<String>,
    pub research_points: i64,
}

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

/// UI Notification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub message: String,
    pub notification_type: NotificationType,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}
```

---

## 3. Phase Transition System

### 3.1 GamePhase Enum

**File**: `src/state/game_phase.rs`

```rust
/// Game phases - explicit state machine for screens
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Initial loading screen
    Loading,

    /// Main menu (new game, continue, settings, exit)
    MainMenu,

    /// Laboratory hub (research, roster management)
    Laboratory,

    /// Breeding interface (select parents, preview offspring)
    Breeding,

    /// Tournament lobby (browse tournaments, view odds)
    TournamentLobby,

    /// Active battle viewing
    Battle,

    /// Post-battle results screen
    Results,

    /// Leaderboard and Hall of Fame
    Leaderboard,

    /// Lineage tree viewer
    LineageViewer(KaijuId),

    /// Kaiju detail view (modal overlay)
    KaijuDetail(KaijuId),
}

impl Default for GamePhase {
    fn default() -> Self {
        Self::Loading
    }
}

impl GamePhase {
    /// Check if phase requires loaded game state
    pub fn requires_game_state(&self) -> bool {
        !matches!(self, Self::Loading | Self::MainMenu)
    }

    /// Check if phase is a modal overlay
    pub fn is_modal(&self) -> bool {
        matches!(self, Self::KaijuDetail(_))
    }
}
```

### 3.2 Phase Transitions

**File**: `src/state/phase_transition.rs`

```rust
use crate::state::GamePhase;
use crate::data::KaijuId;

/// Explicit phase transitions returned by screens
#[derive(Clone, Debug)]
pub enum PhaseTransition {
    None,
    Push(GamePhase),        // Add new phase to stack (for modals)
    Replace(GamePhase),     // Replace current phase
    Pop,                    // Return to previous phase
    Reset(GamePhase),       // Clear stack, set new phase
}

impl PhaseTransition {
    /// Create transition to main menu
    pub fn to_menu() -> Self {
        Self::Reset(GamePhase::MainMenu)
    }

    /// Create transition to laboratory
    pub fn to_laboratory() -> Self {
        Self::Replace(GamePhase::Laboratory)
    }

    /// Create transition to breeding screen
    pub fn to_breeding() -> Self {
        Self::Replace(GamePhase::Breeding)
    }

    /// Create transition to tournament lobby
    pub fn to_tournament_lobby() -> Self {
        Self::Replace(GamePhase::TournamentLobby)
    }

    /// Create transition to battle
    pub fn to_battle() -> Self {
        Self::Replace(GamePhase::Battle)
    }

    /// Create transition to results
    pub fn to_results() -> Self {
        Self::Replace(GamePhase::Results)
    }

    /// Create modal transition to kaiju detail
    pub fn show_kaiju_detail(kaiju_id: KaijuId) -> Self {
        Self::Push(GamePhase::KaijuDetail(kaiju_id))
    }

    /// Close current modal
    pub fn close_modal() -> Self {
        Self::Pop
    }
}

/// Phase stack for modal management
#[derive(Clone, Debug, Default)]
pub struct PhaseStack {
    phases: Vec<GamePhase>,
}

impl PhaseStack {
    pub fn new(initial_phase: GamePhase) -> Self {
        Self {
            phases: vec![initial_phase],
        }
    }

    /// Get current active phase
    pub fn current(&self) -> &GamePhase {
        self.phases.last().expect("Phase stack cannot be empty")
    }

    /// Apply a transition
    pub fn apply(&mut self, transition: PhaseTransition) {
        match transition {
            PhaseTransition::None => {},
            PhaseTransition::Push(phase) => {
                self.phases.push(phase);
            },
            PhaseTransition::Replace(phase) => {
                if let Some(last) = self.phases.last_mut() {
                    *last = phase;
                }
            },
            PhaseTransition::Pop => {
                if self.phases.len() > 1 {
                    self.phases.pop();
                }
            },
            PhaseTransition::Reset(phase) => {
                self.phases.clear();
                self.phases.push(phase);
            },
        }
    }

    /// Check if there's a background phase (for rendering modals over content)
    pub fn background_phase(&self) -> Option<&GamePhase> {
        if self.phases.len() > 1 {
            self.phases.get(self.phases.len() - 2)
        } else {
            None
        }
    }
}
```

---

## 4. Save/Load Implementation

### 4.1 Persistence Module

**File**: `src/state/persistence.rs`

```rust
use crate::state::GameState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SAVE_FILE: &str = "kaiju_sim_save.json";
const SAVE_VERSION: u32 = 1;

/// Save data wrapper with versioning
#[derive(Serialize, Deserialize)]
struct SaveData {
    version: u32,
    game_state: GameState,
    metadata: SaveMetadata,
}

#[derive(Serialize, Deserialize)]
struct SaveMetadata {
    saved_at: String,
    play_time_seconds: u64,
}

/// Save game state to JSON file
pub fn save_game(state: &GameState) -> Result<(), PersistenceError> {
    let save_data = SaveData {
        version: SAVE_VERSION,
        game_state: state.clone(),
        metadata: SaveMetadata {
            saved_at: chrono::Utc::now().to_rfc3339(),
            play_time_seconds: state.game_time.total_ticks / 60, // Assuming 60 ticks/sec
        },
    };

    let json = serde_json::to_string_pretty(&save_data)
        .map_err(|e| PersistenceError::SerializationFailed(e.to_string()))?;

    let path = get_save_path()?;
    std::fs::write(&path, json)
        .map_err(|e| PersistenceError::WriteFailed(e.to_string()))?;

    eprintln!("Game saved to: {}", path.display());
    Ok(())
}

/// Load game state from JSON file
pub fn load_game() -> Result<GameState, PersistenceError> {
    let path = get_save_path()?;

    if !path.exists() {
        return Err(PersistenceError::SaveNotFound);
    }

    let json = std::fs::read_to_string(&path)
        .map_err(|e| PersistenceError::ReadFailed(e.to_string()))?;

    let save_data: SaveData = serde_json::from_str(&json)
        .map_err(|e| PersistenceError::DeserializationFailed(e.to_string()))?;

    // Version migration
    if save_data.version < SAVE_VERSION {
        eprintln!("Migrating save from version {} to {}", save_data.version, SAVE_VERSION);
        // Future: call migration functions
    }

    eprintln!("Game loaded from: {}", path.display());
    Ok(save_data.game_state)
}

/// Check if save file exists
pub fn save_exists() -> bool {
    get_save_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Delete save file
pub fn delete_save() -> Result<(), PersistenceError> {
    let path = get_save_path()?;
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| PersistenceError::DeleteFailed(e.to_string()))?;
    }
    Ok(())
}

/// Get platform-specific save file path
fn get_save_path() -> Result<PathBuf, PersistenceError> {
    // Try to use app data directory, fall back to local
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(data_dir) = dirs::data_dir() {
            let app_dir = data_dir.join("kaiju_sim");
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| PersistenceError::PathError(e.to_string()))?;
            return Ok(app_dir.join(SAVE_FILE));
        }
    }

    // Fallback to current directory
    Ok(PathBuf::from(SAVE_FILE))
}

/// Persistence errors
#[derive(Debug)]
pub enum PersistenceError {
    SaveNotFound,
    SerializationFailed(String),
    DeserializationFailed(String),
    WriteFailed(String),
    ReadFailed(String),
    DeleteFailed(String),
    PathError(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaveNotFound => write!(f, "Save file not found"),
            Self::SerializationFailed(e) => write!(f, "Failed to serialize save: {}", e),
            Self::DeserializationFailed(e) => write!(f, "Failed to deserialize save: {}", e),
            Self::WriteFailed(e) => write!(f, "Failed to write save file: {}", e),
            Self::ReadFailed(e) => write!(f, "Failed to read save file: {}", e),
            Self::DeleteFailed(e) => write!(f, "Failed to delete save file: {}", e),
            Self::PathError(e) => write!(f, "Path error: {}", e),
        }
    }
}

impl std::error::Error for PersistenceError {}
```

### 4.2 Auto-Save System

**File**: `src/state/autosave.rs`

```rust
use crate::state::{GameState, persistence};
use std::time::{Duration, Instant};

/// Auto-save manager
pub struct AutoSaveManager {
    last_save: Instant,
    save_interval: Duration,
    enabled: bool,
}

impl AutoSaveManager {
    pub fn new() -> Self {
        Self {
            last_save: Instant::now(),
            save_interval: Duration::from_secs(60), // Save every 60 seconds
            enabled: true,
        }
    }

    /// Update auto-save, returns true if save was performed
    pub fn update(&mut self, state: &GameState) -> bool {
        if !self.enabled || !state.player.settings.auto_save_enabled {
            return false;
        }

        if self.last_save.elapsed() >= self.save_interval {
            match persistence::save_game(state) {
                Ok(_) => {
                    self.last_save = Instant::now();
                    eprintln!("Auto-save complete");
                    true
                }
                Err(e) => {
                    eprintln!("Auto-save failed: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Force immediate save
    pub fn force_save(&mut self, state: &GameState) -> Result<(), persistence::PersistenceError> {
        persistence::save_game(state)?;
        self.last_save = Instant::now();
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_interval(&mut self, seconds: u64) {
        self.save_interval = Duration::from_secs(seconds);
    }
}
```

---

## 5. Roster Management

### 5.1 Roster Operations

**File**: `src/state/roster.rs`

```rust
use crate::data::{Kaiju, KaijuId};
use crate::state::GameState;

/// Roster management operations
impl GameState {
    /// Add kaiju to roster
    pub fn add_kaiju(&mut self, kaiju: Kaiju) {
        self.roster.push(kaiju);
        self.notify(
            format!("{} joined your roster!", kaiju.name),
            crate::state::NotificationType::Success
        );
    }

    /// Remove kaiju from roster (soft delete, for dead kaiju)
    pub fn remove_kaiju(&mut self, kaiju_id: KaijuId) -> Option<Kaiju> {
        self.roster.iter().position(|k| k.id == kaiju_id)
            .map(|idx| self.roster.remove(idx))
    }

    /// Mark kaiju as dead (permanent)
    pub fn kill_kaiju(&mut self, kaiju_id: KaijuId, tournament_id: Option<uuid::Uuid>) {
        if let Some(kaiju) = self.get_kaiju_mut(kaiju_id) {
            kaiju.alive = false;
            kaiju.death_timestamp = Some(chrono::Utc::now());
            kaiju.death_context = tournament_id.map(|id| format!("Tournament: {}", id));

            self.player.stats.kaiju_deaths += 1;

            self.notify(
                format!("{} has fallen in battle.", kaiju.name),
                crate::state::NotificationType::Error
            );
        }
    }

    /// Get kaiju sorted by specified criteria
    pub fn roster_sorted(&self, sort_by: RosterSortCriteria) -> Vec<&Kaiju> {
        let mut roster: Vec<&Kaiju> = self.roster.iter().collect();

        match sort_by {
            RosterSortCriteria::Name => {
                roster.sort_by(|a, b| a.name.cmp(&b.name));
            }
            RosterSortCriteria::Generation => {
                roster.sort_by(|a, b| a.generation.cmp(&b.generation));
            }
            RosterSortCriteria::Power => {
                roster.sort_by(|a, b| b.combat_power().cmp(&a.combat_power()));
            }
            RosterSortCriteria::Level => {
                roster.sort_by(|a, b| b.experience_level.cmp(&a.experience_level));
            }
        }

        roster
    }

    /// Filter roster
    pub fn roster_filtered(&self, filter: RosterFilter) -> Vec<&Kaiju> {
        self.roster.iter().filter(|k| filter.matches(k)).collect()
    }
}

/// Roster sorting criteria
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RosterSortCriteria {
    Name,
    Generation,
    Power,
    Level,
}

/// Roster filter
#[derive(Clone, Debug)]
pub struct RosterFilter {
    pub alive_only: bool,
    pub generation: Option<u32>,
    pub min_level: Option<u32>,
    pub has_trait: Option<String>,
}

impl RosterFilter {
    pub fn all() -> Self {
        Self {
            alive_only: false,
            generation: None,
            min_level: None,
            has_trait: None,
        }
    }

    pub fn living() -> Self {
        Self {
            alive_only: true,
            generation: None,
            min_level: None,
            has_trait: None,
        }
    }

    pub fn matches(&self, kaiju: &Kaiju) -> bool {
        if self.alive_only && !kaiju.alive {
            return false;
        }

        if let Some(gen) = self.generation {
            if kaiju.generation != gen {
                return false;
            }
        }

        if let Some(min_lvl) = self.min_level {
            if kaiju.experience_level < min_lvl {
                return false;
            }
        }

        if let Some(ref trait_id) = self.has_trait {
            if !kaiju.has_trait(trait_id) {
                return false;
            }
        }

        true
    }
}
```

---

## 6. State Validation

### 6.1 Validation System

**File**: `src/state/validation.rs`

```rust
use crate::state::GameState;

/// Validation result
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub category: ValidationCategory,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationCategory {
    RosterIntegrity,
    PlayerData,
    TournamentState,
    BreedingSession,
    DataConsistency,
}

/// Validate entire game state
pub fn validate_game_state(state: &GameState) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate roster
    if let Err(mut roster_errors) = validate_roster(state) {
        errors.append(&mut roster_errors);
    }

    // Validate player data
    if let Err(mut player_errors) = validate_player_data(state) {
        errors.append(&mut player_errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate roster integrity
fn validate_roster(state: &GameState) -> ValidationResult {
    let mut errors = Vec::new();

    // Check for duplicate IDs
    let mut seen_ids = std::collections::HashSet::new();
    for kaiju in &state.roster {
        if !seen_ids.insert(kaiju.id) {
            errors.push(ValidationError {
                category: ValidationCategory::RosterIntegrity,
                message: format!("Duplicate kaiju ID: {:?}", kaiju.id),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate player data
fn validate_player_data(state: &GameState) -> ValidationResult {
    let mut errors = Vec::new();

    // Check currency isn't negative
    if state.player.gold < 0 {
        errors.push(ValidationError {
            category: ValidationCategory::PlayerData,
            message: format!("Negative gold: {}", state.player.gold),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## 7. Critical Files for Implementation

### Files Most Critical for Phase 5:

1. **`src/state/game_state.rs`** - Core logic
   - **Reason**: Central state authority, all other modules depend on this
   - **Priority**: Implement first
   - **Lines**: ~300

2. **`src/state/phase_transition.rs`** - Navigation system
   - **Reason**: Required for any UI navigation, phase management
   - **Priority**: Implement second
   - **Lines**: ~200

3. **`src/state/persistence.rs`** - Save/load
   - **Reason**: Core requirement for local gameplay, enables testing
   - **Priority**: Implement third
   - **Lines**: ~250

4. **`src/state/validation.rs`** - State integrity
   - **Reason**: Prevents corrupted saves, ensures data consistency
   - **Priority**: Implement fourth
   - **Lines**: ~300

5. **`src/main.rs`** - Integration point
   - **Reason**: Wires everything together, main game loop coordination
   - **Priority**: Modify after state modules complete
   - **Lines**: ~50 modifications

---

## 8. Implementation Order

### Phase 5A: Core State Structures (Day 1-2)

1. **`src/state/mod.rs`** - Module exports
2. **`src/state/game_state.rs`** - GameState struct (~300 lines)
3. **`src/state/player_data.rs`** - PlayerData struct (~150 lines)
4. **`src/state/game_phase.rs`** - Phase enum (~100 lines)
5. **`src/state/phase_transition.rs`** - Phase transitions (~200 lines)

### Phase 5B: Persistence (Day 3-4)

6. **`src/state/persistence.rs`** - Save/load system (~250 lines)
7. **`src/state/autosave.rs`** - Auto-save manager (~100 lines)

### Phase 5C: Specialized State (Day 5)

8. **`src/state/roster.rs`** - Roster management (~150 lines)
9. **`src/state/tournament_state.rs`** - Tournament tracking (~200 lines)

### Phase 5D: Validation & Integration (Day 6-7)

10. **`src/state/validation.rs`** - State validation (~300 lines)
11. **`src/main.rs`** - Integration (~50 lines modified)
12. **`Cargo.toml`** - Dependencies

---

## 9. Success Criteria

Phase 5 is complete when:

- [ ] Can create new game with starter kaiju
- [ ] Can save and load game state
- [ ] Phase transitions work correctly
- [ ] Roster management operates as expected
- [ ] State validation catches errors
- [ ] Auto-save works without performance impact
- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] Save file format documented

---

## Summary

Phase 5 establishes the **foundational state management infrastructure** for Kaiju Breeding Simulator:

**Key Deliverables**:
- GameState structure (single source of truth)
- Phase transition system (explicit navigation)
- Save/load persistence (local JSON files)
- Roster management (kaiju collection)
- Tournament state tracking (active competitions)
- State validation (integrity checks)
- Migration strategy (forward compatibility)

**Architecture Principles**:
- Single source of truth (GameState owns all mutable state)
- Stateless services (engines receive state, return results)
- Explicit transitions (no implicit state changes)
- Data-driven design (JSON configs, not hardcoded values)

This phase is complete when you can:
- Start a new game
- Navigate between phases
- Save and quit
- Resume from save
- All state persists correctly

---

**END OF PHASE 5 IMPLEMENTATION PLAN**
