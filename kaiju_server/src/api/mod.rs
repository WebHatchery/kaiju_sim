//! API modules for the Kaiju server.

pub mod transfer;
pub mod verification;

use axum::Router;
use std::sync::Arc;

use crate::crypto::Verifier;
use crate::transfer_service::TransferService;

/// Shared application state
pub struct AppState {
    pub transfer_service: Arc<TransferService>,
    pub verifier: Verifier,
    pub server_public_keys: Vec<String>,
}

/// Build the complete API router
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(transfer::router())
        .merge(verification::router())
        .with_state(state)
}
