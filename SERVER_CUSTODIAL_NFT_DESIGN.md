# Server-Custodial NFT Design
## Zero-Cost Transfers with Optional Blockchain Export

---

## 1. Core Philosophy

**The server is the authoritative source of ownership. Blockchain is an optional export layer.**

This model eliminates gas costs for 99% of player interactions while preserving:
- Provable ownership (via cryptographic signatures)
- Trade between players (instant, free, server-mediated)
- Optional blockchain withdrawal (for external trading or custody)
- Permanent public history (via server-hosted API + optional IPFS archival)

**Key Insight**: Most players never want to leave your ecosystem. Give them free, instant trades. Charge only those who want true blockchain custody.

---

## 2. Architecture Overview

### Three Ownership States

```
┌─────────────────────────────────────────────────────┐
│                   KAIJU LIFECYCLE                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  [BORN] → SERVER-OWNED (99% of gameplay)           │
│              ↓                                      │
│              ↓ (User pays mint fee)                 │
│              ↓                                      │
│           BLOCKCHAIN-MINTED (L2 NFT)               │
│              ↓                                      │
│              ↓ (User deposits back)                 │
│              ↓                                      │
│           SERVER-OWNED (continue playing)          │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Ownership Modes

| State | Authority | Transfers | Cost | Use Case |
|-------|-----------|-----------|------|----------|
| **Server-Owned** | Game DB | Instant, free | $0 | Active gameplay, breeding, tournaments |
| **Blockchain-Minted** | Smart Contract | Gas required | User pays | External trading, custody, speculation |
| **Deposited Back** | Game DB | Instant, free | $0 | Return to gameplay after external trade |

---

## 3. Database Schema (Server-Authoritative)

### Core Tables

#### `kaiju` (Main Entity)
```sql
CREATE TABLE kaiju (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    generation INT NOT NULL,
    created_at TIMESTAMP NOT NULL,

    -- Ownership
    owner_user_id UUID NOT NULL REFERENCES users(id),
    custody_state TEXT NOT NULL CHECK (custody_state IN ('server', 'blockchain')),
    blockchain_token_id BIGINT NULL,  -- Only set if minted to chain

    -- Immutable Genetics
    parent_a_id UUID NULL REFERENCES kaiju(id),
    parent_b_id UUID NULL REFERENCES kaiju(id),
    genome_hash TEXT NOT NULL,  -- SHA-256 of genome blob
    visual_seed TEXT NOT NULL,

    -- Mutable State
    stats JSONB NOT NULL,  -- {hp, attack, defense, speed}
    visible_traits JSONB NOT NULL,  -- Array of trait IDs
    hidden_traits JSONB NOT NULL,  -- Encrypted or server-only
    experience_level INT DEFAULT 0,
    alive BOOLEAN DEFAULT TRUE,

    -- Auditability
    state_hash TEXT NOT NULL,  -- SHA-256 of current full state
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_kaiju_owner ON kaiju(owner_user_id);
CREATE INDEX idx_kaiju_blockchain_token ON kaiju(blockchain_token_id);
CREATE INDEX idx_kaiju_alive ON kaiju(alive);
```

#### `ownership_history` (Audit Log)
```sql
CREATE TABLE ownership_history (
    id BIGSERIAL PRIMARY KEY,
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),
    from_user_id UUID NULL REFERENCES users(id),
    to_user_id UUID NOT NULL REFERENCES users(id),
    transfer_type TEXT NOT NULL,  -- 'trade', 'gift', 'breeding', 'mint', 'deposit'
    blockchain_tx_hash TEXT NULL,  -- Only for on-chain events
    timestamp TIMESTAMP NOT NULL,
    signature TEXT NULL  -- Server signature proving authenticity
);
```

#### `breeding_rights` (NFT-like, but server-side)
```sql
CREATE TABLE breeding_rights (
    id UUID PRIMARY KEY,
    kaiju_id UUID NOT NULL REFERENCES kaiju(id),
    owner_user_id UUID NOT NULL REFERENCES users(id),
    uses_remaining INT DEFAULT 1,
    created_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NULL
);
```

---

## 4. Transfer Mechanics (Zero Gas Cost)

### Server-Side Transfer Flow

```rust
pub struct TransferRequest {
    kaiju_id: Uuid,
    from_user_id: Uuid,
    to_user_id: Uuid,
    transfer_type: TransferType,
    timestamp: DateTime<Utc>,
}

pub enum TransferType {
    Trade,           // Marketplace or peer-to-peer
    Gift,            // Free transfer
    BreedingRight,   // Purchase of breeding right
}

// Pseudocode for instant server transfer
async fn execute_server_transfer(req: TransferRequest) -> Result<(), TransferError> {
    // 1. Validate ownership
    let kaiju = db.get_kaiju(req.kaiju_id).await?;
    if kaiju.owner_user_id != req.from_user_id {
        return Err(TransferError::NotOwner);
    }

    // 2. Check custody state
    if kaiju.custody_state == "blockchain" {
        return Err(TransferError::OnChainCustody);
    }

    // 3. Validate kaiju is alive
    if !kaiju.alive {
        return Err(TransferError::KaijuDead);
    }

    // 4. Begin transaction
    let mut tx = db.begin_transaction().await?;

    // 5. Update ownership
    tx.update_kaiju_owner(req.kaiju_id, req.to_user_id).await?;

    // 6. Log transfer
    tx.insert_ownership_history(OwnershipHistory {
        kaiju_id: req.kaiju_id,
        from_user_id: Some(req.from_user_id),
        to_user_id: req.to_user_id,
        transfer_type: req.transfer_type,
        timestamp: req.timestamp,
        signature: sign_transfer(&req),  // Server signature
        blockchain_tx_hash: None,
    }).await?;

    // 7. Commit
    tx.commit().await?;

    Ok(())
}
```

**Result**: Instant, free, auditable transfer. No gas, no waiting, no blockchain congestion.

---

## 5. Blockchain Export (Optional, User-Pays)

### When Players Might Want On-Chain Minting

1. **External Trading**: Sell on OpenSea, Blur, etc.
2. **Cold Storage**: Self-custody without trusting game server
3. **Cross-Game Interoperability**: Future games recognize the NFT
4. **Speculation**: Treat as investment asset
5. **Withdrawal from Game**: Quit playing but preserve value

### Minting Flow (User Pays Gas + Optional Service Fee)

```rust
pub struct MintRequest {
    kaiju_id: Uuid,
    user_id: Uuid,
    destination_wallet: Address,  // User's wallet
}

async fn mint_to_blockchain(req: MintRequest) -> Result<TxHash, MintError> {
    // 1. Validate ownership and custody
    let kaiju = db.get_kaiju(req.kaiju_id).await?;
    if kaiju.owner_user_id != req.user_id {
        return Err(MintError::NotOwner);
    }
    if kaiju.custody_state == "blockchain" {
        return Err(MintError::AlreadyMinted);
    }

    // 2. Lock kaiju (prevent double-mint during blockchain confirmation)
    db.lock_kaiju_for_mint(req.kaiju_id).await?;

    // 3. Generate metadata
    let metadata = generate_nft_metadata(&kaiju);
    let metadata_uri = upload_to_ipfs(metadata).await?;  // Or Arweave

    // 4. Call smart contract (server wallet pays gas initially)
    let tx_hash = nft_contract.mint(
        req.destination_wallet,
        metadata_uri,
        kaiju.genome_hash,
        kaiju.parent_a_id.map(|id| get_token_id(id)),
        kaiju.parent_b_id.map(|id| get_token_id(id)),
    ).await?;

    // 5. Wait for confirmation (async)
    wait_for_confirmation(tx_hash).await?;

    // 6. Update DB
    db.update_kaiju_custody(req.kaiju_id, "blockchain", Some(token_id)).await?;

    // 7. Charge user for gas cost + service fee
    billing.charge_user(req.user_id, GAS_COST + SERVICE_FEE).await?;

    Ok(tx_hash)
}
```

**Gas Cost Recovery Options**:
1. **User Pre-Pays**: Charge credits/currency before minting
2. **Credit Card**: Use Stripe/payment processor
3. **Crypto Payment**: User sends ETH to cover gas
4. **Subsidy**: You eat the cost (not recommended at scale)

---

## 6. Depositing Back from Blockchain

Players can return NFTs to server custody for free gameplay.

```rust
async fn deposit_from_blockchain(
    token_id: u64,
    user_wallet: Address,
) -> Result<(), DepositError> {
    // 1. Verify on-chain ownership
    let current_owner = nft_contract.owner_of(token_id).await?;
    if current_owner != user_wallet {
        return Err(DepositError::NotOwner);
    }

    // 2. Request user to transfer NFT to custodial wallet
    // (User initiates transfer to your server's wallet)

    // 3. Listen for Transfer event
    let transfer_event = wait_for_transfer_to_custodial_wallet(token_id).await?;

    // 4. Update DB custody
    let kaiju_id = db.get_kaiju_by_token_id(token_id).await?;
    db.update_kaiju_custody(kaiju_id, "server", Some(token_id)).await?;
    db.update_kaiju_owner(kaiju_id, user_id_from_wallet(user_wallet)).await?;

    // 5. Kaiju now tradeable in-game for free
    Ok(())
}
```

**Important**: Your custodial wallet holds all deposited NFTs. Implement strict security (multi-sig, cold storage for reserves).

---

## 7. Provable Ownership (Off-Chain)

### Cryptographic Signatures for Auditability

Even though ownership is in a database, you can prove authenticity:

```rust
pub struct OwnershipProof {
    kaiju_id: Uuid,
    owner_user_id: Uuid,
    timestamp: DateTime<Utc>,
    state_hash: String,  // SHA-256 of kaiju full state
    server_signature: String,  // ECDSA signature
}

fn generate_ownership_proof(kaiju: &Kaiju, user: &User) -> OwnershipProof {
    let state_hash = hash_kaiju_state(kaiju);
    let timestamp = Utc::now();

    let message = format!(
        "{}:{}:{}:{}",
        kaiju.id, user.id, timestamp.timestamp(), state_hash
    );

    let signature = sign_with_server_key(&message);

    OwnershipProof {
        kaiju_id: kaiju.id,
        owner_user_id: user.id,
        timestamp,
        state_hash,
        server_signature: signature,
    }
}
```

**Why This Matters**:
- Players can export ownership certificates
- Third parties can verify authenticity
- Disputes can be resolved via signed audit trail
- Builds trust even without blockchain

---

## 8. Smart Contract (Minimal, Cheap to Deploy)

### Contract Responsibilities

The on-chain contract is **only for minted NFTs**, not all kaiju.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract KaijuNFT is ERC721, Ownable {
    // Immutable kaiju data
    struct KaijuMetadata {
        string metadataURI;        // IPFS hash
        string genomeHash;         // SHA-256 of genome
        uint256 parentTokenIdA;    // 0 if gen-0
        uint256 parentTokenIdB;    // 0 if gen-0
        uint256 mintedAt;
    }

    mapping(uint256 => KaijuMetadata) public kaijuData;
    mapping(uint256 => bool) public isDead;

    uint256 private _nextTokenId = 1;

    // Only server wallet can mint
    address public serverWallet;

    constructor() ERC721("Kaiju Breeding Simulator", "KAIJU") {
        serverWallet = msg.sender;
    }

    function mint(
        address to,
        string memory metadataURI,
        string memory genomeHash,
        uint256 parentTokenIdA,
        uint256 parentTokenIdB
    ) external onlyOwner returns (uint256) {
        uint256 tokenId = _nextTokenId++;
        _mint(to, tokenId);

        kaijuData[tokenId] = KaijuMetadata({
            metadataURI: metadataURI,
            genomeHash: genomeHash,
            parentTokenIdA: parentTokenIdA,
            parentTokenIdB: parentTokenIdB,
            mintedAt: block.timestamp
        });

        return tokenId;
    }

    function markDead(uint256 tokenId) external onlyOwner {
        require(_exists(tokenId), "Token does not exist");
        require(!isDead[tokenId], "Already dead");
        isDead[tokenId] = true;
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(_exists(tokenId), "Token does not exist");
        return kaijuData[tokenId].metadataURI;
    }

    // Prevent transfer of dead kaiju
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal override {
        require(!isDead[tokenId], "Cannot transfer dead kaiju");
        super._beforeTokenTransfer(from, to, tokenId);
    }
}
```

**Deployment Cost**: ~$10-50 depending on L2 gas prices (Base, Arbitrum, Polygon)

**Per-Mint Cost**: ~$0.10-1.00 per NFT (you charge user more to cover overhead)

---

## 9. Economic Model Options

### Option A: Premium Feature (Recommended)
- Server-side gameplay is free forever
- Blockchain minting costs $5-10 (covers gas + service fee)
- Target: Serious traders and collectors

### Option B: Gated Export
- Free-to-play users cannot mint
- Premium members can mint for gas-cost-only
- Encourages retention in your ecosystem

### Option C: Lazy Minting
- All kaiju "mintable" on-demand
- User pays gas when they want to withdraw
- No upfront costs to you

### Option D: Subsidized Flagship
- First 1000 kaiju minted for free (you pay gas)
- Creates initial liquidity on OpenSea
- After that, users pay

**Recommendation**: Start with **Option A** or **Option C**. Let players choose when to go on-chain.

---

## 10. Death Mechanics (Hybrid Model)

### Server-Authoritative Death

```rust
async fn finalize_death(kaiju_id: Uuid, tournament_id: Uuid) -> Result<(), DeathError> {
    let kaiju = db.get_kaiju(kaiju_id).await?;

    // 1. Mark dead in DB
    db.mark_kaiju_dead(kaiju_id).await?;

    // 2. If minted to blockchain, update contract
    if let Some(token_id) = kaiju.blockchain_token_id {
        nft_contract.mark_dead(token_id).await?;
    }

    // 3. Refund unused breeding rights
    let unused_rights = db.get_breeding_rights(kaiju_id).await?;
    for right in unused_rights {
        refund_breeding_right(right).await?;
    }

    // 4. Log to public Hall of Fame
    hall_of_fame.add_fallen_kaiju(kaiju_id, tournament_id).await?;

    Ok(())
}
```

**Key Points**:
- Death happens on server first (instant)
- Blockchain updated later (async)
- If blockchain transaction fails, server state is still authoritative
- Dead NFTs become immutable records

---

## 11. Security Considerations

### Custodial Wallet Security

Your server wallet holds all deposited NFTs. **This is your biggest risk.**

**Mitigation**:
1. **Multi-Sig Wallet**: Require 2-of-3 or 3-of-5 signatures for withdrawals
2. **Cold Storage**: Keep 90% of NFTs in offline wallet, only 10% in hot wallet
3. **Withdrawal Delays**: 24-hour delay for large transfers
4. **Insurance**: Consider crypto insurance (Nexus Mutual, etc.)
5. **Audits**: Regular smart contract audits

### Database Security

```sql
-- Prevent unauthorized ownership changes
CREATE TRIGGER prevent_ownership_tampering
BEFORE UPDATE ON kaiju
FOR EACH ROW
WHEN (OLD.owner_user_id IS DISTINCT FROM NEW.owner_user_id)
EXECUTE FUNCTION validate_ownership_change();

-- Ensure all transfers are logged
CREATE TRIGGER log_all_transfers
AFTER UPDATE ON kaiju
FOR EACH ROW
WHEN (OLD.owner_user_id IS DISTINCT FROM NEW.owner_user_id)
EXECUTE FUNCTION insert_ownership_history_record();
```

### Replay Protection

Sign all transfers with:
- Timestamp
- Nonce (incremental counter per user)
- Server signature

Prevents:
- Replay attacks
- Timestamp manipulation
- Double-spending

---

## 12. Public History & Transparency

### API Endpoints for Auditability

```
GET /api/kaiju/{id}/history
  → Returns full ownership history with signatures

GET /api/kaiju/{id}/battles
  → Returns all battle logs with deterministic seeds

GET /api/kaiju/{id}/proof
  → Returns current ownership proof (signed)

GET /api/tournament/{id}/results
  → Returns full bracket with replay seeds

GET /api/leaderboard/hall-of-fame
  → Returns all dead kaiju with death context
```

**Why This Matters**:
- Even off-chain, players can verify integrity
- Third-party tools can build on your data
- Builds trust without blockchain costs

---

## 13. Migration Path to Full On-Chain (Optional Future)

If you later want to go fully on-chain:

1. Freeze new server-side minting
2. Offer free minting window for all existing kaiju
3. Migrate DB state to IPFS/Arweave
4. Update contract to support native minting
5. Deprecate server custody mode

**Estimated Cost**: $10,000-100,000 in gas fees depending on kaiju count (use L2 to minimize)

---

## 14. Comparison: Server-Custodial vs Full On-Chain

| Feature | Server-Custodial | Full On-Chain |
|---------|------------------|---------------|
| **Transfer Cost** | $0 | $0.10-5.00 |
| **Transfer Speed** | Instant | 2-30 seconds |
| **Breeding Cost** | $0 | $1-10 |
| **Death Recording** | Instant | $0.50-2.00 |
| **Scalability** | Unlimited | Limited by gas |
| **Trust Model** | Server + signatures | Smart contract |
| **Your Cost** | $50-500/mo (hosting) | $10,000+/mo (gas) |
| **User Friction** | None | Wallet setup, gas funds |
| **Custody Risk** | You hold NFTs | Users hold NFTs |

---

## 15. Recommended Implementation Plan

### Phase 1: Pure Server (No Blockchain)
- Build entire game with DB-based ownership
- Test economics, gameplay, balance
- Launch with cryptographic ownership proofs
- Validate market fit

**Cost**: $0 blockchain expenses

### Phase 2: Add Blockchain Export
- Deploy minimal ERC-721 contract (L2)
- Implement mint/deposit flows
- Charge users $5-10 per mint
- Monitor usage (most players won't mint)

**Cost**: $50-200 one-time deployment

### Phase 3: Scale Based on Demand
- If <10% of players mint → keep server-custodial
- If >50% of players mint → consider full on-chain migration
- Optimize based on actual behavior, not assumptions

---

## 16. Why This Model Works

**Advantages**:
- Zero gas costs for 99% of gameplay
- Instant trades and breeding
- No wallet friction for new players
- Optional blockchain for serious collectors
- You control costs (not Ethereum network)

**Tradeoffs**:
- Custodial risk (you hold deposited NFTs)
- Trust dependency (players trust your signatures)
- Centralization (server can be shut down)

**Mitigation**:
- Regular backups to IPFS/Arweave
- Open-source verification tools
- Escape hatch: mass-mint all kaiju if server dies

---

## 17. Final Recommendation

Start with **100% server-custodial**. No blockchain. Just cryptographic signatures and public APIs.

**Why**:
1. Proves gameplay works before expensive blockchain integration
2. Zero infrastructure costs beyond hosting
3. Instant UX (no wallet setup, no gas, no waiting)
4. Easy to iterate on game design
5. Can add blockchain export later if players demand it

**When to Add Blockchain**:
- You have 1000+ active players
- Players ask for external trading
- You want to list on OpenSea
- You need investor/community credibility

**Total Cost Before Blockchain**: ~$0 (just server hosting)

---

**You avoid paying for transfers. Users pay only when they want true blockchain custody. 99% of gameplay happens off-chain, fast and free.**

This is the model used by games like **Axie Infinity (Ronin)**, **Gods Unchained**, and **Immutable X** - custodial gameplay with optional blockchain export.

Let me know if you want implementation details for:
- Cryptographic signature system
- Multi-sig custodial wallet setup
- IPFS metadata hosting
- Specific L2 recommendations (Base, Arbitrum, Polygon)
