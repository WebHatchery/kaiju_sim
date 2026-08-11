use super::*;

#[test]
fn test_prompt_generation() {
    let genetics = KaijuGenetics {
        body_type: "bipedal".to_string(),
        element: "fire".to_string(),
        primary_color: "dark grey".to_string(),
        secondary_color: "orange".to_string(),
        visual_traits: vec!["spiked tail".to_string(), "horns".to_string()],
    };

    let (pos, neg) = build_kaiju_prompt(&genetics);

    assert!(pos.contains("anime style"));
    assert!(pos.contains("bipedal monster"));
    assert!(pos.contains("flames"));
    assert!(pos.contains("dark grey skin"));
    assert!(pos.contains("spiked tail"));
    assert!(neg.contains("bad anatomy"));
}
