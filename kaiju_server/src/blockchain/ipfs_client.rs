//! IPFS client for metadata and image uploads.

use serde::{Deserialize, Serialize};

/// IPFS upload response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpfsUploadResponse {
    pub hash: String,
    pub uri: String,
}

/// IPFS client configuration
#[derive(Debug, Clone)]
pub struct IpfsConfig {
    pub gateway_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

impl Default for IpfsConfig {
    fn default() -> Self {
        Self {
            gateway_url: "https://gateway.pinata.cloud".to_string(),
            api_key: None,
            api_secret: None,
        }
    }
}

/// IPFS client for uploads
pub struct IpfsClient {
    config: IpfsConfig,
}

impl Default for IpfsClient {
    fn default() -> Self {
        Self::new(IpfsConfig::default())
    }
}

impl IpfsClient {
    pub fn new(config: IpfsConfig) -> Self {
        Self { config }
    }

    /// Upload JSON metadata to IPFS
    pub async fn upload_json(&self, json_content: &str) -> Result<IpfsUploadResponse, IpfsError> {
        // In production, use reqwest to call Pinata API:
        // POST https://api.pinata.cloud/pinning/pinJSONToIPFS
        
        // For now, generate a mock hash
        let hash = format!("Qm{}", generate_mock_hash(json_content));
        let uri = format!("ipfs://{}", hash);

        Ok(IpfsUploadResponse { hash, uri })
    }

    /// Upload image bytes to IPFS
    pub async fn upload_image(&self, image_bytes: &[u8]) -> Result<IpfsUploadResponse, IpfsError> {
        // In production, use reqwest to call Pinata API:
        // POST https://api.pinata.cloud/pinning/pinFileToIPFS
        
        let hash = format!("Qm{}", generate_mock_hash(&format!("{:?}", image_bytes.len())));
        let uri = format!("ipfs://{}", hash);

        Ok(IpfsUploadResponse { hash, uri })
    }

    /// Get gateway URL for an IPFS hash
    pub fn get_gateway_url(&self, hash: &str) -> String {
        format!("{}/ipfs/{}", self.config.gateway_url, hash)
    }

    /// Pin an existing IPFS hash
    pub async fn pin(&self, hash: &str) -> Result<(), IpfsError> {
        // In production, call Pinata pinning endpoint
        eprintln!("Pinning hash: {}", hash);
        Ok(())
    }

    /// Unpin an IPFS hash
    pub async fn unpin(&self, hash: &str) -> Result<(), IpfsError> {
        // In production, call Pinata unpinning endpoint
        eprintln!("Unpinning hash: {}", hash);
        Ok(())
    }
}

/// Generate mock IPFS-like hash for testing
fn generate_mock_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:x}{:x}", hash, hash.rotate_left(32))
}

/// IPFS error
#[derive(Debug)]
pub enum IpfsError {
    UploadFailed(String),
    PinFailed(String),
    NetworkError(String),
    AuthenticationFailed,
}

impl std::fmt::Display for IpfsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UploadFailed(e) => write!(f, "Upload failed: {}", e),
            Self::PinFailed(e) => write!(f, "Pin failed: {}", e),
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::AuthenticationFailed => write!(f, "Authentication failed"),
        }
    }
}

impl std::error::Error for IpfsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_json() {
        let client = IpfsClient::default();
        let result = client.upload_json(r#"{"name": "test"}"#).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.uri.starts_with("ipfs://Qm"));
    }
}
