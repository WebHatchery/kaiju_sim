//! Optional local ComfyUI image generation for bred kaiju.

use crate::data::Kaiju;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::blocking::Client;
#[cfg(not(target_arch = "wasm32"))]
use serde_json::Value;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
const COMFY_URL: &str = "http://127.0.0.1:8188";
#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_MODEL: &str = "juggernautXL_v8Rundiffusion.safetensors";

#[derive(Debug)]
pub enum ComfyImageError {
    Unavailable,
    Network(String),
    Response(String),
    Io(String),
}

impl std::fmt::Display for ComfyImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => write!(f, "ComfyUI unavailable"),
            Self::Network(error) => write!(f, "ComfyUI network error: {}", error),
            Self::Response(error) => write!(f, "ComfyUI response error: {}", error),
            Self::Io(error) => write!(f, "ComfyUI file error: {}", error),
        }
    }
}

impl std::error::Error for ComfyImageError {}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
struct ImageOutput {
    filename: String,
    subfolder: String,
    image_type: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn try_generate_bred_kaiju_image(
    offspring: &Kaiju,
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    output_path: &str,
) -> Result<(), ComfyImageError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .map_err(|error| ComfyImageError::Network(error.to_string()))?;

    if !is_available(&client) {
        return Err(ComfyImageError::Unavailable);
    }

    let model = checkpoint_name(&client).unwrap_or_else(|| DEFAULT_MODEL.to_string());
    let prompt = build_monster_prompt(offspring, parent_a, parent_b);
    let workflow = build_workflow(&model, &prompt, &negative_prompt(), comfy_seed(offspring));
    let prompt_id = queue_prompt(&client, workflow)?;
    let image = wait_for_image(&client, &prompt_id, Duration::from_secs(75))?;
    let bytes = download_image(&client, &image)?;

    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent).map_err(|error| ComfyImageError::Io(error.to_string()))?;
    }
    std::fs::write(output_path, bytes).map_err(|error| ComfyImageError::Io(error.to_string()))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn try_generate_bred_kaiju_image(
    _offspring: &Kaiju,
    _parent_a: &Kaiju,
    _parent_b: &Kaiju,
    _output_path: &str,
) -> Result<(), ComfyImageError> {
    Err(ComfyImageError::Unavailable)
}

#[cfg(not(target_arch = "wasm32"))]
fn is_available(client: &Client) -> bool {
    client
        .get(COMFY_URL)
        .send()
        .map(|response| response.status().is_success())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_name(client: &Client) -> Option<String> {
    let info: Value = client
        .get(format!("{}/object_info/CheckpointLoaderSimple", COMFY_URL))
        .send()
        .ok()?
        .json()
        .ok()?;

    let models = info
        .pointer("/CheckpointLoaderSimple/input/required/ckpt_name/0")
        .and_then(Value::as_array)?;

    let names = models.iter().filter_map(Value::as_str).collect::<Vec<_>>();

    names
        .iter()
        .find(|name| **name == DEFAULT_MODEL)
        .or_else(|| names.iter().find(|name| is_text_to_image_checkpoint(name)))
        .map(|name| (*name).to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn is_text_to_image_checkpoint(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let incompatible = ["hunyuan3d", "clip", "vae", "controlnet", "lora"];
    if incompatible.iter().any(|marker| lower.contains(marker)) {
        return false;
    }

    let preferred = ["juggernaut", "pony", "anime", "sdxl", "diffusion", "model"];
    preferred.iter().any(|marker| lower.contains(marker))
}

#[cfg(not(target_arch = "wasm32"))]
fn queue_prompt(client: &Client, workflow: Value) -> Result<String, ComfyImageError> {
    let payload = serde_json::json!({
        "prompt": workflow,
        "client_id": format!("kaiju-sim-{}", macroquad_toolkit::rng::random_u64())
    });

    let response: Value = client
        .post(format!("{}/prompt", COMFY_URL))
        .json(&payload)
        .send()
        .map_err(|error| ComfyImageError::Network(error.to_string()))?
        .json()
        .map_err(|error| ComfyImageError::Response(error.to_string()))?;

    response
        .get("prompt_id")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| ComfyImageError::Response("missing prompt_id".to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
fn wait_for_image(
    client: &Client,
    prompt_id: &str,
    timeout: Duration,
) -> Result<ImageOutput, ComfyImageError> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        let history: Value = client
            .get(format!("{}/history/{}", COMFY_URL, prompt_id))
            .send()
            .map_err(|error| ComfyImageError::Network(error.to_string()))?
            .json()
            .map_err(|error| ComfyImageError::Response(error.to_string()))?;

        if let Some(image) = first_image_output(&history, prompt_id) {
            return Ok(image);
        }

        std::thread::sleep(Duration::from_millis(1000));
    }

    Err(ComfyImageError::Response(
        "generation timed out".to_string(),
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn first_image_output(history: &Value, prompt_id: &str) -> Option<ImageOutput> {
    let outputs = history.get(prompt_id)?.get("outputs")?.as_object()?;
    for output in outputs.values() {
        let image = output.get("images")?.as_array()?.first()?;
        let filename = image.get("filename")?.as_str()?.to_string();
        let subfolder = image
            .get("subfolder")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let image_type = image
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("output")
            .to_string();
        return Some(ImageOutput {
            filename,
            subfolder,
            image_type,
        });
    }

    None
}

#[cfg(not(target_arch = "wasm32"))]
fn download_image(client: &Client, image: &ImageOutput) -> Result<Vec<u8>, ComfyImageError> {
    let response = client
        .get(format!("{}/view", COMFY_URL))
        .query(&[
            ("filename", image.filename.as_str()),
            ("subfolder", image.subfolder.as_str()),
            ("type", image.image_type.as_str()),
        ])
        .send()
        .map_err(|error| ComfyImageError::Network(error.to_string()))?;

    if !response.status().is_success() {
        return Err(ComfyImageError::Response(format!(
            "download failed: {}",
            response.status()
        )));
    }

    response
        .bytes()
        .map(|bytes| bytes.to_vec())
        .map_err(|error| ComfyImageError::Network(error.to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
fn build_workflow(model: &str, prompt: &str, negative: &str, seed: i64) -> Value {
    serde_json::json!({
        "3": {
            "class_type": "KSampler",
            "inputs": {
                "cfg": 7.0,
                "denoise": 1.0,
                "latent_image": ["5", 0],
                "model": ["4", 0],
                "negative": ["7", 0],
                "positive": ["6", 0],
                "sampler_name": "euler",
                "scheduler": "normal",
                "seed": seed,
                "steps": 24
            }
        },
        "4": {
            "class_type": "CheckpointLoaderSimple",
            "inputs": {
                "ckpt_name": model
            }
        },
        "5": {
            "class_type": "EmptyLatentImage",
            "inputs": {
                "batch_size": 1,
                "height": 768,
                "width": 768
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
                "filename_prefix": "kaiju_breeding",
                "images": ["8", 0]
            }
        }
    })
}

fn build_monster_prompt(offspring: &Kaiju, parent_a: &Kaiju, parent_b: &Kaiju) -> String {
    let body = body_type(offspring.visual_seed);
    let element = element_signal(offspring)
        .or_else(|| element_signal(parent_a))
        .or_else(|| element_signal(parent_b))
        .unwrap_or("storm-lit");
    let trait_text = offspring
        .traits
        .iter()
        .take(3)
        .map(|trait_def| trait_def.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "full body kaiju monster, {}, {} bio-engineered creature, classified evolution facility specimen, premium indie strategy game creature portrait, clean silhouette, dramatic lab rim light, restrained cyan glow, detailed scales and armor plates, generation {}, offspring of {} and {}, {}, dark sci-fi atmosphere, no text, no humans",
        body,
        element,
        offspring.generation,
        parent_a.name,
        parent_b.name,
        trait_text
    )
}

fn negative_prompt() -> String {
    "human, person, pilot, city text, watermark, logo, ui, blurry, cropped, low quality, extra limbs, missing limbs, deformed, bad anatomy, duplicate heads, oversaturated, neon clutter".to_string()
}

fn body_type(seed: u64) -> &'static str {
    match seed % 5 {
        0 => "bipedal titan",
        1 => "quadruped armored beast",
        2 => "serpentine leviathan",
        3 => "winged apex predator",
        _ => "heavy plated monster",
    }
}

fn element_signal(kaiju: &Kaiju) -> Option<&'static str> {
    for trait_def in &kaiju.traits {
        let name = trait_def.name.as_str();
        if name.contains("Fire") {
            return Some("fire-core");
        }
        if name.contains("Ice") || name.contains("Frost") {
            return Some("ice-core");
        }
        if name.contains("Electric") || name.contains("Swift") {
            return Some("electric-core");
        }
        if name.contains("Aqua") || name.contains("Water") {
            return Some("aquatic");
        }
        if name.contains("Armor") {
            return Some("armored");
        }
    }

    None
}

fn comfy_seed(kaiju: &Kaiju) -> i64 {
    (kaiju.visual_seed & 0x7fff_ffff_ffff_ffff) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Kaiju, KaijuStats};

    #[test]
    fn prompt_includes_breeding_context() {
        let parent_a = Kaiju::new_wild("Ignis".to_string(), KaijuStats::default(), Vec::new());
        let parent_b = Kaiju::new_wild("Volt".to_string(), KaijuStats::default(), Vec::new());
        let offspring = Kaiju::new_wild("Iglt-3".to_string(), KaijuStats::default(), Vec::new());

        let prompt = build_monster_prompt(&offspring, &parent_a, &parent_b);

        assert!(prompt.contains("kaiju monster"));
        assert!(prompt.contains("Ignis"));
        assert!(prompt.contains("Volt"));
    }
}
