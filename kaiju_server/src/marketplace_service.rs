//! Marketplace service for trading breeding rights.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::breeding_rights_service::{BreedingRight, BreedingRightsService};
use crate::currency_service::{CurrencyBalance, CurrencyType};

/// Marketplace listing type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListingType {
    BreedingRight,
    Kaiju, // Future: direct kaiju sales
}

/// Marketplace listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceListing {
    pub id: Uuid,
    pub listing_type: ListingType,
    pub asset_id: Uuid, // right_id or kaiju_id
    pub seller_user_id: Uuid,
    pub price: i64,
    pub currency: CurrencyType,
    pub listed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub sold: bool,
    pub cancelled: bool,
}

impl MarketplaceListing {
    /// Check if listing is active
    pub fn is_active(&self) -> bool {
        !self.sold && !self.cancelled && self.expires_at.map(|e| e > Utc::now()).unwrap_or(true)
    }
}

/// Marketplace filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketplaceFilters {
    pub listing_type: Option<ListingType>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
    pub currency: Option<CurrencyType>,
    pub seller_id: Option<Uuid>,
}

/// Marketplace service
pub struct MarketplaceService {
    breeding_rights: BreedingRightsService,
    listings: Vec<MarketplaceListing>, // In production, this would be a database
}

impl Default for MarketplaceService {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketplaceService {
    pub fn new() -> Self {
        Self {
            breeding_rights: BreedingRightsService::new(),
            listings: Vec::new(),
        }
    }

    /// List a breeding right for sale
    pub fn list_breeding_right(
        &mut self,
        right: &BreedingRight,
        price: i64,
        currency: CurrencyType,
        duration_hours: Option<i32>,
    ) -> Result<MarketplaceListing, MarketplaceError> {
        // Validate right is listable
        if !right.is_valid() {
            return Err(MarketplaceError::AssetNotValid);
        }

        // Check not already listed
        if self
            .listings
            .iter()
            .any(|l| l.asset_id == right.id && l.is_active())
        {
            return Err(MarketplaceError::AlreadyListed);
        }

        let expires_at = duration_hours.map(|h| Utc::now() + Duration::hours(h as i64));

        let listing = MarketplaceListing {
            id: Uuid::new_v4(),
            listing_type: ListingType::BreedingRight,
            asset_id: right.id,
            seller_user_id: right.owner_user_id,
            price,
            currency,
            listed_at: Utc::now(),
            expires_at,
            sold: false,
            cancelled: false,
        };

        self.listings.push(listing.clone());
        Ok(listing)
    }

    /// Purchase a listing
    pub fn purchase_listing(
        &mut self,
        listing_id: Uuid,
        buyer_id: Uuid,
        right: &mut BreedingRight,
        buyer_balance: &mut CurrencyBalance,
        seller_balance: &mut CurrencyBalance,
    ) -> Result<MarketplacePurchase, MarketplaceError> {
        // Find listing
        let listing = self
            .listings
            .iter_mut()
            .find(|l| l.id == listing_id)
            .ok_or(MarketplaceError::ListingNotFound)?;

        // Validate
        if !listing.is_active() {
            return Err(MarketplaceError::ListingNotActive);
        }
        if listing.seller_user_id == buyer_id {
            return Err(MarketplaceError::CannotBuyOwn);
        }

        // Execute purchase
        let receipt = self.breeding_rights.purchase_right(
            right,
            buyer_balance,
            seller_balance,
            listing.price,
            listing.currency,
        )?;

        // Mark listing as sold
        listing.sold = true;

        Ok(MarketplacePurchase {
            listing_id,
            purchase_receipt: receipt,
        })
    }

    /// Cancel a listing
    pub fn cancel_listing(
        &mut self,
        listing_id: Uuid,
        seller_id: Uuid,
    ) -> Result<(), MarketplaceError> {
        let listing = self
            .listings
            .iter_mut()
            .find(|l| l.id == listing_id)
            .ok_or(MarketplaceError::ListingNotFound)?;

        if listing.seller_user_id != seller_id {
            return Err(MarketplaceError::NotSeller);
        }
        if !listing.is_active() {
            return Err(MarketplaceError::ListingNotActive);
        }

        listing.cancelled = true;
        Ok(())
    }

    /// Get active listings with filters
    pub fn get_active_listings(&self, filters: &MarketplaceFilters) -> Vec<&MarketplaceListing> {
        self.listings
            .iter()
            .filter(|l| l.is_active())
            .filter(|l| {
                filters
                    .listing_type
                    .map(|t| l.listing_type == t)
                    .unwrap_or(true)
            })
            .filter(|l| filters.min_price.map(|p| l.price >= p).unwrap_or(true))
            .filter(|l| filters.max_price.map(|p| l.price <= p).unwrap_or(true))
            .filter(|l| filters.currency.map(|c| l.currency == c).unwrap_or(true))
            .filter(|l| {
                filters
                    .seller_id
                    .map(|s| l.seller_user_id == s)
                    .unwrap_or(true)
            })
            .collect()
    }

    /// Get listing by ID
    pub fn get_listing(&self, listing_id: Uuid) -> Option<&MarketplaceListing> {
        self.listings.iter().find(|l| l.id == listing_id)
    }
}

/// Marketplace purchase result
#[derive(Debug, Clone)]
pub struct MarketplacePurchase {
    pub listing_id: Uuid,
    pub purchase_receipt: crate::breeding_rights_service::PurchaseReceipt,
}

/// Marketplace error
#[derive(Debug)]
pub enum MarketplaceError {
    ListingNotFound,
    ListingNotActive,
    AssetNotValid,
    AlreadyListed,
    NotSeller,
    CannotBuyOwn,
    BreedingRightError(crate::breeding_rights_service::BreedingRightError),
}

impl From<crate::breeding_rights_service::BreedingRightError> for MarketplaceError {
    fn from(e: crate::breeding_rights_service::BreedingRightError) -> Self {
        MarketplaceError::BreedingRightError(e)
    }
}

impl std::fmt::Display for MarketplaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListingNotFound => write!(f, "Listing not found"),
            Self::ListingNotActive => write!(f, "Listing not active"),
            Self::AssetNotValid => write!(f, "Asset not valid for listing"),
            Self::AlreadyListed => write!(f, "Asset already listed"),
            Self::NotSeller => write!(f, "Not the seller"),
            Self::CannotBuyOwn => write!(f, "Cannot buy own listing"),
            Self::BreedingRightError(e) => write!(f, "Breeding right error: {}", e),
        }
    }
}

impl std::error::Error for MarketplaceError {}

#[cfg(test)]
mod tests {
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
}
