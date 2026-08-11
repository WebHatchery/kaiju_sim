use super::*;

#[tokio::test]
async fn test_upload_json() {
    let client = IpfsClient::default();
    let result = client.upload_json(r#"{"name": "test"}"#).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.uri.starts_with("ipfs://Qm"));
}
