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

/// Runtime-compatible random u32. Uses Macroquad's RNG so WebGL builds do not
/// pull in wasm-bindgen random imports.
pub fn random_u32() -> u32 {
    macroquad::rand::rand()
}

pub fn random_u64() -> u64 {
    ((random_u32() as u64) << 32) ^ random_u32() as u64
}

pub fn random_u128() -> u128 {
    ((random_u64() as u128) << 64) ^ random_u64() as u128
}

pub fn random_unit_f32() -> f32 {
    random_u32() as f32 / u32::MAX as f32
}

pub fn random_range_f32(min: f32, max: f32) -> f32 {
    min + (max - min) * random_unit_f32()
}

pub fn random_index(len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (random_u64() as usize) % len
    }
}

pub fn shuffle_slice<T>(items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let j = random_index(i + 1);
        items.swap(i, j);
    }
}

pub fn new_kaiju_id() -> Uuid {
    Uuid::from_u128(random_u128())
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
