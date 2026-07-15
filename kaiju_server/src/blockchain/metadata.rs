//! NFT metadata for OpenSea/marketplace compatibility.

use serde::{Deserialize, Serialize};

/// ERC-721 compatible metadata (OpenSea standard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: String,
    pub attributes: Vec<Attribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    pub properties: KaijuProperties,
}

/// OpenSea attribute format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: AttributeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_type: Option<String>,
}

/// Attribute value (string or number)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Number(i64),
    Float(f64),
}

/// Kaiju-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaijuProperties {
    pub genome_hash: String,
    pub visual_seed: String,
    pub generation: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_a_token: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_b_token: Option<u64>,
    pub created_at: i64,
    pub alive: bool,
}

impl NftMetadata {
    /// Create metadata for a kaiju
    #[allow(clippy::too_many_arguments)] // mirrors the many independent NFT metadata fields; a param struct would just move the same fields
    pub fn new(
        token_id: u64,
        name: String,
        generation: u32,
        genome_hash: String,
        visual_seed: u64,
        stats: &KaijuStats,
        traits: &[String],
        image_ipfs: String,
        parent_tokens: Option<(u64, u64)>,
        created_at: i64,
        alive: bool,
    ) -> Self {
        let mut attributes = vec![
            Attribute {
                trait_type: "Generation".to_string(),
                value: AttributeValue::Number(generation as i64),
                display_type: Some("number".to_string()),
            },
            Attribute {
                trait_type: "HP".to_string(),
                value: AttributeValue::Number(stats.hp as i64),
                display_type: Some("number".to_string()),
            },
            Attribute {
                trait_type: "Attack".to_string(),
                value: AttributeValue::Number(stats.attack as i64),
                display_type: Some("number".to_string()),
            },
            Attribute {
                trait_type: "Defense".to_string(),
                value: AttributeValue::Number(stats.defense as i64),
                display_type: Some("number".to_string()),
            },
            Attribute {
                trait_type: "Speed".to_string(),
                value: AttributeValue::Number(stats.speed as i64),
                display_type: Some("number".to_string()),
            },
        ];

        // Add traits as attributes
        for t in traits {
            attributes.push(Attribute {
                trait_type: "Trait".to_string(),
                value: AttributeValue::String(t.clone()),
                display_type: None,
            });
        }

        // Add alive status
        attributes.push(Attribute {
            trait_type: "Status".to_string(),
            value: AttributeValue::String(if alive { "Alive" } else { "Deceased" }.to_string()),
            display_type: None,
        });

        let description = format!(
            "Generation {} Kaiju. {}",
            generation,
            if alive {
                "Battle-ready."
            } else {
                "Fallen in battle. Forever remembered."
            }
        );

        Self {
            name: format!("{} #{}", name, token_id),
            description,
            image: image_ipfs,
            external_url: format!("https://kaiju.game/kaiju/{}", token_id),
            attributes,
            animation_url: None,
            background_color: None,
            properties: KaijuProperties {
                genome_hash,
                visual_seed: visual_seed.to_string(),
                generation,
                parent_a_token: parent_tokens.map(|(a, _)| a),
                parent_b_token: parent_tokens.map(|(_, b)| b),
                created_at,
                alive,
            },
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Stats for metadata
#[derive(Debug, Clone)]
pub struct KaijuStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let stats = KaijuStats {
            hp: 300,
            attack: 50,
            defense: 40,
            speed: 60,
        };
        let metadata = NftMetadata::new(
            1234,
            "Volthor".to_string(),
            3,
            "abc123def456".to_string(),
            12345678,
            &stats,
            &["Electric Breath".to_string(), "Armored Scales".to_string()],
            "ipfs://QmXxxx".to_string(),
            Some((100, 200)),
            1700000000,
            true,
        );

        assert_eq!(metadata.name, "Volthor #1234");
        assert!(metadata.attributes.len() >= 5);

        let json = metadata.to_json().unwrap();
        assert!(json.contains("Volthor"));
    }
}
