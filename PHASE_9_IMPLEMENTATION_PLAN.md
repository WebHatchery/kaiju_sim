# PHASE 9 IMPLEMENTATION PLAN: Optional Blockchain Export

Based on my exploration of the codebase, I can now provide you with a comprehensive implementation plan for Phase 9 - Optional Blockchain Export for the Kaiju Breeding Simulator.

## Overview

Phase 9 adds optional blockchain minting capabilities to the server-custodial NFT system built in Phase 0. This allows players to export their kaiju to blockchain for external trading while keeping 99% of gameplay free and instant on the server.

**Key Philosophy**: Blockchain is an optional export layer, not a requirement. The game is fully functional without it.

---

## Prerequisites

Before starting Phase 9, ensure:
- **Phase 0-8 completed** (server infrastructure, core gameplay, UI)
- MySQL database operational with Phase 0 schema
- Server transfer system and signature system functional
- 1000+ active players OR player demand for external trading
- Budget for L2 deployment ($50-200 one-time)

---

## 1. Smart Contract Deployment (Week 1, Days 1-3)

### 1.1 Choose L2 Network

**Recommended Options**:
- **Base** (Coinbase L2) - Best for low gas, growing ecosystem
- **Arbitrum One** - Established, good liquidity
- **Polygon zkEVM** - Low cost alternative

**Decision Factors**:
- Gas costs: $0.10-1.00 per mint on Base/Arbitrum
- Marketplace support: OpenSea, Blur availability
- Bridge reliability: User experience for deposits

### 1.2 Minimal ERC-721 Contract

**File**: `contracts/KaijuNFT.sol`

**Key Features**:
```solidity
- Immutable metadata storage (IPFS URI, genome hash, parent IDs)
- Death flag (prevents transfers of dead kaiju)
- Server-only minting (onlyOwner modifier)
- No upgradeable proxy (security over flexibility)
- Standard ERC-721 compliance for marketplace support
```

**Reference**: See IMPLEMENTATION_GUIDE.md Section 9.3 for complete contract code

### 1.3 Deployment Steps

```bash
# 1. Install dependencies
npm install --save-dev hardhat @openzeppelin/contracts

# 2. Create hardhat config
# File: hardhat.config.js

# 3. Deploy to testnet first (Base Sepolia)
npx hardhat run scripts/deploy.js --network base-sepolia

# 4. Verify contract on block explorer
npx hardhat verify --network base-sepolia <CONTRACT_ADDRESS>

# 5. Deploy to mainnet
npx hardhat run scripts/deploy.js --network base

# 6. Transfer ownership to multi-sig wallet (security)
```

**Cost**: $50-200 one-time deployment

---

## 2. Minting Service Implementation (Week 1, Days 4-7)

### 2.1 Rust Minting Service

**File**: `src/blockchain/mint_service.rs`

**Core Responsibilities**:
- Lock kaiju in database during mint
- Generate NFT metadata
- Upload metadata to IPFS/Arweave
- Execute blockchain mint transaction
- Update database custody state
- Charge user for service

**Key Functions**:
```rust
pub struct MintService {
    contract: KaijuNFTContract,
    pool: PgPool,
    ipfs_client: IpfsClient,
}

impl MintService {
    pub async fn mint_to_blockchain(
        &self,
        kaiju_id: Uuid,
        user_wallet: Address,
    ) -> Result<TxHash, MintError>

    async fn lock_kaiju_for_mint(&self, kaiju_id: Uuid) -> Result<(), MintError>
    async fn generate_metadata(&self, kaiju_id: Uuid) -> Result<NftMetadata, MintError>
    async fn upload_to_ipfs(&self, metadata: NftMetadata) -> Result<String, MintError>
    async fn wait_for_confirmation(&self, tx_hash: TxHash) -> Result<(), MintError>
}
```

**Reference**: IMPLEMENTATION_GUIDE.md Section 9.4

### 2.2 Database Schema Updates

**Add to kaiju table**:
```sql
ALTER TABLE kaiju
ADD COLUMN IF NOT EXISTS blockchain_contract_address TEXT;

ALTER TABLE kaiju
ADD COLUMN IF NOT EXISTS mint_requested_at TIMESTAMPTZ;

ALTER TABLE kaiju
ADD COLUMN IF NOT EXISTS mint_completed_at TIMESTAMPTZ;
```

**New table for mint tracking**:
```sql
CREATE TABLE mint_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),
    user_id UUID NOT NULL REFERENCES users(id),
    destination_wallet TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'uploading', 'minting', 'confirming', 'completed', 'failed')),
    ipfs_uri TEXT,
    tx_hash TEXT,
    gas_cost_wei BIGINT,
    service_fee_cents INT,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_mint_requests_kaiju ON mint_requests(kaiju_id);
CREATE INDEX idx_mint_requests_user ON mint_requests(user_id);
CREATE INDEX idx_mint_requests_status ON mint_requests(status) WHERE status IN ('pending', 'minting');
```

### 2.3 Ethereum Integration Dependencies

**Cargo.toml additions**:
```toml
[dependencies]
# Ethereum
ethers = { version = "2.0", features = ["ws", "rustls"] }
ethers-contract = "2.0"

# IPFS
ipfs-api = "0.17"
# OR for Pinata
reqwest = { version = "0.11", features = ["json"] }

# Async
tokio = { version = "1.35", features = ["full"] }
```

---

## 3. IPFS/Arweave Metadata Upload (Week 2, Days 1-2)

### 3.1 Metadata Structure

**File**: `src/blockchain/metadata.rs`

**ERC-721 Standard Metadata** (OpenSea compatible):
```rust
#[derive(Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,              // "Volthor #1234"
    pub description: String,       // "Generation 3 Electric/Aquatic hybrid"
    pub image: String,             // "ipfs://Qm.../volthor.png"
    pub external_url: String,      // "https://kaiju.game/kaiju/1234"
    pub attributes: Vec<Attribute>,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: Value,  // String or number
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    pub genome_hash: String,
    pub parent_a_token: Option<u64>,
    pub parent_b_token: Option<u64>,
    pub visual_seed: String,
    pub generation: u32,
}
```

**Reference**: nft_design.md Section 5, IMPLEMENTATION_GUIDE.md Section 9.9

### 3.2 IPFS Upload Options

**Option A: Self-Hosted IPFS Node**
- Pros: Full control, no API limits
- Cons: Infrastructure maintenance
- Cost: $10-50/month hosting

**Option B: Pinata (Recommended)**
- Pros: Reliable, simple API, generous free tier
- Cons: API dependency
- Cost: Free for <1GB, then $20/month

**Option C: Arweave (Permanent Storage)**
- Pros: Permanent, one-time payment
- Cons: Higher upfront cost per file
- Cost: ~$0.01-0.10 per metadata file

**Implementation**:
```rust
// src/blockchain/ipfs_client.rs

pub struct IpfsClient {
    pinata_api_key: String,
    pinata_secret: String,
}

impl IpfsClient {
    pub async fn upload_json(&self, metadata: &NftMetadata) -> Result<String, IpfsError> {
        // POST to Pinata API
        // Returns: "ipfs://QmXxx..."
    }

    pub async fn upload_image(&self, image_bytes: &[u8]) -> Result<String, IpfsError> {
        // Upload AI-generated kaiju portrait
        // Returns: "ipfs://QmYyy..."
    }
}
```

**Recommendation**: Start with Pinata, migrate to self-hosted IPFS if volume justifies it.

---

## 4. Deposit Service Implementation (Week 2, Days 3-5)

### 4.1 Blockchain Event Listener

**File**: `src/blockchain/deposit_service.rs`

**Core Functionality**:
```rust
pub struct DepositService {
    contract: KaijuNFTContract,
    pool: PgPool,
    custodial_wallet: Address,
}

impl DepositService {
    pub async fn start_listening(&self) {
        // Listen for Transfer events to custodial wallet
    }

    pub async fn handle_deposit(
        &self,
        token_id: u64,
        from_wallet: Address,
        tx_hash: TxHash,
    ) -> Result<(), DepositError> {
        // 1. Verify transfer is to custodial wallet
        // 2. Find kaiju by token_id
        // 3. Update custody_state to 'server'
        // 4. Update owner_user_id based on wallet mapping
        // 5. Log in ownership_history
    }
}
```

### 4.2 User Flow

**Frontend UI**:
1. User clicks "Deposit NFT to Game"
2. Shows custodial wallet address
3. User initiates transfer via MetaMask/wallet
4. Frontend polls server for deposit confirmation
5. Success notification when DB updated

**Backend**:
```rust
// API endpoint
POST /api/blockchain/deposit/initiate
{
    "token_id": 1234,
    "user_wallet": "0xABC..."
}

// Response includes custodial address and instructions
```

### 4.3 Deposit Tracking Table

```sql
CREATE TABLE deposit_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id BIGINT NOT NULL,
    user_wallet TEXT NOT NULL,
    expected_by TIMESTAMPTZ NOT NULL,  -- Timeout if not received
    status TEXT NOT NULL CHECK (status IN ('awaiting_transfer', 'detected', 'confirmed', 'completed', 'timeout')),
    tx_hash TEXT,
    detected_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## 5. Death Synchronization (Week 2, Days 6-7)

### 5.1 Async Death Updates

**File**: `src/blockchain/death_sync.rs`

**Key Challenge**: Server death is instant, blockchain update is async

**Solution**: Queue-based approach

```rust
pub struct DeathSyncService {
    contract: KaijuNFTContract,
    pool: PgPool,
}

impl DeathSyncService {
    pub async fn finalize_death(
        &self,
        kaiju_id: Uuid,
        tournament_id: Uuid,
    ) -> Result<(), DeathError> {
        // 1. Mark dead in DB (instant, authoritative)
        self.pool.mark_kaiju_dead(kaiju_id, tournament_id).await?;

        // 2. If minted, queue blockchain update
        if let Some(token_id) = self.get_blockchain_token_id(kaiju_id).await? {
            self.queue_blockchain_death(token_id).await?;
        }

        // 3. Refund breeding rights (instant)
        self.refund_unused_breeding_rights(kaiju_id).await?;

        // 4. Add to Hall of Fame
        self.add_to_hall_of_fame(kaiju_id, tournament_id).await?;

        Ok(())
    }

    async fn process_death_queue(&self) {
        // Background task: Process queued blockchain deaths
        // Retry on failure with exponential backoff
    }
}
```

### 5.2 Death Queue Table

```sql
CREATE TABLE blockchain_death_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),
    token_id BIGINT NOT NULL,
    tournament_id UUID NOT NULL REFERENCES tournaments(id),
    status TEXT NOT NULL CHECK (status IN ('pending', 'processing', 'confirmed', 'failed')),
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    tx_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ
);
```

**Reference**: IMPLEMENTATION_GUIDE.md Section 9.6, SERVER_CUSTODIAL_NFT_DESIGN.md Section 10

---

## 6. Custodial Wallet Security (Week 3, Days 1-3)

### 6.1 Multi-Sig Wallet Setup

**Critical**: Your custodial wallet holds all deposited NFTs. This is the highest security risk.

**Recommended Setup**:
1. **Gnosis Safe** multi-sig (3-of-5 signers)
2. **Signers**:
   - 2 founding team members
   - 1 trusted advisor
   - 1 cold wallet (vault)
   - 1 hot wallet (automated deposits)

**Implementation**:
```bash
# Deploy Gnosis Safe via their UI
# https://app.safe.global/

# Configure signers
# Set threshold: 3 signatures required for withdrawals

# Grant minting permission to hot wallet only
```

### 6.2 Cold/Hot Wallet Split

**Strategy**:
- **Hot Wallet**: Holds 10% of NFTs for active deposits (automated)
- **Cold Wallet**: Holds 90% of NFTs in multi-sig (manual transfers only)

**Transfer Policy**:
```rust
// Automated rule: If hot wallet exceeds 10% threshold, move excess to cold storage
pub async fn rebalance_custody(&self) -> Result<(), Error> {
    let total_nfts = self.count_custodial_nfts().await?;
    let hot_wallet_nfts = self.count_hot_wallet_nfts().await?;
    let threshold = (total_nfts as f64 * 0.1) as u64;

    if hot_wallet_nfts > threshold {
        let excess = hot_wallet_nfts - threshold;
        self.transfer_to_cold_storage(excess).await?;
    }

    Ok(())
}
```

### 6.3 Security Checklist

- [ ] Multi-sig wallet deployed (3-of-5 or higher)
- [ ] Hot wallet has limited permissions (deposits only)
- [ ] Cold storage set up with hardware wallets
- [ ] 24-hour withdrawal delay for large transfers
- [ ] Emergency pause mechanism in contract
- [ ] Regular security audits scheduled
- [ ] Crypto insurance evaluated (Nexus Mutual, InsurAce)
- [ ] Backup keys stored in secure vault (not digital)
- [ ] Key rotation policy defined (every 6-12 months)

**Reference**: SERVER_CUSTODIAL_NFT_DESIGN.md Section 11

---

## 7. Payment Processing Integration (Week 3, Days 4-5)

### 7.1 Pricing Model

**Recommended Structure**:
- Gas cost: $0.10-1.00 (actual L2 cost)
- Service fee: $5-10 (covers IPFS, overhead, profit)
- **Total user pays**: $5-10 per mint

**Dynamic Pricing**:
```rust
pub async fn calculate_mint_price(&self) -> Result<MintPrice, Error> {
    let gas_price = self.get_current_gas_price().await?;
    let estimated_gas = 150_000; // Typical ERC-721 mint
    let gas_cost_wei = gas_price * estimated_gas;
    let gas_cost_usd = self.wei_to_usd(gas_cost_wei).await?;

    MintPrice {
        gas_cost_usd,
        service_fee_usd: 5.0,
        total_usd: gas_cost_usd + 5.0,
    }
}
```

### 7.2 Payment Options

**Option A: Credit Card (Stripe)**
```rust
// Cargo.toml
[dependencies]
stripe-rust = "0.24"

// Implementation
pub async fn charge_user_card(
    user_id: Uuid,
    amount_cents: u64,
) -> Result<PaymentIntent, StripeError> {
    let client = stripe::Client::new(env::var("STRIPE_SECRET_KEY")?);

    let payment_intent = PaymentIntent::create(
        &client,
        CreatePaymentIntent {
            amount: amount_cents as i64,
            currency: Currency::USD,
            payment_method_types: Some(vec!["card"]),
            metadata: Some(HashMap::from([
                ("user_id", user_id.to_string()),
                ("product", "kaiju_mint".to_string()),
            ])),
            ..Default::default()
        },
    ).await?;

    Ok(payment_intent)
}
```

**Option B: Crypto Payment**
```rust
// User sends ETH/USDC to payment wallet
// Server monitors deposits and credits account
pub async fn monitor_crypto_payments(&self) {
    // Listen for transfers to payment wallet
    // Match amount to pending mint requests
    // Auto-process when payment confirmed
}
```

**Option C: In-Game Currency**
```sql
-- User account balance
CREATE TABLE user_balances (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    gems INT NOT NULL DEFAULT 0,
    premium_credits INT NOT NULL DEFAULT 0
);

-- Charge from balance
UPDATE user_balances
SET premium_credits = premium_credits - 100
WHERE user_id = ? AND premium_credits >= 100;
```

**Recommendation**: Start with **Option A (Stripe)** for simplicity, add crypto later if demanded.

---

## 8. OpenSea Metadata API (Week 3, Days 6-7)

### 8.1 OpenSea Metadata Endpoint

**File**: `src/api/opensea.rs`

**Required Endpoint**:
```rust
// GET /api/opensea/:contract_address/:token_id
pub async fn get_opensea_metadata(
    Path((contract_address, token_id)): Path<(String, u64)>,
    State(state): State<Arc<AppState>>,
) -> Json<OpenSeaMetadata> {
    let kaiju = state.db.get_kaiju_by_token_id(token_id).await.unwrap();

    Json(OpenSeaMetadata {
        name: format!("{} #{}", kaiju.name, token_id),
        description: generate_description(&kaiju),
        image: kaiju.image_uri.unwrap_or_default(),
        external_url: format!("https://kaiju.game/kaiju/{}", token_id),
        attributes: generate_attributes(&kaiju),
        background_color: None,
        animation_url: None,
    })
}
```

### 8.2 Collection Verification

**Steps for OpenSea**:
1. Deploy contract to mainnet
2. Mint first NFT
3. Submit collection for verification at opensea.io/get-listed
4. Provide:
   - Contract address
   - Collection name/description
   - Social links (Twitter, Discord)
   - Banner/logo images
   - Creator wallet address

**Timeline**: 1-4 weeks for OpenSea review

### 8.3 Rarity/Trait Filters

**Implement trait rarity calculation**:
```rust
pub fn calculate_trait_rarity(
    trait_id: &str,
    all_kaiju: &[Kaiju],
) -> f64 {
    let count = all_kaiju.iter()
        .filter(|k| k.has_trait(trait_id))
        .count();

    (count as f64 / all_kaiju.len() as f64) * 100.0
}
```

**Add to OpenSea metadata**:
```json
{
    "attributes": [
        {
            "trait_type": "Electric Breath",
            "value": "Legendary",
            "rarity": 2.5
        }
    ]
}
```

**Reference**: IMPLEMENTATION_GUIDE.md Section 9.8 (Phase 9E)

---

## 9. Testing on Testnet (Week 4)

### 9.1 Testnet Deployment Checklist

**Networks**:
- Base Sepolia (recommended)
- Arbitrum Goerli
- Polygon Mumbai

**Testing Scenarios**:

1. **Mint Flow**:
   ```bash
   # 1. Lock kaiju in DB
   # 2. Upload metadata to IPFS testnet
   # 3. Mint on testnet
   # 4. Verify custody update
   # 5. Check OpenSea testnet
   ```

2. **Deposit Flow**:
   ```bash
   # 1. Transfer NFT to custodial wallet (testnet)
   # 2. Verify event detected
   # 3. Check DB custody update
   # 4. Confirm in-game ownership
   ```

3. **Death Sync**:
   ```bash
   # 1. Kill kaiju in tournament
   # 2. Verify DB death instant
   # 3. Check blockchain queue
   # 4. Process queue
   # 5. Verify on-chain death flag
   ```

4. **Payment Testing**:
   ```bash
   # Use Stripe test mode
   # Test card: 4242 4242 4242 4242
   # Verify payment captured before mint
   ```

---

## 10. Mainnet Launch (Week 5)

### 10.1 Pre-Launch Checklist

**Security**:
- [ ] Smart contract audited (Certik, OpenZeppelin, or similar)
- [ ] Multi-sig wallet deployed and tested
- [ ] Emergency pause mechanism tested
- [ ] Key backup procedures documented
- [ ] Cold storage set up

**Infrastructure**:
- [ ] IPFS/Arweave upload tested at scale
- [ ] Blockchain RPC endpoints reliable (Alchemy, Infura)
- [ ] Database backups automated
- [ ] Monitoring/alerting configured

**Legal**:
- [ ] Terms of Service updated (blockchain terms)
- [ ] Privacy Policy updated (wallet address collection)
- [ ] User warnings about blockchain finality

**Financial**:
- [ ] Payment processing tested in production mode
- [ ] Pricing confirmed ($5-10 range)
- [ ] Refund policy defined
- [ ] Gas cost buffer calculated

### 10.2 Deployment Steps

```bash
# Day 1: Deploy contract
npx hardhat run scripts/deploy.js --network base
# Save contract address: 0xABCD1234...

# Day 2: Verify contract
npx hardhat verify --network base 0xABCD1234...

# Day 3: Deploy backend services
cargo build --release
systemctl start kaiju-mint-service
systemctl start kaiju-deposit-listener
systemctl start kaiju-death-sync

# Day 4: Enable minting in UI
# Feature flag: BLOCKCHAIN_MINTING_ENABLED=true

# Day 5: Announce to players
# Blog post, Discord, Twitter
```

### 10.3 Phased Rollout

**Phase 1: Invite-Only (Week 1)**
- 10-50 trusted users
- Monitor closely for issues
- Collect feedback

**Phase 2: Limited Access (Week 2-3)**
- Premium members only
- 100-500 mints
- Tune pricing/UX

**Phase 3: Public Access (Week 4+)**
- All players can mint
- Full marketing push
- Monitor gas costs and adjust

---

## Critical Files for Implementation

Here are the 5 most critical files you'll need to create for Phase 9:

1. **`contracts/KaijuNFT.sol`** - Minimal ERC-721 smart contract with death mechanics and immutable metadata storage. This is the blockchain layer foundation.

2. **`src/blockchain/mint_service.rs`** - Core minting orchestration: locks kaiju, generates metadata, uploads to IPFS, executes blockchain transaction, updates database custody state.

3. **`src/blockchain/deposit_service.rs`** - Listens for blockchain Transfer events to custodial wallet, validates deposits, updates database ownership when NFTs return to server custody.

4. **`src/blockchain/death_sync.rs`** - Queue-based async death synchronization: marks kaiju dead instantly in DB (authoritative), then processes blockchain updates with retry logic.

5. **`src/api/opensea.rs`** - OpenSea-compatible metadata API endpoints for marketplace integration, including dynamic trait rarity calculation and collection verification support.

---

## Cost Summary

| Phase | One-Time Cost | Monthly Cost | User Pays |
|-------|--------------|--------------|-----------|
| **Contract Deployment** | $50-200 | $0 | $0 |
| **IPFS/Pinata** | $0 | $0-20 | Included in mint fee |
| **Blockchain RPC** (Alchemy/Infura) | $0 | $0-50 | $0 |
| **Payment Processing** (Stripe 2.9%) | $0 | Variable | Included |
| **Multi-Sig Setup** | $100-500 | $0 | $0 |
| **Security Audit** | $5,000-50,000 | $0 | $0 |
| **Per-Mint Gas** | $0 | $0 | $0.10-1.00 |
| **Per-Mint Service Fee** | $0 | $0 | $5-10 |
| **Total** | $5,150-50,720 | $0-70 | $5-10 per mint |

**Key Insight**: After initial setup, blockchain costs are $0 to you. Users pay only when they want external custody.

---

## Success Criteria

Phase 9 is successful when:
- [ ] <10% of players mint (validates hybrid model)
- [ ] 95%+ mint success rate
- [ ] Mint completes in <5 minutes
- [ ] Zero loss of deposited NFTs (security)
- [ ] OpenSea listing approved
- [ ] Death sync 100% reliable
- [ ] User feedback positive on UX

---

This implementation plan provides a complete roadmap for adding optional blockchain export to your server-custodial kaiju game. The key is maintaining the "web2.5" philosophy: blockchain as an optional export layer, not a requirement, keeping 99% of gameplay free and instant.
