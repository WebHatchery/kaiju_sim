use super::*;

#[test]
fn test_environment_multipliers() {
    let storm = Environment::Storm;
    let multipliers = storm.get_trait_multipliers();

    assert_eq!(multipliers.get("Electric"), Some(&1.15));
    assert_eq!(multipliers.get("Fire"), Some(&0.90));
}

#[test]
fn test_neutral_no_modifiers() {
    let neutral = Environment::Neutral;
    let multipliers = neutral.get_trait_multipliers();

    assert!(multipliers.is_empty());
}

#[test]
fn test_volcanic_effects() {
    let volcanic = Environment::Volcanic;
    let effects = volcanic.get_per_turn_effects();

    assert_eq!(effects.len(), 1);
    assert!(
        matches!(effects[0], PerTurnEffect::Damage { percent } if (percent - 0.02).abs() < 0.001)
    );
}
