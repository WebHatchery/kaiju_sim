use super::*;

#[test]
fn test_create_right() {
    let service = BreedingRightsService::new();
    let kaiju_id = Uuid::new_v4();
    let issuer_id = Uuid::new_v4();

    let right = service.create_right(kaiju_id, issuer_id, 3, Some(30));

    assert_eq!(right.kaiju_id, kaiju_id);
    assert_eq!(right.uses_total, 3);
    assert_eq!(right.uses_remaining, 3);
    assert!(right.is_valid());
}

#[test]
fn test_consume_right() {
    let service = BreedingRightsService::new();
    let kaiju_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let mut right = service.create_right(kaiju_id, user_id, 2, None);

    assert!(service.consume_right(&mut right, user_id).is_ok());
    assert_eq!(right.uses_remaining, 1);

    assert!(service.consume_right(&mut right, user_id).is_ok());
    assert_eq!(right.uses_remaining, 0);
    assert!(!right.active);
}

#[test]
fn test_consume_wrong_owner() {
    let service = BreedingRightsService::new();
    let mut right = service.create_right(Uuid::new_v4(), Uuid::new_v4(), 1, None);

    let wrong_user = Uuid::new_v4();
    assert!(service.consume_right(&mut right, wrong_user).is_err());
}
