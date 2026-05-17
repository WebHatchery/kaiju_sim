use crate::breeding_service::KaijuStats;
use rand::Rng;

pub struct BattleResult {
    pub winner_is_a: bool,
    pub log: Vec<String>,
}

pub fn simulate_battle(a_name: &str, a: &KaijuStats, b_name: &str, b: &KaijuStats) -> BattleResult {
    let mut rng = rand::thread_rng();
    let mut log = Vec::new();

    log.push(format!("Battle Start: {} vs {}", a_name, b_name));

    // Simple calc for now
    let score_a =
        (a.hp as f32) + (a.attack as f32 * 2.0) + (a.defense as f32 * 1.5) + (a.speed as f32);
    let score_b =
        (b.hp as f32) + (b.attack as f32 * 2.0) + (b.defense as f32 * 1.5) + (b.speed as f32);

    // Add some variance (luck factor)
    let variance_a = rng.gen_range(0.9..1.1);
    let variance_b = rng.gen_range(0.9..1.1);

    let final_a = score_a * variance_a;
    let final_b = score_b * variance_b;

    log.push(format!(
        "{} Power: {:.0} (Luck: {:.2})",
        a_name, final_a, variance_a
    ));
    log.push(format!(
        "{} Power: {:.0} (Luck: {:.2})",
        b_name, final_b, variance_b
    ));

    let winner_is_a = final_a >= final_b;

    if winner_is_a {
        log.push(format!("Winner: {}", a_name));
    } else {
        log.push(format!("Winner: {}", b_name));
    }

    BattleResult { winner_is_a, log }
}
