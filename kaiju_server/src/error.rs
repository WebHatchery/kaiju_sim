//! Error types for the Kaiju transfer system.

use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("Kaiju not found: {0}")]
    KaijuNotFound(Uuid),

    #[error("User {0} is not the owner of kaiju {1}")]
    NotOwner(Uuid, Uuid),

    #[error("Kaiju {0} is in blockchain custody and cannot be transferred server-side")]
    OnChainCustody(Uuid),

    #[error("Kaiju {0} is dead and cannot be transferred")]
    KaijuDead(Uuid),

    #[error("Kaiju {0} is locked: {1}")]
    KaijuLocked(Uuid, String),

    #[error("Invalid transfer type")]
    InvalidTransferType,

    #[error("User not found: {0}")]
    UserNotFound(Uuid),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Signature error: {0}")]
    Signature(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

pub type Result<T> = std::result::Result<T, TransferError>;

impl TransferError {
    /// Returns an appropriate HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            Self::KaijuNotFound(_) | Self::UserNotFound(_) => 404,
            Self::NotOwner(_, _) => 403,
            Self::OnChainCustody(_) | Self::KaijuDead(_) | Self::KaijuLocked(_, _) => 409,
            Self::InvalidTransferType | Self::InvalidState(_) => 400,
            Self::Database(_) => 500,
            Self::Signature(_) | Self::Configuration(_) => 500,
        }
    }
}
