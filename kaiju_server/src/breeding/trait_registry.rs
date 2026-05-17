//! Trait Registry
//! Loads and manages trait definitions and synergy data.

use super::{InheritanceType, SynergyDefinition, TraitCategory, TraitCondition, TraitDefinition};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static DEFAULT_TRAITS: Lazy<Vec<TraitDefinition>> = Lazy::new(|| {
    vec![
        // Element Traits
        TraitDefinition {
            id: "E01".into(),
            name: "Electric Breath".into(),
            description: "+10 damage, +15% in Storm".into(),
            category: TraitCategory::Element,
            inheritance_type: InheritanceType::Dominant,
            power: 10,
            is_hidden: false,
            visual_keywords: vec!["lightning".into(), "electricity".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "E02".into(),
            name: "Flame Core".into(),
            description: "+12 damage, +20% in Volcanic".into(),
            category: TraitCategory::Element,
            inheritance_type: InheritanceType::Dominant,
            power: 12,
            is_hidden: false,
            visual_keywords: vec!["molten glow".into(), "flames".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "E03".into(),
            name: "Aqua Hide".into(),
            description: "+8 damage, +15% in Ocean".into(),
            category: TraitCategory::Element,
            inheritance_type: InheritanceType::Dominant,
            power: 8,
            is_hidden: false,
            visual_keywords: vec!["water shimmer".into(), "scales".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "E04".into(),
            name: "Frost Aura".into(),
            description: "+9 damage, slow enemy 5%".into(),
            category: TraitCategory::Element,
            inheritance_type: InheritanceType::Dominant,
            power: 9,
            is_hidden: false,
            visual_keywords: vec!["ice crystals".into(), "frost".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "E05".into(),
            name: "Toxic Breath".into(),
            description: "+11 damage, DoT 3/turn".into(),
            category: TraitCategory::Element,
            inheritance_type: InheritanceType::Recessive,
            power: 11,
            is_hidden: false,
            visual_keywords: vec!["poison drip".into(), "toxic".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        // Modifier Traits
        TraitDefinition {
            id: "M01".into(),
            name: "Armored Hide".into(),
            description: "+15 defense".into(),
            category: TraitCategory::Modifier,
            inheritance_type: InheritanceType::Dominant,
            power: 15,
            is_hidden: false,
            visual_keywords: vec!["thick plates".into(), "armor".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "M02".into(),
            name: "Regeneration".into(),
            description: "+5 HP per turn".into(),
            category: TraitCategory::Modifier,
            inheritance_type: InheritanceType::Recessive,
            power: 10,
            is_hidden: true,
            visual_keywords: vec!["healing aura".into()],
            condition: Some(TraitCondition::HpBelow(50)),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "M03".into(),
            name: "Berserker Rage".into(),
            description: "+18 attack, -10 defense".into(),
            category: TraitCategory::Modifier,
            inheritance_type: InheritanceType::Recessive,
            power: 18,
            is_hidden: true,
            visual_keywords: vec!["red glow".into(), "rage".into()],
            condition: Some(TraitCondition::HpBelow(30)),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "M04".into(),
            name: "Speed Boost".into(),
            description: "+12 speed".into(),
            category: TraitCategory::Modifier,
            inheritance_type: InheritanceType::Dominant,
            power: 12,
            is_hidden: false,
            visual_keywords: vec!["streamlined".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "M06".into(),
            name: "Endurance".into(),
            description: "+20% max HP".into(),
            category: TraitCategory::Modifier,
            inheritance_type: InheritanceType::Polygenic { threshold: 4 },
            power: 20,
            is_hidden: false,
            visual_keywords: vec!["robust build".into()],
            condition: Some(TraitCondition::Always),
            polygenic_points: 0,
        },
        TraitDefinition {
            id: "M10".into(),
            name: "Camouflage".into(),
            description: "Dodge first attack".into(),
            category: TraitCategory::Modifier,
            inheritance_type: InheritanceType::Recessive,
            power: 8,
            is_hidden: true,
            visual_keywords: vec!["adaptive skin".into()],
            condition: Some(TraitCondition::FirstTurn),
            polygenic_points: 0,
        },
    ]
});

pub static DEFAULT_SYNERGIES: Lazy<Vec<SynergyDefinition>> = Lazy::new(|| {
    vec![
        SynergyDefinition {
            id: "S01".into(),
            name: "Storm Dragon".into(),
            required_traits: vec!["E01".into(), "M04".into()],
            power: 20,
            activation_chance: 1.0,
            effect: "+20 damage in Storm".into(),
            visual_keywords: vec!["lightning wings".into()],
        },
        SynergyDefinition {
            id: "S02".into(),
            name: "Volcanic Titan".into(),
            required_traits: vec!["E02".into(), "M01".into()],
            power: 25,
            activation_chance: 1.0,
            effect: "+25% all stats in Volcanic".into(),
            visual_keywords: vec!["lava-plated".into()],
        },
        SynergyDefinition {
            id: "S03".into(),
            name: "Deep Sea Horror".into(),
            required_traits: vec!["E03".into(), "M10".into()],
            power: 18,
            activation_chance: 1.0,
            effect: "+18 defense in Ocean".into(),
            visual_keywords: vec!["abyssal form".into()],
        },
        SynergyDefinition {
            id: "S05".into(),
            name: "Toxic Regenerator".into(),
            required_traits: vec!["E05".into(), "M02".into()],
            power: 15,
            activation_chance: 1.0,
            effect: "Heal 10 HP/turn, poison enemy".into(),
            visual_keywords: vec!["oozing heal".into()],
        },
    ]
});

pub struct TraitRegistry {
    traits: HashMap<String, TraitDefinition>,
    synergies: Vec<SynergyDefinition>,
}

impl TraitRegistry {
    pub fn new() -> Self {
        let mut traits = HashMap::new();
        for t in DEFAULT_TRAITS.iter() {
            traits.insert(t.id.clone(), t.clone());
        }
        Self {
            traits,
            synergies: DEFAULT_SYNERGIES.clone(),
        }
    }

    pub fn get_trait(&self, id: &str) -> Option<&TraitDefinition> {
        self.traits.get(id)
    }
    pub fn get_all_traits(&self) -> Vec<&TraitDefinition> {
        self.traits.values().collect()
    }
    pub fn get_synergies(&self) -> &[SynergyDefinition] {
        &self.synergies
    }
}

impl Default for TraitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
