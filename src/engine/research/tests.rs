use super::*;

#[test]
fn test_facility_capabilities() {
    let facility = ResearchFacility::new(3);
    let caps = facility.capabilities();

    assert!(caps.contains(&ResearchCapability::DecodeLayer1Basic));
    assert!(caps.contains(&ResearchCapability::DecodeLayer1Advanced));
    assert!(caps.contains(&ResearchCapability::DecodeLayer1Full));
    assert!(!caps.contains(&ResearchCapability::HiddenTraitDetection));
}

#[test]
fn test_confidence_calculation() {
    assert_eq!(calculate_confidence(0), 0.0);
    assert!(calculate_confidence(5) > 0.5);
    assert!(calculate_confidence(20) > 0.9);
    assert!(calculate_confidence(100) <= 0.95);
}

#[test]
fn test_trait_knowledge_progression() {
    let mut knowledge = TraitKnowledge::new("E01".to_string());

    assert_eq!(knowledge.confidence, 0.0);

    knowledge.add_observation(12, Some("Storm boost".to_string()));
    assert!(knowledge.confidence > 0.0);

    knowledge.add_observation(10, None);
    knowledge.add_observation(11, None);
    knowledge.add_observation(12, None);
    knowledge.add_observation(11, None);

    assert!(knowledge.confidence > 0.5);
    assert!(knowledge.power_estimate.1 < 30); // Should have narrowed
}
