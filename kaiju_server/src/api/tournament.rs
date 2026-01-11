use axum::{
    extract::{State, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
    http::StatusCode,
};
use crate::api::{AppState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use std::sync::Arc;
use sqlx::Row;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_current_tournament))
        .route("/enroll", post(enroll_kaiju))
}

#[derive(Debug, Serialize)]
struct TournamentDto {
    id: String,
    start_time: NaiveDateTime,
    state: String,
    current_round: i32,
    participants_count: i64,
    matches: Vec<MatchDto>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct MatchDto {
    round_number: i32,
    match_index: i32,
    kaiju_a_id: Option<String>,
    kaiju_b_id: Option<String>,
    kaiju_a_name: Option<String>,
    kaiju_b_name: Option<String>,
    winner_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EnrollRequest {
    user_id: Uuid,
    kaiju_id: Uuid,
}

#[derive(Debug, Serialize)]
struct EnrollResponse {
    success: bool,
    message: String,
}

async fn get_current_tournament(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Get active tournament
    let current = sqlx::query("SELECT id, start_time, state, current_round FROM tournaments WHERE state != 'Finished' ORDER BY created_at DESC LIMIT 1")
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(row) = current {
        let id: String = row.get("id");
        let start_time: NaiveDateTime = row.get("start_time");
        let status: String = row.get("state");
        let current_round: i32 = row.get("current_round");
        
        // 2. Count participants
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tournament_participants WHERE tournament_id = ?")
            .bind(&id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
        // 3. Get matches with names
        let matches = sqlx::query_as::<_, MatchDto>(r#"
            SELECT 
                m.round_number, m.match_index, 
                m.kaiju_a_id, m.kaiju_b_id, m.winner_id,
                ka.name as kaiju_a_name,
                kb.name as kaiju_b_name
            FROM tournament_matches m
            LEFT JOIN kaiju ka ON m.kaiju_a_id = ka.id
            LEFT JOIN kaiju kb ON m.kaiju_b_id = kb.id
            WHERE m.tournament_id = ?
            ORDER BY m.round_number, m.match_index
        "#)
            .bind(&id)
            .fetch_all(&state.db_pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
        Ok(Json(TournamentDto {
            id,
            start_time,
            state: status,
            current_round,
            participants_count: count,
            matches,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "No active tournament".to_string()))
    }
}

async fn enroll_kaiju(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EnrollRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Check if tournament is in Registration
    let current = sqlx::query("SELECT id, state FROM tournaments WHERE state = 'Registration' ORDER BY created_at DESC LIMIT 1")
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    let (tournament_id, _) = match current {
        Some(row) => (row.get::<String, _>("id"), row.get::<String, _>("state")),
        None => return Err((StatusCode::BAD_REQUEST, "Registration closed or no tournament".to_string())),
    };
    
    // 2. Verify Kaiju ownership
    let kid_str = payload.kaiju_id.to_string();
    let owner_id: Option<String> = sqlx::query_scalar("SELECT owner_user_id FROM kaiju WHERE id = ?")
        .bind(&kid_str)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    if let Some(oid) = owner_id {
        if oid != payload.user_id.to_string() {
             return Err((StatusCode::FORBIDDEN, "You do not own this Kaiju".to_string()));
        }
    } else {
         return Err((StatusCode::NOT_FOUND, "Kaiju not found".to_string()));
    }
    
    // 3. Enroll (Insert, ignore duplicates)
    // Using simple check instead of INSERT IGNORE to return better error
    let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tournament_participants WHERE tournament_id = ? AND kaiju_id = ?")
        .bind(&tournament_id)
        .bind(&kid_str)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    if exists > 0 {
        return Ok(Json(EnrollResponse { success: false, message: "Already enrolled".to_string() }));
    }
    
    sqlx::query("INSERT INTO tournament_participants (tournament_id, kaiju_id, user_id) VALUES (?, ?, ?)")
        .bind(&tournament_id)
        .bind(&kid_str)
        .bind(payload.user_id.to_string())
        .execute(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    Ok(Json(EnrollResponse { success: true, message: "Enrolled successfully".to_string() }))
}
