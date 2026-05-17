//! Game phase state machine.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Game phases - explicit state machine for screens
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Initial loading screen
    Loading,
    /// Main menu
    MainMenu,
    /// Laboratory hub
    Laboratory,
    /// Roster view
    Roster,
    /// Breeding interface
    Breeding,
    /// Tournament lobby
    TournamentLobby,
    /// Active battle viewing
    Battle,
    /// Post-battle results
    Results,
    /// Leaderboard and Hall of Fame
    Leaderboard,
    /// Lineage tree viewer
    LineageViewer(Uuid),
    /// Kaiju detail view (modal)
    KaijuDetail(Uuid),
    /// Starter Selection (New Game)
    StarterSelection,
    /// Marketplace
    Marketplace,
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

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Loading => "Loading",
            Self::MainMenu => "Main Menu",
            Self::Laboratory => "Laboratory",
            Self::Roster => "Roster",
            Self::Breeding => "Breeding",
            Self::TournamentLobby => "Tournament Lobby",
            Self::Battle => "Battle",
            Self::Results => "Results",
            Self::Leaderboard => "Leaderboard",
            Self::LineageViewer(_) => "Lineage Tree",
            Self::KaijuDetail(_) => "Kaiju Detail",
            Self::StarterSelection => "Choose Starter",
            Self::Marketplace => "Marketplace",
        }
    }
}

/// Explicit phase transitions
#[derive(Clone, Debug)]
pub enum PhaseTransition {
    /// No transition
    None,
    /// Push new phase to stack (for modals)
    Push(GamePhase),
    /// Replace current phase
    Replace(GamePhase),
    /// Pop to previous phase
    Pop,
    /// Clear stack and set new phase
    Reset(GamePhase),
}

impl PhaseTransition {
    pub fn to_menu() -> Self {
        Self::Reset(GamePhase::MainMenu)
    }

    pub fn to_laboratory() -> Self {
        Self::Replace(GamePhase::Laboratory)
    }

    pub fn to_breeding() -> Self {
        Self::Replace(GamePhase::Breeding)
    }

    pub fn to_tournament_lobby() -> Self {
        Self::Push(GamePhase::TournamentLobby)
    }

    pub fn to_battle() -> Self {
        Self::Replace(GamePhase::Battle)
    }

    pub fn to_results() -> Self {
        Self::Replace(GamePhase::Results)
    }

    pub fn show_kaiju_detail(kaiju_id: Uuid) -> Self {
        Self::Push(GamePhase::KaijuDetail(kaiju_id))
    }

    pub fn close_modal() -> Self {
        Self::Pop
    }
}

/// Phase stack for modal management
#[derive(Clone, Debug)]
pub struct PhaseStack {
    phases: Vec<GamePhase>,
}

impl Default for PhaseStack {
    fn default() -> Self {
        Self::new(GamePhase::Loading)
    }
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
            PhaseTransition::None => {}
            PhaseTransition::Push(phase) => {
                self.phases.push(phase);
            }
            PhaseTransition::Replace(phase) => {
                if let Some(last) = self.phases.last_mut() {
                    *last = phase;
                }
            }
            PhaseTransition::Pop => {
                if self.phases.len() > 1 {
                    self.phases.pop();
                }
            }
            PhaseTransition::Reset(phase) => {
                self.phases.clear();
                self.phases.push(phase);
            }
        }
    }

    /// Check if there's a background phase
    pub fn background_phase(&self) -> Option<&GamePhase> {
        if self.phases.len() > 1 {
            self.phases.get(self.phases.len() - 2)
        } else {
            None
        }
    }

    /// Get stack depth
    pub fn depth(&self) -> usize {
        self.phases.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_stack() {
        let mut stack = PhaseStack::new(GamePhase::MainMenu);
        assert_eq!(stack.current(), &GamePhase::MainMenu);

        stack.apply(PhaseTransition::Replace(GamePhase::Laboratory));
        assert_eq!(stack.current(), &GamePhase::Laboratory);

        stack.apply(PhaseTransition::Push(
            GamePhase::KaijuDetail(Uuid::new_v4()),
        ));
        assert_eq!(stack.depth(), 2);

        stack.apply(PhaseTransition::Pop);
        assert_eq!(stack.current(), &GamePhase::Laboratory);
    }

    #[test]
    fn test_reset() {
        let mut stack = PhaseStack::new(GamePhase::MainMenu);
        stack.apply(PhaseTransition::Push(GamePhase::Laboratory));
        stack.apply(PhaseTransition::Push(GamePhase::Breeding));
        assert_eq!(stack.depth(), 3);

        stack.apply(PhaseTransition::Reset(GamePhase::MainMenu));
        assert_eq!(stack.depth(), 1);
    }
}
