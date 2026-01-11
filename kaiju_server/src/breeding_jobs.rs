//! Breeding Job Management
//! Tracks async breeding operations with parent locking.

use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::breeding_service::{KaijuData, KaijuStats};

/// Status of a breeding job
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreedingJobStatus {
    Pending,
    Generating,
    Complete,
    Failed,
}

/// A breeding job record
#[derive(Debug, Clone, Serialize)]
pub struct BreedingJob {
    pub job_id: Uuid,
    pub user_id: Uuid,
    pub parent_a_id: Uuid,
    pub parent_b_id: Uuid,
    pub status: BreedingJobStatus,
    pub image_request_id: Option<Uuid>,
    pub offspring: Option<KaijuData>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

/// Manages breeding jobs and parent locking
pub struct BreedingJobManager {
    jobs: RwLock<HashMap<Uuid, BreedingJob>>,
    locked_kaiju: RwLock<HashSet<Uuid>>,
}

impl BreedingJobManager {
    pub fn new() -> Self {
        Self {
            jobs: RwLock::new(HashMap::new()),
            locked_kaiju: RwLock::new(HashSet::new()),
        }
    }

    /// Check if a Kaiju is locked (in breeding)
    pub async fn is_locked(&self, kaiju_id: Uuid) -> bool {
        self.locked_kaiju.read().await.contains(&kaiju_id)
    }

    /// Check if either parent is locked
    pub async fn are_parents_available(&self, parent_a: Uuid, parent_b: Uuid) -> bool {
        let locked = self.locked_kaiju.read().await;
        !locked.contains(&parent_a) && !locked.contains(&parent_b)
    }

    /// Create a new breeding job and lock parents
    pub async fn create_job(
        &self,
        user_id: Uuid,
        parent_a_id: Uuid,
        parent_b_id: Uuid,
    ) -> Result<Uuid, String> {
        // Check if parents are available
        if !self.are_parents_available(parent_a_id, parent_b_id).await {
            return Err("One or both parents are currently breeding".to_string());
        }

        let job_id = Uuid::new_v4();
        let job = BreedingJob {
            job_id,
            user_id,
            parent_a_id,
            parent_b_id,
            status: BreedingJobStatus::Pending,
            image_request_id: None,
            offspring: None,
            created_at: Utc::now(),
            completed_at: None,
            error_message: None,
        };

        // Lock parents
        {
            let mut locked = self.locked_kaiju.write().await;
            locked.insert(parent_a_id);
            locked.insert(parent_b_id);
        }

        // Store job
        {
            let mut jobs = self.jobs.write().await;
            jobs.insert(job_id, job);
        }

        Ok(job_id)
    }

    /// Update job to generating status
    pub async fn set_generating(&self, job_id: Uuid, image_request_id: Uuid) {
        let mut jobs = self.jobs.write().await;
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = BreedingJobStatus::Generating;
            job.image_request_id = Some(image_request_id);
        }
    }

    /// Mark job as complete with offspring
    pub async fn complete_job(&self, job_id: Uuid, offspring: KaijuData) {
        let (parent_a, parent_b) = {
            let mut jobs = self.jobs.write().await;
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = BreedingJobStatus::Complete;
                job.offspring = Some(offspring);
                job.completed_at = Some(Utc::now());
                (job.parent_a_id, job.parent_b_id)
            } else {
                return;
            }
        };

        // Unlock parents
        let mut locked = self.locked_kaiju.write().await;
        locked.remove(&parent_a);
        locked.remove(&parent_b);
    }

    /// Mark job as failed
    pub async fn fail_job(&self, job_id: Uuid, error: String) {
        let (parent_a, parent_b) = {
            let mut jobs = self.jobs.write().await;
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = BreedingJobStatus::Failed;
                job.error_message = Some(error);
                job.completed_at = Some(Utc::now());
                (job.parent_a_id, job.parent_b_id)
            } else {
                return;
            }
        };

        // Unlock parents
        let mut locked = self.locked_kaiju.write().await;
        locked.remove(&parent_a);
        locked.remove(&parent_b);
    }

    /// Get job status
    pub async fn get_job(&self, job_id: Uuid) -> Option<BreedingJob> {
        self.jobs.read().await.get(&job_id).cloned()
    }

    /// Get all jobs for a user
    pub async fn get_user_jobs(&self, user_id: Uuid) -> Vec<BreedingJob> {
        self.jobs.read().await
            .values()
            .filter(|j| j.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Get all locked Kaiju IDs
    pub async fn get_locked_kaiju(&self) -> Vec<Uuid> {
        self.locked_kaiju.read().await.iter().cloned().collect()
    }
}
