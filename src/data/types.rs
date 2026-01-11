//! Common type aliases and utility types used throughout the game.

use uuid::Uuid;

/// Unique identifier for a kaiju
pub type KaijuId = Uuid;

/// NFT token ID (future blockchain integration)
pub type TokenId = u64;

/// Wallet address (future blockchain integration)
pub type WalletAddress = String;

/// Unix timestamp (seconds since epoch)
pub type Timestamp = i64;

/// Trait identifier
pub type TraitId = String;

/// Visual seed for deterministic image generation
pub type VisualSeed = u64;

/// Generation number (0 = wild/original, 1+ = bred)
pub type Generation = u32;
