use super::*;

#[test]
fn test_create_and_verify_export() {
    let service = ExportService::new("test_key");

    let package = service.create_export(
        Uuid::new_v4(),
        "TestKaiju".to_string(),
        2,
        "abc123".to_string(),
        12345,
        ExportedStats {
            hp: 300,
            attack: 50,
            defense: 40,
            speed: 60,
        },
        vec!["Fire Breath".to_string()],
        5,
        Utc::now(),
        None,
        Uuid::new_v4(),
        true,
    );

    let result = service.verify_export(&package);
    assert!(matches!(result, VerificationResult::Valid { .. }));
}

#[test]
fn test_tampered_export() {
    let service = ExportService::new("test_key");

    let mut package = service.create_export(
        Uuid::new_v4(),
        "TestKaiju".to_string(),
        2,
        "abc123".to_string(),
        12345,
        ExportedStats {
            hp: 300,
            attack: 50,
            defense: 40,
            speed: 60,
        },
        vec![],
        5,
        Utc::now(),
        None,
        Uuid::new_v4(),
        true,
    );

    // Tamper with data
    package.data.stats.hp = 9999;

    let result = service.verify_export(&package);
    assert!(matches!(
        result,
        VerificationResult::TamperedData | VerificationResult::GenomeMismatch
    ));
}

#[test]
fn test_json_roundtrip() {
    let service = ExportService::new("test_key");

    let package = service.create_export(
        Uuid::new_v4(),
        "TestKaiju".to_string(),
        1,
        "def456".to_string(),
        54321,
        ExportedStats {
            hp: 200,
            attack: 40,
            defense: 30,
            speed: 50,
        },
        vec!["Ice Breath".to_string()],
        3,
        Utc::now(),
        None,
        Uuid::new_v4(),
        true,
    );

    let json = service.to_json(&package).unwrap();
    let restored = service.from_json(&json).unwrap();

    assert!(matches!(
        service.verify_export(&restored),
        VerificationResult::Valid { .. }
    ));
}
