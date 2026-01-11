//! Service to manage the image generation queue.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::sync::Arc;

use super::comfy_client::{ComfyClient, ComfyConfig};
use super::prompt_builder::{build_kaiju_prompt, KaijuGenetics};

/// Status of a generation request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

/// Generation request record
#[derive(Debug, Clone)]
pub struct GenerationRecord {
    pub request_id: Uuid,
    pub kaiju_id: Uuid,
    pub genetics: KaijuGenetics,
    pub visual_seed: i64,
    pub status: GenerationStatus,
    pub prompt_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub output_path: Option<String>,
}

/// Main service struct
pub struct ImageGenerationService {
    client: ComfyClient,
    queue: Arc<Mutex<HashMap<Uuid, GenerationRecord>>>, // Map request_id -> Record
    output_dir: String,
}

impl ImageGenerationService {
    pub fn new(output_dir: &str) -> Self {
        Self {
            client: ComfyClient::new(ComfyConfig::default()),
            queue: Arc::new(Mutex::new(HashMap::new())),
            output_dir: output_dir.to_string(),
        }
    }

    /// Check if backend is available
    pub async fn is_available(&self) -> bool {
        self.client.check_connection().await
    }

    /// Queue a new generation request
    pub async fn queue_generation(
        &self,
        kaiju_id: Uuid,
        genetics: KaijuGenetics,
        visual_seed: i64,
    ) -> Uuid {
        let request_id = Uuid::new_v4();
        let record = GenerationRecord {
            request_id,
            kaiju_id,
            genetics,
            visual_seed,
            status: GenerationStatus::Queued,
            prompt_id: None,
            created_at: Utc::now(),
            completed_at: None,
            output_path: None,
        };

        let mut queue = self.queue.lock().await;
        queue.insert(request_id, record);

        request_id
    }

    /// Process the next item in the queue (would be called by a background loop)
    pub async fn process_queue_item(&self, request_id: Uuid) -> Result<(), String> {
        // 1. Get record and set to Processing
        let (prompt, negative, seed) = {
            let mut queue = self.queue.lock().await;
            let record = queue.get_mut(&request_id).ok_or("Request not found")?;
            
            if record.status != GenerationStatus::Queued {
                return Ok(()); // Already processed or processing
            }

            record.status = GenerationStatus::Processing;
            let (p, n) = build_kaiju_prompt(&record.genetics);
            (p, n, record.visual_seed)
        };

        // 2. ComfyUI: Queue Prompt
        tracing::info!("Sending prompt to ComfyUI (seed={})", seed);
        tracing::debug!("Positive prompt: {}", prompt);
        tracing::debug!("Negative prompt: {}", negative);
        match self.client.queue_prompt(&prompt, &negative, seed).await {
            Ok(prompt_id) => {
                let mut queue = self.queue.lock().await;
                if let Some(record) = queue.get_mut(&request_id) {
                    record.prompt_id = Some(prompt_id);
                }
            }
            Err(e) => {
                self.mark_failed(request_id).await;
                return Err(e.to_string());
            }
        }

        // 3. Poll for completion (simplified version - immediate wait)
        // In a real system, this would be a separate polling loop based on prompt_id
        // For this implementation, we'll assume the caller manages the polling or we block
        Ok(())
    }

    /// Check status of a running prompt and download if ready
    pub async fn check_progress(&self, request_id: Uuid) -> Result<bool, String> {
        let prompt_id = {
            let queue = self.queue.lock().await;
            match queue.get(&request_id) {
                Some(record) => match &record.prompt_id {
                    Some(pid) => pid.clone(),
                    None => return Ok(false), // Not submitted yet
                },
                None => return Err("Request not found".to_string()),
            }
        };

        // Check history
        tracing::debug!("Checking ComfyUI history for prompt: {}", prompt_id);
        match self.client.check_history(&prompt_id).await {
            Ok(Some(filename)) => {
                tracing::info!("Image ready from ComfyUI: {}", filename);
                // Image ready, download it
                match self.client.download_image(&filename).await {
                    Ok(bytes) => {
                        // Ensure output directory exists
                        let _ = std::fs::create_dir_all(&self.output_dir);
                        
                        // Save to disk
                        let output_path = format!("{}/{}.png", self.output_dir, request_id);
                        match std::fs::write(&output_path, &bytes) {
                            Ok(_) => {
                                tracing::info!("Saved generated image to: {}", output_path);
                            }
                            Err(e) => {
                                tracing::error!("Failed to write image: {}", e);
                                return Err(format!("Failed to save image: {}", e));
                            }
                        }
                        
                        let mut queue = self.queue.lock().await;
                        if let Some(record) = queue.get_mut(&request_id) {
                            record.status = GenerationStatus::Completed;
                            record.completed_at = Some(Utc::now());
                            record.output_path = Some(output_path);
                        }
                        Ok(true)
                    }
                    Err(e) => {
                        tracing::error!("Failed to download image: {}", e);
                        Err(e.to_string())
                    }
                }
            }
            Ok(None) => {
                tracing::trace!("Prompt {} still running", prompt_id);
                Ok(false)
            }
            Err(e) => {
                tracing::error!("Error checking history: {}", e);
                Err(e.to_string())
            }
        }
    }

    async fn mark_failed(&self, request_id: Uuid) {
        let mut queue = self.queue.lock().await;
        if let Some(record) = queue.get_mut(&request_id) {
            record.status = GenerationStatus::Failed;
        }
    }
}
