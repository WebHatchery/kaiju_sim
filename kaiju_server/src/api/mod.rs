//! API modules for the Kaiju server.

pub mod transfer;
pub mod verification;
pub mod breeding;
pub mod user;
pub mod marketplace;
pub mod tournament;

use axum::Router;
use std::sync::Arc;

use crate::crypto::Verifier;
use crate::transfer_service::TransferService;
use crate::breeding_service::BreedingService;
use crate::breeding::AdvancedBreedingService;
use crate::breeding_jobs::BreedingJobManager;
use crate::image_gen::ImageGenerationService;
use crate::kaiju_repo::KaijuRepository;
use sqlx::mysql::MySqlPool;

/// Shared application state
pub struct AppState {
    pub db_pool: MySqlPool,
    pub kaiju_repo: KaijuRepository,
    pub transfer_service: Arc<TransferService>,
    pub breeding_service: Arc<BreedingService>,
    pub advanced_breeding_service: Arc<AdvancedBreedingService>,
    pub breeding_job_manager: Arc<BreedingJobManager>,
    pub image_gen_service: Arc<ImageGenerationService>,
    pub verifier: Verifier,
    pub server_public_keys: Vec<String>,
}

use axum::routing::get;

/// Build the complete API router
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(user::router())
        .merge(transfer::router())
        .merge(verification::router())
        .merge(breeding::router())
        .merge(marketplace::router())
        .merge(tournament::router())
        .with_state(state)
}
