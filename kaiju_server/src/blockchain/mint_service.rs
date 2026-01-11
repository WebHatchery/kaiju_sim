//! Minting service for blockchain NFT creation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ipfs_client::{IpfsClient, IpfsError};
use super::metadata::{KaijuStats, NftMetadata};

/// Mint request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintStatus {
    Pending,
    UploadingMetadata,
    Minting,
    Confirming,
    Completed,
    Failed,
}

/// Mint request record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub id: Uuid,
    pub kaiju_id: Uuid,
    pub user_id: Uuid,
    pub destination_wallet: String,
    pub status: MintStatus,
    pub ipfs_uri: Option<String>,
    pub tx_hash: Option<String>,
    pub token_id: Option<u64>,
    pub gas_cost_wei: Option<u64>,
    pub service_fee_cents: i32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Minting service
pub struct MintService {
    ipfs_client: IpfsClient,
    contract_address: String,
    next_token_id: u64, // In production, query from blockchain
}

impl MintService {
    pub fn new(contract_address: String) -> Self {
        Self {
            ipfs_client: IpfsClient::default(),
            contract_address,
            next_token_id: 1,
        }
    }

    /// Create a new mint request
    pub fn create_mint_request(
        &self,
        kaiju_id: Uuid,
        user_id: Uuid,
        destination_wallet: String,
        service_fee_cents: i32,
    ) -> MintRequest {
        MintRequest {
            id: Uuid::new_v4(),
            kaiju_id,
            user_id,
            destination_wallet,
            status: MintStatus::Pending,
            ipfs_uri: None,
            tx_hash: None,
            token_id: None,
            gas_cost_wei: None,
            service_fee_cents,
            error_message: None,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Process a mint request (full flow)
    pub async fn process_mint(
        &mut self,
        request: &mut MintRequest,
        name: String,
        generation: u32,
        genome_hash: String,
        visual_seed: u64,
        stats: KaijuStats,
        traits: Vec<String>,
        image_uri: String,
        parent_tokens: Option<(u64, u64)>,
        created_at: i64,
    ) -> Result<MintResult, MintError> {
        // Step 1: Upload metadata to IPFS
        request.status = MintStatus::UploadingMetadata;
        
        let token_id = self.next_token_id;
        let metadata = NftMetadata::new(
            token_id,
            name,
            generation,
            genome_hash,
            visual_seed,
            &stats,
            &traits,
            image_uri,
            parent_tokens,
            created_at,
            true, // alive
        );

        let metadata_json = metadata.to_json()
            .map_err(|e| MintError::MetadataError(e.to_string()))?;

        let ipfs_response = self.ipfs_client.upload_json(&metadata_json).await
            .map_err(|e| MintError::IpfsError(e))?;

        request.ipfs_uri = Some(ipfs_response.uri.clone());

        // Step 2: Execute blockchain mint
        request.status = MintStatus::Minting;
        
        // In production, this would call the smart contract
        let tx_hash = self.simulate_blockchain_mint(
            token_id,
            &request.destination_wallet,
            &ipfs_response.uri,
        ).await?;

        request.tx_hash = Some(tx_hash.clone());

        // Step 3: Wait for confirmation
        request.status = MintStatus::Confirming;
        
        // In production, wait for block confirmations
        self.wait_for_confirmation(&tx_hash).await?;

        // Step 4: Complete
        request.status = MintStatus::Completed;
        request.token_id = Some(token_id);
        request.completed_at = Some(Utc::now());
        
        self.next_token_id += 1;

        Ok(MintResult {
            token_id,
            tx_hash,
            ipfs_uri: ipfs_response.uri,
            contract_address: self.contract_address.clone(),
        })
    }

    /// Simulate blockchain mint (placeholder for actual implementation)
    async fn simulate_blockchain_mint(
        &self,
        token_id: u64,
        to_wallet: &str,
        metadata_uri: &str,
    ) -> Result<String, MintError> {
        // In production, use ethers-rs:
        // let contract = KaijuNFT::new(address, client);
        // let tx = contract.mint(to_wallet, token_id, metadata_uri).send().await?;
        
        eprintln!(
            "Minting token {} to {} with metadata {}",
            token_id, to_wallet, metadata_uri
        );

        // Return mock transaction hash
        Ok(format!("0x{:064x}", token_id))
    }

    /// Wait for blockchain confirmation
    async fn wait_for_confirmation(&self, tx_hash: &str) -> Result<(), MintError> {
        // In production, poll for transaction receipt
        eprintln!("Waiting for confirmation: {}", tx_hash);
        Ok(())
    }

    /// Get mint price in cents
    pub fn get_mint_price_cents(&self) -> i32 {
        // Base service fee + estimated gas
        500 // $5.00
    }
}

/// Mint result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintResult {
    pub token_id: u64,
    pub tx_hash: String,
    pub ipfs_uri: String,
    pub contract_address: String,
}

/// Mint error
#[derive(Debug)]
pub enum MintError {
    KaijuNotFound,
    KaijuNotOwned,
    KaijuAlreadyMinted,
    InsufficientFunds,
    MetadataError(String),
    IpfsError(IpfsError),
    BlockchainError(String),
    TransactionFailed(String),
}

impl std::fmt::Display for MintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KaijuNotFound => write!(f, "Kaiju not found"),
            Self::KaijuNotOwned => write!(f, "Kaiju not owned by user"),
            Self::KaijuAlreadyMinted => write!(f, "Kaiju already minted"),
            Self::InsufficientFunds => write!(f, "Insufficient funds"),
            Self::MetadataError(e) => write!(f, "Metadata error: {}", e),
            Self::IpfsError(e) => write!(f, "IPFS error: {}", e),
            Self::BlockchainError(e) => write!(f, "Blockchain error: {}", e),
            Self::TransactionFailed(e) => write!(f, "Transaction failed: {}", e),
        }
    }
}

impl std::error::Error for MintError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mint_request_creation() {
        let service = MintService::new("0x1234".to_string());
        let request = service.create_mint_request(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "0xABCD".to_string(),
            500,
        );

        assert_eq!(request.status, MintStatus::Pending);
        assert!(request.tx_hash.is_none());
    }
}
