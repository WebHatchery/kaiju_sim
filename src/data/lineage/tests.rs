use super::*;

#[test]
fn test_wild_lineage() {
    let lineage = Lineage::new_wild(1, "Primordial".to_string());
    assert_eq!(lineage.generation, 0);
    assert_eq!(lineage.depth(), 1);
    assert!(lineage.parents.is_none());
}

#[test]
fn test_incest_prevention() {
    let parent_a = Lineage::new_wild(1, "Parent A".to_string());
    let parent_b = Lineage::new_wild(2, "Parent B".to_string());

    // Parents can breed
    assert!(Lineage::can_breed_with(&parent_a, &parent_b));

    // Cannot breed with self
    assert!(!Lineage::can_breed_with(&parent_a, &parent_a));
}
