//! Breeding rights system for kaiju economy.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::currency_service::{CurrencyBalance, CurrencyService, CurrencyType};

/// A breeding right token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingRight {
    pub id: Uuid,
    pub kaiju_id: Uuid,
    pub owner_user_id: Uuid,
    pub issuer_user_id: Uuid,
    pub uses_total: i32,
    pub uses_remaining: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub purchase_price: Option<i64>,
    pub purchase_currency: Option<CurrencyType>,
    pub refund_eligible: bool,
    pub active: bool,
    pub revoked: bool,
    pub refunded: bool,
}

impl BreedingRight {
    /// Check if right is valid for use
    pub fn is_valid(&self) -> bool {
        self.active
            && !self.revoked
            && !self.refunded
            && self.uses_remaining > 0
            && self.expires_at.map(|e| e > Utc::now()).unwrap_or(true)
    }

    /// Check if right is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|e| e <= Utc::now()).unwrap_or(false)
    }
}

/// Offer to create a breeding right
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingRightOffer {
    pub kaiju_id: Uuid,
    pub price: i64,
    pub currency: CurrencyType,
    pub uses: i32,
    pub duration_days: Option<i32>,
}

/// Breeding rights service
pub struct BreedingRightsService {
    currency_service: CurrencyService,
}

impl Default for BreedingRightsService {
    fn default() -> Self {
        Self::new()
    }
}

impl BreedingRightsService {
    pub fn new() -> Self {
        Self {
            currency_service: CurrencyService::new(),
        }
    }

    /// Create a breeding right (issuer keeps it initially)
    pub fn create_right(
        &self,
        kaiju_id: Uuid,
        issuer_user_id: Uuid,
        uses: i32,
        duration_days: Option<i32>,
    ) -> BreedingRight {
        let expires_at = duration_days.map(|days| Utc::now() + Duration::days(days as i64));

        BreedingRight {
            id: Uuid::new_v4(),
            kaiju_id,
            owner_user_id: issuer_user_id, // Issuer owns initially
            issuer_user_id,
            uses_total: uses,
            uses_remaining: uses,
            created_at: Utc::now(),
            expires_at,
            purchase_price: None,
            purchase_currency: None,
            refund_eligible: true,
            active: true,
            revoked: false,
            refunded: false,
        }
    }

    /// Purchase a breeding right from marketplace
    pub fn purchase_right(
        &self,
        right: &mut BreedingRight,
        buyer_balance: &mut CurrencyBalance,
        seller_balance: &mut CurrencyBalance,
        price: i64,
        currency: CurrencyType,
    ) -> Result<PurchaseReceipt, BreedingRightError> {
        // Validate right is purchasable
        if !right.active {
            return Err(BreedingRightError::RightNotActive);
        }
        if right.owner_user_id == buyer_balance.user_id {
            return Err(BreedingRightError::AlreadyOwned);
        }

        // Transfer currency
        let transfer = self
            .currency_service
            .transfer(
                buyer_balance,
                seller_balance,
                currency,
                price,
                "Breeding right purchase",
            )
            .ok_or(BreedingRightError::InsufficientFunds)?;

        // Transfer ownership
        let previous_owner = right.owner_user_id;
        right.owner_user_id = buyer_balance.user_id;
        right.purchase_price = Some(price);
        right.purchase_currency = Some(currency);

        Ok(PurchaseReceipt {
            id: Uuid::new_v4(),
            right_id: right.id,
            buyer_user_id: buyer_balance.user_id,
            seller_user_id: previous_owner,
            price,
            currency,
            timestamp: Utc::now(),
        })
    }

    /// Consume one use of a breeding right
    pub fn consume_right(
        &self,
        right: &mut BreedingRight,
        user_id: Uuid,
    ) -> Result<(), BreedingRightError> {
        // Validate ownership
        if right.owner_user_id != user_id {
            return Err(BreedingRightError::NotOwner);
        }

        // Validate usable
        if !right.is_valid() {
            return Err(BreedingRightError::RightNotValid);
        }

        // Consume
        right.uses_remaining -= 1;
        if right.uses_remaining == 0 {
            right.active = false;
        }

        Ok(())
    }

    /// Revoke a breeding right (issuer only, before any use)
    pub fn revoke_right(
        &self,
        right: &mut BreedingRight,
        issuer_id: Uuid,
    ) -> Result<(), BreedingRightError> {
        if right.issuer_user_id != issuer_id {
            return Err(BreedingRightError::NotIssuer);
        }
        if right.uses_remaining < right.uses_total {
            return Err(BreedingRightError::AlreadyUsed);
        }

        right.revoked = true;
        right.active = false;
        Ok(())
    }

    /// Process refund for kaiju death
    pub fn refund_for_death(
        &self,
        right: &mut BreedingRight,
        owner_balance: &mut CurrencyBalance,
    ) -> Option<RefundReceipt> {
        if !right.refund_eligible || right.refunded {
            return None;
        }

        let (price, currency) = match (right.purchase_price, right.purchase_currency) {
            (Some(p), Some(c)) => (p, c),
            _ => return None,
        };

        // Calculate partial refund based on remaining uses
        let refund_ratio = right.uses_remaining as f64 / right.uses_total as f64;
        let refund_amount = (price as f64 * refund_ratio) as i64;

        if refund_amount <= 0 {
            return None;
        }

        let tx = self.currency_service.credit(
            owner_balance,
            currency,
            refund_amount,
            "Breeding right refund: kaiju death",
        );

        right.refunded = true;
        right.active = false;

        Some(RefundReceipt {
            id: Uuid::new_v4(),
            right_id: right.id,
            user_id: owner_balance.user_id,
            refund_amount,
            currency,
            original_price: price,
            uses_refunded: right.uses_remaining,
            timestamp: Utc::now(),
        })
    }
}

/// Purchase receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseReceipt {
    pub id: Uuid,
    pub right_id: Uuid,
    pub buyer_user_id: Uuid,
    pub seller_user_id: Uuid,
    pub price: i64,
    pub currency: CurrencyType,
    pub timestamp: DateTime<Utc>,
}

/// Refund receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundReceipt {
    pub id: Uuid,
    pub right_id: Uuid,
    pub user_id: Uuid,
    pub refund_amount: i64,
    pub currency: CurrencyType,
    pub original_price: i64,
    pub uses_refunded: i32,
    pub timestamp: DateTime<Utc>,
}

/// Breeding right error
#[derive(Debug)]
pub enum BreedingRightError {
    RightNotFound,
    RightNotActive,
    RightNotValid,
    NotOwner,
    NotIssuer,
    AlreadyOwned,
    AlreadyUsed,
    InsufficientFunds,
    KaijuDead,
}

impl std::fmt::Display for BreedingRightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RightNotFound => write!(f, "Breeding right not found"),
            Self::RightNotActive => write!(f, "Breeding right not active"),
            Self::RightNotValid => write!(f, "Breeding right not valid"),
            Self::NotOwner => write!(f, "Not owner of breeding right"),
            Self::NotIssuer => write!(f, "Not issuer of breeding right"),
            Self::AlreadyOwned => write!(f, "Already own this breeding right"),
            Self::AlreadyUsed => write!(f, "Breeding right already used"),
            Self::InsufficientFunds => write!(f, "Insufficient funds"),
            Self::KaijuDead => write!(f, "Kaiju is dead"),
        }
    }
}

impl std::error::Error for BreedingRightError {}

#[cfg(test)]
mod tests {
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
}
