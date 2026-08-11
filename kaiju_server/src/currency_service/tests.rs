use super::*;

#[test]
fn test_credit() {
    let service = CurrencyService::new();
    let mut balance = CurrencyBalance::new(Uuid::new_v4());

    let initial = balance.gold;
    service.credit(&mut balance, CurrencyType::Gold, 500, "Test credit");

    assert_eq!(balance.gold, initial + 500);
}

#[test]
fn test_debit_success() {
    let service = CurrencyService::new();
    let mut balance = CurrencyBalance::new(Uuid::new_v4());
    balance.gold = 1000;

    let tx = service.debit(&mut balance, CurrencyType::Gold, 500, "Test debit");

    assert!(tx.is_some());
    assert_eq!(balance.gold, 500);
}

#[test]
fn test_debit_insufficient() {
    let service = CurrencyService::new();
    let mut balance = CurrencyBalance::new(Uuid::new_v4());
    balance.gold = 100;

    let tx = service.debit(&mut balance, CurrencyType::Gold, 500, "Test debit");

    assert!(tx.is_none());
    assert_eq!(balance.gold, 100);
}

#[test]
fn test_transfer() {
    let service = CurrencyService::new();
    let mut from = CurrencyBalance::new(Uuid::new_v4());
    let mut to = CurrencyBalance::new(Uuid::new_v4());
    from.gold = 1000;
    to.gold = 0;

    let receipt = service.transfer(&mut from, &mut to, CurrencyType::Gold, 500, "Test transfer");

    assert!(receipt.is_some());
    assert_eq!(from.gold, 500);
    assert_eq!(to.gold, 500);
}
