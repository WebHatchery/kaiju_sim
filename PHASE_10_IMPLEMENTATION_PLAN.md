# PHASE 10 IMPLEMENTATION PLAN: AI Image Generation

**Kaiju Breeding Simulator - Phase 10 Detailed Specification**

**Version**: 1.0
**Date**: 2026-01-10
**Status**: Ready for Implementation
**Dependencies**: Phases 1-9 complete (especially Phase 2 genetics and NFT metadata system)

---

## 1. OVERVIEW

### 1.1 Goals

Phase 10 implements AI-generated kaiju portraits based on genetic data, creating unique, deterministic visual representations for each kaiju. The system must:

1. **Generate unique portraits** from genome data and traits
2. **Maintain visual consistency** across generations (children resemble parents)
3. **Support queued generation** for batch processing
4. **Provide fallback placeholders** during generation
5. **Upload to IPFS** for permanent NFT metadata
6. **Enable lineage visualization** with ancestry trees
7. **Support multiple AI services** (Stable Diffusion, DALL-E, Midjourney)
8. **Implement quality assurance** workflows

### 1.2 Architecture Overview

```
Input Data → Prompt Builder → Generation Queue → AI Service → Storage → QA → Display
(Genome,     (Trait Visual    (Priority-based   (SD/DALL-E)  (IPFS)  (Validation)  (UI)
 Traits,      Keywords)        Job System)
 Ancestry)
```

---

## 2. PROMPT BUILDER SYSTEM

### 2.1 Prompt Structure

Each AI generation prompt follows this template:

```
[Base Description] + [Trait Visual Keywords] + [Ancestry Modifiers] + [Style Suffix] + [Negative Prompt]
```

**Example**:
```
A monstrous kaiju creature, detailed portrait, crackling with blue electricity,
lightning patterns, scales with ocean-blue shimmer, water droplets,
golden aura from champion bloodline, regal bearing,
fantasy creature art, dramatic lighting, high detail, 512x512 resolution
Negative: blurry, low quality, human, anime style, cartoonish
```

### 2.2 Trait-to-Visual Mapping

From **TRAIT_SYSTEM_DESIGN.md**, we have 50 traits with visual keywords. The mapping system extracts these:

#### Implementation: `src/engine/image_gen/trait_mapper.rs`

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitVisualMapping {
    pub trait_id: String,
    pub trait_name: String,
    pub visual_keywords: Vec<String>,
    pub required_feature: Option<String>, // e.g., "wings", "spikes"
    pub color_hints: Vec<String>,
    pub texture_hints: Vec<String>,
}

pub struct TraitMapper {
    mappings: HashMap<String, TraitVisualMapping>,
}

impl TraitMapper {
    pub fn load_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = std::fs::read_to_string(path)?;
        let mappings: Vec<TraitVisualMapping> = serde_json::from_str(&json_data)?;

        let mapping_map = mappings.into_iter()
            .map(|m| (m.trait_id.clone(), m))
            .collect();

        Ok(TraitMapper { mappings: mapping_map })
    }

    pub fn extract_visual_keywords(&self, traits: &[String]) -> Vec<String> {
        traits.iter()
            .filter_map(|trait_id| self.mappings.get(trait_id))
            .flat_map(|m| m.visual_keywords.iter().cloned())
            .collect()
    }

    pub fn get_required_features(&self, traits: &[String]) -> Vec<String> {
        traits.iter()
            .filter_map(|trait_id| self.mappings.get(trait_id))
            .filter_map(|m| m.required_feature.clone())
            .collect()
    }
}
```

### 2.3 Ancestry Modifiers

Ancestry influences appearance through **bloodline themes**:

```rust
pub struct AncestryModifier {
    pub notable_ancestors: Vec<AncestorData>,
}

pub struct AncestorData {
    pub name: String,
    pub generation: u32,
    pub achievement: String, // "Champion Gen 4 Lethal", "Hall of Fame"
    pub dominant_traits: Vec<String>,
}

impl AncestryModifier {
    pub fn generate_modifiers(&self) -> Vec<String> {
        let mut modifiers = Vec::new();

        // Champion bloodline
        if self.notable_ancestors.iter().any(|a| a.achievement.contains("Champion")) {
            modifiers.push("golden aura".to_string());
            modifiers.push("regal bearing".to_string());
        }

        // Mutation-heavy lineage
        let mutation_count = self.notable_ancestors.iter()
            .filter(|a| a.dominant_traits.iter().any(|t| t.contains("Mutation")))
            .count();

        if mutation_count >= 2 {
            modifiers.push("asymmetric features".to_string());
            modifiers.push("chaotic patterns".to_string());
        }

        // Ancient bloodline (low generation ancestors)
        if self.notable_ancestors.iter().any(|a| a.generation <= 2) {
            modifiers.push("primal appearance".to_string());
            modifiers.push("ancient markings".to_string());
        }

        modifiers
    }
}
```

### 2.4 Prompt Composition Service

#### Implementation: `src/engine/image_gen/prompt_builder.rs`

```rust
use crate::data::kaiju::Kaiju;
use crate::data::traits::Trait;

pub struct PromptBuilder {
    trait_mapper: TraitMapper,
    style_config: StyleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub base_prompt: String,
    pub style_suffix: String,
    pub negative_prompt: String,
    pub resolution: (u32, u32),
}

impl PromptBuilder {
    pub fn new(trait_mapper: TraitMapper, style_config: StyleConfig) -> Self {
        PromptBuilder { trait_mapper, style_config }
    }

    pub fn build_prompt(&self, kaiju: &Kaiju, ancestry: Option<&AncestryModifier>) -> GenerationPrompt {
        let mut prompt_parts = vec![self.style_config.base_prompt.clone()];

        // Extract trait visual keywords
        let trait_ids: Vec<String> = kaiju.traits.iter().map(|t| t.id.clone()).collect();
        let visual_keywords = self.trait_mapper.extract_visual_keywords(&trait_ids);
        prompt_parts.extend(visual_keywords);

        // Add ancestry modifiers
        if let Some(ancestry) = ancestry {
            let ancestry_mods = ancestry.generate_modifiers();
            prompt_parts.extend(ancestry_mods);
        }

        // Add generation-based scale hints
        let scale_hint = match kaiju.generation {
            0..=2 => "primordial scale, ancient appearance",
            3..=5 => "mature form, balanced proportions",
            6..=10 => "refined features, specialized morphology",
            _ => "highly evolved, intricate details"
        };
        prompt_parts.push(scale_hint.to_string());

        // Add style suffix
        prompt_parts.push(self.style_config.style_suffix.clone());

        GenerationPrompt {
            positive_prompt: prompt_parts.join(", "),
            negative_prompt: self.style_config.negative_prompt.clone(),
            width: self.style_config.resolution.0,
            height: self.style_config.resolution.1,
            seed: derive_prompt_seed(kaiju.visual_seed),
            required_features: self.trait_mapper.get_required_features(&trait_ids),
        }
    }
}

pub struct GenerationPrompt {
    pub positive_prompt: String,
    pub negative_prompt: String,
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub required_features: Vec<String>,
}

fn derive_prompt_seed(visual_seed: u64) -> u64 {
    // Use visual seed for deterministic but varied generation
    // Add salt to prevent exact visual seed collision revealing genetics
    visual_seed.wrapping_mul(0x9E3779B97F4A7C15)
}
```

### 2.5 Prompt Catalog Structure

#### File: `assets\image_prompts.json`

```json
{
  "version": "1.0",
  "base_prompts": {
    "default": "A monstrous kaiju creature, full body portrait, menacing presence",
    "detail_focus": "A detailed kaiju creature portrait, intricate features, high resolution",
    "action_pose": "A dynamic kaiju creature in battle stance, powerful presence"
  },
  "style_configs": {
    "fantasy_realistic": {
      "base_prompt": "A monstrous kaiju creature, detailed portrait",
      "style_suffix": "fantasy creature art, dramatic lighting, volumetric fog, cinematic composition, 4k quality",
      "negative_prompt": "blurry, low quality, anime, cartoon, human features, cute, chibi, text, watermark",
      "resolution": [512, 512]
    }
  },
  "trait_mappings": [
    {
      "trait_id": "E01",
      "trait_name": "Electric Breath",
      "visual_keywords": ["crackling with blue electricity", "lightning patterns", "electric arcs"],
      "required_feature": null,
      "color_hints": ["electric blue", "white-blue glow"],
      "texture_hints": ["energy discharge", "static charge"]
    }
  ]
}
```

---

## 3. AI SERVICE INTEGRATION

### 3.1 Service Abstraction Layer

#### Implementation: `src/engine/image_gen/ai_service.rs`

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AIImageService: Send + Sync {
    async fn generate_image(
        &self,
        prompt: &GenerationPrompt,
        job_id: Uuid,
    ) -> Result<GeneratedImage, AIServiceError>;

    fn service_name(&self) -> &str;
    fn max_concurrent_jobs(&self) -> usize;
    fn estimated_generation_time(&self) -> Duration;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub image_data: Vec<u8>,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub generation_seed: u64,
    pub service_metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WebP,
}

#[derive(Debug, thiserror::Error)]
pub enum AIServiceError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Timeout after {0:?}")]
    Timeout(Duration),

    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}
```

### 3.2 Stable Diffusion Service

#### Implementation: `src/engine/image_gen/stable_diffusion.rs`

```rust
use reqwest::Client;
use serde_json::json;

pub struct StableDiffusionService {
    client: Client,
    api_url: String,
    api_key: String,
    model: String,
}

impl StableDiffusionService {
    pub fn new(api_url: String, api_key: String) -> Self {
        StableDiffusionService {
            client: Client::new(),
            api_url,
            api_key,
            model: "stable-diffusion-xl-1024-v1-0".to_string(),
        }
    }
}

#[async_trait]
impl AIImageService for StableDiffusionService {
    async fn generate_image(
        &self,
        prompt: &GenerationPrompt,
        job_id: Uuid,
    ) -> Result<GeneratedImage, AIServiceError> {
        let request_body = json!({
            "text_prompts": [
                {
                    "text": prompt.positive_prompt,
                    "weight": 1.0
                },
                {
                    "text": prompt.negative_prompt,
                    "weight": -1.0
                }
            ],
            "cfg_scale": 7.0,
            "height": prompt.height,
            "width": prompt.width,
            "samples": 1,
            "steps": 50,
            "seed": prompt.seed,
        });

        let response = self.client
            .post(&format!("{}/v1/generation/{}/text-to-image", self.api_url, self.model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .timeout(Duration::from_secs(300))
            .send()
            .await
            .map_err(|e| AIServiceError::ApiError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AIServiceError::ApiError(
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default())
            ));
        }

        let result: StableDiffusionResponse = response.json().await
            .map_err(|e| AIServiceError::ApiError(e.to_string()))?;

        let artifact = result.artifacts.into_iter().next()
            .ok_or_else(|| AIServiceError::ApiError("No image generated".to_string()))?;

        let image_data = base64::decode(&artifact.base64)
            .map_err(|e| AIServiceError::ApiError(e.to_string()))?;

        Ok(GeneratedImage {
            image_data,
            format: ImageFormat::PNG,
            width: prompt.width,
            height: prompt.height,
            generation_seed: prompt.seed,
            service_metadata: json!({
                "model": self.model,
                "finish_reason": artifact.finish_reason,
                "seed": artifact.seed,
            }),
        })
    }

    fn service_name(&self) -> &str {
        "Stable Diffusion XL"
    }

    fn max_concurrent_jobs(&self) -> usize {
        5 // Adjust based on API plan
    }

    fn estimated_generation_time(&self) -> Duration {
        Duration::from_secs(30)
    }
}

#[derive(Debug, Deserialize)]
struct StableDiffusionResponse {
    artifacts: Vec<Artifact>,
}

#[derive(Debug, Deserialize)]
struct Artifact {
    base64: String,
    seed: u64,
    finish_reason: String,
}
```

---

## 4. QUEUE-BASED GENERATION SYSTEM

### 4.1 Database Schema for Job Queue

Add to **DATABASE_SCHEMA.md**:

```sql
-- Image generation job queue
CREATE TABLE image_generation_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),

    -- Job state
    status TEXT NOT NULL CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'cancelled')),
    priority INTEGER NOT NULL DEFAULT 50, -- Higher = more urgent
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,

    -- Generation data
    prompt_data JSONB NOT NULL, -- Serialized GenerationPrompt
    service_name TEXT, -- 'stable_diffusion', 'dalle', 'midjourney'

    -- Result data
    image_ipfs_hash TEXT, -- CID after upload
    image_width INTEGER,
    image_height INTEGER,
    image_format TEXT,
    metadata JSONB, -- Service-specific metadata

    -- Error tracking
    error_message TEXT,
    last_error_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Indexing
    CONSTRAINT unique_active_job_per_kaiju UNIQUE (kaiju_id) WHERE status IN ('pending', 'processing')
);

CREATE INDEX idx_generation_jobs_status ON image_generation_jobs(status) WHERE status IN ('pending', 'processing');
CREATE INDEX idx_generation_jobs_priority ON image_generation_jobs(priority DESC, created_at ASC) WHERE status = 'pending';
CREATE INDEX idx_generation_jobs_kaiju ON image_generation_jobs(kaiju_id);
```

### 4.2 Job Queue Service

#### Implementation: `src/engine/image_gen/job_queue.rs`

```rust
use sqlx::PgPool;
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct ImageGenerationQueue {
    pool: PgPool,
    services: Vec<Box<dyn AIImageService>>,
    max_concurrent: Arc<Semaphore>,
    placeholder_service: PlaceholderService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationJob {
    pub id: Uuid,
    pub kaiju_id: Uuid,
    pub status: JobStatus,
    pub priority: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub prompt_data: GenerationPrompt,
    pub service_name: Option<String>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl ImageGenerationQueue {
    pub fn new(pool: PgPool, services: Vec<Box<dyn AIImageService>>) -> Self {
        let max_concurrent = services.iter().map(|s| s.max_concurrent_jobs()).sum();

        ImageGenerationQueue {
            pool,
            services,
            max_concurrent: Arc::new(Semaphore::new(max_concurrent)),
            placeholder_service: PlaceholderService::new(),
        }
    }

    pub async fn enqueue_generation(
        &self,
        kaiju_id: Uuid,
        prompt: GenerationPrompt,
        priority: i32,
    ) -> Result<Uuid, Box<dyn std::error::Error>>

    pub async fn process_queue(&self) -> Result<(), Box<dyn std::error::Error>>
}
```

---

## 5. IPFS UPLOAD PIPELINE

### 5.1 IPFS Service Integration

```rust
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;

pub struct IPFSUploadService {
    client: IpfsClient,
    pinata_jwt: Option<String>, // Optional pinning service
}

impl IPFSUploadService {
    pub fn new() -> Self {
        IPFSUploadService {
            client: IpfsClient::default(),
            pinata_jwt: std::env::var("PINATA_JWT").ok(),
        }
    }

    pub async fn upload_image(
        &self,
        image_data: &[u8],
        metadata: &ImageMetadata,
    ) -> Result<IPFSUpload, Box<dyn std::error::Error>> {
        // Upload to local IPFS node
        let cursor = Cursor::new(image_data);
        let response = self.client.add(cursor).await?;
        let ipfs_hash = response.hash;

        // Pin to Pinata for redundancy
        if let Some(jwt) = &self.pinata_jwt {
            self.pin_to_pinata(&ipfs_hash, jwt).await?;
        }

        // Generate full IPFS URI
        let ipfs_uri = format!("ipfs://{}", ipfs_hash);
        let https_gateway = format!("https://ipfs.io/ipfs/{}", ipfs_hash);

        Ok(IPFSUpload {
            ipfs_hash,
            ipfs_uri,
            https_gateway,
            size_bytes: image_data.len(),
        })
    }
}
```

---

## 6. QUALITY ASSURANCE WORKFLOW

### 6.1 Automated Validation

```rust
pub struct ImageValidator {
    required_features_detector: FeatureDetector,
}

impl ImageValidator {
    pub fn validate_generated_image(
        &self,
        image: &GeneratedImage,
        prompt: &GenerationPrompt,
    ) -> ValidationResult {
        let mut issues = Vec::new();

        // 1. Check dimensions
        if image.width != prompt.width || image.height != prompt.height {
            issues.push(ValidationIssue::IncorrectDimensions {
                expected: (prompt.width, prompt.height),
                actual: (image.width, image.height),
            });
        }

        // 2. Check for required features (e.g., wings, spikes)
        for feature in &prompt.required_features {
            if !self.required_features_detector.detect(image, feature) {
                issues.push(ValidationIssue::MissingRequiredFeature(feature.clone()));
            }
        }

        // 3. Check image quality (not blurry, not corrupted)
        let quality_score = self.assess_quality(image);
        if quality_score < 0.7 {
            issues.push(ValidationIssue::LowQuality(quality_score));
        }

        ValidationResult {
            passed: issues.is_empty(),
            issues,
            quality_score,
        }
    }
}
```

---

## CRITICAL FILES FOR IMPLEMENTATION

### Critical Files to Create

1. **src\engine\image_gen\prompt_builder.rs** - Core prompt composition logic that converts kaiju genetics into AI generation prompts using trait visual keywords and ancestry modifiers

2. **src\engine\image_gen\job_queue.rs** - Queue management system for async image generation with priority handling, retry logic, and IPFS upload integration

3. **src\engine\image_gen\ai_service.rs** - AI service abstraction layer with implementations for Stable Diffusion, DALL-E, and placeholder generation

4. **assets\image_prompts.json** - Complete catalog of 50 trait visual mappings, ancestry modifiers, style configurations, and prompt templates

5. **src\ui\components\lineage_tree.rs** - Interactive lineage tree renderer showing kaiju ancestry with portraits, achievements, and death markers

These five files form the complete AI image generation pipeline: genetic data → prompt building → queued generation → IPFS storage → UI display with lineage visualization.

---

## IMPLEMENTATION CHECKLIST

### Phase 10A: Foundation (Week 1)

- [ ] Create `src/engine/image_gen/` module structure
- [ ] Implement `TraitMapper` with JSON loading
- [ ] Create `assets/image_prompts.json` with all 50 trait mappings
- [ ] Implement `PromptBuilder` service
- [ ] Write unit tests for prompt composition
- [ ] Validate prompt catalog completeness

### Phase 10B: AI Service Integration (Week 2)

- [ ] Implement `AIImageService` trait abstraction
- [ ] Create `StableDiffusionService` with API integration
- [ ] Create `DallEService` with OpenAI API
- [ ] Implement `PlaceholderService` for fallbacks
- [ ] Add service configuration (API keys, endpoints)
- [ ] Test each service independently

### Phase 10C: Queue System (Week 3)

- [ ] Add database schema for `image_generation_jobs`
- [ ] Implement `ImageGenerationQueue` service
- [ ] Create job enqueueing logic
- [ ] Implement priority-based job processing
- [ ] Add retry logic for failed jobs
- [ ] Create background worker task

### Phase 10D: IPFS Upload (Week 4)

- [ ] Integrate IPFS client library
- [ ] Implement `IPFSUploadService`
- [ ] Add Pinata pinning service integration
- [ ] Create NFT metadata update trigger
- [ ] Test IPFS upload and retrieval
- [ ] Add Arweave backup (optional)

### Phase 10E: Lineage Visualization (Week 5)

- [ ] Implement `LineageTreeRenderer` UI component
- [ ] Create lineage data fetching with recursive SQL
- [ ] Add ancestry modifier extraction
- [ ] Integrate with UI screens
- [ ] Add interactive navigation (click ancestors)
- [ ] Test with deep lineages (10+ generations)

### Phase 10F: Quality Assurance (Week 6)

- [ ] Implement `ImageValidator` with basic checks
- [ ] Add manual review queue database tables
- [ ] Create admin UI for image review
- [ ] Implement approve/reject workflow
- [ ] Add automatic requeue on rejection
- [ ] Test QA workflow end-to-end

### Phase 10G: Integration & Testing (Week 7)

- [ ] Integrate with breeding system (auto-enqueue)
- [ ] Add UI loading states for pending images
- [ ] Implement placeholder → real image transitions
- [ ] Test full pipeline: breed → prompt → generate → upload → display
- [ ] Performance testing (100+ concurrent jobs)
- [ ] Load testing IPFS upload pipeline

### Phase 10H: Polish & Optimization (Week 8)

- [ ] Add image caching layer (CDN)
- [ ] Optimize prompt templates based on results
- [ ] Implement batch generation for NPCs
- [ ] Add analytics (success rate, avg time, costs)
- [ ] Create monitoring dashboard
- [ ] Document deployment procedures

---

## COST ESTIMATES

### AI Generation Costs

| Service | Cost per Image | Quality | Speed | Recommended Use |
|---------|---------------|---------|-------|-----------------|
| **Stable Diffusion XL (Stability AI)** | $0.02-0.04 | High | 30s | Primary service |
| **DALL-E 3 (OpenAI)** | $0.04-0.08 | Very High | 45s | High-value kaiju (champions) |
| **Midjourney** | ~$0.08 | Excellent | 60s | Manual/special events (no API yet) |
| **Local Stable Diffusion** | $0 (compute cost) | High | Variable | Cost reduction (requires GPU server) |

**Estimated Costs** (assuming 10,000 kaiju):
- Stable Diffusion: $200-400
- DALL-E 3: $400-800
- Hybrid (90% SD, 10% DALL-E): $220-440

### Storage Costs

| Service | Cost | Purpose |
|---------|------|---------|
| **IPFS (Pinata)** | $20/month (1TB) | Primary storage |
| **Arweave** | ~$5 per GB (one-time) | Permanent backup |
| **CDN (Cloudflare)** | $20-50/month | Fast delivery |

**Estimated Monthly Cost** (10,000 kaiju @ 200KB each = 2GB):
- IPFS: $20/month
- Arweave: $10 one-time
- CDN: $20/month
- **Total**: ~$40/month

---

## SUMMARY

Phase 10 completes the visual layer of Kaiju Breeding Simulator with AI-generated, genetically-driven portraits that create a rich visual history for each kaiju lineage. The deterministic generation ensures consistency while the queue system allows for scalable batch processing.
