-- Complete audit log of all ownership changes
CREATE TABLE ownership_history (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,

    kaiju_id CHAR(36) NOT NULL,
    from_user_id CHAR(36),
    to_user_id CHAR(36) NOT NULL,

    transfer_type ENUM(
        'mint',
        'trade',
        'gift',
        'breeding_payment',
        'blockchain_mint',
        'blockchain_deposit',
        'admin_transfer'
    ) NOT NULL,

    price_amount BIGINT,
    price_currency VARCHAR(50),

    blockchain_tx_hash VARCHAR(66),
    blockchain_confirmed BOOLEAN DEFAULT FALSE,

    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    server_signature TEXT NOT NULL,
    nonce BIGINT NOT NULL,

    metadata JSON DEFAULT ('{}'),

    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (from_user_id) REFERENCES users(id),
    FOREIGN KEY (to_user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_ownership_history_kaiju ON ownership_history(kaiju_id);
CREATE INDEX idx_ownership_history_from_user ON ownership_history(from_user_id);
CREATE INDEX idx_ownership_history_to_user ON ownership_history(to_user_id);
CREATE INDEX idx_ownership_history_timestamp ON ownership_history(timestamp DESC);
CREATE INDEX idx_ownership_history_type ON ownership_history(transfer_type);
