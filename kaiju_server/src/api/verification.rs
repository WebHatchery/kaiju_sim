//! Verification API endpoints.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::AppState;
use crate::crypto::signer::OwnershipProofPayload;

/// GET /api/verify/keys - Get server's public keys
pub async fn get_public_keys(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(json!({
        "keys": state.server_public_keys,
        "algorithm": "secp256k1"
    }))
}

/// POST /api/verify/ownership - Verify ownership proof
pub async fn verify_ownership_proof(
    State(state): State<Arc<AppState>>,
    Json(proof): Json<OwnershipProofPayload>,
) -> impl IntoResponse {
    let is_valid = state.verifier.verify_ownership(&proof);

    if is_valid {
        (
            StatusCode::OK,
            Json(json!({
                "valid": true,
                "kaiju_id": proof.kaiju_id,
                "owner_user_id": proof.owner_user_id,
                "timestamp": proof.timestamp,
            })),
        )
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "valid": false,
                "error": "Invalid signature"
            })),
        )
    }
}

/// GET /api/kaiju/:id/certificate - Export ownership certificate
pub async fn export_certificate(
    State(state): State<Arc<AppState>>,
    Path(kaiju_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(json!({
        "certificate_version": "1.0",
        "kaiju_id": kaiju_id,
        "export_timestamp": chrono::Utc::now(),
        "instructions": "Use /api/verify/ownership to validate this certificate",
        "public_keys": state.server_public_keys,
    }))
}

/// Build verification router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/verify/keys", get(get_public_keys))
        .route("/api/verify/ownership", post(verify_ownership_proof))
        .route("/api/kaiju/{id}/certificate", get(export_certificate))
}
