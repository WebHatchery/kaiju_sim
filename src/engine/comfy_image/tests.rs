use super::*;
use crate::data::{Kaiju, KaijuStats};

#[test]
fn prompt_includes_breeding_context() {
    let parent_a = Kaiju::new_wild("Ignis".to_string(), KaijuStats::default(), Vec::new());
    let parent_b = Kaiju::new_wild("Volt".to_string(), KaijuStats::default(), Vec::new());
    let offspring = Kaiju::new_wild("Iglt-3".to_string(), KaijuStats::default(), Vec::new());

    let prompt = build_monster_prompt(&offspring, &parent_a, &parent_b);

    assert!(prompt.contains("kaiju monster"));
    assert!(prompt.contains("Ignis"));
    assert!(prompt.contains("Volt"));
}
