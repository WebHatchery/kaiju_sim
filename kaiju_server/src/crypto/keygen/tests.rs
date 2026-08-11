use super::*;

#[test]
fn test_key_generation() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);

    // Verify key can be serialized and deserialized
    let hex = key_pair.secret_key_hex();
    let restored = ServerKeyPair::from_hex(&hex, KeyPurpose::Ownership).unwrap();

    assert_eq!(key_pair.public_key_hex(), restored.public_key_hex());
}

#[test]
fn test_eth_address_format() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);
    let address = key_pair.eth_address();

    assert!(address.starts_with("0x"));
    assert_eq!(address.len(), 42); // 0x + 40 hex chars
}

#[test]
fn test_generate_all_keys() {
    let keys = generate_all_keys();
    assert_eq!(keys.len(), 4);
    assert_eq!(keys[0].purpose, KeyPurpose::Ownership);
    assert_eq!(keys[1].purpose, KeyPurpose::Battle);
    assert_eq!(keys[2].purpose, KeyPurpose::Mint);
    assert_eq!(keys[3].purpose, KeyPurpose::Admin);
}
