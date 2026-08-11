use super::*;

#[test]
fn test_genome_roundtrip() {
    let stats = GenomeStats {
        hp: 300,
        attack: 60,
        defense: 40,
        speed: 30,
    };

    let traits = vec![
        TraitSlot {
            trait_id: 1,
            inheritance_mode: 0,
            is_visible: true,
        },
        TraitSlot {
            trait_id: 5,
            inheritance_mode: 1,
            is_visible: false,
        },
    ];

    let genome = Genome::new(5, 2, stats, traits, HiddenTraitData::default(), 12345);

    // Test byte roundtrip
    let bytes = genome.to_bytes();
    let decoded = Genome::from_bytes(&bytes).unwrap();

    assert_eq!(decoded.header.generation, 5);
    assert_eq!(decoded.header.mutation_count, 2);
    assert_eq!(decoded.stats.hp, 300);
    assert_eq!(decoded.stats.attack, 60);
    assert_eq!(decoded.trait_slots.len(), 2);
}

#[test]
fn test_genome_hex_roundtrip() {
    let genome = Genome::new(
        3,
        1,
        GenomeStats {
            hp: 200,
            attack: 50,
            defense: 35,
            speed: 25,
        },
        vec![],
        HiddenTraitData::default(),
        99999,
    );

    let hex = genome.to_hex();
    let decoded = Genome::from_hex(&hex).unwrap();

    assert_eq!(decoded.header.generation, 3);
    assert_eq!(decoded.stats.hp, 200);
}

#[test]
fn test_checksum_validation() {
    let genome = Genome::new(
        1,
        0,
        GenomeStats::default(),
        vec![],
        HiddenTraitData::default(),
        0,
    );

    let mut bytes = genome.to_bytes();
    // Corrupt a byte
    bytes[5] ^= 0xFF;

    let result = Genome::from_bytes(&bytes);
    assert!(result.is_err());
}
