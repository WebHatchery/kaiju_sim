//! Deposit service for receiving NFTs back to server custody.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Deposit request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepositStatus {
    AwaitingTransfer,
    Detected,
    Confirmed,
    Completed,
    Timeout,
    Failed,
}

/// Deposit request record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRequest {
    pub id: Uuid,
    pub token_id: u64,
    pub user_wallet: String,
    pub user_id: Uuid,
    pub status: DepositStatus,
    pub tx_hash: Option<String>,
    pub expected_by: DateTime<Utc>,
    pub detected_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Deposit service for handling NFT returns to server
pub struct DepositService {
    custodial_wallet: String,
    contract_address: String,
}

impl DepositService {
    pub fn new(custodial_wallet: String, contract_address: String) -> Self {
        Self {
            custodial_wallet,
            contract_address,
        }
    }

    /// Get the custodial wallet address (where users send NFTs)
    pub fn get_custodial_wallet(&self) -> &str {
        &self.custodial_wallet
    }

    /// Create a deposit request
    pub fn create_deposit_request(
        &self,
        token_id: u64,
        user_wallet: String,
        user_id: Uuid,
        timeout_hours: i64,
    ) -> DepositRequest {
        DepositRequest {
            id: Uuid::new_v4(),
            token_id,
            user_wallet,
            user_id,
            status: DepositStatus::AwaitingTransfer,
            tx_hash: None,
            expected_by: Utc::now() + chrono::Duration::hours(timeout_hours),
            detected_at: None,
            completed_at: None,
            created_at: Utc::now(),
        }
    }

    /// Process a detected transfer event
    pub fn process_transfer_detected(
        &self,
        request: &mut DepositRequest,
        tx_hash: String,
    ) {
        request.status = DepositStatus::Detected;
        request.tx_hash = Some(tx_hash);
        request.detected_at = Some(Utc::now());
    }

    /// Confirm a deposit after blockchain confirmations
    pub fn confirm_deposit(&self, request: &mut DepositRequest) {
        request.status = DepositStatus::Confirmed;
    }

    /// Complete deposit (update database ownership)
    pub fn complete_deposit(
        &self,
        request: &mut DepositRequest,
        kaiju_id: Uuid,
    ) -> DepositResult {
        request.status = DepositStatus::Completed;
        request.completed_at = Some(Utc::now());

        DepositResult {
            deposit_id: request.id,
            kaiju_id,
            token_id: request.token_id,
            user_id: request.user_id,
            completed_at: Utc::now(),
        }
    }

    /// Mark request as timed out
    pub fn mark_timeout(&self, request: &mut DepositRequest) {
        request.status = DepositStatus::Timeout;
    }

    /// Check if request has expired
    pub fn is_expired(&self, request: &DepositRequest) -> bool {
        request.status == DepositStatus::AwaitingTransfer && Utc::now() > request.expected_by
    }

    /// Verify a transfer is to the custodial wallet
    pub fn verify_transfer_destination(&self, to_address: &str) -> bool {
        to_address.eq_ignore_ascii_case(&self.custodial_wallet)
    }

    /// Start listening for Transfer events (placeholder)
    pub async fn start_listening(&self) -> Result<(), DepositError> {
        // In production, use ethers-rs to subscribe to Transfer events:
        // let filter = contract.transfer_filter().to(custodial_wallet);
        // let mut stream = filter.subscribe().await?;
        // while let Some(event) = stream.next().await { ... }
        
        eprintln!(
            "Listening for deposits to {} on {}",
            self.custodial_wallet, self.contract_address
        );
        Ok(())
    }
}

/// Deposit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositResult {
    pub deposit_id: Uuid,
    pub kaiju_id: Uuid,
    pub token_id: u64,
    pub user_id: Uuid,
    pub completed_at: DateTime<Utc>,
}

/// Deposit error
#[derive(Debug)]
pub enum DepositError {
    RequestNotFound,
    InvalidTransfer,
    AlreadyProcessed,
    TokenNotFound,
    VerificationFailed,
    BlockchainError(String),
}

impl std::fmt::Display for DepositError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestNotFound => write!(f, "Deposit request not found"),
            Self::InvalidTransfer => write!(f, "Invalid transfer"),
            Self::AlreadyProcessed => write!(f, "Deposit already processed"),
            Self::TokenNotFound => write!(f, "Token not found"),
            Self::VerificationFailed => write!(f, "Verification failed"),
            Self::BlockchainError(e) => write!(f, "Blockchain error: {}", e),
        }
    }
}

impl std::error::Error for DepositError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_deposit_request() {
        let service = DepositService::new(
            "0xCUSTODIAL".to_string(),
            "0xCONTRACT".to_string(),
        );

        let request = service.create_deposit_request(
            1234,
            "0xUSER".to_string(),
            Uuid::new_v4(),
            24,
        );

        assert_eq!(request.status, DepositStatus::AwaitingTransfer);
        assert_eq!(request.token_id, 1234);
    }

    #[test]
    fn test_verify_destination() {
        let service = DepositService::new(
            "0xABCD1234".to_string(),
            "0xCONTRACT".to_string(),
        );

        assert!(service.verify_transfer_destination("0xabcd1234"));
        assert!(service.verify_transfer_destination("0xABCD1234"));
        assert!(!service.verify_transfer_destination("0xOTHER"));
    }
}
