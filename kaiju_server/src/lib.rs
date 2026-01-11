//! Kaiju Server - Server-custodial NFT infrastructure
//!
//! This library provides the core functionality for:
//! - Instant, zero-cost ownership transfers
//! - Cryptographic proof generation and verification
//! - Audit trail maintenance
//! - API endpoints for transfer operations
//! - Economy and breeding rights marketplace (Phase 8)
//! - Blockchain NFT minting and deposits (Phase 9)

pub mod api;
pub mod blockchain;
pub mod breeding_rights_service;
pub mod crypto;
pub mod currency_service;
pub mod error;
pub mod export_service;
pub mod image_gen;
pub mod marketplace_service;
pub mod transfer_service;
pub mod types;

pub use api::AppState;
pub use blockchain::{DepositService, IpfsClient, MintService, NftMetadata};
pub use breeding_rights_service::{BreedingRight, BreedingRightsService};
pub use crypto::{KeyPurpose, ServerKeyPair, Signer, Verifier};
pub use currency_service::{CurrencyBalance, CurrencyService, CurrencyType};
pub use error::{Result, TransferError};
pub use export_service::{ExportService, KaijuExportPackage};
pub use image_gen::{ImageGenerationService, ComfyClient};
pub use marketplace_service::{MarketplaceListing, MarketplaceService};
pub use transfer_service::TransferService;
pub use types::*;
