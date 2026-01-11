-- Create marketplace items table
CREATE TABLE IF NOT EXISTS marketplace_items (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description TEXT NOT NULL,
    price BIGINT NOT NULL,
    image_url VARCHAR(255) NOT NULL,
    base_stats JSON NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Seed initial items (Idempotent)
INSERT IGNORE INTO marketplace_items (id, name, description, price, image_url, base_stats) VALUES
('bipedal_neutral', 'Brutus', 'A sturdy bipedal Kaiju with balanced stats.', 500, 'http://localhost:3000/assets/kaiju/kaiju_bipedal_neutral_1768091093175.png', '{"hp": 130, "attack": 22, "defense": 18, "speed": 12, "energy": 60}'),
('quadruped_neutral', 'Stomper', 'A four-legged beast with high defense.', 600, 'http://localhost:3000/assets/kaiju/kaiju_quadruped_neutral_1768091073894.png', '{"hp": 150, "attack": 18, "defense": 25, "speed": 8, "energy": 50}'),
('serpentine_neutral', 'Slither', 'A swift serpentine Kaiju with high speed.', 550, 'http://localhost:3000/assets/kaiju/kaiju_serpentine_neutral_1768091108255.png', '{"hp": 100, "attack": 20, "defense": 12, "speed": 28, "energy": 70}');
