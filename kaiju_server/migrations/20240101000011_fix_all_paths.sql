-- Add image_url to kaiju table safely
SET @exist := (SELECT count(*) FROM information_schema.columns WHERE table_schema = DATABASE() AND table_name = 'kaiju' AND column_name = 'image_url');
SET @sql := IF(@exist = 0, "ALTER TABLE kaiju ADD COLUMN image_url VARCHAR(255) NOT NULL DEFAULT 'http://localhost:3000/assets/sprites/kaiju/unknown.png'", 'SELECT "Column already exists"');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Fix paths in marketplace (column exists)
UPDATE marketplace_items 
SET image_url = REPLACE(image_url, '/assets/kaiju/', '/assets/sprites/kaiju/')
WHERE image_url LIKE '%/assets/kaiju/%';

-- Populate existing Kaiju with known images based on Name (Fallbacks)
UPDATE kaiju SET image_url = 'http://localhost:3000/assets/sprites/kaiju/kaiju_electric_elemental_1768091175509.png' WHERE name = 'Volt';
UPDATE kaiju SET image_url = 'http://localhost:3000/assets/sprites/kaiju/kaiju_ice_elemental_1768091156648.png' WHERE name = 'Glacies';
UPDATE kaiju SET image_url = 'http://localhost:3000/assets/sprites/kaiju/kaiju_bipedal_neutral_1768091093175.png' WHERE name = 'Brutus';

UPDATE marketplace_items 
SET image_url = REPLACE(image_url, '/assets/kaiju/', '/assets/sprites/kaiju/')
WHERE image_url LIKE '%/assets/kaiju/%';
