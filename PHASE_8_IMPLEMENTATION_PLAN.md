# PHASE 8 IMPLEMENTATION PLAN: Economy & Multiplayer Hooks

Based on my analysis of the Kaiju Breeding Simulator codebase, I've created a detailed implementation plan for Phase 8 (Economy & Multiplayer Hooks). This phase prepares the economy and multiplayer systems while maintaining the server-custodial model established in Phase 0.

---

## Overview

**Phase 8 Goal**: Implement the breeding rights economy, currency system, and multiplayer preparation hooks to enable trading, validation, and future expansion without blockchain dependency.

**Dependencies**:
- Phase 0 (Server Infrastructure & Database) - REQUIRED
- Phase 1 (Foundation & Data Models) - REQUIRED
- Phase 2 (Genetics & Breeding Engine) - REQUIRED
- Phase 4 (Tournament System) - REQUIRED
- Phase 5 (State Management) - REQUIRED

**Duration Estimate**: 1-2 weeks

**Key Philosophy**: Economy and multiplayer work entirely server-side with cryptographic proofs. Blockchain export (Phase 9) remains optional.

---

## 1. Breeding Rights Economy Implementation

### 1.1 Breeding Rights Core System

**File**: `kaiju_server\src\breeding_rights_service.rs`

The breeding rights system is already defined in DATABASE_SCHEMA.md. Implementation tasks:

#### Data Structures
```rust
pub struct BreedingRight {
    pub id: Uuid,
    pub kaiju_id: Uuid,
    pub owner_user_id: Uuid,
    pub issuer_user_id: Uuid,
    pub uses_total: i32,
    pub uses_remaining: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub purchase_price: Option<i64>,
    pub purchase_currency: Option<String>,
    pub refund_eligible: bool,
    pub active: bool,
    pub revoked: bool,
    pub refunded: bool,
}

pub struct BreedingRightOffer {
    pub kaiju_id: Uuid,
    pub price: i64,
    pub currency: CurrencyType,
    pub uses: i32,
    pub duration_days: Option<i32>,
}
```

#### Core Operations

**Create Breeding Right**:
```rust
pub async fn create_breeding_right(
    pool: &PgPool,
    kaiju_id: Uuid,
    issuer_user_id: Uuid,
    offer: BreedingRightOffer,
) -> Result<BreedingRight, BreedingRightError> {
    // 1. Validate kaiju ownership
    // 2. Check kaiju is alive
    // 3. Verify issuer owns the kaiju
    // 4. Create breeding right in DB
    // 5. Return breeding right with signature proof
}
```

**Purchase Breeding Right**:
```rust
pub async fn purchase_breeding_right(
    pool: &PgPool,
    right_id: Uuid,
    buyer_user_id: Uuid,
    payment: PaymentProof,
) -> Result<TransferReceipt, BreedingRightError> {
    // 1. Validate right exists and is active
    // 2. Verify buyer has sufficient currency
    // 3. Deduct currency from buyer
    // 4. Credit currency to issuer
    // 5. Transfer breeding right ownership
    // 6. Log transaction with signature
}
```

**Consume Breeding Right**:
```rust
pub async fn consume_breeding_right(
    pool: &PgPool,
    right_id: Uuid,
    breeder_user_id: Uuid,
) -> Result<(), BreedingRightError> {
    // 1. Validate ownership
    // 2. Check uses_remaining > 0
    // 3. Decrement uses_remaining
    // 4. Set used_at timestamp
    // 5. If uses_remaining == 0, set active = false
}
```

### 1.2 Breeding Rights Validation

**File**: `kaiju_server\src\breeding_rights_validator.rs`

```rust
pub struct BreedingRightValidator;

impl BreedingRightValidator {
    pub async fn validate_for_breeding(
        pool: &PgPool,
        right_id: Uuid,
        parent_kaiju_id: Uuid,
        breeder_user_id: Uuid,
    ) -> Result<(), ValidationError> {
        // 1. Verify right exists
        // 2. Verify right is active
        // 3. Verify uses_remaining > 0
        // 4. Verify right.kaiju_id matches parent_kaiju_id
        // 5. Verify right.owner_user_id matches breeder_user_id
        // 6. Check expiry date if set
        // 7. Verify parent kaiju is alive
    }

    pub async fn validate_for_purchase(
        pool: &PgPool,
        right_id: Uuid,
    ) -> Result<BreedingRight, ValidationError> {
        // 1. Verify right exists
        // 2. Verify right is for sale (owner != issuer)
        // 3. Verify kaiju is alive
        // 4. Verify not expired
        // 5. Verify uses_remaining > 0
    }
}
```

---

## 2. Rights Purchase/Sale Interface

### 2.1 Marketplace Service

**File**: `kaiju_server\src\marketplace_service.rs`

```rust
pub struct MarketplaceListing {
    pub id: Uuid,
    pub listing_type: ListingType,  // BreedingRight, Kaiju
    pub asset_id: Uuid,             // right_id or kaiju_id
    pub seller_user_id: Uuid,
    pub price: i64,
    pub currency: CurrencyType,
    pub listed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub active: bool,
}

pub enum ListingType {
    BreedingRight,
    Kaiju,  // Future: direct kaiju sales
}

pub struct MarketplaceService {
    pool: PgPool,
}

impl MarketplaceService {
    pub async fn list_breeding_right(
        &self,
        right_id: Uuid,
        seller_user_id: Uuid,
        price: i64,
        currency: CurrencyType,
        duration_hours: Option<i32>,
    ) -> Result<MarketplaceListing, MarketplaceError> {
        // 1. Validate seller owns right
        // 2. Validate right is not already listed
        // 3. Create marketplace listing
        // 4. Lock right from other operations
        // 5. Return listing with signature proof
    }

    pub async fn purchase_listing(
        &self,
        listing_id: Uuid,
        buyer_user_id: Uuid,
    ) -> Result<PurchaseReceipt, MarketplaceError> {
        // 1. Validate listing is active
        // 2. Verify buyer != seller
        // 3. Check buyer has sufficient funds
        // 4. Execute atomic transaction:
        //    - Deduct buyer currency
        //    - Credit seller currency (with marketplace fee)
        //    - Transfer breeding right ownership
        //    - Mark listing as sold
        //    - Log transaction
        // 5. Generate signed receipt
    }

    pub async fn cancel_listing(
        &self,
        listing_id: Uuid,
        seller_user_id: Uuid,
    ) -> Result<(), MarketplaceError> {
        // 1. Validate seller owns listing
        // 2. Mark listing inactive
        // 3. Unlock breeding right
    }

    pub async fn get_active_listings(
        &self,
        filters: MarketplaceFilters,
        pagination: Pagination,
    ) -> Result<Vec<MarketplaceListing>, MarketplaceError> {
        // Query marketplace with filters:
        // - Listing type
        // - Price range
        // - Currency
        // - Kaiju generation
        // - Trait filters
    }
}
```

### 2.2 API Endpoints

**File**: `kaiju_server\src\api\marketplace.rs`

```rust
// POST /api/marketplace/list-breeding-right
pub async fn list_breeding_right(
    State(state): State<AppState>,
    Json(request): Json<ListBreedingRightRequest>,
) -> Result<Json<MarketplaceListing>, ApiError>

// GET /api/marketplace/listings
pub async fn get_listings(
    State(state): State<AppState>,
    Query(filters): Query<MarketplaceFilters>,
) -> Result<Json<Vec<MarketplaceListing>>, ApiError>

// POST /api/marketplace/purchase/:listing_id
pub async fn purchase_listing(
    State(state): State<AppState>,
    Path(listing_id): Path<Uuid>,
    Json(request): Json<PurchaseRequest>,
) -> Result<Json<PurchaseReceipt>, ApiError>

// DELETE /api/marketplace/listing/:listing_id
pub async fn cancel_listing(
    State(state): State<AppState>,
    Path(listing_id): Path<Uuid>,
) -> Result<StatusCode, ApiError>

// GET /api/breeding-rights/:kaiju_id/available
pub async fn get_available_breeding_rights(
    State(state): State<AppState>,
    Path(kaiju_id): Path<Uuid>,
) -> Result<Json<Vec<BreedingRight>>, ApiError>
```

---

## 3. Rights Tracking and Validation

### 3.1 Breeding Rights Tracker

**File**: `kaiju_server\src\breeding_rights_tracker.rs`

```rust
pub struct BreedingRightsTracker {
    pool: PgPool,
}

impl BreedingRightsTracker {
    pub async fn get_user_rights(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<BreedingRight>, TrackerError> {
        // Query all active breeding rights owned by user
    }

    pub async fn get_kaiju_issued_rights(
        &self,
        kaiju_id: Uuid,
    ) -> Result<Vec<BreedingRight>, TrackerError> {
        // Query all breeding rights issued for a kaiju
    }

    pub async fn get_rights_usage_stats(
        &self,
        right_id: Uuid,
    ) -> Result<RightUsageStats, TrackerError> {
        // Return usage statistics:
        // - Total uses
        // - Remaining uses
        // - Successful breedings
        // - Failed attempts
    }

    pub async fn track_expiring_rights(
        &self,
        within_hours: i32,
    ) -> Result<Vec<BreedingRight>, TrackerError> {
        // Find rights expiring soon for notifications
    }
}

pub struct RightUsageStats {
    pub right_id: Uuid,
    pub uses_total: i32,
    pub uses_remaining: i32,
    pub successful_breeds: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub time_remaining_hours: Option<i32>,
}
```

---

## 4. Refund System on Death

### 4.1 Automatic Death Refunds

**File**: `kaiju_server\src\death_refund_service.rs`

The database trigger already handles refunds (DATABASE_SCHEMA.md), but we need the service layer:

```rust
pub struct DeathRefundService {
    pool: PgPool,
    currency_service: Arc<CurrencyService>,
}

impl DeathRefundService {
    pub async fn process_death_refunds(
        &self,
        kaiju_id: Uuid,
    ) -> Result<RefundSummary, RefundError> {
        // 1. Query all unused breeding rights for kaiju
        let unused_rights = sqlx::query_as!(
            BreedingRight,
            r#"
            SELECT * FROM breeding_rights
            WHERE kaiju_id = $1
              AND uses_remaining > 0
              AND refund_eligible = TRUE
              AND active = TRUE
            "#,
            kaiju_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut refunds = Vec::new();

        // 2. Process each refund
        for right in unused_rights {
            if let Some(price) = right.purchase_price {
                // Refund full purchase price
                let refund = self.currency_service
                    .credit_user(
                        right.owner_user_id,
                        price,
                        right.purchase_currency.as_deref().unwrap_or("gold"),
                        "Breeding right refund: kaiju death",
                    )
                    .await?;

                refunds.push(refund);
            }

            // Mark right as refunded
            sqlx::query!(
                r#"
                UPDATE breeding_rights
                SET active = FALSE, refunded = TRUE
                WHERE id = $1
                "#,
                right.id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(RefundSummary {
            kaiju_id,
            total_refunds: refunds.len() as i32,
            total_amount: refunds.iter().map(|r| r.amount).sum(),
            refunds,
        })
    }

    pub async fn validate_refund_eligibility(
        &self,
        right_id: Uuid,
    ) -> Result<bool, RefundError> {
        // Check if right is eligible for refund
    }
}

pub struct RefundSummary {
    pub kaiju_id: Uuid,
    pub total_refunds: i32,
    pub total_amount: i64,
    pub refunds: Vec<CurrencyTransaction>,
}
```

---

## 5. Currency System

### 5.1 Currency Types and Management

**File**: `kaiju_server\src\currency_service.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrencyType {
    Gold,       // Primary in-game currency
    Gems,       // Premium currency (future)
    Credits,    // Tournament rewards
}

pub struct CurrencyBalance {
    pub user_id: Uuid,
    pub gold: i64,
    pub gems: i64,
    pub credits: i64,
    pub updated_at: DateTime<Utc>,
}

pub struct CurrencyService {
    pool: PgPool,
    signature_service: Arc<SignatureService>,
}

impl CurrencyService {
    pub async fn get_balance(
        &self,
        user_id: Uuid,
    ) -> Result<CurrencyBalance, CurrencyError>

    pub async fn credit_user(
        &self,
        user_id: Uuid,
        amount: i64,
        currency: &str,
        reason: &str,
    ) -> Result<CurrencyTransaction, CurrencyError>

    pub async fn debit_user(
        &self,
        user_id: Uuid,
        amount: i64,
        currency: &str,
        reason: &str,
    ) -> Result<CurrencyTransaction, CurrencyError>

    pub async fn transfer_between_users(
        &self,
        from_user_id: Uuid,
        to_user_id: Uuid,
        amount: i64,
        currency: &str,
        reason: &str,
    ) -> Result<TransferReceipt, CurrencyError>
}

pub struct CurrencyTransaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub currency: String,
    pub balance_before: i64,
    pub balance_after: i64,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub signature: String,  // Server signature for proof
}

pub enum TransactionType {
    Credit,
    Debit,
    Transfer,
    Refund,
    Reward,
    Purchase,
}
```

---

## 6. Multiplayer Preparation (Export/Import)

### 6.1 Kaiju Export System

**File**: `kaiju_server\src\export_service.rs`

```rust
pub struct KaijuExportData {
    pub kaiju_id: Uuid,
    pub name: String,
    pub generation: i32,
    pub genome_hash: String,
    pub genome_data: Vec<u8>,
    pub visual_seed: String,
    pub stats: serde_json::Value,
    pub visible_traits: Vec<String>,
    pub experience_level: i32,
    pub created_at: DateTime<Utc>,
    pub parent_ids: Option<(Uuid, Uuid)>,
    pub owner_user_id: Uuid,
    pub alive: bool,

    // Cryptographic proof
    pub state_hash: String,
    pub export_timestamp: DateTime<Utc>,
    pub server_signature: String,
}

pub struct ExportService {
    pool: PgPool,
    signature_service: Arc<SignatureService>,
}

impl ExportService {
    pub async fn export_kaiju(
        &self,
        kaiju_id: Uuid,
        user_id: Uuid,
        export_format: ExportFormat,
    ) -> Result<KaijuExportPackage, ExportError>
}

pub enum ExportFormat {
    Json,     // Human-readable JSON
    Binary,   // Compact binary format
    QrCode,   // QR code image
}
```

### 6.2 Kaiju Import/Verification System

**File**: `kaiju_server\src\import_service.rs`

```rust
pub struct ImportService {
    pool: PgPool,
    signature_service: Arc<SignatureService>,
}

impl ImportService {
    pub async fn verify_kaiju_export(
        &self,
        export_package: KaijuExportPackage,
    ) -> Result<VerificationResult, ImportError> {
        // 1. Parse export data
        // 2. Verify server signature
        // 3. Verify state hash
        // 4. Verify genome integrity
        // 5. Check if kaiju exists in database
    }
}

pub enum VerificationResult {
    Valid {
        kaiju_id: Uuid,
        signature_valid: bool,
        data_integrity: bool,
        exists_in_database: bool,
        export_timestamp: DateTime<Utc>,
    },
    InvalidSignature,
    TamperedData,
    GenomeMismatch,
    NotFound,
}
```

---

## 7. Tournament Result Signatures

### 7.1 Tournament Result Signing

**File**: `kaiju_server\src\tournament_signature_service.rs`

```rust
pub struct TournamentSignatureService {
    signature_service: Arc<SignatureService>,
    pool: PgPool,
}

impl TournamentSignatureService {
    pub async fn sign_tournament_result(
        &self,
        tournament_id: Uuid,
        results: TournamentResults,
    ) -> Result<SignedTournamentResults, SignatureError>

    pub async fn verify_tournament_result(
        &self,
        signed_results: SignedTournamentResults,
    ) -> Result<bool, SignatureError>
}
```

---

## 8. Lineage Verification

### 8.1 Lineage Verification Service

**File**: `kaiju_server\src\lineage_verification_service.rs`

```rust
pub struct LineageVerificationService {
    pool: PgPool,
    signature_service: Arc<SignatureService>,
}

impl LineageVerificationService {
    pub async fn verify_lineage(
        &self,
        kaiju_id: Uuid,
        claimed_lineage: LineageTree,
    ) -> Result<LineageVerificationResult, VerificationError>

    pub async fn build_lineage_tree(
        &self,
        kaiju_id: Uuid,
        max_depth: i32,
    ) -> Result<LineageTree, VerificationError>

    pub async fn verify_parent_genomes(
        &self,
        kaiju_id: Uuid,
    ) -> Result<bool, VerificationError>

    pub async fn check_lineage_validity(
        &self,
        lineage: &LineageTree,
    ) -> Result<bool, VerificationError>
}
```

---

## Critical Files for Implementation

### Files to Create (Priority Order)

1. **kaiju_server\src\currency_service.rs**
   - **Reason**: Foundation for all economy features. Required before breeding rights marketplace.
   - **Dependencies**: Database schema (user_balances, currency_transactions tables)
   - **Size**: ~300 lines
   - **Priority**: HIGHEST

2. **kaiju_server\src\breeding_rights_service.rs**
   - **Reason**: Core breeding rights CRUD operations and validation
   - **Dependencies**: currency_service.rs, existing breeding_rights table
   - **Size**: ~400 lines
   - **Priority**: HIGHEST

3. **kaiju_server\src\marketplace_service.rs**
   - **Reason**: Enables trading of breeding rights between players
   - **Dependencies**: breeding_rights_service.rs, currency_service.rs
   - **Size**: ~350 lines
   - **Priority**: HIGH

4. **kaiju_server\src\death_refund_service.rs**
   - **Reason**: Automatic refunds when kaiju dies (critical for economy trust)
   - **Dependencies**: currency_service.rs, breeding_rights_service.rs
   - **Size**: ~200 lines
   - **Priority**: HIGH

5. **kaiju_server\src\export_service.rs**
   - **Reason**: Export kaiju data with cryptographic signatures for verification
   - **Dependencies**: SIGNATURE_SYSTEM.md implementation, kaiju data structures
   - **Size**: ~300 lines
   - **Priority**: MEDIUM

---

## Implementation Checklist

### Phase 8.1: Currency System (Week 1, Days 1-2)
- [ ] Create currency_service.rs with credit/debit/transfer operations
- [ ] Create currency API endpoints
- [ ] Add currency balance to user state
- [ ] Unit tests for currency operations
- [ ] Integration tests for transactions

### Phase 8.2: Breeding Rights Marketplace (Week 1, Days 3-4)
- [ ] Implement BreedingRightsService CRUD operations
- [ ] Implement MarketplaceService
- [ ] Create marketplace API endpoints
- [ ] Integration with currency service for purchases
- [ ] Unit tests for marketplace operations

### Phase 8.3: Death Refund System (Week 1, Day 5)
- [ ] Implement DeathRefundService
- [ ] Integrate with tournament death finalization
- [ ] Test automatic refunds on kaiju death
- [ ] Add refund notifications

### Phase 8.4: Export/Import System (Week 2, Days 1-2)
- [ ] Implement ExportService
- [ ] Implement ImportService with verification
- [ ] Add export API endpoints
- [ ] Test JSON, binary, and QR code export formats

### Phase 8.5: Lineage Verification (Week 2, Day 3)
- [ ] Implement LineageVerificationService
- [ ] Add lineage verification API endpoints
- [ ] Test incest detection
- [ ] Test genome integrity verification

### Phase 8.6: Tournament & Battle Signatures (Week 2, Day 4)
- [ ] Implement TournamentSignatureService
- [ ] Implement BattleSignatureService
- [ ] Add signature verification endpoints
- [ ] Test deterministic replay verification

---

## Summary

Phase 8 establishes the **economic foundation** for Kaiju Breeding Simulator:

**Core Achievements**:
1. Fully functional currency system with cryptographic proofs
2. Breeding rights marketplace with automatic refunds
3. Export/import system for multiplayer preparation
4. Comprehensive verification (lineage, tournaments, battles)
5. Extension hooks for future features

**Key Benefits**:
- Players can trade breeding rights safely
- Economy protected by automatic refunds on death
- All economic activity is auditable via signatures
- Ready for multiplayer expansion (Phase 9+)
- Zero blockchain dependency (Phase 9 remains optional)

**Time to Complete**: 1-2 weeks for core features
