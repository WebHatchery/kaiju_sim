//! Marketplace API
//! Provides endpoints to list and purchase Kaiju from the server-controlled catalog.

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    breeding_service::{KaijuData, KaijuStats},
};

/// A Kaiju available for purchase in the marketplace
#[derive(Debug, Clone, Serialize)]
pub struct MarketplaceItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub image_url: String,
    pub stats: KaijuStats,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub items: Vec<MarketplaceItem>,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseRequest {
    pub user_id: Uuid,
    pub item_id: String,
}

#[derive(Debug, Serialize)]
pub struct PurchaseResponse {
    pub success: bool,
    pub kaiju: Option<KaijuData>,
    pub new_gold: i64,
    pub message: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/marketplace/list", get(handle_list))
        .route("/marketplace/purchase", post(handle_purchase))
}

/// Database row for marketplace items
#[derive(sqlx::FromRow)]
struct MarketplaceItemDbRow {
    id: String,
    name: String,
    description: String,
    price: i64,
    image_url: String,
    base_stats: sqlx::types::Json<KaijuStats>,
}

impl MarketplaceItemDbRow {
    fn into_item(self) -> MarketplaceItem {
        MarketplaceItem {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price as i32,
            image_url: self.image_url,
            stats: self.base_stats.0,
        }
    }
}

/// Get the marketplace catalog
async fn handle_list(State(state): State<Arc<AppState>>) -> Json<ListResponse> {
    let rows: Vec<MarketplaceItemDbRow> =
        sqlx::query_as("SELECT * FROM marketplace_items ORDER BY price ASC")
            .fetch_all(&state.db_pool)
            .await
            .unwrap_or_default();

    let items = rows.into_iter().map(|r| r.into_item()).collect();
    Json(ListResponse { items })
}

/// Purchase a Kaiju from the marketplace
async fn handle_purchase(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PurchaseRequest>,
) -> Result<Json<PurchaseResponse>, (StatusCode, String)> {
    tracing::info!(
        "Purchase request: user={}, item={}",
        payload.user_id,
        payload.item_id
    );

    // Fetch item from DB
    let item_row: Option<MarketplaceItemDbRow> =
        sqlx::query_as("SELECT * FROM marketplace_items WHERE id = ?")
            .bind(&payload.item_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match item_row {
        Some(row) => {
            let market_item = row.into_item();
            let price = market_item.price as i64;

            // 1. Transaction to check and deduct gold
            let mut tx = state
                .db_pool
                .begin()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let user_id_str = payload.user_id.to_string();

            let user_gold: i64 =
                sqlx::query_scalar("SELECT gold FROM users WHERE id = ? FOR UPDATE")
                    .bind(&user_id_str)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                    .unwrap_or(0);

            if user_gold < price {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Insufficient Gold: Have {}, Need {}", user_gold, price),
                ));
            }

            let new_gold = user_gold - price;

            sqlx::query("UPDATE users SET gold = ? WHERE id = ?")
                .bind(new_gold)
                .bind(&user_id_str)
                .execute(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            tx.commit()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            // 3. Create a new Kaiju record owned by the user
            use crate::breeding_service::{Trait, TraitInheritance};

            // Add traits based on item ID (e.g. bipedal_neutral)
            let mut traits = Vec::new();
            if market_item.id.contains("bipedal") {
                traits.push(Trait {
                    id: "body".to_string(),
                    name: "Bipedal".to_string(),
                    description: "Bipedal body type".to_string(),
                    inheritance: TraitInheritance::Dominant,
                    is_hidden: false,
                    power: 10,
                });
            } else if market_item.id.contains("quadruped") {
                traits.push(Trait {
                    id: "body".to_string(),
                    name: "Quadruped".to_string(),
                    description: "Quadruped body type".to_string(),
                    inheritance: TraitInheritance::Dominant,
                    is_hidden: false,
                    power: 10,
                });
            } else if market_item.id.contains("serpentine") {
                traits.push(Trait {
                    id: "body".to_string(),
                    name: "Serpentine".to_string(),
                    description: "Serpentine body type".to_string(),
                    inheritance: TraitInheritance::Dominant,
                    is_hidden: false,
                    power: 10,
                });
            }

            traits.push(Trait {
                id: "element".to_string(),
                name: "Neutral".to_string(),
                description: "Neutral element".to_string(),
                inheritance: TraitInheritance::Dominant,
                is_hidden: false,
                power: 10,
            });

            let kaiju = KaijuData {
                id: Uuid::new_v4(),
                name: market_item.name.clone(),
                generation: 0,
                parent_ids: None,
                visual_seed: rand::random::<u64>(),
                genome_hash: format!("market_{}", market_item.id),
                stats: market_item.stats.clone(),
                traits,
                owner_id: payload.user_id,
                image_url: market_item.image_url.clone(),
                tournaments_won: 0,
            };

            // Save to database
            if let Err(e) = state.kaiju_repo.insert(&kaiju).await {
                tracing::error!("Failed to save purchased Kaiju: {}", e);
                // Note: Gold was already deducted. Ideally we would refund here or use distributed tx.
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to persist purchase".to_string(),
                ));
            }

            Ok(Json(PurchaseResponse {
                success: true,
                kaiju: Some(kaiju),
                new_gold, // Use calculated new_gold
                message: format!("Successfully purchased {}!", market_item.name),
            }))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Item '{}' not found", payload.item_id),
        )),
    }
}
