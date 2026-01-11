//! Blockchain integration module for NFT minting and management.

pub mod deposit_service;
pub mod ipfs_client;
pub mod metadata;
pub mod mint_service;

pub use deposit_service::DepositService;
pub use ipfs_client::IpfsClient;
pub use metadata::NftMetadata;
pub use mint_service::MintService;
