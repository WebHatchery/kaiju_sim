-- Breeding logs table for admin debugging
CREATE TABLE breeding_logs (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    event_id VARCHAR(64) NOT NULL UNIQUE,
    
    -- Parents
    parent_a_id CHAR(36) NOT NULL,
    parent_b_id CHAR(36) NOT NULL,
    
    -- Offspring
    offspring_id CHAR(36),
    offspring_generation INT,
    
    -- Log data (full JSON)
    log_data JSON NOT NULL,
    
    -- Summary fields for querying
    mutation_occurred BOOLEAN NOT NULL DEFAULT FALSE,
    mutation_type VARCHAR(64),
    trait_count_inherited INT,
    trait_count_lost INT,
    rarity VARCHAR(32),
    
    -- Materials used
    materials_used JSON DEFAULT ('[]'),
    
    -- Timing
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_breeding_logs_parent_a (parent_a_id),
    INDEX idx_breeding_logs_parent_b (parent_b_id),
    INDEX idx_breeding_logs_offspring (offspring_id),
    INDEX idx_breeding_logs_mutation (mutation_occurred),
    INDEX idx_breeding_logs_created (created_at DESC)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Breeding cooldowns table
CREATE TABLE breeding_cooldowns (
    kaiju_id CHAR(36) PRIMARY KEY,
    last_breeding_at TIMESTAMP NOT NULL,
    cooldown_ends_at TIMESTAMP NOT NULL,
    
    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Breeding materials inventory
CREATE TABLE breeding_materials (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    material_type VARCHAR(64) NOT NULL,
    quantity INT NOT NULL DEFAULT 1 CHECK (quantity >= 0),
    metadata JSON,
    
    acquired_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_materials_user (user_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
