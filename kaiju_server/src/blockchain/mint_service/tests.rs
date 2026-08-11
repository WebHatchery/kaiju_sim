use super::*;

#[tokio::test]
async fn test_mint_request_creation() {
    let service = MintService::new("0x1234".to_string());
    let request =
        service.create_mint_request(Uuid::new_v4(), Uuid::new_v4(), "0xABCD".to_string(), 500);

    assert_eq!(request.status, MintStatus::Pending);
    assert!(request.tx_hash.is_none());
}
