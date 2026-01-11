//! Currency system for in-game economy.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Currency types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyType {
    Gold,     // Primary in-game currency
    Gems,     // Premium currency
    Credits,  // Tournament rewards
}

impl CurrencyType {
    pub fn name(&self) -> &'static str {
        match self {
            CurrencyType::Gold => "gold",
            CurrencyType::Gems => "gems",
            CurrencyType::Credits => "credits",
        }
    }
}

/// User currency balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyBalance {
    pub user_id: Uuid,
    pub gold: i64,
    pub gems: i64,
    pub credits: i64,
    pub updated_at: DateTime<Utc>,
}

impl CurrencyBalance {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            gold: 1000, // Starting gold
            gems: 0,
            credits: 0,
            updated_at: Utc::now(),
        }
    }

    pub fn get(&self, currency: CurrencyType) -> i64 {
        match currency {
            CurrencyType::Gold => self.gold,
            CurrencyType::Gems => self.gems,
            CurrencyType::Credits => self.credits,
        }
    }

    pub fn has_sufficient(&self, currency: CurrencyType, amount: i64) -> bool {
        self.get(currency) >= amount
    }
}

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Credit,
    Debit,
    Transfer,
    Refund,
    Reward,
    Purchase,
}

/// Currency transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyTransaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub currency: CurrencyType,
    pub balance_before: i64,
    pub balance_after: i64,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

/// Currency service for managing balances
pub struct CurrencyService {
    // In production, this would have a database pool
    // For now, we'll define the interface
}

impl Default for CurrencyService {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyService {
    pub fn new() -> Self {
        Self {}
    }

    /// Credit currency to user
    pub fn credit(
        &self,
        balance: &mut CurrencyBalance,
        currency: CurrencyType,
        amount: i64,
        reason: &str,
    ) -> CurrencyTransaction {
        let balance_before = balance.get(currency);
        
        match currency {
            CurrencyType::Gold => balance.gold += amount,
            CurrencyType::Gems => balance.gems += amount,
            CurrencyType::Credits => balance.credits += amount,
        }
        balance.updated_at = Utc::now();
        
        CurrencyTransaction {
            id: Uuid::new_v4(),
            user_id: balance.user_id,
            transaction_type: TransactionType::Credit,
            amount,
            currency,
            balance_before,
            balance_after: balance.get(currency),
            reason: reason.to_string(),
            timestamp: Utc::now(),
            signature: self.sign_transaction(balance.user_id, amount, currency),
        }
    }

    /// Debit currency from user (returns None if insufficient)
    pub fn debit(
        &self,
        balance: &mut CurrencyBalance,
        currency: CurrencyType,
        amount: i64,
        reason: &str,
    ) -> Option<CurrencyTransaction> {
        if !balance.has_sufficient(currency, amount) {
            return None;
        }

        let balance_before = balance.get(currency);
        
        match currency {
            CurrencyType::Gold => balance.gold -= amount,
            CurrencyType::Gems => balance.gems -= amount,
            CurrencyType::Credits => balance.credits -= amount,
        }
        balance.updated_at = Utc::now();
        
        Some(CurrencyTransaction {
            id: Uuid::new_v4(),
            user_id: balance.user_id,
            transaction_type: TransactionType::Debit,
            amount,
            currency,
            balance_before,
            balance_after: balance.get(currency),
            reason: reason.to_string(),
            timestamp: Utc::now(),
            signature: self.sign_transaction(balance.user_id, amount, currency),
        })
    }

    /// Transfer between users
    pub fn transfer(
        &self,
        from: &mut CurrencyBalance,
        to: &mut CurrencyBalance,
        currency: CurrencyType,
        amount: i64,
        reason: &str,
    ) -> Option<TransferReceipt> {
        if !from.has_sufficient(currency, amount) {
            return None;
        }

        let debit_tx = self.debit(from, currency, amount, reason)?;
        let credit_tx = self.credit(to, currency, amount, reason);

        Some(TransferReceipt {
            id: Uuid::new_v4(),
            from_user_id: from.user_id,
            to_user_id: to.user_id,
            amount,
            currency,
            debit_transaction: debit_tx,
            credit_transaction: credit_tx,
            timestamp: Utc::now(),
        })
    }

    /// Generate signature for transaction
    fn sign_transaction(&self, user_id: Uuid, amount: i64, currency: CurrencyType) -> String {
        // In production, use proper cryptographic signing
        format!("sig:{}:{}:{}", user_id, amount, currency.name())
    }
}

/// Transfer receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferReceipt {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub amount: i64,
    pub currency: CurrencyType,
    pub debit_transaction: CurrencyTransaction,
    pub credit_transaction: CurrencyTransaction,
    pub timestamp: DateTime<Utc>,
}

/// Currency error
#[derive(Debug)]
pub enum CurrencyError {
    InsufficientFunds,
    InvalidAmount,
    UserNotFound,
    TransactionFailed(String),
}

impl std::fmt::Display for CurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientFunds => write!(f, "Insufficient funds"),
            Self::InvalidAmount => write!(f, "Invalid amount"),
            Self::UserNotFound => write!(f, "User not found"),
            Self::TransactionFailed(e) => write!(f, "Transaction failed: {}", e),
        }
    }
}

impl std::error::Error for CurrencyError {}

#[cfg(test)]
mod tests {
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
}
