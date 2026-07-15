//! ComfyUI HTTP Client for queueing prompts and retrieving images.
//! Based on image_gen.ps1 reference implementation.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// ComfyUI client configuration
#[derive(Debug, Clone)]
pub struct ComfyConfig {
    pub server_url: String, // e.g. "http://127.0.0.1:8188"
    pub model: String,      // e.g. "juggernautXL_v8Rundiffusion.safetensors"
    pub steps: i32,
    pub cfg: f32,
    pub width: i32,
    pub height: i32,
}

impl Default for ComfyConfig {
    fn default() -> Self {
        Self {
            server_url: "http://127.0.0.1:8188".to_string(),
            model: "juggernautXL_v8Rundiffusion.safetensors".to_string(),
            steps: 20,
            cfg: 8.0,
            width: 1024,
            height: 1024,
        }
    }
}

/// Start generation response
#[derive(Debug, Deserialize)]
struct PromptResponse {
    prompt_id: String,
    // other fields omitted
}

/// History response wrapper
#[derive(Debug, Deserialize)]
struct HistoryResponse {
    #[serde(flatten)]
    prompts: HashMap<String, HistoryData>,
}

#[derive(Debug, Deserialize)]
struct HistoryData {
    outputs: HashMap<String, NodeOutput>,
    // status: ...
}

#[derive(Debug, Deserialize)]
struct NodeOutput {
    images: Vec<ImageOutput>,
}

#[derive(Debug, Deserialize)]
struct ImageOutput {
    filename: String,
    subfolder: String,
    #[serde(rename = "type")]
    image_type: String,
}

/// ComfyUI Client
pub struct ComfyClient {
    config: ComfyConfig,
    client: reqwest::Client,
}

impl ComfyClient {
    pub fn new(config: ComfyConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Check connection to ComfyUI
    pub async fn check_connection(&self) -> bool {
        match self.client.get(&self.config.server_url).send().await {
            Ok(res) => res.status().is_success(),
            Err(_) => false,
        }
    }

    /// Queue a generation prompt
    pub async fn queue_prompt(
        &self,
        positive_prompt: &str,
        negative_prompt: &str,
        seed: i64,
    ) -> Result<String, ComfyError> {
        let client_id = Uuid::new_v4().to_string();

        // Construct workflow JSON structure based on template
        let workflow = self.build_workflow(positive_prompt, negative_prompt, seed);

        let payload = serde_json::json!({
            "prompt": workflow,
            "client_id": client_id
        });

        let url = format!("{}/prompt", self.config.server_url);
        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| ComfyError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ComfyError::RequestFailed(format!(
                "Status: {}",
                response.status()
            )));
        }

        let prompt_res: PromptResponse = response
            .json()
            .await
            .map_err(|e| ComfyError::ParseError(e.to_string()))?;

        Ok(prompt_res.prompt_id)
    }

    /// Check status and get image filename if complete
    pub async fn check_history(&self, prompt_id: &str) -> Result<Option<String>, ComfyError> {
        let url = format!("{}/history/{}", self.config.server_url, prompt_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ComfyError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            // 404 likely means strictly "not in history yet" or invalid ID
            // ComfyUI usually returns empty JSON {} if not found in history but ID valid
            return Ok(None);
        }

        let history: HashMap<String, HistoryData> = response
            .json()
            .await
            .map_err(|e| ComfyError::ParseError(e.to_string()))?;

        if let Some(data) = history.get(prompt_id) {
            // Find first image output
            for output in data.outputs.values() {
                if let Some(img) = output.images.first() {
                    return Ok(Some(img.filename.clone()));
                }
            }
        }

        Ok(None)
    }

    /// Download generated image
    pub async fn download_image(&self, filename: &str) -> Result<Vec<u8>, ComfyError> {
        let url = format!("{}/view", self.config.server_url);

        let response = self
            .client
            .get(&url)
            .query(&[("filename", filename), ("type", "output")])
            .send()
            .await
            .map_err(|e| ComfyError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ComfyError::RequestFailed(format!(
                "Download failed: {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| ComfyError::NetworkError(e.to_string()))?;

        Ok(bytes.to_vec())
    }

    /// Build the ComfyUI workflow JSON
    /// Matches the template structure in image_gen.ps1
    fn build_workflow(&self, prompt: &str, negative: &str, seed: i64) -> Value {
        serde_json::json!({
            "3": {
                "class_type": "KSampler",
                "inputs": {
                    "cfg": self.config.cfg,
                    "denoise": 1,
                    "latent_image": ["5", 0],
                    "model": ["4", 0],
                    "negative": ["7", 0],
                    "positive": ["6", 0],
                    "sampler_name": "euler",
                    "scheduler": "normal",
                    "seed": seed,
                    "steps": self.config.steps
                }
            },
            "4": {
                "class_type": "CheckpointLoaderSimple",
                "inputs": {
                    "ckpt_name": self.config.model
                }
            },
            "5": {
                "class_type": "EmptyLatentImage",
                "inputs": {
                    "batch_size": 1,
                    "height": self.config.height,
                    "width": self.config.width
                }
            },
            "6": {
                "class_type": "CLIPTextEncode",
                "inputs": {
                    "clip": ["4", 1],
                    "text": prompt
                }
            },
            "7": {
                "class_type": "CLIPTextEncode",
                "inputs": {
                    "clip": ["4", 1],
                    "text": negative
                }
            },
            "8": {
                "class_type": "VAEDecode",
                "inputs": {
                    "samples": ["3", 0],
                    "vae": ["4", 2]
                }
            },
            "9": {
                "class_type": "SaveImage",
                "inputs": {
                    "filename_prefix": "Kaiju_Server_Gen",
                    "images": ["8", 0]
                }
            }
        })
    }
}

#[derive(Debug)]
pub enum ComfyError {
    NetworkError(String),
    RequestFailed(String),
    ParseError(String),
}

impl std::fmt::Display for ComfyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::RequestFailed(e) => write!(f, "Request failed: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ComfyError {}

#[cfg(test)]
mod tests {
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
}
