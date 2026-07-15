//! Breeding API - Async job-based breeding with ComfyUI image generation
//! Enhanced with Advanced Breeding System depth features.

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    breeding::{BreedingMaterial, ElementType, KaijuRarity},
    breeding_jobs::BreedingJobStatus,
    breeding_service::KaijuData,
    image_gen::prompt_builder::KaijuGenetics,
    name_generator::generate_kaiju_name,
};

/// Material specification in API request
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "element")]
pub enum MaterialRequest {
    #[serde(rename = "elemental_essence")]
    ElementalEssence(String),
    #[serde(rename = "mutation_catalyst")]
    MutationCatalyst,
    #[serde(rename = "genetic_stabilizer")]
    GeneticStabilizer,
    #[serde(rename = "fertility_idol")]
    FertilityIdol,
}

impl MaterialRequest {
    fn to_breeding_material(&self) -> BreedingMaterial {
        match self {
            MaterialRequest::ElementalEssence(elem) => {
                let element = ElementType::from_name(elem).unwrap_or(ElementType::Neutral);
                BreedingMaterial::ElementalEssence(element)
            }
            MaterialRequest::MutationCatalyst => BreedingMaterial::MutationCatalyst,
            MaterialRequest::GeneticStabilizer => BreedingMaterial::GeneticStabilizer,
            MaterialRequest::FertilityIdol => BreedingMaterial::FertilityIdol,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BreedRequest {
    pub parent_a_id: Uuid,
    pub parent_b_id: Uuid,
    pub client_seed: u64,
    pub user_id: Uuid,
    /// Optional materials to use during breeding
    #[serde(default)]
    pub materials: Vec<MaterialRequest>,
}

/// Response when breeding is initiated (async)
#[derive(Debug, Serialize)]
pub struct BreedStartResponse {
    pub job_id: Uuid,
    pub message: String,
    pub locked_kaiju: Vec<Uuid>,
    /// Estimated gestation time in hours
    pub estimated_gestation_hours: Option<f32>,
}

/// Response for job status check
#[derive(Debug, Serialize)]
pub struct BreedStatusResponse {
    pub job_id: Uuid,
    pub status: BreedingJobStatus,
    pub offspring: Option<KaijuData>,
    pub error_message: Option<String>,
    /// Breeding log summary for admin debugging
    pub breeding_log_event_id: Option<String>,
}

/// Response for locked kaiju query
#[derive(Debug, Serialize)]
pub struct LockedKaijuResponse {
    pub locked_ids: Vec<Uuid>,
}

/// Response for breeding cost calculation
#[derive(Debug, Serialize)]
pub struct BreedingCostResponse {
    pub parent_a_rarity: String,
    pub parent_b_rarity: String,
    pub base_cost: i64,
    pub material_costs: Vec<MaterialCost>,
    pub total_cost: i64,
    pub estimated_gestation_hours: f32,
    pub estimated_maturation_hours: f32,
}

#[derive(Debug, Serialize)]
pub struct MaterialCost {
    pub material_type: String,
    pub cost: i64,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/breeding/breed", post(handle_start_breed))
        .route("/breeding/status/:job_id", get(handle_breed_status))
        .route("/breeding/locked", get(handle_get_locked))
        .route("/breeding/cost", post(handle_calculate_cost))
}

/// Start an async breeding job
async fn handle_start_breed(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BreedRequest>,
) -> Result<Json<BreedStartResponse>, (StatusCode, String)> {
    tracing::info!(
        "Breeding request: {} & {}",
        payload.parent_a_id,
        payload.parent_b_id
    );

    // Check if parents are available
    if !state
        .breeding_job_manager
        .are_parents_available(payload.parent_a_id, payload.parent_b_id)
        .await
    {
        return Err((
            StatusCode::CONFLICT,
            "One or both parents are currently breeding".to_string(),
        ));
    }

    // Create the breeding job
    let job_id = state
        .breeding_job_manager
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
        estimated_gestation_hours: None, // Will be updated when job starts
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
            state
                .breeding_job_manager
                .fail_job(job_id, "Parent A not found".to_string())
                .await;
            return;
        }
        Err(e) => {
            tracing::error!("Database error fetching parent A: {}", e);
            state
                .breeding_job_manager
                .fail_job(job_id, format!("DB error: {}", e))
                .await;
            return;
        }
    };

    let parent_b = match state.kaiju_repo.get_by_id(parent_b_id).await {
        Ok(Some(kaiju)) => kaiju,
        Ok(None) => {
            tracing::error!("Parent B ({}) not found in database", parent_b_id);
            state
                .breeding_job_manager
                .fail_job(job_id, "Parent B not found".to_string())
                .await;
            return;
        }
        Err(e) => {
            tracing::error!("Database error fetching parent B: {}", e);
            state
                .breeding_job_manager
                .fail_job(job_id, format!("DB error: {}", e))
                .await;
            return;
        }
    };

    tracing::info!(
        "Breeding {} ({:?}) x {} ({:?})",
        parent_a.name,
        parent_a.traits.iter().map(|t| &t.name).collect::<Vec<_>>(),
        parent_b.name,
        parent_b.traits.iter().map(|t| &t.name).collect::<Vec<_>>()
    );

    // 2. Execute ADVANCED breeding logic with full logging
    // For now, use empty materials list (can be extended via API later)
    let materials = Vec::new();
    let breeding_result = state
        .advanced_breeding_service
        .breed(&parent_a, &parent_b, seed, materials);

    let (result, breeding_log) = match breeding_result {
        Ok((result, log)) => {
            // Log the detailed breeding log at debug level
            tracing::debug!("Breeding Log: {}", log.to_json_string());
            (result, log)
        }
        Err(e) => {
            tracing::error!("Breeding logic failed: {}", e);
            state.breeding_job_manager.fail_job(job_id, e).await;
            return;
        }
    };

    let mut offspring = result.offspring;

    // Log mutations if any occurred
    if !result.mutations.is_empty() {
        tracing::info!(
            "Offspring {} has mutations: {:?}",
            offspring.name,
            result.mutations
        );
    }

    // 2. Generate unique name
    offspring.name = generate_kaiju_name();

    // 3. Build KaijuGenetics from offspring traits
    // Look for element trait by name
    let element_names = [
        "fire", "ice", "electric", "water", "earth", "wind", "aquatic", "nature",
    ];
    let element = offspring
        .traits
        .iter()
        .find(|t| {
            element_names
                .iter()
                .any(|e| t.name.to_lowercase().contains(e))
        })
        .map(|t| t.name.to_lowercase())
        .unwrap_or_else(|| "neutral".to_string());

    // Look for body type trait by name
    let body_names = ["bipedal", "quadruped", "serpentine", "winged", "aquatic"];
    let body_type = offspring
        .traits
        .iter()
        .find(|t| body_names.iter().any(|b| t.name.to_lowercase().contains(b)))
        .map(|t| t.name.to_lowercase())
        .unwrap_or_else(|| "hybrid".to_string());

    // Generate colors based on element
    let (primary_color, secondary_color) = match element.as_str() {
        s if s.contains("fire") => ("#ff4400".to_string(), "#ffaa00".to_string()),
        s if s.contains("ice") => ("#00aaff".to_string(), "#aaddff".to_string()),
        s if s.contains("electric") => ("#ffff00".to_string(), "#ffffaa".to_string()),
        s if s.contains("water") || s.contains("aquatic") => {
            ("#0066ff".to_string(), "#00ccff".to_string())
        }
        s if s.contains("earth") || s.contains("nature") => {
            ("#228b22".to_string(), "#8b4513".to_string())
        }
        _ => ("#4488ff".to_string(), "#22aaff".to_string()),
    };

    let visual_traits: Vec<String> = offspring.traits.iter().map(|t| t.name.clone()).collect();

    let genetics = KaijuGenetics {
        element: element.clone(),
        body_type: body_type.clone(),
        primary_color: primary_color.clone(),
        secondary_color: secondary_color.clone(),
        visual_traits: visual_traits.clone(),
    };

    // Log the genetics and prompt
    tracing::info!("Generating image for offspring: {}", offspring.name);
    tracing::debug!(
        "Genetics: element={}, body_type={}, colors={}/{}",
        element,
        body_type,
        primary_color,
        secondary_color
    );
    tracing::debug!("Visual traits: {:?}", visual_traits);

    // Ensure seed is positive and within i64::MAX range for ComfyUI
    let seed = (offspring.visual_seed % (i64::MAX as u64)) as i64;
    let image_request_id = state
        .image_gen_service
        .queue_generation(offspring.id, genetics, seed)
        .await;

    state
        .breeding_job_manager
        .set_generating(job_id, image_request_id)
        .await;

    // 4. Try to process the image generation
    if let Err(e) = state
        .image_gen_service
        .process_queue_item(image_request_id)
        .await
    {
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

            match state
                .image_gen_service
                .check_progress(image_request_id)
                .await
            {
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
                    tracing::warn!(
                        "Timeout or error after {} attempts - using fallback",
                        attempts
                    );
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
    state
        .breeding_job_manager
        .complete_job(job_id, offspring)
        .await;
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
            breeding_log_event_id: None, // TODO: Store and retrieve from job
        })),
        None => Err((StatusCode::NOT_FOUND, "Breeding job not found".to_string())),
    }
}

/// Get list of currently locked (breeding) Kaiju
async fn handle_get_locked(State(state): State<Arc<AppState>>) -> Json<LockedKaijuResponse> {
    Json(LockedKaijuResponse {
        locked_ids: state.breeding_job_manager.get_locked_kaiju().await,
    })
}

/// Calculate breeding costs before starting
#[derive(Debug, Deserialize)]
pub struct BreedingCostRequest {
    pub parent_a_id: Uuid,
    pub parent_b_id: Uuid,
    #[serde(default)]
    pub materials: Vec<MaterialRequest>,
}

async fn handle_calculate_cost(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BreedingCostRequest>,
) -> Result<Json<BreedingCostResponse>, (StatusCode, String)> {
    use crate::breeding::{BreedingConfig, StatCalculator};

    // Fetch parents
    let parent_a = state
        .kaiju_repo
        .get_by_id(payload.parent_a_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Parent A not found".to_string()))?;

    let parent_b = state
        .kaiju_repo
        .get_by_id(payload.parent_b_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Parent B not found".to_string()))?;

    // Calculate rarity for each parent
    let a_trait_power: i32 = parent_a.traits.iter().map(|t| t.power).sum();
    let a_stat_total =
        parent_a.stats.hp + parent_a.stats.attack + parent_a.stats.defense + parent_a.stats.speed;
    let a_rarity = KaijuRarity::calculate(a_trait_power, a_stat_total);

    let b_trait_power: i32 = parent_b.traits.iter().map(|t| t.power).sum();
    let b_stat_total =
        parent_b.stats.hp + parent_b.stats.attack + parent_b.stats.defense + parent_b.stats.speed;
    let b_rarity = KaijuRarity::calculate(b_trait_power, b_stat_total);

    // Calculate base cost based on rarity
    let config = BreedingConfig::default();
    let base_cost = match (a_rarity, b_rarity) {
        (KaijuRarity::Legendary, _) | (_, KaijuRarity::Legendary) => {
            config.economy.breeding_costs.legendary_any
        }
        (KaijuRarity::Rare, KaijuRarity::Rare)
        | (KaijuRarity::Epic, _)
        | (_, KaijuRarity::Epic) => config.economy.breeding_costs.rare_rare,
        (KaijuRarity::Rare, _)
        | (_, KaijuRarity::Rare)
        | (KaijuRarity::Uncommon, KaijuRarity::Uncommon) => {
            config.economy.breeding_costs.common_rare
        }
        _ => config.economy.breeding_costs.common_common,
    };

    // Calculate material costs
    let material_costs: Vec<MaterialCost> = payload
        .materials
        .iter()
        .map(|m| {
            let cost = match m {
                MaterialRequest::ElementalEssence(_) => 2000,
                MaterialRequest::MutationCatalyst => 5000,
                MaterialRequest::GeneticStabilizer => 3000,
                MaterialRequest::FertilityIdol => 1500,
            };
            MaterialCost {
                material_type: format!("{:?}", m),
                cost,
            }
        })
        .collect();

    let total_cost = base_cost + material_costs.iter().map(|m| m.cost).sum::<i64>();

    // Calculate timing
    let offspring_gen = parent_a.generation.max(parent_b.generation) + 1;
    let total_power = a_trait_power + b_trait_power;
    let stat_calc = StatCalculator::new(config);
    let gestation = stat_calc.calculate_gestation_hours(offspring_gen, total_power);
    let maturation = stat_calc.calculate_maturation_hours(offspring_gen);

    Ok(Json(BreedingCostResponse {
        parent_a_rarity: format!("{:?}", a_rarity),
        parent_b_rarity: format!("{:?}", b_rarity),
        base_cost,
        material_costs,
        total_cost,
        estimated_gestation_hours: gestation,
        estimated_maturation_hours: maturation,
    }))
}
