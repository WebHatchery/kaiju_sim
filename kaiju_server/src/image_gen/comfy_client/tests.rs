use super::*;

#[test]
fn test_workflow_structure() {
    let config = ComfyConfig::default();
    let client = ComfyClient::new(config);

    let workflow = client.build_workflow("test prompt", "bad prompt", 12345);

    assert!(workflow.get("3").is_some()); // KSampler
    assert_eq!(workflow["3"]["inputs"]["seed"], 12345);
    assert_eq!(workflow["6"]["inputs"]["text"], "test prompt");
}
