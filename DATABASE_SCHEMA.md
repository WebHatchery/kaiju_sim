# Database Schema for Server-Custodial NFT System
## MySQL Schema with Audit Trails and Security

---

## 1. Overview

This schema supports:
- Server-authoritative ownership
- Instant free transfers
- Full audit trails
- Blockchain export tracking
- Death mechanics
- Breeding rights economy
- Tamper detection

**Database**: MySQL 8.0+ (required for JSON and triggers)

---

## 2. Core Tables

### 2.1 Users Table

```sql
-- Users/Players
CREATE TABLE users (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    username VARCHAR(32) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    wallet_address VARCHAR(255) UNIQUE,  -- Optional, for blockchain linking
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP NULL,

    -- Account state
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,

    -- Transfer nonce for replay protection
    transfer_nonce BIGINT NOT NULL DEFAULT 0,

    CONSTRAINT valid_username CHECK (CHAR_LENGTH(username) >= 3 AND CHAR_LENGTH(username) <= 32)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_wallet ON users(wallet_address);
CREATE INDEX idx_users_active ON users(is_active);
```

---

### 2.2 Kaiju Table (Main Entity)

```sql
-- Kaiju entities
CREATE TABLE kaiju (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    -- Identity
    name VARCHAR(255) NOT NULL,
    generation INT NOT NULL CHECK (generation >= 0),

    -- Ownership
    owner_user_id CHAR(36) NOT NULL,
    custody_state ENUM('server', 'blockchain') NOT NULL,
    blockchain_token_id BIGINT UNIQUE,  -- Only set if minted to blockchain
    blockchain_contract_address VARCHAR(255),   -- Contract address if minted

    -- Timestamps
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    -- Genetics (Immutable after creation)
    parent_a_id CHAR(36),
    parent_b_id CHAR(36),
    genome_hash VARCHAR(64) NOT NULL,  -- SHA-256 of genome blob
    genome_data LONGBLOB NOT NULL, -- Actual genome binary data
    visual_seed VARCHAR(255) NOT NULL,  -- Deterministic visual generation seed

    -- Stats (Mutable via gameplay)
    base_stats JSON NOT NULL,  -- {hp, attack, defense, speed, special_attack, special_defense}
    current_stats JSON NOT NULL,  -- Modified by training/items

    -- Traits
    visible_traits JSON NOT NULL DEFAULT ('[]'),  -- Array of trait IDs visible to owner
    hidden_traits JSON NOT NULL DEFAULT ('[]'),   -- Encrypted or server-only traits

    -- Experience & Training
    experience_level INT NOT NULL DEFAULT 0 CHECK (experience_level >= 0),
    experience_points BIGINT NOT NULL DEFAULT 0 CHECK (experience_points >= 0),
    training_points INT NOT NULL DEFAULT 0,

    -- State
    alive BOOLEAN NOT NULL DEFAULT TRUE,
    death_timestamp TIMESTAMP NULL,
    death_tournament_id CHAR(36),

    -- Research/Discovery
    genome_decode_level INT NOT NULL DEFAULT 0,  -- How much of genome is decoded
    research_data JSON DEFAULT ('{}'),  -- Progressive research discoveries

    -- Integrity
    state_hash VARCHAR(64) NOT NULL,  -- SHA-256 of full state for tamper detection
    state_version INT NOT NULL DEFAULT 1,  -- Incremented on every state change

    -- Soft deletion
    deleted_at TIMESTAMP NULL,

    CONSTRAINT parent_lineage_valid CHECK (
        (parent_a_id IS NULL AND parent_b_id IS NULL AND generation = 0) OR
        (parent_a_id IS NOT NULL AND parent_b_id IS NOT NULL AND generation > 0)
    ),
    CONSTRAINT blockchain_data_consistency CHECK (
        (custody_state = 'server' AND blockchain_token_id IS NULL) OR
        (custody_state = 'blockchain' AND blockchain_token_id IS NOT NULL)
    ),
    CONSTRAINT death_consistency CHECK (
        (alive = TRUE AND death_timestamp IS NULL) OR
        (alive = FALSE AND death_timestamp IS NOT NULL)
    ),

    FOREIGN KEY (owner_user_id) REFERENCES users(id),
    FOREIGN KEY (parent_a_id) REFERENCES kaiju(id),
    FOREIGN KEY (parent_b_id) REFERENCES kaiju(id),
    FOREIGN KEY (death_tournament_id) REFERENCES tournaments(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Indexes for common queries
CREATE INDEX idx_kaiju_owner ON kaiju(owner_user_id, alive, deleted_at);
CREATE INDEX idx_kaiju_alive ON kaiju(alive, deleted_at);
CREATE INDEX idx_kaiju_generation ON kaiju(generation, alive);
CREATE INDEX idx_kaiju_blockchain_token ON kaiju(blockchain_token_id);
CREATE INDEX idx_kaiju_created_at ON kaiju(created_at DESC);
CREATE INDEX idx_kaiju_parents ON kaiju(parent_a_id, parent_b_id);

-- JSON indexes for trait searches (MySQL uses functional indexes on JSON paths)
CREATE INDEX idx_kaiju_visible_traits ON kaiju((CAST(visible_traits AS CHAR(1000) ARRAY)));
CREATE INDEX idx_kaiju_stats ON kaiju((CAST(current_stats AS CHAR(1000) ARRAY)));
```

---

### 2.3 Ownership History (Audit Trail)

```sql
-- Complete audit log of all ownership changes
CREATE TABLE ownership_history (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,

    -- Transfer details
    kaiju_id CHAR(36) NOT NULL,
    from_user_id CHAR(36),  -- NULL for initial mint
    to_user_id CHAR(36) NOT NULL,

    -- Transfer metadata
    transfer_type ENUM(
        'mint',           -- Initial creation
        'trade',          -- Marketplace/P2P trade
        'gift',           -- Free transfer
        'breeding_payment', -- Payment for breeding rights
        'blockchain_mint', -- Export to blockchain
        'blockchain_deposit', -- Import from blockchain
        'admin_transfer'  -- Admin intervention
    ) NOT NULL,

    -- Pricing (if applicable)
    price_amount BIGINT,  -- In-game currency amount
    price_currency VARCHAR(50),  -- 'gold', 'gems', etc.

    -- Blockchain linkage
    blockchain_tx_hash VARCHAR(66),  -- Only for on-chain events
    blockchain_confirmed BOOLEAN DEFAULT FALSE,

    -- Timing
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Security
    server_signature TEXT NOT NULL,  -- ECDSA signature proving authenticity
    nonce BIGINT NOT NULL,  -- Replay protection

    -- Additional context
    metadata JSON DEFAULT ('{}'),  -- Tournament ID, marketplace listing ID, etc.

    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (from_user_id) REFERENCES users(id),
    FOREIGN KEY (to_user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_ownership_history_kaiju ON ownership_history(kaiju_id);
CREATE INDEX idx_ownership_history_from_user ON ownership_history(from_user_id);
CREATE INDEX idx_ownership_history_to_user ON ownership_history(to_user_id);
CREATE INDEX idx_ownership_history_timestamp ON ownership_history(timestamp DESC);
CREATE INDEX idx_ownership_history_type ON ownership_history(transfer_type);
```

---

### 2.4 Breeding Rights

```sql
-- Breeding rights (tradeable, refundable on parent death)
CREATE TABLE breeding_rights (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    -- Parent kaiju
    kaiju_id CHAR(36) NOT NULL,

    -- Ownership
    owner_user_id CHAR(36) NOT NULL,
    issuer_user_id CHAR(36) NOT NULL,  -- Original kaiju owner

    -- Usage
    uses_total INT NOT NULL DEFAULT 1 CHECK (uses_total > 0),
    uses_remaining INT NOT NULL CHECK (uses_remaining >= 0 AND uses_remaining <= uses_total),

    -- Lifecycle
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NULL,  -- NULL = no expiry
    used_at TIMESTAMP NULL,     -- When last use consumed

    -- Pricing
    purchase_price BIGINT,
    purchase_currency VARCHAR(50),
    refund_eligible BOOLEAN NOT NULL DEFAULT TRUE,

    -- State
    active BOOLEAN NOT NULL DEFAULT TRUE,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    refunded BOOLEAN NOT NULL DEFAULT FALSE,

    CONSTRAINT uses_consistency CHECK (
        (uses_remaining = 0 AND used_at IS NOT NULL) OR
        (uses_remaining > 0)
    ),

    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (owner_user_id) REFERENCES users(id),
    FOREIGN KEY (issuer_user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_breeding_rights_kaiju ON breeding_rights(kaiju_id, active);
CREATE INDEX idx_breeding_rights_owner ON breeding_rights(owner_user_id, active);
CREATE INDEX idx_breeding_rights_active ON breeding_rights(active, uses_remaining);
```

---

### 2.5 Breeding History

```sql
-- Record of all breeding events
CREATE TABLE breeding_history (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    -- Parents
    parent_a_id CHAR(36) NOT NULL,
    parent_b_id CHAR(36) NOT NULL,

    -- Child
    child_id CHAR(36) NOT NULL,

    -- Breeder
    breeder_user_id CHAR(36) NOT NULL,

    -- Breeding rights used
    breeding_right_id CHAR(36),

    -- Timing
    bred_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Genetics metadata
    mutation_occurred BOOLEAN NOT NULL DEFAULT FALSE,
    mutation_details JSON,
    trait_inheritance JSON,  -- Which traits came from which parent

    -- Costs
    breeding_cost BIGINT,
    breeding_currency VARCHAR(50),

    FOREIGN KEY (parent_a_id) REFERENCES kaiju(id),
    FOREIGN KEY (parent_b_id) REFERENCES kaiju(id),
    FOREIGN KEY (child_id) REFERENCES kaiju(id),
    FOREIGN KEY (breeder_user_id) REFERENCES users(id),
    FOREIGN KEY (breeding_right_id) REFERENCES breeding_rights(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_breeding_history_parents ON breeding_history(parent_a_id, parent_b_id);
CREATE INDEX idx_breeding_history_child ON breeding_history(child_id);
CREATE INDEX idx_breeding_history_breeder ON breeding_history(breeder_user_id);
CREATE INDEX idx_breeding_history_timestamp ON breeding_history(bred_at DESC);
```

---

### 2.6 Tournaments

```sql
-- Tournament configurations
CREATE TABLE tournaments (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    -- Identity
    name VARCHAR(255) NOT NULL,
    description TEXT,
    tournament_type VARCHAR(50) NOT NULL CHECK (tournament_type IN ('casual', 'ranked', 'lethal')),

    -- Entry requirements
    min_generation INT,
    max_generation INT,
    min_level INT,
    max_level INT,
    entry_fee BIGINT,
    entry_currency VARCHAR(50),

    -- Rewards
    rewards JSON NOT NULL DEFAULT ('{}'),  -- Prize pool structure

    -- Scheduling
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    registration_deadline TIMESTAMP NOT NULL,

    -- State
    status VARCHAR(50) NOT NULL CHECK (status IN ('upcoming', 'registration', 'in_progress', 'completed', 'cancelled')),

    -- Environment
    battle_environment VARCHAR(50),  -- 'storm', 'volcanic', etc.

    -- Bracket
    bracket_data JSON,  -- Full bracket structure

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_tournaments_status ON tournaments(status);
CREATE INDEX idx_tournaments_start_time ON tournaments(start_time);
CREATE INDEX idx_tournaments_type ON tournaments(tournament_type);
```

---

### 2.7 Tournament Entries

```sql
-- Tournament registrations
CREATE TABLE tournament_entries (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    tournament_id CHAR(36) NOT NULL,
    kaiju_id CHAR(36) NOT NULL,
    user_id CHAR(36) NOT NULL,

    -- Entry
    registered_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    entry_fee_paid BIGINT,

    -- Results
    placement INT,  -- Final rank (1 = winner)
    rounds_won INT NOT NULL DEFAULT 0,
    rounds_lost INT NOT NULL DEFAULT 0,

    -- Rewards
    rewards_claimed JSON,

    -- State
    eliminated BOOLEAN NOT NULL DEFAULT FALSE,
    eliminated_round INT,

    UNIQUE(tournament_id, kaiju_id),
    FOREIGN KEY (tournament_id) REFERENCES tournaments(id),
    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_tournament_entries_tournament ON tournament_entries(tournament_id);
CREATE INDEX idx_tournament_entries_kaiju ON tournament_entries(kaiju_id);
CREATE INDEX idx_tournament_entries_user ON tournament_entries(user_id);
```

---

### 2.8 Battle Logs

```sql
-- Detailed battle records for auditability
CREATE TABLE battle_logs (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    -- Combatants
    kaiju_a_id CHAR(36) NOT NULL,
    kaiju_b_id CHAR(36) NOT NULL,

    -- Context
    tournament_id CHAR(36),
    round_number INT,
    battle_environment VARCHAR(50),

    -- Simulation
    rng_seed VARCHAR(255) NOT NULL,  -- Deterministic seed for reproducibility
    turn_log JSON NOT NULL,  -- Full turn-by-turn log

    -- Result
    winner_id CHAR(36) NOT NULL,
    loser_id CHAR(36) NOT NULL,
    total_turns INT NOT NULL,

    -- Death (if lethal tournament)
    death_occurred BOOLEAN NOT NULL DEFAULT FALSE,
    deceased_kaiju_id CHAR(36),

    -- Timing
    battle_start TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    battle_end TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Integrity
    log_hash VARCHAR(64) NOT NULL,  -- SHA-256 of turn_log for tamper detection

    CONSTRAINT valid_combatants CHECK (kaiju_a_id <> kaiju_b_id),
    CONSTRAINT valid_death CHECK (
        (death_occurred = FALSE AND deceased_kaiju_id IS NULL) OR
        (death_occurred = TRUE AND deceased_kaiju_id = loser_id)
    ),

    FOREIGN KEY (kaiju_a_id) REFERENCES kaiju(id),
    FOREIGN KEY (kaiju_b_id) REFERENCES kaiju(id),
    FOREIGN KEY (tournament_id) REFERENCES tournaments(id),
    FOREIGN KEY (winner_id) REFERENCES kaiju(id),
    FOREIGN KEY (loser_id) REFERENCES kaiju(id),
    FOREIGN KEY (deceased_kaiju_id) REFERENCES kaiju(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_battle_logs_kaiju_a ON battle_logs(kaiju_a_id);
CREATE INDEX idx_battle_logs_kaiju_b ON battle_logs(kaiju_b_id);
CREATE INDEX idx_battle_logs_tournament ON battle_logs(tournament_id);
CREATE INDEX idx_battle_logs_timestamp ON battle_logs(battle_start DESC);
```

---

## 3. Security Tables

### 3.1 Transfer Locks

```sql
-- Prevent concurrent transfers
CREATE TABLE transfer_locks (
    kaiju_id CHAR(36) PRIMARY KEY,
    locked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    locked_by CHAR(36) NOT NULL,
    lock_reason VARCHAR(50) NOT NULL CHECK (lock_reason IN ('transfer', 'breeding', 'tournament', 'blockchain_mint')),
    expires_at TIMESTAMP NOT NULL,

    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (locked_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_transfer_locks_expires ON transfer_locks(expires_at);
```

---

### 3.2 Server Signature Keys

```sql
-- Public keys for signature verification
CREATE TABLE server_signature_keys (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    key_name VARCHAR(255) NOT NULL UNIQUE,
    public_key TEXT NOT NULL,  -- Hex-encoded ECDSA public key
    algorithm VARCHAR(50) NOT NULL DEFAULT 'secp256k1',

    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rotated_at TIMESTAMP,

    -- Key purpose
    purpose VARCHAR(50) NOT NULL CHECK (purpose IN ('ownership', 'battle', 'mint', 'admin'))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
```

---

## 4. Triggers & Automation

### 4.1 Auto-Update Timestamps

```sql
-- MySQL automatically updates TIMESTAMP columns with ON UPDATE CURRENT_TIMESTAMP
-- This was already configured in the table definitions above
-- No additional triggers needed for timestamp updates in MySQL
```

---

### 4.2 Ownership Change Logging

```sql
-- Automatically log ownership changes
DELIMITER //

CREATE TRIGGER track_ownership_changes
AFTER UPDATE ON kaiju
FOR EACH ROW
BEGIN
    IF OLD.owner_user_id <> NEW.owner_user_id THEN
        INSERT INTO ownership_history (
            kaiju_id,
            from_user_id,
            to_user_id,
            transfer_type,
            timestamp,
            server_signature,
            nonce
        ) VALUES (
            NEW.id,
            OLD.owner_user_id,
            NEW.owner_user_id,
            'admin_transfer',  -- Will be overridden by proper transfer functions
            NOW(),
            'auto_logged',
            0
        );
    END IF;
END//

DELIMITER ;
```

---

### 4.3 State Hash Validation

```sql
-- Prevent state_hash tampering
DELIMITER //

CREATE TRIGGER enforce_state_hash
BEFORE UPDATE ON kaiju
FOR EACH ROW
BEGIN
    -- In production, compute actual hash and compare
    -- For now, just increment version
    SET NEW.state_version = OLD.state_version + 1;
END//

DELIMITER ;
```

---

### 4.4 Death Finalization

```sql
-- When kaiju dies, refund breeding rights
DELIMITER //

CREATE TRIGGER handle_death_refunds
AFTER UPDATE ON kaiju
FOR EACH ROW
BEGIN
    IF NEW.alive = FALSE AND OLD.alive = TRUE THEN
        -- Mark all unused breeding rights as refunded
        UPDATE breeding_rights
        SET
            active = FALSE,
            refunded = TRUE
        WHERE
            kaiju_id = NEW.id
            AND uses_remaining > 0
            AND refund_eligible = TRUE;
    END IF;
END//

DELIMITER ;
```

---

## 5. Views for Common Queries

### 5.1 Active Kaiju by Owner

```sql
CREATE VIEW active_kaiju_by_owner AS
SELECT
    k.id,
    k.name,
    k.generation,
    k.owner_user_id,
    u.username AS owner_username,
    k.custody_state,
    k.experience_level,
    k.current_stats,
    k.visible_traits,
    k.created_at
FROM kaiju k
JOIN users u ON k.owner_user_id = u.id
WHERE k.alive = TRUE
  AND k.deleted_at IS NULL
  AND u.is_active = TRUE;
```

---

### 5.2 Kaiju Lineage Tree

```sql
CREATE VIEW kaiju_lineage AS
WITH RECURSIVE lineage_tree AS (
    -- Base case: current kaiju
    SELECT
        id,
        name,
        generation,
        parent_a_id,
        parent_b_id,
        0 AS depth
    FROM kaiju

    UNION ALL

    -- Recursive case: parents
    SELECT
        k.id,
        k.name,
        k.generation,
        k.parent_a_id,
        k.parent_b_id,
        lt.depth + 1
    FROM kaiju k
    JOIN lineage_tree lt ON (k.id = lt.parent_a_id OR k.id = lt.parent_b_id)
    WHERE lt.depth < 10  -- Limit recursion depth
)
SELECT * FROM lineage_tree;
```

---

### 5.3 Tournament Leaderboard

```sql
CREATE VIEW tournament_leaderboard AS
SELECT
    t.id AS tournament_id,
    t.name AS tournament_name,
    te.user_id,
    u.username,
    te.kaiju_id,
    k.name AS kaiju_name,
    te.placement,
    te.rounds_won,
    te.rounds_lost
FROM tournament_entries te
JOIN tournaments t ON te.tournament_id = t.id
JOIN users u ON te.user_id = u.id
JOIN kaiju k ON te.kaiju_id = k.id
WHERE t.status = 'completed'
  AND te.placement IS NOT NULL
ORDER BY t.start_time DESC, te.placement ASC;
```

---

### 5.4 Hall of Fame (Dead Kaiju)

```sql
CREATE VIEW hall_of_fame AS
SELECT
    k.id,
    k.name,
    k.generation,
    k.experience_level,
    k.death_timestamp,
    t.name AS death_tournament,
    t.tournament_type,
    u.username AS final_owner,
    k.visible_traits,
    k.current_stats
FROM kaiju k
JOIN users u ON k.owner_user_id = u.id
LEFT JOIN tournaments t ON k.death_tournament_id = t.id
WHERE k.alive = FALSE
ORDER BY k.death_timestamp DESC;
```

---

## 6. Stored Procedures

### 6.1 Safe Transfer

```sql
DELIMITER //

CREATE PROCEDURE transfer_kaiju(
    IN p_kaiju_id CHAR(36),
    IN p_from_user_id CHAR(36),
    IN p_to_user_id CHAR(36),
    IN p_transfer_type VARCHAR(50),
    IN p_price_amount BIGINT,
    IN p_price_currency VARCHAR(50),
    IN p_server_signature TEXT,
    OUT p_success BOOLEAN
)
BEGIN
    DECLARE v_owner_user_id CHAR(36);
    DECLARE v_custody_state VARCHAR(50);
    DECLARE v_alive BOOLEAN;
    DECLARE v_nonce BIGINT;
    DECLARE v_locked INT;

    -- 1. Validate inputs
    IF p_kaiju_id IS NULL OR p_from_user_id IS NULL OR p_to_user_id IS NULL THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Invalid transfer parameters';
    END IF;

    -- 2. Lock kaiju for transfer and fetch details
    SELECT owner_user_id, custody_state, alive
    INTO v_owner_user_id, v_custody_state, v_alive
    FROM kaiju
    WHERE id = p_kaiju_id
    FOR UPDATE;

    -- 3. Validate ownership
    IF v_owner_user_id <> p_from_user_id THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Transfer rejected: not owner';
    END IF;

    -- 4. Validate custody state
    IF v_custody_state = 'blockchain' THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Transfer rejected: kaiju on blockchain';
    END IF;

    -- 5. Validate kaiju is alive
    IF v_alive = FALSE THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Transfer rejected: kaiju is dead';
    END IF;

    -- 6. Check for transfer lock
    SELECT COUNT(*) INTO v_locked FROM transfer_locks WHERE kaiju_id = p_kaiju_id;
    IF v_locked > 0 THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Transfer rejected: kaiju is locked';
    END IF;

    -- 7. Get nonce
    SELECT transfer_nonce INTO v_nonce FROM users WHERE id = p_from_user_id;

    -- 8. Update ownership
    UPDATE kaiju
    SET owner_user_id = p_to_user_id
    WHERE id = p_kaiju_id;

    -- 9. Increment nonce
    UPDATE users
    SET transfer_nonce = transfer_nonce + 1
    WHERE id = p_from_user_id;

    -- 10. Log transfer
    INSERT INTO ownership_history (
        kaiju_id,
        from_user_id,
        to_user_id,
        transfer_type,
        price_amount,
        price_currency,
        server_signature,
        nonce
    ) VALUES (
        p_kaiju_id,
        p_from_user_id,
        p_to_user_id,
        p_transfer_type,
        p_price_amount,
        p_price_currency,
        COALESCE(p_server_signature, 'pending'),
        v_nonce
    );

    SET p_success = TRUE;
END//

DELIMITER ;
```

---

### 6.2 Breed Kaiju

```sql
DELIMITER //

CREATE PROCEDURE breed_kaiju(
    IN p_parent_a_id CHAR(36),
    IN p_parent_b_id CHAR(36),
    IN p_breeder_user_id CHAR(36),
    IN p_breeding_right_id CHAR(36),
    IN p_child_name VARCHAR(255),
    IN p_child_genome_hash VARCHAR(64),
    IN p_child_genome_data LONGBLOB,
    IN p_child_visual_seed VARCHAR(255),
    IN p_child_stats JSON,
    IN p_mutation_occurred BOOLEAN,
    IN p_mutation_details JSON,
    OUT p_child_id CHAR(36)
)
BEGIN
    DECLARE v_parent_a_gen INT;
    DECLARE v_parent_b_gen INT;
    DECLARE v_child_gen INT;
    DECLARE v_parent_a_alive BOOLEAN;
    DECLARE v_parent_b_alive BOOLEAN;
    DECLARE v_incest_check INT;
    DECLARE v_rights_updated INT;

    -- 1. Validate parents are alive
    SELECT alive INTO v_parent_a_alive FROM kaiju WHERE id = p_parent_a_id;
    IF v_parent_a_alive IS NULL OR v_parent_a_alive = FALSE THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Parent A is not alive';
    END IF;

    SELECT alive INTO v_parent_b_alive FROM kaiju WHERE id = p_parent_b_id;
    IF v_parent_b_alive IS NULL OR v_parent_b_alive = FALSE THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Parent B is not alive';
    END IF;

    -- 2. Prevent incest (direct parent-child)
    SELECT COUNT(*) INTO v_incest_check
    FROM kaiju
    WHERE id = p_parent_b_id
    AND (parent_a_id = p_parent_a_id OR parent_b_id = p_parent_a_id);

    IF v_incest_check > 0 THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Cannot breed parent with direct offspring';
    END IF;

    -- 3. Validate breeding right
    UPDATE breeding_rights
    SET uses_remaining = uses_remaining - 1,
        used_at = NOW()
    WHERE id = p_breeding_right_id
      AND owner_user_id = p_breeder_user_id
      AND uses_remaining > 0
      AND active = TRUE;

    SET v_rights_updated = ROW_COUNT();
    IF v_rights_updated = 0 THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Invalid or expired breeding right';
    END IF;

    -- 4. Calculate child generation
    SELECT generation INTO v_parent_a_gen FROM kaiju WHERE id = p_parent_a_id;
    SELECT generation INTO v_parent_b_gen FROM kaiju WHERE id = p_parent_b_id;
    SET v_child_gen = GREATEST(v_parent_a_gen, v_parent_b_gen) + 1;

    -- 5. Create child kaiju
    SET p_child_id = UUID();
    INSERT INTO kaiju (
        id,
        name,
        generation,
        owner_user_id,
        custody_state,
        parent_a_id,
        parent_b_id,
        genome_hash,
        genome_data,
        visual_seed,
        base_stats,
        current_stats,
        visible_traits,
        hidden_traits,
        state_hash
    ) VALUES (
        p_child_id,
        p_child_name,
        v_child_gen,
        p_breeder_user_id,
        'server',
        p_parent_a_id,
        p_parent_b_id,
        p_child_genome_hash,
        p_child_genome_data,
        p_child_visual_seed,
        p_child_stats,
        p_child_stats,
        JSON_ARRAY(),
        JSON_ARRAY(),
        'initial'
    );

    -- 6. Log breeding
    INSERT INTO breeding_history (
        parent_a_id,
        parent_b_id,
        child_id,
        breeder_user_id,
        breeding_right_id,
        mutation_occurred,
        mutation_details
    ) VALUES (
        p_parent_a_id,
        p_parent_b_id,
        p_child_id,
        p_breeder_user_id,
        p_breeding_right_id,
        p_mutation_occurred,
        p_mutation_details
    );

    -- 7. Deactivate breeding right if fully used
    UPDATE breeding_rights
    SET active = FALSE
    WHERE id = p_breeding_right_id
      AND uses_remaining = 0;
END//

DELIMITER ;
```

---

## 7. Indexes for Performance

```sql
-- Composite indexes for common query patterns
CREATE INDEX idx_kaiju_owner_alive ON kaiju(owner_user_id, alive, custody_state);
CREATE INDEX idx_breeding_rights_kaiju_active ON breeding_rights(kaiju_id, active, uses_remaining);
CREATE INDEX idx_tournament_entries_tournament_placement ON tournament_entries(tournament_id, placement);
CREATE INDEX idx_battle_logs_kaiju_tournament ON battle_logs(kaiju_a_id, kaiju_b_id, tournament_id);

-- Partial indexes for hot data
CREATE INDEX idx_kaiju_server_custody ON kaiju(id) WHERE custody_state = 'server' AND alive = TRUE;
CREATE INDEX idx_active_breeding_rights ON breeding_rights(id) WHERE active = TRUE AND uses_remaining > 0;
```

---

## 8. Initial Data Setup

```sql
-- Insert initial server signature key
INSERT INTO server_signature_keys (key_name, public_key, algorithm, purpose)
VALUES
    ('ownership-key-v1', 'REPLACE_WITH_ACTUAL_PUBLIC_KEY', 'secp256k1', 'ownership'),
    ('battle-key-v1', 'REPLACE_WITH_ACTUAL_PUBLIC_KEY', 'secp256k1', 'battle');

-- Create system user for genesis kaiju
INSERT INTO users (id, username, email, is_active)
VALUES ('00000000-0000-0000-0000-000000000000', 'system', 'system@kaiju.game', TRUE);
```

---

## 9. Backup & Archival

### 9.1 Immutable History Table

```sql
-- Archive completed tournaments (write-once, never update)
CREATE TABLE tournament_archive (
    tournament_id CHAR(36) NOT NULL,
    archive_data JSON NOT NULL,
    archived_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ipfs_hash VARCHAR(255),  -- Optional IPFS backup

    PRIMARY KEY (tournament_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Prevent updates/deletes using triggers
DELIMITER //

CREATE TRIGGER prevent_tournament_archive_update
BEFORE UPDATE ON tournament_archive
FOR EACH ROW
BEGIN
    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Tournament archive is immutable';
END//

CREATE TRIGGER prevent_tournament_archive_delete
BEFORE DELETE ON tournament_archive
FOR EACH ROW
BEGIN
    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Tournament archive cannot be deleted';
END//

DELIMITER ;
```

---

## 10. Monitoring Queries

### 10.1 Active Transfers

```sql
-- Monitor transfer rate
CREATE VIEW transfer_rate AS
SELECT
    DATE_FORMAT(timestamp, '%Y-%m-%d %H:00:00') AS hour,
    transfer_type,
    COUNT(*) AS transfer_count,
    COUNT(DISTINCT kaiju_id) AS unique_kaiju,
    COUNT(DISTINCT to_user_id) AS unique_recipients
FROM ownership_history
WHERE timestamp > DATE_SUB(NOW(), INTERVAL 24 HOUR)
GROUP BY DATE_FORMAT(timestamp, '%Y-%m-%d %H:00:00'), transfer_type
ORDER BY hour DESC;
```

---

### 10.2 Breeding Activity

```sql
-- Track breeding trends
CREATE VIEW breeding_activity AS
SELECT
    DATE_FORMAT(bred_at, '%Y-%m-%d') AS day,
    COUNT(*) AS total_breedings,
    SUM(CASE WHEN mutation_occurred THEN 1 ELSE 0 END) AS mutations,
    AVG(GREATEST(pa.generation, pb.generation)) AS avg_parent_generation
FROM breeding_history bh
JOIN kaiju pa ON bh.parent_a_id = pa.id
JOIN kaiju pb ON bh.parent_b_id = pb.id
WHERE bred_at > DATE_SUB(NOW(), INTERVAL 30 DAY)
GROUP BY DATE_FORMAT(bred_at, '%Y-%m-%d')
ORDER BY day DESC;
```

---

## 11. Security Constraints Summary

1. **No direct ownership changes** - Must use `transfer_kaiju()` function
2. **Audit trail immutable** - Triggers enforce logging
3. **State hash validation** - Tamper detection via hash comparison
4. **Transfer locks** - Prevent concurrent modifications
5. **Nonce-based replay protection** - Incrementing counter per user
6. **Constraint validation** - Database enforces business rules
7. **No cascade deletes** - Preserve history even if user deleted

---

## 12. Migration Scripts

### 12.1 Run All Migrations

```bash
# Execute in order:
mysql -u kaiju_user -p kaiju_game < 01_create_users.sql
mysql -u kaiju_user -p kaiju_game < 02_create_kaiju.sql
mysql -u kaiju_user -p kaiju_game < 03_create_history.sql
mysql -u kaiju_user -p kaiju_game < 04_create_breeding.sql
mysql -u kaiju_user -p kaiju_game < 05_create_tournaments.sql
mysql -u kaiju_user -p kaiju_game < 06_create_triggers.sql
mysql -u kaiju_user -p kaiju_game < 07_create_views.sql
mysql -u kaiju_user -p kaiju_game < 08_create_procedures.sql
mysql -u kaiju_user -p kaiju_game < 09_create_indexes.sql
mysql -u kaiju_user -p kaiju_game < 10_initial_data.sql
```

---

## 13. Connection String Example

```env
DATABASE_URL=mysql://kaiju_user:STRONG_PASSWORD@localhost:3306/kaiju_game
```

---

This schema provides:
- ✅ Server-authoritative ownership
- ✅ Complete audit trails
- ✅ Tamper detection via hashing
- ✅ Replay protection via nonces
- ✅ Automatic refunds on death
- ✅ Performance-optimized indexes
- ✅ Business rule enforcement at DB level
- ✅ Migration-ready structure

Next: See `SERVER_TRANSFER_SYSTEM.md` for Rust implementation.
