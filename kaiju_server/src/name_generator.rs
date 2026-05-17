//! Random Kaiju Name Generator
//! Generates unique names by combining prefixes and suffixes.

use rand::seq::SliceRandom;
use rand::Rng;

const PREFIXES: &[&str] = &[
    "Blaze", "Frost", "Shadow", "Storm", "Thunder", "Volt", "Crystal", "Ember", "Iron", "Steel",
    "Obsidian", "Crimson", "Azure", "Jade", "Onyx", "Ruby", "Venom", "Fang", "Claw", "Spike",
    "Razor", "Titan", "Chaos", "Primal", "Ancient", "Savage", "Feral", "Grim", "Dark", "Bright",
    "Moon", "Sun", "Night", "Dawn", "Dusk", "Star", "Comet", "Nova", "Void", "Apex",
];

const SUFFIXES: &[&str] = &[
    "maw", "claw", "fang", "spine", "horn", "talon", "scale", "wing", "bite", "strike", "crush",
    "roar", "howl", "fury", "rage", "wrath", "storm", "flame", "frost", "shock", "quake", "surge",
    "blast", "rend", "tear", "hunter", "stalker", "reaper", "bringer", "lord", "king", "beast",
    "tooth", "heart", "soul", "core", "bane", "slayer", "crusher", "render",
];

const SINGLE_NAMES: &[&str] = &[
    "Brutus",
    "Goliath",
    "Titan",
    "Behemoth",
    "Leviathan",
    "Kraken",
    "Hydra",
    "Phoenix",
    "Drake",
    "Wyrm",
    "Chimera",
    "Griffin",
    "Cerberus",
    "Basilisk",
    "Zephyr",
    "Tempest",
    "Cyclone",
    "Inferno",
    "Glacier",
    "Tremor",
    "Cascade",
    "Nexus",
    "Apex",
    "Omega",
    "Alpha",
    "Prime",
    "Zenith",
    "Eclipse",
    "Nebula",
];

/// Generate a random unique Kaiju name
pub fn generate_kaiju_name() -> String {
    let mut rng = rand::thread_rng();

    // 30% chance of single name, 70% chance of prefix+suffix
    if rng.gen_bool(0.3) {
        SINGLE_NAMES.choose(&mut rng).unwrap().to_string()
    } else {
        let prefix = PREFIXES.choose(&mut rng).unwrap();
        let suffix = SUFFIXES.choose(&mut rng).unwrap();
        format!("{}{}", prefix, suffix)
    }
}

/// Generate a name with a specific seed for reproducibility
pub fn generate_kaiju_name_seeded(seed: u64) -> String {
    use rand::SeedableRng;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    if rng.gen_bool(0.3) {
        SINGLE_NAMES.choose(&mut rng).unwrap().to_string()
    } else {
        let prefix = PREFIXES.choose(&mut rng).unwrap();
        let suffix = SUFFIXES.choose(&mut rng).unwrap();
        format!("{}{}", prefix, suffix)
    }
}
