//! Transfer API endpoints.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::AppState;
use crate::error::TransferError;
use crate::types::TransferRequest;

/// API error wrapper
pub struct ApiError(TransferError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0.status_code() {
            400 => StatusCode::BAD_REQUEST,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            409 => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(json!({ "error": self.0.to_string() }))).into_response()
    }
}

impl From<TransferError> for ApiError {
    fn from(err: TransferError) -> Self {
        Self(err)
    }
}

/// POST /api/transfer - Execute a transfer
pub async fn handle_transfer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TransferRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let result = state.transfer_service.execute_transfer(request).await?;
    Ok((StatusCode::OK, Json(result)))
}

/// GET /api/kaiju/:id/proof - Get ownership proof
pub async fn handle_get_proof(
    State(state): State<Arc<AppState>>,
    Path(kaiju_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let proof = state
        .transfer_service
        .generate_ownership_proof(kaiju_id)
        .await?;
    Ok((StatusCode::OK, Json(proof)))
}

/// GET /api/kaiju/:id/history - Get transfer history
pub async fn handle_get_history(
    State(state): State<Arc<AppState>>,
    Path(kaiju_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let history = state
        .transfer_service
        .get_transfer_history(kaiju_id, 100)
        .await?;
    Ok((StatusCode::OK, Json(history)))
}

/// Build transfer router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/transfer", post(handle_transfer))
        .route("/api/kaiju/{id}/proof", get(handle_get_proof))
        .route("/api/kaiju/{id}/history", get(handle_get_history))
}
