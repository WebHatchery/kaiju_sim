//! UI actions - type-safe user intents.

use uuid::Uuid;

use crate::data::TrainingFocus;

/// Master UI action enum
#[derive(Debug, Clone)]
pub enum UiAction {
    // Navigation
    GoToMenu,
    GoToLaboratory,
    GoToRoster,
    GoToTraining,
    GoToBreeding,
    GoToTournament,
    GoToLeaderboard,
    GoToSettings,
    Back,

    // Kaiju Management
    SelectKaiju(Uuid),
    DeselectKaiju,
    ViewKaijuDetails(Uuid),
    TrainKaiju {
        kaiju_id: Uuid,
        focus: TrainingFocus,
    },

    // Breeding
    SelectParentA(Uuid),
    SelectParentB(Uuid),
    PreviewOffspring,
    ConfirmBreeding,
    CancelBreeding,

    // Tournament
    EnterTournament {
        tournament_id: Uuid,
        kaiju_id: Uuid,
    },
    WatchTournament(Uuid),
    ViewBracket(Uuid),

    // Battle
    StartBattle(Uuid),
    SkipBattle,
    PauseBattle,
    ResumeBattle,

    // System
    NewGame,
    SelectStarter(String), // Added for starter selection
    ContinueGame,
    SaveGame,
    LoadGame,
    ExitGame,

    // Marketplace
    GoToMarketplace,
    PurchaseKaiju(String), // item_id
}

impl UiAction {
    /// Check if action requires game state
    pub fn requires_game_state(&self) -> bool {
        !matches!(
            self,
            UiAction::GoToMenu | UiAction::NewGame | UiAction::LoadGame | UiAction::ExitGame
        )
    }
}
