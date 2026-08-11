use super::*;

#[test]
fn test_list_breeding_right() {
    let mut service = MarketplaceService::new();
    let right = BreedingRight {
        id: Uuid::new_v4(),
        kaiju_id: Uuid::new_v4(),
        owner_user_id: Uuid::new_v4(),
        issuer_user_id: Uuid::new_v4(),
        uses_total: 3,
        uses_remaining: 3,
        created_at: Utc::now(),
        expires_at: None,
        purchase_price: None,
        purchase_currency: None,
        refund_eligible: true,
        active: true,
        revoked: false,
        refunded: false,
    };

    let listing = service.list_breeding_right(&right, 500, CurrencyType::Gold, Some(24));

    assert!(listing.is_ok());
    let listing = listing.unwrap();
    assert_eq!(listing.price, 500);
    assert!(listing.is_active());
}

#[test]
fn test_filter_listings() {
    let mut service = MarketplaceService::new();

    // Add some listings
    for i in 0..5 {
        let right = BreedingRight {
            id: Uuid::new_v4(),
            kaiju_id: Uuid::new_v4(),
            owner_user_id: Uuid::new_v4(),
            issuer_user_id: Uuid::new_v4(),
            uses_total: 1,
            uses_remaining: 1,
            created_at: Utc::now(),
            expires_at: None,
            purchase_price: None,
            purchase_currency: None,
            refund_eligible: true,
            active: true,
            revoked: false,
            refunded: false,
        };
        let _ = service.list_breeding_right(&right, (i + 1) * 100, CurrencyType::Gold, None);
    }

    let filters = MarketplaceFilters {
        min_price: Some(200),
        max_price: Some(400),
        ..Default::default()
    };

    let results = service.get_active_listings(&filters);
    assert_eq!(results.len(), 3); // 200, 300, 400
}
