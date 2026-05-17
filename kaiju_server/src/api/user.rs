use crate::api::AppState;
use crate::breeding_service::{KaijuData, KaijuStats, Trait, TraitInheritance};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub user_id: Option<Uuid>,          // If None, create new
    pub starter_choice: Option<String>, // "Fire", "Ice", "Electric"
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub gold: i64,
    pub roster: Vec<KaijuData>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/user/login", post(handle_login))
}

async fn handle_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user_id = payload.user_id.unwrap_or_else(Uuid::new_v4);
    tracing::info!("User Login: {}", user_id);

    // Ensure user exists in DB (to satisfy Foreign Key)
    let username = format!("Guest_{}", &user_id.to_string()[..8]);
    // Try insert (ignore if exists)
    let _ = sqlx::query(
        r#"
        INSERT IGNORE INTO users (id, username, created_at, gold)
        VALUES (?, ?, NOW(), 1000)
    "#,
    )
    .bind(user_id.to_string())
    .bind(username)
    .execute(&state.db_pool)
    .await;

    // Fetch gold balance (should exist now)
    let gold_balance: i64 = match sqlx::query_scalar("SELECT gold FROM users WHERE id = ?")
        .bind(user_id.to_string())
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB Error: {}", e),
            )
        })? {
        Some(gold) => gold,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "User record creation failed (login)".to_string(),
            ))
        }
    };

    // Check if user already has Kaiju in DB
    let existing_roster = state
        .kaiju_repo
        .get_by_owner(user_id)
        .await
        .unwrap_or_default();

    if !existing_roster.is_empty() {
        tracing::info!("Returning {} existing Kaiju from DB", existing_roster.len());
        return Ok(Json(LoginResponse {
            user_id,
            gold: gold_balance,
            roster: existing_roster,
        }));
    }

    // New user (or existing user with no Kaiju) - create starter Kaiju
    let choice = payload.starter_choice.as_deref().unwrap_or("Electric");
    let (name, image, seed, hp, atk, def, spd, element) = match choice {
        "Fire" => (
            "Ignis",
            "kaiju_fire_elemental_1768091138860.png",
            101,
            120,
            25,
            10,
            12,
            "Fire",
        ),
        "Ice" => (
            "Glacies",
            "kaiju_ice_elemental_1768091156648.png",
            102,
            110,
            18,
            20,
            10,
            "Ice",
        ),
        "Electric" | _ => (
            "Volt",
            "kaiju_electric_elemental_1768091175509.png",
            103,
            100,
            20,
            15,
            15,
            "Electric",
        ),
    };

    // Server generates the Starter Kaiju with traits
    let starter = KaijuData {
        id: Uuid::new_v4(),
        name: name.to_string(),
        generation: 0,
        parent_ids: None,
        visual_seed: seed,
        genome_hash: format!("starter_{}", choice.to_lowercase()),
        tournaments_won: 0,
        stats: KaijuStats::new(hp, atk, def, spd, 50),
        traits: vec![
            Trait {
                id: "element".to_string(),
                name: element.to_string(),
                description: format!("{} element", element),
                inheritance: TraitInheritance::Dominant,
                is_hidden: false,
                power: 10,
            },
            Trait {
                id: "body".to_string(),
                name: "Bipedal".to_string(),
                description: "Bipedal body type".to_string(),
                inheritance: TraitInheritance::Dominant,
                is_hidden: false,
                power: 10,
            },
        ],
        owner_id: user_id,
        image_url: format!("http://localhost:3000/assets/sprites/kaiju/{}", image),
    };

    // Save to database
    if let Err(e) = state.kaiju_repo.insert(&starter).await {
        tracing::error!("Failed to save starter Kaiju: {}", e);
        // Continue anyway - starter will be in memory for this session
    }

    Ok(Json(LoginResponse {
        user_id,
        gold: gold_balance, // Authoritative Starting Gold
        roster: vec![starter],
    }))
}
