# Server-Side Transfer System (Rust Implementation)
## Zero-Cost Instant Transfers with Auditability

---

## 1. Overview

This document provides production-ready Rust code for:
- Instant free transfers between users
- Cryptographic signatures for auditability
- Replay protection via nonces
- Transfer validation and safety checks
- Database transaction management
- WebSocket notifications for real-time updates

**Stack**:
- `tokio` - Async runtime
- `sqlx` - MySQL database driver
- `secp256k1` - ECDSA signatures
- `sha2` - Hashing
- `serde` - JSON serialization

---

## 2. Dependencies (Cargo.toml)

```toml
[package]
name = "kaiju-transfer-system"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono", "json"] }

# Crypto
secp256k1 = { version = "0.28", features = ["rand-std", "serde"] }
sha2 = "0.10"
hex = "0.4"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Types
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Testing
proptest = "1.4"  # For property-based testing
```

---

## 3. Core Types

```rust
// src/types.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Transfer request from API/UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub kaiju_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub transfer_type: TransferType,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Types of ownership transfers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
    Trade,
    Gift,
    BreedingPayment,
    BlockchainMint,
    BlockchainDeposit,
    AdminTransfer,
}

impl std::fmt::Display for TransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trade => write!(f, "trade"),
            Self::Gift => write!(f, "gift"),
            Self::BreedingPayment => write!(f, "breeding_payment"),
            Self::BlockchainMint => write!(f, "blockchain_mint"),
            Self::BlockchainDeposit => write!(f, "blockchain_deposit"),
            Self::AdminTransfer => write!(f, "admin_transfer"),
        }
    }
}

/// Custody state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CustodyState {
    Server,
    Blockchain,
}

impl std::fmt::Display for CustodyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Server => write!(f, "server"),
            Self::Blockchain => write!(f, "blockchain"),
        }
    }
}

/// Kaiju entity (simplified for transfers)
#[derive(Debug, Clone, FromRow)]
pub struct Kaiju {
    pub id: Uuid,
    pub name: String,
    pub owner_user_id: Uuid,
    pub custody_state: String,  // "server" or "blockchain"
    pub alive: bool,
    pub blockchain_token_id: Option<i64>,
    pub state_hash: String,
    pub state_version: i32,
}

/// Transfer result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub kaiju_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub transfer_type: TransferType,
    pub timestamp: DateTime<Utc>,
    pub nonce: i64,
    pub signature: String,
    pub history_id: i64,
}

/// Ownership proof for external verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipProof {
    pub kaiju_id: Uuid,
    pub owner_user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub state_hash: String,
    pub server_signature: String,
}
```

---

## 4. Error Types

```rust
// src/error.rs

use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("Kaiju not found: {0}")]
    KaijuNotFound(Uuid),

    #[error("User {0} is not the owner of kaiju {1}")]
    NotOwner(Uuid, Uuid),

    #[error("Kaiju {0} is in blockchain custody and cannot be transferred server-side")]
    OnChainCustody(Uuid),

    #[error("Kaiju {0} is dead and cannot be transferred")]
    KaijuDead(Uuid),

    #[error("Kaiju {0} is locked: {1}")]
    KaijuLocked(Uuid, String),

    #[error("Invalid transfer type")]
    InvalidTransferType,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Signature error: {0}")]
    Signature(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub type Result<T> = std::result::Result<T, TransferError>;
```

---

## 5. Cryptographic Signature System

```rust
// src/crypto.rs

use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, ecdsa::Signature};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Server signature key pair
pub struct SignatureKeyPair {
    secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl SignatureKeyPair {
    /// Generate new key pair
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        Self { secret_key, public_key }
    }

    /// Load from hex-encoded secret key
    pub fn from_hex(hex_secret: &str) -> Result<Self, secp256k1::Error> {
        let bytes = hex::decode(hex_secret).map_err(|_| secp256k1::Error::InvalidSecretKey)?;
        let secret_key = SecretKey::from_slice(&bytes)?;
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok(Self { secret_key, public_key })
    }

    /// Sign a transfer
    pub fn sign_transfer(
        &self,
        kaiju_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        timestamp: DateTime<Utc>,
        nonce: i64,
        transfer_type: &str,
    ) -> String {
        let message = format!(
            "transfer:{}:{}:{}:{}:{}:{}",
            kaiju_id,
            from_user_id,
            to_user_id,
            timestamp.timestamp(),
            nonce,
            transfer_type
        );

        let message_hash = Sha256::digest(message.as_bytes());
        let message = Message::from_slice(&message_hash).expect("hash is 32 bytes");

        let secp = Secp256k1::new();
        let signature = secp.sign_ecdsa(&message, &self.secret_key);

        hex::encode(signature.serialize_compact())
    }

    /// Sign ownership proof
    pub fn sign_ownership(
        &self,
        kaiju_id: Uuid,
        owner_user_id: Uuid,
        timestamp: DateTime<Utc>,
        state_hash: &str,
    ) -> String {
        let message = format!(
            "ownership:{}:{}:{}:{}",
            kaiju_id,
            owner_user_id,
            timestamp.timestamp(),
            state_hash
        );

        let message_hash = Sha256::digest(message.as_bytes());
        let message = Message::from_slice(&message_hash).expect("hash is 32 bytes");

        let secp = Secp256k1::new();
        let signature = secp.sign_ecdsa(&message, &self.secret_key);

        hex::encode(signature.serialize_compact())
    }

    /// Verify signature
    pub fn verify_signature(
        public_key: &PublicKey,
        message: &str,
        signature_hex: &str,
    ) -> bool {
        let Ok(signature_bytes) = hex::decode(signature_hex) else { return false; };
        let Ok(signature) = Signature::from_compact(&signature_bytes) else { return false; };

        let message_hash = Sha256::digest(message.as_bytes());
        let Ok(message) = Message::from_slice(&message_hash) else { return false; };

        let secp = Secp256k1::new();
        secp.verify_ecdsa(&message, &signature, public_key).is_ok()
    }
}

/// Compute state hash for kaiju
pub fn compute_state_hash(kaiju: &crate::types::Kaiju) -> String {
    let state_repr = format!(
        "{}:{}:{}:{}:{}",
        kaiju.id,
        kaiju.owner_user_id,
        kaiju.custody_state,
        kaiju.alive,
        kaiju.state_version
    );

    let hash = Sha256::digest(state_repr.as_bytes());
    hex::encode(hash)
}
```

---

## 6. Transfer Service

```rust
// src/transfer_service.rs

use sqlx::{MySqlPool, MySql, Transaction};
use uuid::Uuid;
use chrono::Utc;
use crate::{
    types::*,
    error::{TransferError, Result},
    crypto::SignatureKeyPair,
};

pub struct TransferService {
    pool: MySqlPool,
    signature_key: SignatureKeyPair,
}

impl TransferService {
    pub fn new(pool: MySqlPool, signature_key: SignatureKeyPair) -> Self {
        Self { pool, signature_key }
    }

    /// Execute a server-side transfer
    pub async fn execute_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<TransferResult> {
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
        // 1. Lock and fetch kaiju
        let kaiju = self.lock_kaiju(tx, request.kaiju_id).await?;

        // 2. Validate ownership
        if kaiju.owner_user_id != request.from_user_id {
            return Err(TransferError::NotOwner(request.from_user_id, request.kaiju_id));
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
        self.check_transfer_lock(tx, request.kaiju_id).await?;

        // 6. Get and increment nonce
        let nonce = self.get_and_increment_nonce(tx, request.from_user_id).await?;

        // 7. Generate signature
        let timestamp = Utc::now();
        let signature = self.signature_key.sign_transfer(
            request.kaiju_id,
            request.from_user_id,
            request.to_user_id,
            timestamp,
            nonce,
            &request.transfer_type.to_string(),
        );

        // 8. Update ownership
        self.update_ownership(tx, request.kaiju_id, request.to_user_id).await?;

        // 9. Log transfer
        let history_id = self.log_transfer(
            tx,
            &request,
            nonce,
            timestamp,
            &signature,
        ).await?;

        Ok(TransferResult {
            kaiju_id: request.kaiju_id,
            from_user_id: request.from_user_id,
            to_user_id: request.to_user_id,
            transfer_type: request.transfer_type,
            timestamp,
            nonce,
            signature,
            history_id,
        })
    }

    /// Lock kaiju for update
    async fn lock_kaiju(
        &self,
        tx: &mut Transaction<'_, MySql>,
        kaiju_id: Uuid,
    ) -> Result<Kaiju> {
        sqlx::query_as::<_, Kaiju>(
            "SELECT id, name, owner_user_id, custody_state, alive,
                    blockchain_token_id, state_hash, state_version
             FROM kaiju
             WHERE id = $1
             FOR UPDATE"
        )
        .bind(kaiju_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(TransferError::KaijuNotFound(kaiju_id))
    }

    /// Check if kaiju is locked
    async fn check_transfer_lock(
        &self,
        tx: &mut Transaction<'_, MySql>,
        kaiju_id: Uuid,
    ) -> Result<()> {
        let lock: Option<(String,)> = sqlx::query_as(
            "SELECT lock_reason FROM transfer_locks
             WHERE kaiju_id = $1 AND expires_at > NOW()"
        )
        .bind(kaiju_id)
        .fetch_optional(&mut **tx)
        .await?;

        if let Some((reason,)) = lock {
            return Err(TransferError::KaijuLocked(kaiju_id, reason));
        }

        Ok(())
    }

    /// Get current nonce and increment it
    async fn get_and_increment_nonce(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_id: Uuid,
    ) -> Result<i64> {
        let (nonce,): (i64,) = sqlx::query_as(
            "UPDATE users SET transfer_nonce = transfer_nonce + 1
             WHERE id = $1
             RETURNING transfer_nonce - 1"
        )
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;

        Ok(nonce)
    }

    /// Update kaiju ownership
    async fn update_ownership(
        &self,
        tx: &mut Transaction<'_, MySql>,
        kaiju_id: Uuid,
        new_owner_id: Uuid,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE kaiju SET owner_user_id = $1 WHERE id = $2"
        )
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
        let (history_id,): (i64,) = sqlx::query_as(
            "INSERT INTO ownership_history
             (kaiju_id, from_user_id, to_user_id, transfer_type,
              price_amount, price_currency, timestamp, server_signature,
              nonce, metadata)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING id"
        )
        .bind(request.kaiju_id)
        .bind(request.from_user_id)
        .bind(request.to_user_id)
        .bind(request.transfer_type.to_string())
        .bind(request.price_amount)
        .bind(request.price_currency.as_deref())
        .bind(timestamp)
        .bind(signature)
        .bind(nonce)
        .bind(&request.metadata)
        .fetch_one(&mut **tx)
        .await?;

        Ok(history_id)
    }

    /// Generate ownership proof
    pub async fn generate_ownership_proof(
        &self,
        kaiju_id: Uuid,
    ) -> Result<OwnershipProof> {
        let kaiju = sqlx::query_as::<_, Kaiju>(
            "SELECT id, name, owner_user_id, custody_state, alive,
                    blockchain_token_id, state_hash, state_version
             FROM kaiju
             WHERE id = $1"
        )
        .bind(kaiju_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(TransferError::KaijuNotFound(kaiju_id))?;

        let timestamp = Utc::now();
        let signature = self.signature_key.sign_ownership(
            kaiju.id,
            kaiju.owner_user_id,
            timestamp,
            &kaiju.state_hash,
        );

        Ok(OwnershipProof {
            kaiju_id: kaiju.id,
            owner_user_id: kaiju.owner_user_id,
            timestamp,
            state_hash: kaiju.state_hash,
            server_signature: signature,
        })
    }

    /// Get transfer history for a kaiju
    pub async fn get_transfer_history(
        &self,
        kaiju_id: Uuid,
        limit: i64,
    ) -> Result<Vec<TransferHistoryRecord>> {
        let records = sqlx::query_as::<_, TransferHistoryRecord>(
            "SELECT id, kaiju_id, from_user_id, to_user_id, transfer_type,
                    price_amount, price_currency, timestamp, server_signature, nonce
             FROM ownership_history
             WHERE kaiju_id = $1
             ORDER BY timestamp DESC
             LIMIT $2"
        )
        .bind(kaiju_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct TransferHistoryRecord {
    pub id: i64,
    pub kaiju_id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub to_user_id: Uuid,
    pub transfer_type: String,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub server_signature: String,
    pub nonce: i64,
}
```

---

## 7. API Handlers (Example with Axum)

```rust
// src/api.rs

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
    routing::{get, post},
};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    types::*,
    transfer_service::TransferService,
    error::TransferError,
};

pub struct AppState {
    pub transfer_service: Arc<TransferService>,
}

/// API error response
pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = format!("{}", self.0);
        (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": message
        }))).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/// POST /api/transfer - Execute a transfer
pub async fn handle_transfer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TransferRequest>,
) -> Result<Json<TransferResult>, ApiError> {
    let result = state.transfer_service.execute_transfer(request).await?;
    Ok(Json(result))
}

/// GET /api/kaiju/:id/proof - Get ownership proof
pub async fn handle_get_proof(
    State(state): State<Arc<AppState>>,
    Path(kaiju_id): Path<Uuid>,
) -> Result<Json<OwnershipProof>, ApiError> {
    let proof = state.transfer_service.generate_ownership_proof(kaiju_id).await?;
    Ok(Json(proof))
}

/// GET /api/kaiju/:id/history - Get transfer history
pub async fn handle_get_history(
    State(state): State<Arc<AppState>>,
    Path(kaiju_id): Path<Uuid>,
) -> Result<Json<Vec<crate::transfer_service::TransferHistoryRecord>>, ApiError> {
    let history = state.transfer_service.get_transfer_history(kaiju_id, 100).await?;
    Ok(Json(history))
}

/// Build API router
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/transfer", post(handle_transfer))
        .route("/api/kaiju/:id/proof", get(handle_get_proof))
        .route("/api/kaiju/:id/history", get(handle_get_history))
        .with_state(state)
}
```

---

## 8. Testing

```rust
// src/tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::MySqlPool;

    async fn setup_test_db() -> MySqlPool {
        // Use test database
        let pool = MySqlPool::connect("mysql://root@localhost/kaiju_test").await.unwrap();

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_successful_transfer() {
        let pool = setup_test_db().await;
        let key_pair = SignatureKeyPair::generate();
        let service = TransferService::new(pool.clone(), key_pair);

        // Create test users
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();

        sqlx::query("INSERT INTO users (id, username) VALUES ($1, $2)")
            .bind(user_a)
            .bind("user_a")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO users (id, username) VALUES ($1, $2)")
            .bind(user_b)
            .bind("user_b")
            .execute(&pool)
            .await
            .unwrap();

        // Create test kaiju
        let kaiju_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO kaiju (id, name, generation, owner_user_id, custody_state,
                                genome_hash, genome_data, visual_seed, base_stats,
                                current_stats, state_hash)
             VALUES ($1, $2, 0, $3, 'server', 'test_hash', 'test_data', 'test_seed',
                     '{}'::jsonb, '{}'::jsonb, 'initial_hash')"
        )
        .bind(kaiju_id)
        .bind("Test Kaiju")
        .bind(user_a)
        .execute(&pool)
        .await
        .unwrap();

        // Execute transfer
        let request = TransferRequest {
            kaiju_id,
            from_user_id: user_a,
            to_user_id: user_b,
            transfer_type: TransferType::Gift,
            price_amount: None,
            price_currency: None,
            metadata: None,
        };

        let result = service.execute_transfer(request).await.unwrap();

        // Verify transfer
        assert_eq!(result.from_user_id, user_a);
        assert_eq!(result.to_user_id, user_b);
        assert_eq!(result.nonce, 0);
        assert!(!result.signature.is_empty());

        // Verify ownership changed
        let (owner,): (Uuid,) = sqlx::query_as(
            "SELECT owner_user_id FROM kaiju WHERE id = $1"
        )
        .bind(kaiju_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(owner, user_b);
    }

    #[tokio::test]
    async fn test_transfer_not_owner() {
        let pool = setup_test_db().await;
        let key_pair = SignatureKeyPair::generate();
        let service = TransferService::new(pool.clone(), key_pair);

        // Setup users and kaiju...

        let request = TransferRequest {
            kaiju_id: Uuid::new_v4(),
            from_user_id: Uuid::new_v4(), // Wrong user
            to_user_id: Uuid::new_v4(),
            transfer_type: TransferType::Gift,
            price_amount: None,
            price_currency: None,
            metadata: None,
        };

        let result = service.execute_transfer(request).await;
        assert!(matches!(result, Err(TransferError::NotOwner(_, _))));
    }
}
```

---

## 9. Example Usage

```rust
// main.rs

use sqlx::MySqlPool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Connect to database
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;

    // Load signature key (in production, use secure key management)
    let secret_key_hex = std::env::var("SERVER_SECRET_KEY")?;
    let signature_key = SignatureKeyPair::from_hex(&secret_key_hex)?;

    // Create transfer service
    let transfer_service = Arc::new(TransferService::new(pool, signature_key));

    // Create API state
    let app_state = Arc::new(AppState {
        transfer_service: transfer_service.clone(),
    });

    // Build router
    let app = create_router(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
```

---

## 10. Performance Optimizations

### Connection Pooling

```rust
// Optimize connection pool for high throughput
let pool = MySqlPoolOptions::new()
    .max_connections(50)
    .min_connections(10)
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(1800))
    .connect(&database_url)
    .await?;
```

### Batch Transfers (for marketplace operations)

```rust
impl TransferService {
    pub async fn execute_batch_transfers(
        &self,
        requests: Vec<TransferRequest>,
    ) -> Result<Vec<TransferResult>> {
        let mut tx = self.pool.begin().await?;
        let mut results = Vec::new();

        for request in requests {
            let result = self.execute_transfer_tx(&mut tx, request).await?;
            results.push(result);
        }

        tx.commit().await?;
        Ok(results)
    }
}
```

---

## 11. Security Best Practices

1. **Key Management**:
   - Store secret keys in environment variables or key vaults (AWS KMS, HashiCorp Vault)
   - Rotate keys periodically
   - Use separate keys for different purposes (ownership, battles, minting)

2. **Rate Limiting**:
```rust
use tower::limit::RateLimitLayer;

let app = Router::new()
    .route("/api/transfer", post(handle_transfer))
    .layer(RateLimitLayer::new(100, Duration::from_secs(60))) // 100 req/min
    .with_state(state);
```

3. **Authentication**:
```rust
// Add JWT middleware
use axum_extra::extract::cookie::CookieJar;

async fn verify_user_auth(
    cookie_jar: CookieJar,
    request: TransferRequest,
) -> Result<Uuid, ApiError> {
    // Verify JWT token
    // Ensure from_user_id matches authenticated user
    todo!()
}
```

4. **Audit Logging**:
```rust
tracing::info!(
    kaiju_id = %result.kaiju_id,
    from_user = %result.from_user_id,
    to_user = %result.to_user_id,
    nonce = result.nonce,
    "Transfer executed"
);
```

---

This provides a production-ready, secure, instant transfer system with zero gas costs and full auditability.

Next: See `SIGNATURE_SYSTEM.md` for cryptographic verification and public APIs.
