use super::*;
use crate::data::{new_kaiju_id, now_timestamp};

fn create_test_legacy(name: &str, wins: u32) -> LegacyRecord {
    LegacyRecord {
        kaiju_id: new_kaiju_id(),
        name: name.to_string(),
        generation: 5,
        final_stats: KaijuStats::new(300, 60, 40, 30, 100),
        visible_traits: vec![],
        hidden_traits: vec![],
        lifetime_record: MatchRecord {
            total_matches: wins + 2,
            wins,
            losses: 2,
            tournament_matches: wins,
            lethal_matches_survived: 1,
        },
        tournament_victories: vec![],
        offspring_count: 3,
        notable_descendants: vec![],
        death_timestamp: now_timestamp(),
        death_context: "Defeated in Champion's Crucible".to_string(),
        achievements: vec![],
    }
}

#[test]
fn test_prestige_calculation() {
    let legacy = create_test_legacy("Champion", 20);
    let score = calculate_prestige_score(&legacy);

    assert!(score > 0);
    // 20 wins * 10 = 200, plus win rate, plus other bonuses
    assert!(score > 200);
}

#[test]
fn test_hall_of_fame_search() {
    let mut hof = HallOfFame::new();

    let entry1 = create_hall_of_fame_entry(create_test_legacy("Thunder Dragon", 15));
    let entry2 = create_hall_of_fame_entry(create_test_legacy("Shadow Wolf", 10));
    let entry3 = create_hall_of_fame_entry(create_test_legacy("Fire Dragon", 20));

    hof.add_entry(entry1);
    hof.add_entry(entry2);
    hof.add_entry(entry3);

    // Search for dragons
    let dragons = hof.search_by_name("dragon");
    assert_eq!(dragons.len(), 2);
}

#[test]
fn test_hall_of_fame_sorting() {
    let mut hof = HallOfFame::new();

    let low = create_hall_of_fame_entry(create_test_legacy("Weak", 5));
    let high = create_hall_of_fame_entry(create_test_legacy("Strong", 50));

    hof.add_entry(low);
    hof.add_entry(high);

    // Higher prestige should be first
    assert!(hof.entries[0].prestige_score > hof.entries[1].prestige_score);
    assert_eq!(hof.entries[0].legacy.name, "Strong");
}
