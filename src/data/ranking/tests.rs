use super::*;

#[test]
fn test_default_rating() {
    let rating = EloRating::new();
    assert_eq!(rating.rating, STARTING_ELO);
    assert_eq!(rating.matches_played, 0);
}

#[test]
fn test_expected_score() {
    let higher = EloRating {
        rating: 1600,
        ..Default::default()
    };
    let lower = EloRating {
        rating: 1400,
        ..Default::default()
    };

    // Higher rated should expect to win more
    assert!(higher.expected_score(1400) > 0.5);
    assert!(lower.expected_score(1600) < 0.5);

    // Equal ratings = 50%
    let equal = EloRating::new();
    let expected = equal.expected_score(1500);
    assert!((expected - 0.5).abs() < 0.01);
}

#[test]
fn test_rating_update() {
    let mut winner = EloRating::new();
    let mut loser = EloRating::new();

    update_elo_ratings(&mut winner, &mut loser);

    // Winner should gain, loser should lose
    assert!(winner.rating > STARTING_ELO);
    assert!(loser.rating < STARTING_ELO);

    assert_eq!(winner.wins, 1);
    assert_eq!(loser.losses, 1);
}

#[test]
fn test_upset_detection() {
    // Underdog wins (rating 1300 beats 1700)
    let (is_upset, prob) = calculate_upset(1300, 1700);
    assert!(is_upset);
    assert!(prob > 0.5);

    // Favorite wins (rating 1700 beats 1300)
    let (is_upset, prob) = calculate_upset(1700, 1300);
    assert!(!is_upset);
    assert!(prob < 0.5);
}

#[test]
fn test_rating_floor() {
    let mut low = EloRating {
        rating: 10,
        ..Default::default()
    };
    low.record_loss(2000);
    assert!(low.rating >= 0);
}
