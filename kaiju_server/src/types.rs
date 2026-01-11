//! Core data types for the Kaiju transfer system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Transfer request from API/UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub kaiju_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub transfer_type: TransferType,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Types of ownership transfers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
    Mint,
    Trade,
    Gift,
    BreedingPayment,
    BlockchainMint,
    BlockchainDeposit,
    AdminTransfer,
}

impl std::fmt::Display for TransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mint => write!(f, "mint"),
            Self::Trade => write!(f, "trade"),
            Self::Gift => write!(f, "gift"),
            Self::BreedingPayment => write!(f, "breeding_payment"),
            Self::BlockchainMint => write!(f, "blockchain_mint"),
            Self::BlockchainDeposit => write!(f, "blockchain_deposit"),
            Self::AdminTransfer => write!(f, "admin_transfer"),
        }
    }
}

/// Custody state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CustodyState {
    Server,
    Blockchain,
}

impl std::fmt::Display for CustodyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Server => write!(f, "server"),
            Self::Blockchain => write!(f, "blockchain"),
        }
    }
}

/// Kaiju entity (simplified for transfers)
#[derive(Debug, Clone, FromRow)]
pub struct Kaiju {
    pub id: String,
    pub name: String,
    pub owner_user_id: String,
    pub custody_state: String, // "server" or "blockchain"
    pub alive: bool,
    pub blockchain_token_id: Option<i64>,
    pub state_hash: String,
    pub state_version: i32,
}

/// Transfer result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub kaiju_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub transfer_type: TransferType,
    pub timestamp: DateTime<Utc>,
    pub nonce: i64,
    pub signature: String,
    pub history_id: i64,
}

/// Ownership proof for external verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipProof {
    pub kaiju_id: Uuid,
    pub owner_user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub state_hash: String,
    pub server_signature: String,
    pub public_key: String,
}

/// Transfer history record from database
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TransferHistoryRecord {
    pub id: i64,
    pub kaiju_id: String,
    pub from_user_id: Option<String>,
    pub to_user_id: String,
    pub transfer_type: String,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub server_signature: String,
    pub nonce: i64,
}
