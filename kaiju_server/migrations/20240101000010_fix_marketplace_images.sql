-- Fix marketplace image paths to use /assets/sprites/kaiju/
UPDATE marketplace_items 
SET image_url = 'http://localhost:3000/assets/sprites/kaiju/kaiju_bipedal_neutral_1768091093175.png'
WHERE id = 'bipedal_neutral';

UPDATE marketplace_items 
SET image_url = 'http://localhost:3000/assets/sprites/kaiju/kaiju_quadruped_neutral_1768091073894.png'
WHERE id = 'quadruped_neutral';

UPDATE marketplace_items 
SET image_url = 'http://localhost:3000/assets/sprites/kaiju/kaiju_serpentine_neutral_1768091108255.png'
WHERE id = 'serpentine_neutral';
