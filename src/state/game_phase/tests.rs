use super::*;

#[test]
fn test_phase_stack() {
    let mut stack = PhaseStack::new(GamePhase::MainMenu);
    assert_eq!(stack.current(), &GamePhase::MainMenu);

    stack.apply(PhaseTransition::Replace(GamePhase::Laboratory));
    assert_eq!(stack.current(), &GamePhase::Laboratory);

    stack.apply(PhaseTransition::Push(GamePhase::KaijuDetail(
        crate::data::new_kaiju_id(),
    )));
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
