-- Breeding rights (tradeable, refundable on parent death)
DROP TABLE IF EXISTS breeding_rights;
CREATE TABLE breeding_rights (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    kaiju_id CHAR(36) NOT NULL,

    owner_user_id CHAR(36) NOT NULL,
    issuer_user_id CHAR(36) NOT NULL,

    uses_total INT NOT NULL DEFAULT 1 CHECK (uses_total > 0),
    uses_remaining INT NOT NULL CHECK (uses_remaining >= 0),

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NULL,
    used_at TIMESTAMP NULL,

    purchase_price BIGINT,
    purchase_currency VARCHAR(50),
    refund_eligible BOOLEAN NOT NULL DEFAULT TRUE,

    active BOOLEAN NOT NULL DEFAULT TRUE,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    refunded BOOLEAN NOT NULL DEFAULT FALSE,

    CONSTRAINT uses_consistency CHECK (
        (uses_remaining = 0 AND used_at IS NOT NULL) OR
        (uses_remaining > 0)
    ),
    CONSTRAINT uses_limit_consistency CHECK (
        uses_remaining <= uses_total
    ),

    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (owner_user_id) REFERENCES users(id),
    FOREIGN KEY (issuer_user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_breeding_rights_kaiju ON breeding_rights(kaiju_id, active);
CREATE INDEX idx_breeding_rights_owner ON breeding_rights(owner_user_id, active);
CREATE INDEX idx_breeding_rights_active ON breeding_rights(active, uses_remaining);
