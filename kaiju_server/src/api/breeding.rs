//! Breeding API - Async job-based breeding with ComfyUI image generation

use axum::{
    extract::{State, Json, Path},
    routing::{post, get},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    breeding_service::KaijuData,
    breeding_jobs::BreedingJobStatus,
    name_generator::generate_kaiju_name,
    image_gen::prompt_builder::KaijuGenetics,
};

#[derive(Debug, Deserialize)]
pub struct BreedRequest {
    pub parent_a_id: Uuid,
    pub parent_b_id: Uuid,
    pub client_seed: u64,
    pub user_id: Uuid,
}

/// Response when breeding is initiated (async)
#[derive(Debug, Serialize)]
pub struct BreedStartResponse {
    pub job_id: Uuid,
    pub message: String,
    pub locked_kaiju: Vec<Uuid>,
}

/// Response for job status check
#[derive(Debug, Serialize)]
pub struct BreedStatusResponse {
    pub job_id: Uuid,
    pub status: BreedingJobStatus,
    pub offspring: Option<KaijuData>,
    pub error_message: Option<String>,
}

/// Response for locked kaiju query
#[derive(Debug, Serialize)]
pub struct LockedKaijuResponse {
    pub locked_ids: Vec<Uuid>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/breeding/breed", post(handle_start_breed))
        .route("/breeding/status/:job_id", get(handle_breed_status))
        .route("/breeding/locked", get(handle_get_locked))
}

/// Start an async breeding job
async fn handle_start_breed(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BreedRequest>,
) -> Result<Json<BreedStartResponse>, (StatusCode, String)> {
    tracing::info!("Breeding request: {} & {}", payload.parent_a_id, payload.parent_b_id);

    // Check if parents are available
    if !state.breeding_job_manager.are_parents_available(payload.parent_a_id, payload.parent_b_id).await {
        return Err((StatusCode::CONFLICT, "One or both parents are currently breeding".to_string()));
    }

    // Create the breeding job
    let job_id = state.breeding_job_manager
        .create_job(payload.user_id, payload.parent_a_id, payload.parent_b_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // Spawn background task to process breeding
    let state_clone = state.clone();
    let seed = payload.client_seed;
    let parent_a = payload.parent_a_id;
    let parent_b = payload.parent_b_id;
    let user_id = payload.user_id;
    
    tokio::spawn(async move {
        process_breeding_job(state_clone, job_id, parent_a, parent_b, user_id, seed).await;
    });

    Ok(Json(BreedStartResponse {
        job_id,
        message: "Breeding started! Your Kaiju are now breeding.".to_string(),
        locked_kaiju: vec![payload.parent_a_id, payload.parent_b_id],
    }))
}

/// Process breeding job in background
async fn process_breeding_job(
    state: Arc<AppState>,
    job_id: Uuid,
    parent_a_id: Uuid,
    parent_b_id: Uuid,
    user_id: Uuid,
    seed: u64,
) {
    tracing::info!("Processing breeding job {}", job_id);

    // 1. Fetch real parents from database
    let parent_a = match state.kaiju_repo.get_by_id(parent_a_id).await {
        Ok(Some(kaiju)) => kaiju,
        Ok(None) => {
            tracing::error!("Parent A ({}) not found in database", parent_a_id);
            state.breeding_job_manager.fail_job(job_id, "Parent A not found".to_string()).await;
            return;
        }
        Err(e) => {
            tracing::error!("Database error fetching parent A: {}", e);
            state.breeding_job_manager.fail_job(job_id, format!("DB error: {}", e)).await;
            return;
        }
    };
    
    let parent_b = match state.kaiju_repo.get_by_id(parent_b_id).await {
        Ok(Some(kaiju)) => kaiju,
        Ok(None) => {
            tracing::error!("Parent B ({}) not found in database", parent_b_id);
            state.breeding_job_manager.fail_job(job_id, "Parent B not found".to_string()).await;
            return;
        }
        Err(e) => {
            tracing::error!("Database error fetching parent B: {}", e);
            state.breeding_job_manager.fail_job(job_id, format!("DB error: {}", e)).await;
            return;
        }
    };
    
    tracing::info!("Breeding {} ({:?}) x {} ({:?})", 
        parent_a.name, parent_a.traits.iter().map(|t| &t.name).collect::<Vec<_>>(),
        parent_b.name, parent_b.traits.iter().map(|t| &t.name).collect::<Vec<_>>()
    );

    // 2. Execute breeding logic
    let breeding_result = state.breeding_service.breed(&parent_a, &parent_b, seed);

    let mut offspring = match breeding_result {
        Ok(result) => result.offspring,
        Err(e) => {
            tracing::error!("Breeding logic failed: {}", e);
            state.breeding_job_manager.fail_job(job_id, e).await;
            return;
        }
    };

    // 2. Generate unique name
    offspring.name = generate_kaiju_name();

    // 3. Build KaijuGenetics from offspring traits
    // Look for element trait by name
    let element_names = ["fire", "ice", "electric", "water", "earth", "wind", "aquatic", "nature"];
    let element = offspring.traits.iter()
        .find(|t| element_names.iter().any(|e| t.name.to_lowercase().contains(e)))
        .map(|t| t.name.to_lowercase())
        .unwrap_or_else(|| "neutral".to_string());
    
    // Look for body type trait by name
    let body_names = ["bipedal", "quadruped", "serpentine", "winged", "aquatic"];
    let body_type = offspring.traits.iter()
        .find(|t| body_names.iter().any(|b| t.name.to_lowercase().contains(b)))
        .map(|t| t.name.to_lowercase())
        .unwrap_or_else(|| "hybrid".to_string());
    
    // Generate colors based on element
    let (primary_color, secondary_color) = match element.as_str() {
        s if s.contains("fire") => ("#ff4400".to_string(), "#ffaa00".to_string()),
        s if s.contains("ice") => ("#00aaff".to_string(), "#aaddff".to_string()),
        s if s.contains("electric") => ("#ffff00".to_string(), "#ffffaa".to_string()),
        s if s.contains("water") || s.contains("aquatic") => ("#0066ff".to_string(), "#00ccff".to_string()),
        s if s.contains("earth") || s.contains("nature") => ("#228b22".to_string(), "#8b4513".to_string()),
        _ => ("#4488ff".to_string(), "#22aaff".to_string()),
    };
    
    let visual_traits: Vec<String> = offspring.traits.iter()
        .map(|t| t.name.clone())
        .collect();
    
    let genetics = KaijuGenetics {
        element: element.clone(),
        body_type: body_type.clone(),
        primary_color: primary_color.clone(),
        secondary_color: secondary_color.clone(),
        visual_traits: visual_traits.clone(),
    };

    // Log the genetics and prompt
    tracing::info!("Generating image for offspring: {}", offspring.name);
    tracing::debug!("Genetics: element={}, body_type={}, colors={}/{}", 
        element, body_type, primary_color, secondary_color);
    tracing::debug!("Visual traits: {:?}", visual_traits);

    // Ensure seed is positive and within i64::MAX range for ComfyUI
    let seed = (offspring.visual_seed % (i64::MAX as u64)) as i64;
    let image_request_id = state.image_gen_service
        .queue_generation(offspring.id, genetics, seed)
        .await;

    state.breeding_job_manager.set_generating(job_id, image_request_id).await;

    // 4. Try to process the image generation
    if let Err(e) = state.image_gen_service.process_queue_item(image_request_id).await {
        tracing::warn!("ComfyUI generation failed (may be unavailable): {}", e);
        // Fall back to existing image
        offspring.image_url = get_fallback_image();
    } else {
        // Poll for completion (with timeout)
        let mut attempts = 0;
        let max_attempts = 60; // 60 seconds max
        
        tracing::info!("Polling for image completion...");
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            attempts += 1;
            
            match state.image_gen_service.check_progress(image_request_id).await {
                Ok(true) => {
                    tracing::info!("Image generation completed after {} seconds!", attempts);
                    // Image ready - would get the path from the service
                    offspring.image_url = format!(
                        "http://localhost:3000/assets/kaiju/generated/{}.png",
                        image_request_id
                    );
                    break;
                }
                Ok(false) if attempts < max_attempts => {
                    if attempts % 10 == 0 {
                        tracing::debug!("Still waiting... {} seconds", attempts);
                    }
                    continue;
                }
                _ => {
                    tracing::warn!("Timeout or error after {} attempts - using fallback", attempts);
                    offspring.image_url = get_fallback_image();
                    break;
                }
            }
        }
    }

    // 5. Save offspring to database
    if let Err(e) = state.kaiju_repo.insert(&offspring).await {
        tracing::error!("Failed to save offspring to DB: {}", e);
        // We still complete the job so the user sees the Kaiju (even if not persisted)
        // ideally we would retry or fail, but for now we proceed
    } else {
        tracing::info!("Saved offspring {} to database", offspring.name);
    }

    // 6. Complete the job
    tracing::info!("Marking breeding job {} as complete", job_id);
    state.breeding_job_manager.complete_job(job_id, offspring).await;
    tracing::info!("Breeding job {} completed and parents unlocked", job_id);
}

/// Get random fallback image from existing assets
fn get_fallback_image() -> String {
    use rand::seq::SliceRandom;
    let assets = [
        "kaiju_fire_elemental_1768091138860.png",
        "kaiju_ice_elemental_1768091156648.png",
        "kaiju_electric_elemental_1768091175509.png",
        "kaiju_bipedal_neutral_1768091093175.png",
        "kaiju_quadruped_neutral_1768091073894.png",
        "kaiju_serpentine_neutral_1768091108255.png",
    ];
    let chosen = assets.choose(&mut rand::thread_rng()).unwrap();
    format!("http://localhost:3000/assets/sprites/kaiju/{}", chosen)
}

/// Check breeding job status
async fn handle_breed_status(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<BreedStatusResponse>, (StatusCode, String)> {
    match state.breeding_job_manager.get_job(job_id).await {
        Some(job) => Ok(Json(BreedStatusResponse {
            job_id: job.job_id,
            status: job.status,
            offspring: job.offspring,
            error_message: job.error_message,
        })),
        None => Err((StatusCode::NOT_FOUND, "Breeding job not found".to_string())),
    }
}

/// Get list of currently locked (breeding) Kaiju
async fn handle_get_locked(
    State(state): State<Arc<AppState>>,
) -> Json<LockedKaijuResponse> {
    Json(LockedKaijuResponse {
        locked_ids: state.breeding_job_manager.get_locked_kaiju().await,
    })
}
