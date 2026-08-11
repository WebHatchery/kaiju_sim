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
