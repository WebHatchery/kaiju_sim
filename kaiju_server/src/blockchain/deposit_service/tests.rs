use super::*;

#[test]
fn test_create_deposit_request() {
    let service = DepositService::new("0xCUSTODIAL".to_string(), "0xCONTRACT".to_string());

    let request = service.create_deposit_request(1234, "0xUSER".to_string(), Uuid::new_v4(), 24);

    assert_eq!(request.status, DepositStatus::AwaitingTransfer);
    assert_eq!(request.token_id, 1234);
}

#[test]
fn test_verify_destination() {
    let service = DepositService::new("0xABCD1234".to_string(), "0xCONTRACT".to_string());

    assert!(service.verify_transfer_destination("0xabcd1234"));
    assert!(service.verify_transfer_destination("0xABCD1234"));
    assert!(!service.verify_transfer_destination("0xOTHER"));
}
