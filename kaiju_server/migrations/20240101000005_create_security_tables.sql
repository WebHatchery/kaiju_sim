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

-- Public keys for signature verification
CREATE TABLE server_signature_keys (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    key_name VARCHAR(255) NOT NULL UNIQUE,
    public_key TEXT NOT NULL,
    algorithm VARCHAR(50) NOT NULL DEFAULT 'secp256k1',

    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rotated_at TIMESTAMP,

    purpose VARCHAR(50) NOT NULL CHECK (purpose IN ('ownership', 'battle', 'mint', 'admin'))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
