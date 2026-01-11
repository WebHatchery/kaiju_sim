//! Kaiju Server - Server-custodial NFT infrastructure
//!
//! This library provides the core functionality for:
//! - Instant, zero-cost ownership transfers
//! - Cryptographic proof generation and verification
//! - Audit trail maintenance
//! - API endpoints for transfer operations

pub mod api;
pub mod crypto;
pub mod error;
pub mod transfer_service;
pub mod types;

pub use api::AppState;
pub use crypto::{KeyPurpose, ServerKeyPair, Signer, Verifier};
pub use error::{Result, TransferError};
pub use transfer_service::TransferService;
pub use types::*;
