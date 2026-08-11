use super::*;

#[test]
fn test_synergy_detection() {
    let synergies = SynergyDatabase::new(vec![SynergyDefinition {
        id: "S01".to_string(),
        name: "Storm Dragon".to_string(),
        required_traits: vec!["E01".to_string(), "M04".to_string()],
        power: 20,
        effect: "Add 20 damage in Storm environment".to_string(),
    }]);

    // Has both required traits
    let traits_with_synergy = vec!["E01".to_string(), "M04".to_string(), "M02".to_string()];
    let active = synergies.check_synergies(&traits_with_synergy);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, "S01");

    // Missing one required trait
    let traits_without_synergy = vec!["E01".to_string(), "M02".to_string()];
    let active = synergies.check_synergies(&traits_without_synergy);
    assert!(active.is_empty());
}

#[test]
fn test_incompatibility_check() {
    let matrix = IncompatibilityMatrix::new(vec![
        ("M03".to_string(), "M08".to_string()),
        ("M10".to_string(), "MU04".to_string()),
    ]);

    assert!(matrix.are_incompatible("M03", "M08"));
    assert!(matrix.are_incompatible("M08", "M03")); // Order doesn't matter
    assert!(!matrix.are_incompatible("M03", "M10"));
}
