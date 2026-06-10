//! Common type aliases and utility types used throughout the game.

use uuid::Uuid;

/// Unique identifier for a kaiju
pub type KaijuId = Uuid;

/// Persistent local legacy ID used for lineage and saved records.
pub type TokenId = u64;

/// Player or keeper identifier used by local saves and future sync.
pub type WalletAddress = String;

/// Unix timestamp (seconds since epoch)
pub type Timestamp = i64;

/// Trait identifier
pub type TraitId = String;

/// Visual seed for deterministic image generation
pub type VisualSeed = u64;

/// Generation number (0 = wild/original, 1+ = bred)
pub type Generation = u32;

pub fn new_kaiju_id() -> Uuid {
    let high = macroquad_toolkit::rng::random_u64() as u128;
    let low = macroquad_toolkit::rng::random_u64() as u128;
    Uuid::from_u128((high << 64) ^ low)
}

pub fn now_timestamp() -> i64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or_default()
    }

    #[cfg(target_arch = "wasm32")]
    {
        macroquad::time::get_time() as i64
    }
}

pub fn now_timestamp_string() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        now_timestamp().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    {
        format!("session+{}s", now_timestamp())
    }
}
