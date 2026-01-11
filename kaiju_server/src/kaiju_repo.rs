//! Kaiju Repository - Database operations for Kaiju entities

use sqlx::mysql::MySqlPool;
use uuid::Uuid;
use serde_json;

use crate::breeding_service::{KaijuData, KaijuStats, Trait, TraitInheritance};

/// Kaiju repository for database operations
pub struct KaijuRepository {
    pool: MySqlPool,
}

impl KaijuRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Insert a new Kaiju into the database
    pub async fn insert(&self, kaiju: &KaijuData) -> Result<(), sqlx::Error> {
        let id = kaiju.id.to_string();
        let owner_id = kaiju.owner_id.to_string();
        let parent_a_id = kaiju.parent_ids.as_ref().map(|(a, _)| a.to_string());
        let parent_b_id = kaiju.parent_ids.as_ref().map(|(_, b)| b.to_string());
        
        let base_stats = serde_json::json!({
            "hp": kaiju.stats.hp,
            "attack": kaiju.stats.attack,
            "defense": kaiju.stats.defense,
            "speed": kaiju.stats.speed,
            "energy": kaiju.stats.energy,
        });
        
        let visible_traits = serde_json::to_string(&kaiju.traits).unwrap_or_else(|_| "[]".to_string());
        
        // Generate state hash
        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(format!("{}{}", kaiju.genome_hash, kaiju.visual_seed).as_bytes());
        let state_hash = format!("{:x}", hasher.finalize());

        sqlx::query(r#"
            INSERT INTO kaiju (
                id, name, generation, owner_user_id, custody_state,
                parent_a_id, parent_b_id, genome_hash, genome_data, visual_seed,
                base_stats, current_stats, visible_traits, hidden_traits,
                state_hash, image_url, tournaments_won
            ) VALUES (
                ?, ?, ?, ?, 'server',
                ?, ?, ?, '', ?,
                ?, ?, ?, '[]',
                ?, ?, ?
            )
        "#)
        .bind(&id)
        .bind(&kaiju.name)
        .bind(kaiju.generation as i32)
        .bind(&owner_id)
        .bind(parent_a_id)
        .bind(parent_b_id)
        .bind(&kaiju.genome_hash)
        .bind(kaiju.visual_seed.to_string())
        .bind(base_stats.to_string())
        .bind(base_stats.to_string()) // current_stats same as base initially
        .bind(&visible_traits)
        .bind(&state_hash)
        .bind(&kaiju.image_url) // Bind image_url
        .bind(kaiju.tournaments_won)
        .execute(&self.pool)
        .await?;

        tracing::info!("Saved Kaiju {} ({}) to database", kaiju.name, id);
        Ok(())
    }

    /// Get a Kaiju by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<KaijuData>, sqlx::Error> {
        let id_str = id.to_string();
        
        let row: Option<KaijuDbRow> = sqlx::query_as(r#"
            SELECT id, name, generation, owner_user_id, 
                   parent_a_id, parent_b_id, genome_hash, visual_seed,
                   base_stats, visible_traits, image_url, tournaments_won
            FROM kaiju 
            WHERE id = ? AND deleted_at IS NULL AND alive = TRUE
        "#)
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into_kaiju_data()))
    }

    /// Get all Kaiju for a user
    pub async fn get_by_owner(&self, owner_id: Uuid) -> Result<Vec<KaijuData>, sqlx::Error> {
        let owner_str = owner_id.to_string();
        
        let rows: Vec<KaijuDbRow> = sqlx::query_as(r#"
            SELECT id, name, generation, owner_user_id, 
                   parent_a_id, parent_b_id, genome_hash, visual_seed,
                   base_stats, visible_traits, image_url, tournaments_won
            FROM kaiju 
            WHERE owner_user_id = ? AND deleted_at IS NULL AND alive = TRUE
            ORDER BY created_at DESC
        "#)
        .bind(&owner_str)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_kaiju_data()).collect())
    }
}

/// Database row representation
#[derive(sqlx::FromRow)]
struct KaijuDbRow {
    id: String,
    name: String,
    generation: i32,
    owner_user_id: String,
    parent_a_id: Option<String>,
    parent_b_id: Option<String>,
    genome_hash: String,
    visual_seed: String,
    base_stats: sqlx::types::Json<KaijuStats>,
    visible_traits: sqlx::types::Json<Vec<Trait>>,
    image_url: String,
    tournaments_won: i32,
}

impl KaijuDbRow {
    fn into_kaiju_data(self) -> KaijuData {
        let id = Uuid::parse_str(&self.id).unwrap_or_default();
        let owner_id = Uuid::parse_str(&self.owner_user_id).unwrap_or_default();
        
        let parent_ids = match (&self.parent_a_id, &self.parent_b_id) {
            (Some(a), Some(b)) => {
                let a_uuid = Uuid::parse_str(a).unwrap_or_default();
                let b_uuid = Uuid::parse_str(b).unwrap_or_default();
                Some((a_uuid, b_uuid))
            }
            _ => None,
        };
        
        let visual_seed: u64 = self.visual_seed.parse().unwrap_or(0);

        KaijuData {
            id,
            name: self.name,
            generation: self.generation as u32,
            parent_ids,
            visual_seed,
            genome_hash: self.genome_hash,
            stats: self.base_stats.0,
            traits: self.visible_traits.0,
            owner_id,
            image_url: self.image_url,
            tournaments_won: self.tournaments_won,
        }
    }
}
