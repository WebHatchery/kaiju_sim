//! Transfer service for executing ownership transfers.

use chrono::Utc;
use sqlx::{MySql, MySqlPool, Transaction};
use uuid::Uuid;

use crate::crypto::signer::Signer;
use crate::error::{Result, TransferError};
use crate::types::{Kaiju, OwnershipProof, TransferHistoryRecord, TransferRequest, TransferResult};

/// Service for handling kaiju ownership transfers
pub struct TransferService {
    pool: MySqlPool,
    signer: Signer,
}

impl TransferService {
    /// Create a new transfer service
    pub fn new(pool: MySqlPool, signer: Signer) -> Self {
        Self { pool, signer }
    }

    /// Execute a server-side transfer
    pub async fn execute_transfer(&self, request: TransferRequest) -> Result<TransferResult> {
        // Start transaction
        let mut tx = self.pool.begin().await?;

        // Execute transfer within transaction
        let result = self.execute_transfer_tx(&mut tx, request).await?;

        // Commit transaction
        tx.commit().await?;

        Ok(result)
    }

    /// Execute transfer within existing transaction
    async fn execute_transfer_tx(
        &self,
        tx: &mut Transaction<'_, MySql>,
        request: TransferRequest,
    ) -> Result<TransferResult> {
        let kaiju_id_str = request.kaiju_id.to_string();
        let from_user_id_str = request.from_user_id.to_string();
        let to_user_id_str = request.to_user_id.to_string();

        // 1. Lock and fetch kaiju
        let kaiju = self.lock_kaiju(&mut *tx, &kaiju_id_str).await?;

        // 2. Validate ownership
        if kaiju.owner_user_id != from_user_id_str {
            return Err(TransferError::NotOwner(
                request.from_user_id,
                request.kaiju_id,
            ));
        }

        // 3. Validate custody state
        if kaiju.custody_state == "blockchain" {
            return Err(TransferError::OnChainCustody(request.kaiju_id));
        }

        // 4. Validate kaiju is alive
        if !kaiju.alive {
            return Err(TransferError::KaijuDead(request.kaiju_id));
        }

        // 5. Check for transfer lock
        self.check_transfer_lock(&mut *tx, &kaiju_id_str).await?;

        // 6. Get and increment nonce
        let nonce = self
            .get_and_increment_nonce(&mut *tx, &from_user_id_str)
            .await?;

        // 7. Generate signature
        let timestamp = Utc::now();
        let signature = self.signer.sign_transfer(
            request.kaiju_id,
            request.from_user_id,
            request.to_user_id,
            timestamp,
            nonce,
            &request.transfer_type.to_string(),
        );

        // 8. Update ownership
        self.update_ownership(&mut *tx, &kaiju_id_str, &to_user_id_str)
            .await?;

        // 9. Log transfer
        let history_id = self
            .log_transfer(&mut *tx, &request, nonce, timestamp, &signature.signature)
            .await?;

        Ok(TransferResult {
            kaiju_id: request.kaiju_id,
            from_user_id: request.from_user_id,
            to_user_id: request.to_user_id,
            transfer_type: request.transfer_type,
            timestamp,
            nonce,
            signature: signature.signature,
            history_id,
        })
    }

    /// Lock kaiju for update
    async fn lock_kaiju(&self, tx: &mut Transaction<'_, MySql>, kaiju_id: &str) -> Result<Kaiju> {
        let kaiju: Option<Kaiju> = sqlx::query_as(
            "SELECT id, name, owner_user_id, custody_state, alive,
                    blockchain_token_id, state_hash, state_version
             FROM kaiju
             WHERE id = ?
             FOR UPDATE",
        )
        .bind(kaiju_id)
        .fetch_optional(&mut **tx)
        .await?;

        kaiju.ok_or_else(|| {
            TransferError::KaijuNotFound(Uuid::parse_str(kaiju_id).unwrap_or_default())
        })
    }

    /// Check if kaiju is locked
    async fn check_transfer_lock(
        &self,
        tx: &mut Transaction<'_, MySql>,
        kaiju_id: &str,
    ) -> Result<()> {
        let lock: Option<(String,)> = sqlx::query_as(
            "SELECT lock_reason FROM transfer_locks
             WHERE kaiju_id = ? AND expires_at > NOW()",
        )
        .bind(kaiju_id)
        .fetch_optional(&mut **tx)
        .await?;

        if let Some((reason,)) = lock {
            return Err(TransferError::KaijuLocked(
                Uuid::parse_str(kaiju_id).unwrap_or_default(),
                reason,
            ));
        }

        Ok(())
    }

    /// Get current nonce and increment it
    async fn get_and_increment_nonce(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_id: &str,
    ) -> Result<i64> {
        // First get the current nonce
        let current: Option<(i64,)> =
            sqlx::query_as("SELECT transfer_nonce FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_optional(&mut **tx)
                .await?;

        let nonce = current
            .ok_or_else(|| {
                TransferError::UserNotFound(Uuid::parse_str(user_id).unwrap_or_default())
            })?
            .0;

        // Increment nonce
        sqlx::query("UPDATE users SET transfer_nonce = transfer_nonce + 1 WHERE id = ?")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;

        Ok(nonce)
    }

    /// Update kaiju ownership
    async fn update_ownership(
        &self,
        tx: &mut Transaction<'_, MySql>,
        kaiju_id: &str,
        new_owner_id: &str,
    ) -> Result<()> {
        sqlx::query("UPDATE kaiju SET owner_user_id = ? WHERE id = ?")
            .bind(new_owner_id)
            .bind(kaiju_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    /// Log transfer to history
    async fn log_transfer(
        &self,
        tx: &mut Transaction<'_, MySql>,
        request: &TransferRequest,
        nonce: i64,
        timestamp: chrono::DateTime<Utc>,
        signature: &str,
    ) -> Result<i64> {
        let kaiju_id_str = request.kaiju_id.to_string();
        let from_user_id_str = request.from_user_id.to_string();
        let to_user_id_str = request.to_user_id.to_string();
        let transfer_type_str = request.transfer_type.to_string();
        let metadata_str = request
            .metadata
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "{}".to_string());

        sqlx::query(
            "INSERT INTO ownership_history
             (kaiju_id, from_user_id, to_user_id, transfer_type,
              price_amount, price_currency, timestamp, server_signature,
              nonce, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&kaiju_id_str)
        .bind(&from_user_id_str)
        .bind(&to_user_id_str)
        .bind(&transfer_type_str)
        .bind(request.price_amount)
        .bind(request.price_currency.as_deref())
        .bind(timestamp)
        .bind(signature)
        .bind(nonce)
        .bind(&metadata_str)
        .execute(&mut **tx)
        .await?;

        // Get last insert ID
        let (id,): (i64,) = sqlx::query_as("SELECT LAST_INSERT_ID()")
            .fetch_one(&mut **tx)
            .await?;

        Ok(id)
    }

    /// Generate ownership proof
    pub async fn generate_ownership_proof(&self, kaiju_id: Uuid) -> Result<OwnershipProof> {
        let kaiju_id_str = kaiju_id.to_string();

        let kaiju: Option<Kaiju> = sqlx::query_as(
            "SELECT id, name, owner_user_id, custody_state, alive,
                    blockchain_token_id, state_hash, state_version
             FROM kaiju
             WHERE id = ?",
        )
        .bind(&kaiju_id_str)
        .fetch_optional(&self.pool)
        .await?;

        let kaiju = kaiju.ok_or(TransferError::KaijuNotFound(kaiju_id))?;

        let timestamp = Utc::now();
        let owner_uuid = Uuid::parse_str(&kaiju.owner_user_id)
            .map_err(|e| TransferError::InvalidState(format!("Invalid owner UUID: {}", e)))?;

        let proof = self
            .signer
            .sign_ownership(kaiju_id, owner_uuid, timestamp, kaiju.state_hash);

        Ok(OwnershipProof {
            kaiju_id,
            owner_user_id: owner_uuid,
            timestamp,
            state_hash: proof.state_hash,
            server_signature: proof.signature,
            public_key: proof.public_key,
        })
    }

    /// Get transfer history for a kaiju
    pub async fn get_transfer_history(
        &self,
        kaiju_id: Uuid,
        limit: i64,
    ) -> Result<Vec<TransferHistoryRecord>> {
        let kaiju_id_str = kaiju_id.to_string();

        let records: Vec<TransferHistoryRecord> = sqlx::query_as(
            "SELECT id, kaiju_id, from_user_id, to_user_id, transfer_type,
                    price_amount, price_currency, timestamp, server_signature, nonce
             FROM ownership_history
             WHERE kaiju_id = ?
             ORDER BY timestamp DESC
             LIMIT ?",
        )
        .bind(&kaiju_id_str)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}
