//! Prompt builder for translating Kaiju genetics to Stable Diffusion prompts.

use uuid::Uuid;

// Placeholder structs until actual logic is imported - using string representations matching our enums
#[derive(Debug, Clone)]
pub struct KaijuGenetics {
    pub body_type: String,
    pub element: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub visual_traits: Vec<String>,
}

/// Build positive and negative prompts for a Kaiju
pub fn build_kaiju_prompt(genetics: &KaijuGenetics) -> (String, String) {
    let base_style = "anime style, cel shaded, vibrant colors, masterpiece, high quality, 4k";

    // Body description
    let body_desc = match genetics.body_type.to_lowercase().as_str() {
        "quadruped" => "four legged monster, beast, wolf-like creature",
        "bipedal" => "bipedal monster, godzilla-like, standing tall, muscular",
        "serpentine" => "long dragon body, snake-like, coiled, floating",
        "winged" => "winged beast, dragon, wyvern, flying",
        "aquatic" => "sea monster, aquatic creature, fins, leviathan",
        _ => "monster creature",
    };

    // Element description
    let element_desc = match genetics.element.to_lowercase().as_str() {
        "fire" => "surrounded by flames, burning, embers, fire aura, glowing red",
        "ice" => "frozen crystals, ice spikes, cold mist, blue aura, frosty",
        "electric" => "lightning sparks, electricity, thunder, glowing yellow, energy crackles",
        "aquatic" | "water" => "water bubbles, wet skin, ocean waves, blue liquid aura",
        "nature" | "earth" => "vines, rocks, moss, earthy textures, roots",
        "wind" | "air" => "swirling wind, clouds, speed lines, floating",
        _ => "neutral energy",
    };

    // Color
    let color_desc = format!(
        "{} skin, {} accents",
        genetics.primary_color, genetics.secondary_color
    );

    // Specific traits
    let traits_desc = genetics.visual_traits.join(", ");

    // Final positive prompt
    let positive = format!(
        "{}, {}, {}, {}, {}, full body, white background, simple background",
        base_style, body_desc, element_desc, color_desc, traits_desc
    );

    // Negative prompt
    let negative = generate_negative_prompt();

    (positive, negative)
}

/// Standard negative prompt
pub fn generate_negative_prompt() -> String {
    "bad hands, humans, text, watermark, blurry, low quality, cropped, worst quality, lowres, extra limbs, missing limbs, deformed, disfigured, bad anatomy, mutation, ugly, glitch".to_string()
}

#[cfg(test)]
mod tests;
