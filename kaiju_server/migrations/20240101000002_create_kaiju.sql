-- Kaiju entities
CREATE TABLE kaiju (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),

    name VARCHAR(255) NOT NULL,
    generation INT NOT NULL CHECK (generation >= 0),

    owner_user_id CHAR(36) NOT NULL,
    custody_state ENUM('server', 'blockchain') NOT NULL,
    blockchain_token_id BIGINT UNIQUE,
    blockchain_contract_address VARCHAR(255),

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    parent_a_id CHAR(36),
    parent_b_id CHAR(36),
    genome_hash VARCHAR(64) NOT NULL,
    genome_data LONGBLOB NOT NULL,
    visual_seed VARCHAR(255) NOT NULL,

    base_stats JSON NOT NULL,
    current_stats JSON NOT NULL,

    visible_traits JSON NOT NULL DEFAULT ('[]'),
    hidden_traits JSON NOT NULL DEFAULT ('[]'),

    experience_level INT NOT NULL DEFAULT 0 CHECK (experience_level >= 0),
    experience_points BIGINT NOT NULL DEFAULT 0 CHECK (experience_points >= 0),
    training_points INT NOT NULL DEFAULT 0,

    alive BOOLEAN NOT NULL DEFAULT TRUE,
    death_timestamp TIMESTAMP NULL,
    death_tournament_id CHAR(36),

    genome_decode_level INT NOT NULL DEFAULT 0,
    research_data JSON DEFAULT ('{}'),

    state_hash VARCHAR(64) NOT NULL,
    state_version INT NOT NULL DEFAULT 1,

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
    FOREIGN KEY (parent_b_id) REFERENCES kaiju(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE INDEX idx_kaiju_owner ON kaiju(owner_user_id, alive, deleted_at);
CREATE INDEX idx_kaiju_alive ON kaiju(alive, deleted_at);
CREATE INDEX idx_kaiju_generation ON kaiju(generation, alive);
CREATE INDEX idx_kaiju_blockchain_token ON kaiju(blockchain_token_id);
CREATE INDEX idx_kaiju_created_at ON kaiju(created_at DESC);
CREATE INDEX idx_kaiju_parents ON kaiju(parent_a_id, parent_b_id);
