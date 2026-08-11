use super::*;

#[test]
fn test_trait_condition_always_active() {
    let trait_def = Trait {
        id: "test_trait".to_string(),
        name: "Test".to_string(),
        category: TraitCategory::Element,
        power: 10,
        inheritance: TraitInheritance::Dominant,
        condition: TraitCondition::Simple("Always".to_string()),
        is_hidden: false,
        description: "Test trait".to_string(),
    };

    assert!(trait_def.is_active(100, 100, "any"));
}

#[test]
fn test_trait_condition_low_health() {
    let trait_def = Trait {
        id: "berserker".to_string(),
        name: "Berserker".to_string(),
        category: TraitCategory::Modifier,
        power: 15,
        inheritance: TraitInheritance::Recessive,
        condition: TraitCondition::Complex(TraitConditionVariant::LowHealth(30)),
        is_hidden: false,
        description: "Activates when HP < 30%".to_string(),
    };

    assert!(trait_def.is_active(25, 100, "any"));
    assert!(!trait_def.is_active(50, 100, "any"));
}

#[test]
fn test_trait_condition_environment() {
    let trait_def = Trait {
        id: "electric_breath".to_string(),
        name: "Electric Breath".to_string(),
        category: TraitCategory::Element,
        power: 12,
        inheritance: TraitInheritance::Dominant,
        condition: TraitCondition::Complex(TraitConditionVariant::Environment("storm".to_string())),
        is_hidden: false,
        description: "Boosted in storm environments".to_string(),
    };

    assert!(trait_def.is_active(100, 100, "storm"));
    assert!(!trait_def.is_active(100, 100, "volcanic"));
}
